use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{Duration, SystemTime},
};

use anyhow::{Context as _, Result, bail, ensure};
use directories::ProjectDirs;
use futures::StreamExt as _;
use reqwest::{
    Client,
    header::{ACCEPT, HeaderMap, HeaderValue, USER_AGENT},
    redirect::Policy,
};
use semver::Version;
use serde::Deserialize;
use sha2::{Digest as _, Sha256};
use tokio::io::AsyncWriteExt as _;

const REPOSITORY: &str = "markbang/memos-desktop";
const LATEST_RELEASE_URL: &str =
    "https://api.github.com/repos/markbang/memos-desktop/releases/latest";
const UPDATE_DIRECTORY_PREFIX: &str = "memos-desktop-update-";
const COMPLETED_UPDATE_MARKER: &str = "completed-update";
const MAX_RELEASE_METADATA_BYTES: usize = 1024 * 1024;
const MAX_CHECKSUMS_BYTES: usize = 1024 * 1024;
const MAX_INSTALLER_BYTES: usize = 256 * 1024;
const NETWORK_TIMEOUT: Duration = Duration::from_secs(30);
const STALE_UPDATE_AGE: Duration = Duration::from_secs(24 * 60 * 60);

pub const INITIAL_CHECK_DELAY: Duration = Duration::from_secs(5);
pub const POLL_INTERVAL: Duration = Duration::from_secs(60 * 60);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Release {
    pub version: Version,
    pub tag: String,
    pub html_url: String,
    installer_url: String,
    checksums_url: String,
    installer_digest: Option<String>,
    checksums_digest: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Installation {
    ManagedLinux { install_root: PathBuf },
    ManagedMacOs { applications_dir: PathBuf },
    External { reason: String },
}

impl Installation {
    pub fn external_reason(&self) -> Option<&str> {
        match self {
            Self::External { reason } => Some(reason),
            Self::ManagedLinux { .. } | Self::ManagedMacOs { .. } => None,
        }
    }

    pub fn restart_path(&self) -> Option<PathBuf> {
        match self {
            Self::ManagedLinux { install_root } => Some(
                install_root
                    .join("lib")
                    .join("memos-desktop")
                    .join("memos-desktop"),
            ),
            Self::ManagedMacOs { applications_dir } => Some(
                applications_dir
                    .join("Memos Desktop.app")
                    .join("Contents")
                    .join("MacOS")
                    .join("memos-desktop"),
            ),
            Self::External { .. } => None,
        }
    }

    fn is_managed(&self) -> bool {
        !matches!(self, Self::External { .. })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AutoUpdateStatus {
    Idle,
    Checking,
    Downloading { release: Release },
    Installing { release: Release },
    ReadyToRestart { release: Release },
    Available { release: Release, reason: String },
    UpToDate,
    Updated { version: Version, html_url: String },
    Errored { message: String },
}

impl AutoUpdateStatus {
    pub fn is_busy(&self) -> bool {
        matches!(
            self,
            Self::Checking | Self::Downloading { .. } | Self::Installing { .. }
        )
    }

    pub fn release_url(&self) -> Option<&str> {
        match self {
            Self::Downloading { release }
            | Self::Installing { release }
            | Self::ReadyToRestart { release }
            | Self::Available { release, .. } => Some(&release.html_url),
            Self::Updated { html_url, .. } => Some(html_url),
            Self::Idle | Self::Checking | Self::UpToDate | Self::Errored { .. } => None,
        }
    }

    pub fn label(&self) -> String {
        match self {
            Self::Idle => "Ready".into(),
            Self::Checking => "Checking for updates...".into(),
            Self::Downloading { release } => {
                format!("Downloading Memos Desktop {}...", release.version)
            }
            Self::Installing { release } => {
                format!("Installing Memos Desktop {}...", release.version)
            }
            Self::ReadyToRestart { release } => {
                format!("Version {} is ready to restart", release.version)
            }
            Self::Available { release, .. } => {
                format!("Version {} is available", release.version)
            }
            Self::UpToDate => "Memos Desktop is up to date".into(),
            Self::Updated { version, .. } => format!("Updated to Memos Desktop {version}"),
            Self::Errored { .. } => "Update failed".into(),
        }
    }
}

#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
    html_url: String,
    draft: bool,
    prerelease: bool,
    assets: Vec<GitHubAsset>,
}

#[derive(Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
    digest: Option<String>,
}

pub fn current_version() -> Version {
    Version::parse(env!("CARGO_PKG_VERSION")).expect("package version must be valid SemVer")
}

pub fn release_url_for_version(version: &Version) -> String {
    format!("https://github.com/{REPOSITORY}/releases/tag/v{version}")
}

pub fn record_completed_update(version: &Version) -> Result<()> {
    record_completed_update_at(&completed_update_marker_path()?, version)
}

pub fn take_completed_update(current_version: &Version) -> Result<Option<Version>> {
    take_completed_update_at(&completed_update_marker_path()?, current_version)
}

pub async fn check_for_update(current: &Version) -> Result<Option<Release>> {
    let client = http_client()?;
    let body = fetch_bytes(
        &client,
        LATEST_RELEASE_URL,
        MAX_RELEASE_METADATA_BYTES,
        "latest release metadata",
    )
    .await?;
    let release: GitHubRelease =
        serde_json::from_slice(&body).context("latest release metadata is not valid JSON")?;
    let release = parse_release(release)?;
    Ok((release.version > *current).then_some(release))
}

pub struct PreparedInstaller {
    directory: PathBuf,
    script_path: PathBuf,
    checksums_path: PathBuf,
}

impl Drop for PreparedInstaller {
    fn drop(&mut self) {
        _ = fs::remove_dir_all(&self.directory);
    }
}

pub async fn download_verified_installer(release: &Release) -> Result<PreparedInstaller> {
    ensure!(
        release.installer_url.starts_with("https://github.com/"),
        "installer URL is not a trusted GitHub HTTPS URL"
    );
    ensure!(
        release.checksums_url.starts_with("https://github.com/"),
        "checksum URL is not a trusted GitHub HTTPS URL"
    );

    let client = http_client()?;
    let (installer, checksums) = tokio::try_join!(
        fetch_bytes(
            &client,
            &release.installer_url,
            MAX_INSTALLER_BYTES,
            "update installer",
        ),
        fetch_bytes(
            &client,
            &release.checksums_url,
            MAX_CHECKSUMS_BYTES,
            "release checksums",
        ),
    )?;

    if let Some(checksums_digest) = release.checksums_digest.as_deref() {
        ensure!(
            sha256_hex(&checksums) == checksums_digest,
            "SHA256SUMS does not match GitHub's asset digest"
        );
    }
    let expected = checksum_for_asset(&checksums, "install.sh")?;
    let actual = sha256_hex(&installer);
    ensure!(
        actual == expected,
        "update installer checksum does not match SHA256SUMS"
    );
    if let Some(asset_digest) = release.installer_digest.as_deref() {
        ensure!(
            actual == asset_digest,
            "update installer checksum does not match GitHub's asset digest"
        );
    }

    let directory = create_update_directory().await?;
    let script_path = directory.join("install.sh");
    let checksums_path = directory.join("SHA256SUMS");
    let write_result = async {
        write_new_file(&script_path, &installer).await?;
        write_new_file(&checksums_path, &checksums).await?;
        anyhow::Ok(())
    }
    .await;
    if let Err(error) = write_result {
        _ = tokio::fs::remove_dir_all(&directory).await;
        return Err(error);
    }

    Ok(PreparedInstaller {
        directory,
        script_path,
        checksums_path,
    })
}

pub async fn install_update(
    prepared: PreparedInstaller,
    release: &Release,
    installation: &Installation,
) -> Result<()> {
    ensure!(
        installation.is_managed(),
        "this installation is externally managed"
    );

    let script_path = prepared.script_path.clone();
    let checksums_path = prepared.checksums_path.clone();
    let tag = release.tag.clone();
    let installation = installation.clone();
    tokio::task::spawn_blocking(move || {
        let _prepared = prepared;
        let mut command = Command::new("/bin/sh");
        command
            .arg(script_path)
            .arg("--update")
            .arg("--version")
            .arg(tag)
            .env_remove("MEMOS_DESKTOP_ARCHIVE_PATH")
            .env("MEMOS_DESKTOP_CHECKSUMS_PATH", checksums_path)
            .env_remove("MEMOS_DESKTOP_RESOLVED_VERSION")
            .env_remove("MEMOS_DESKTOP_PLATFORM")
            .env_remove("MEMOS_DESKTOP_ARCH");

        match installation {
            Installation::ManagedLinux { install_root } => {
                command.env("MEMOS_DESKTOP_INSTALL_ROOT", install_root);
            }
            Installation::ManagedMacOs { applications_dir } => {
                command.env("MEMOS_DESKTOP_APPLICATIONS_DIR", applications_dir);
            }
            Installation::External { .. } => unreachable!("validated above"),
        }

        let output = command
            .output()
            .context("could not start the update installer")?;
        if !output.status.success() {
            let stderr = output_snippet(&output.stderr);
            let stdout = output_snippet(&output.stdout);
            bail!(
                "installer exited with {}{}{}",
                output.status,
                if stderr.is_empty() { "" } else { ": " },
                if stderr.is_empty() { &stdout } else { &stderr }
            );
        }
        Ok(())
    })
    .await
    .context("update installer task stopped unexpectedly")?
}

pub fn detect_installation() -> Installation {
    match std::env::current_exe() {
        Ok(executable) => detect_installation_from_executable(
            &executable,
            std::env::consts::OS,
            std::env::var_os("HOME").as_deref().map(Path::new),
        ),
        Err(error) => Installation::External {
            reason: format!("Could not locate the running executable: {error}"),
        },
    }
}

fn detect_installation_from_executable(
    executable: &Path,
    os: &str,
    home_directory: Option<&Path>,
) -> Installation {
    match os {
        "linux" => {
            let Some(app_directory) = executable.parent() else {
                return external_installation();
            };
            let Some(lib_directory) = app_directory.parent() else {
                return external_installation();
            };
            let Some(install_root) = lib_directory.parent() else {
                return external_installation();
            };
            let installer_managed = app_directory.join(".installer-managed").is_file()
                || home_directory.is_some_and(|home| install_root == home.join(".local"));
            if executable.file_name() == Some(OsStr::new("memos-desktop"))
                && app_directory.file_name() == Some(OsStr::new("memos-desktop"))
                && lib_directory.file_name() == Some(OsStr::new("lib"))
                && installer_managed
            {
                Installation::ManagedLinux {
                    install_root: install_root.to_path_buf(),
                }
            } else {
                external_installation()
            }
        }
        "macos" => {
            let Some(mac_os_directory) = executable.parent() else {
                return external_installation();
            };
            let Some(contents_directory) = mac_os_directory.parent() else {
                return external_installation();
            };
            let Some(app_bundle) = contents_directory.parent() else {
                return external_installation();
            };
            let Some(applications_dir) = app_bundle.parent() else {
                return external_installation();
            };
            let installer_managed = contents_directory
                .join("Resources")
                .join(".installer-managed")
                .is_file()
                || home_directory.is_some_and(|home| applications_dir == home.join("Applications"));
            if executable.file_name() == Some(OsStr::new("memos-desktop"))
                && mac_os_directory.file_name() == Some(OsStr::new("MacOS"))
                && contents_directory.file_name() == Some(OsStr::new("Contents"))
                && app_bundle.file_name() == Some(OsStr::new("Memos Desktop.app"))
                && installer_managed
            {
                Installation::ManagedMacOs {
                    applications_dir: applications_dir.to_path_buf(),
                }
            } else {
                external_installation()
            }
        }
        "windows" => Installation::External {
            reason: "Windows updates require a signed installer; download the release archive"
                .into(),
        },
        _ => Installation::External {
            reason: format!("Automatic installation is not supported on {os}"),
        },
    }
}

pub fn cleanup_stale_update_directories() {
    let Ok(entries) = fs::read_dir(std::env::temp_dir()) else {
        return;
    };
    let now = SystemTime::now();
    for entry in entries.flatten() {
        if !entry
            .file_name()
            .to_string_lossy()
            .starts_with(UPDATE_DIRECTORY_PREFIX)
        {
            continue;
        }
        let stale = entry
            .metadata()
            .ok()
            .filter(|metadata| metadata.is_dir())
            .and_then(|metadata| metadata.modified().ok())
            .and_then(|modified| now.duration_since(modified).ok())
            .is_some_and(|age| age > STALE_UPDATE_AGE);
        if stale {
            _ = fs::remove_dir_all(entry.path());
        }
    }
}

fn external_installation() -> Installation {
    Installation::External {
        reason: "This copy is managed by a package manager or was built from source".into(),
    }
}

fn completed_update_marker_path() -> Result<PathBuf> {
    ProjectDirs::from("com", "Memos Desktop", "Memos Desktop")
        .map(|directories| directories.config_dir().join(COMPLETED_UPDATE_MARKER))
        .context("could not locate the application configuration directory")
}

fn record_completed_update_at(path: &Path, version: &Version) -> Result<()> {
    let parent = path
        .parent()
        .context("completed update marker has no parent directory")?;
    fs::create_dir_all(parent).with_context(|| format!("could not create {}", parent.display()))?;
    let temporary = path.with_extension(format!("tmp-{:016x}", rand::random::<u64>()));
    if let Err(error) = fs::write(&temporary, format!("{version}\n")) {
        _ = fs::remove_file(&temporary);
        return Err(error).with_context(|| format!("could not write {}", temporary.display()));
    }
    match fs::rename(&temporary, path) {
        Ok(()) => Ok(()),
        Err(error) if !path.is_file() => {
            _ = fs::remove_file(&temporary);
            Err(error).with_context(|| format!("could not replace {}", path.display()))
        }
        Err(_) => {
            if let Err(error) = fs::remove_file(path) {
                _ = fs::remove_file(&temporary);
                return Err(error).with_context(|| format!("could not replace {}", path.display()));
            }
            if let Err(error) = fs::rename(&temporary, path) {
                _ = fs::remove_file(&temporary);
                return Err(error).with_context(|| format!("could not replace {}", path.display()));
            }
            Ok(())
        }
    }
}

fn take_completed_update_at(path: &Path, current_version: &Version) -> Result<Option<Version>> {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(error).with_context(|| format!("could not read {}", path.display()));
        }
    };
    let pending_version = match Version::parse(content.trim()) {
        Ok(version) => version,
        Err(error) => {
            _ = fs::remove_file(path);
            return Err(error).context("completed update marker is not valid SemVer");
        }
    };
    if pending_version > *current_version {
        return Ok(None);
    }
    fs::remove_file(path).with_context(|| format!("could not remove {}", path.display()))?;
    Ok(Some(current_version.clone()))
}

fn http_client() -> Result<Client> {
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/vnd.github+json"),
    );
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("memos-desktop-auto-update"),
    );
    Client::builder()
        .default_headers(headers)
        .redirect(Policy::limited(5))
        .timeout(NETWORK_TIMEOUT)
        .build()
        .context("could not create update HTTP client")
}

async fn fetch_bytes(client: &Client, url: &str, limit: usize, resource: &str) -> Result<Vec<u8>> {
    let response = client
        .get(url)
        .send()
        .await
        .with_context(|| format!("could not download {resource}"))?;
    ensure!(
        response.status().is_success(),
        "could not download {resource}: HTTP {} from {url}",
        response.status()
    );
    if let Some(length) = response.content_length() {
        ensure!(
            length <= limit as u64,
            "{resource} is larger than the {limit}-byte limit"
        );
    }

    let mut body = Vec::new();
    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.with_context(|| format!("could not read {resource}"))?;
        ensure!(
            body.len().saturating_add(chunk.len()) <= limit,
            "{resource} is larger than the {limit}-byte limit"
        );
        body.extend_from_slice(&chunk);
    }
    Ok(body)
}

fn parse_release(release: GitHubRelease) -> Result<Release> {
    ensure!(!release.draft, "latest release is still a draft");
    ensure!(!release.prerelease, "latest release is a prerelease");
    let version_text = release
        .tag_name
        .strip_prefix('v')
        .context("release tag must start with 'v'")?;
    let version = Version::parse(version_text).context("release tag is not valid SemVer")?;
    ensure!(
        release.tag_name == format!("v{version}"),
        "release tag is not canonical SemVer"
    );
    ensure!(
        release.html_url.starts_with("https://github.com/"),
        "release page is not a trusted GitHub HTTPS URL"
    );

    let installer = release
        .assets
        .iter()
        .find(|asset| asset.name == "install.sh")
        .context("release has no install.sh asset")?;
    let checksums = release
        .assets
        .iter()
        .find(|asset| asset.name == "SHA256SUMS")
        .context("release has no SHA256SUMS asset")?;
    let installer_digest = installer
        .digest
        .as_deref()
        .map(parse_asset_digest)
        .transpose()?;
    let checksums_digest = checksums
        .digest
        .as_deref()
        .map(parse_asset_digest)
        .transpose()?;

    Ok(Release {
        version,
        tag: release.tag_name,
        html_url: release.html_url,
        installer_url: installer.browser_download_url.clone(),
        checksums_url: checksums.browser_download_url.clone(),
        installer_digest,
        checksums_digest,
    })
}

fn parse_asset_digest(digest: &str) -> Result<String> {
    let digest = digest
        .strip_prefix("sha256:")
        .context("GitHub asset digest is not SHA-256")?;
    validate_sha256(digest)?;
    Ok(digest.to_ascii_lowercase())
}

fn checksum_for_asset(checksums: &[u8], asset_name: &str) -> Result<String> {
    let checksums = std::str::from_utf8(checksums).context("SHA256SUMS is not UTF-8")?;
    for line in checksums.lines() {
        let mut fields = line.split_whitespace();
        let Some(checksum) = fields.next() else {
            continue;
        };
        let Some(name) = fields.next() else {
            continue;
        };
        if name.trim_start_matches('*') == asset_name {
            ensure!(
                fields.next().is_none(),
                "invalid SHA256SUMS entry for {asset_name}"
            );
            validate_sha256(checksum)?;
            return Ok(checksum.to_ascii_lowercase());
        }
    }
    bail!("{asset_name} is missing from SHA256SUMS")
}

fn validate_sha256(value: &str) -> Result<()> {
    ensure!(
        value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit()),
        "invalid SHA-256 value"
    );
    Ok(())
}

fn sha256_hex(content: &[u8]) -> String {
    format!("{:x}", Sha256::digest(content))
}

async fn write_new_file(path: &Path, content: &[u8]) -> Result<()> {
    let mut file = tokio::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .await
        .with_context(|| format!("could not create {}", path.display()))?;
    file.write_all(content).await?;
    file.flush().await?;
    Ok(())
}

async fn create_update_directory() -> Result<PathBuf> {
    let temp = std::env::temp_dir();
    for _ in 0..16 {
        let directory = temp.join(format!(
            "{UPDATE_DIRECTORY_PREFIX}{:016x}",
            rand::random::<u64>()
        ));
        match tokio::fs::create_dir(&directory).await {
            Ok(()) => return Ok(directory),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => {
                return Err(error).with_context(|| {
                    format!("could not create update directory {}", directory.display())
                });
            }
        }
    }
    bail!("could not allocate a unique update directory")
}

fn output_snippet(output: &[u8]) -> String {
    const LIMIT: usize = 4096;
    let start = output.len().saturating_sub(LIMIT);
    String::from_utf8_lossy(&output[start..]).trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn release_json(version: &str, digest: &str) -> GitHubRelease {
        GitHubRelease {
            tag_name: format!("v{version}"),
            html_url: format!("https://github.com/{REPOSITORY}/releases/tag/v{version}"),
            draft: false,
            prerelease: false,
            assets: vec![
                GitHubAsset {
                    name: "install.sh".into(),
                    browser_download_url: format!(
                        "https://github.com/{REPOSITORY}/releases/download/v{version}/install.sh"
                    ),
                    digest: Some(format!("sha256:{digest}")),
                },
                GitHubAsset {
                    name: "SHA256SUMS".into(),
                    browser_download_url: format!(
                        "https://github.com/{REPOSITORY}/releases/download/v{version}/SHA256SUMS"
                    ),
                    digest: Some(format!("sha256:{digest}")),
                },
            ],
        }
    }

    #[test]
    fn release_metadata_requires_canonical_semver_and_assets() {
        let digest = "a".repeat(64);
        let release = parse_release(release_json("1.2.3", &digest)).unwrap();
        assert_eq!(release.version, Version::new(1, 2, 3));
        assert_eq!(release.installer_digest.as_deref(), Some(digest.as_str()));

        let mut invalid = release_json("1.2.3", &digest);
        invalid.tag_name = "1.2.3".into();
        assert!(parse_release(invalid).is_err());
    }

    #[test]
    fn checksums_match_exact_asset_names() {
        let checksum = "b".repeat(64);
        let body = format!(
            "{checksum}  install.sh\n{}  other-install.sh\n",
            "c".repeat(64)
        );
        assert_eq!(
            checksum_for_asset(body.as_bytes(), "install.sh").unwrap(),
            checksum
        );
        assert!(checksum_for_asset(body.as_bytes(), "missing.sh").is_err());
    }

    #[test]
    fn installation_detection_only_accepts_official_layouts() {
        let linux = detect_installation_from_executable(
            Path::new("/home/alice/.local/lib/memos-desktop/memos-desktop"),
            "linux",
            Some(Path::new("/home/alice")),
        );
        assert_eq!(
            linux,
            Installation::ManagedLinux {
                install_root: PathBuf::from("/home/alice/.local"),
            }
        );
        assert_eq!(
            linux.restart_path(),
            Some(PathBuf::from(
                "/home/alice/.local/lib/memos-desktop/memos-desktop"
            ))
        );
        assert!(matches!(
            detect_installation_from_executable(
                Path::new("/usr/lib/memos-desktop/memos-desktop"),
                "linux",
                Some(Path::new("/home/alice")),
            ),
            Installation::External { .. }
        ));

        let custom_root = std::env::temp_dir().join(format!(
            "memos-installer-detection-test-{:016x}",
            rand::random::<u64>()
        ));
        let custom_app = custom_root.join("lib").join("memos-desktop");
        fs::create_dir_all(&custom_app).unwrap();
        fs::write(custom_app.join(".installer-managed"), "1.0.0\n").unwrap();
        assert_eq!(
            detect_installation_from_executable(&custom_app.join("memos-desktop"), "linux", None,),
            Installation::ManagedLinux {
                install_root: custom_root.clone(),
            }
        );
        fs::remove_dir_all(custom_root).unwrap();

        assert!(matches!(
            detect_installation_from_executable(
                Path::new("/home/alice/src/target/release/memos-desktop"),
                "linux",
                Some(Path::new("/home/alice")),
            ),
            Installation::External { .. }
        ));
        let macos = detect_installation_from_executable(
            Path::new("/Users/alice/Applications/Memos Desktop.app/Contents/MacOS/memos-desktop"),
            "macos",
            Some(Path::new("/Users/alice")),
        );
        assert_eq!(
            macos,
            Installation::ManagedMacOs {
                applications_dir: PathBuf::from("/Users/alice/Applications"),
            }
        );
        assert_eq!(
            macos.restart_path(),
            Some(PathBuf::from(
                "/Users/alice/Applications/Memos Desktop.app/Contents/MacOS/memos-desktop"
            ))
        );
        assert!(matches!(
            detect_installation_from_executable(
                Path::new("C:/Memos/memos-desktop.exe"),
                "windows",
                None,
            ),
            Installation::External { .. }
        ));
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn installer_receives_fixed_update_inputs() {
        let root = std::env::temp_dir().join(format!(
            "memos-update-command-test-{:016x}",
            rand::random::<u64>()
        ));
        fs::create_dir_all(&root).unwrap();
        let script_path = root.join("install.sh");
        let checksums_path = root.join("SHA256SUMS");
        fs::write(
            &script_path,
            r#"#!/bin/sh
set -eu
test "$1" = "--update"
test "$2" = "--version"
test "$3" = "v2.0.0"
test -f "$MEMOS_DESKTOP_CHECKSUMS_PATH"
test "$MEMOS_DESKTOP_INSTALL_ROOT" = "/tmp/managed root"
"#,
        )
        .unwrap();
        fs::write(&checksums_path, "verified\n").unwrap();
        let prepared = PreparedInstaller {
            directory: root.clone(),
            script_path,
            checksums_path,
        };
        let release = parse_release(release_json("2.0.0", &"a".repeat(64))).unwrap();

        install_update(
            prepared,
            &release,
            &Installation::ManagedLinux {
                install_root: PathBuf::from("/tmp/managed root"),
            },
        )
        .await
        .unwrap();
        assert!(!root.exists());
    }

    #[tokio::test]
    #[ignore = "requires GitHub release access"]
    async fn live_latest_release_metadata_is_compatible() {
        let release = check_for_update(&Version::new(0, 0, 0))
            .await
            .unwrap()
            .expect("the repository should have a release");
        assert!(release.version >= Version::new(0, 1, 0));
        assert!(release.installer_digest.is_some());
        assert!(release.checksums_digest.is_some());
        let installer = download_verified_installer(&release).await.unwrap();
        assert!(
            tokio::fs::metadata(&installer.script_path)
                .await
                .unwrap()
                .is_file()
        );
        assert!(
            tokio::fs::metadata(&installer.checksums_path)
                .await
                .unwrap()
                .is_file()
        );
    }

    #[test]
    fn completed_update_marker_waits_for_the_new_binary() {
        let root = std::env::temp_dir().join(format!(
            "memos-update-marker-test-{:016x}",
            rand::random::<u64>()
        ));
        let marker = root.join(COMPLETED_UPDATE_MARKER);
        record_completed_update_at(&marker, &Version::new(2, 0, 0)).unwrap();

        assert_eq!(
            take_completed_update_at(&marker, &Version::new(1, 9, 0)).unwrap(),
            None
        );
        assert!(marker.is_file());
        assert_eq!(
            take_completed_update_at(&marker, &Version::new(2, 0, 0)).unwrap(),
            Some(Version::new(2, 0, 0))
        );
        assert!(!marker.exists());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn invalid_completed_update_marker_is_removed() {
        let root = std::env::temp_dir().join(format!(
            "memos-update-marker-test-{:016x}",
            rand::random::<u64>()
        ));
        let marker = root.join(COMPLETED_UPDATE_MARKER);
        fs::create_dir_all(&root).unwrap();
        fs::write(&marker, "not-semver").unwrap();

        assert!(take_completed_update_at(&marker, &Version::new(2, 0, 0)).is_err());
        assert!(!marker.exists());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn status_exposes_actionable_release_information() {
        let digest = "d".repeat(64);
        let release = parse_release(release_json("2.0.0", &digest)).unwrap();
        let status = AutoUpdateStatus::ReadyToRestart {
            release: release.clone(),
        };
        assert!(!status.is_busy());
        assert!(matches!(
            &status,
            AutoUpdateStatus::ReadyToRestart { release }
                if release.version == Version::new(2, 0, 0)
        ));
        assert_eq!(status.release_url(), Some(release.html_url.as_str()));
    }
}
