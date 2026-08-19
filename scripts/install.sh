#!/usr/bin/env sh
set -eu

REPOSITORY="markbang/memos-desktop"
VERSION="${MEMOS_DESKTOP_VERSION:-latest}"
INSTALL_ROOT="${MEMOS_DESKTOP_INSTALL_ROOT:-$HOME/.local}"
APPLICATIONS_DIR="${MEMOS_DESKTOP_APPLICATIONS_DIR:-$HOME/Applications}"

usage() {
  cat <<'EOF'
Memos Desktop installer

Usage:
  install.sh [--version VERSION]
  install.sh --uninstall

Environment:
  MEMOS_DESKTOP_VERSION           Release to install, default: latest
  MEMOS_DESKTOP_INSTALL_ROOT      Command/data root, default: ~/.local
  MEMOS_DESKTOP_APPLICATIONS_DIR  macOS app directory, default: ~/Applications
EOF
}

fail() {
  echo "memos-desktop: $*" >&2
  exit 1
}

platform_name() {
  platform="${MEMOS_DESKTOP_PLATFORM:-$(uname -s)}"
  case "$platform" in
    Linux | linux) echo "linux" ;;
    Darwin | macos) echo "macos" ;;
    *) fail "unsupported platform: $platform" ;;
  esac
}

architecture_name() {
  architecture="${MEMOS_DESKTOP_ARCH:-$(uname -m)}"
  case "$architecture" in
    x86_64 | amd64) echo "x86_64" ;;
    arm64 | aarch64) echo "aarch64" ;;
    *) fail "unsupported architecture: $architecture" ;;
  esac
}

download() {
  url="$1"
  destination="$2"
  if command -v curl >/dev/null 2>&1; then
    curl --proto '=https' --tlsv1.2 -fsSL "$url" -o "$destination"
  elif command -v wget >/dev/null 2>&1; then
    wget -qO "$destination" "$url"
  else
    fail "curl or wget is required"
  fi
}

sha256() {
  file="$1"
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$file" | awk '{print $1}'
  elif command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$file" | awk '{print $1}'
  elif command -v openssl >/dev/null 2>&1; then
    openssl dgst -sha256 "$file" | awk '{print $NF}'
  else
    fail "sha256sum, shasum, or openssl is required"
  fi
}

verify_archive() {
  archive="$1"
  checksums="$2"
  asset="$3"
  expected="$(awk -v name="$asset" '$2 == name || $2 == "*" name { print $1; exit }' "$checksums")"
  [ -n "$expected" ] || fail "$asset is missing from SHA256SUMS"
  actual="$(sha256 "$archive")"
  [ "$actual" = "$expected" ] || fail "checksum verification failed for $asset"
}

path_instructions() {
  if command -v memos-desktop >/dev/null 2>&1; then
    echo "Run Memos Desktop with: memos-desktop"
    return
  fi

  echo "Add $INSTALL_ROOT/bin to PATH to run Memos Desktop from a terminal."
  case "${SHELL:-}" in
    *zsh) echo "  echo 'export PATH=\"$INSTALL_ROOT/bin:\$PATH\"' >> ~/.zshrc" ;;
    *fish) echo "  fish_add_path -U '$INSTALL_ROOT/bin'" ;;
    *) echo "  echo 'export PATH=\"$INSTALL_ROOT/bin:\$PATH\"' >> ~/.bashrc" ;;
  esac
}

install_linux() {
  source_dir="$1"
  app_dir="$INSTALL_ROOT/lib/memos-desktop"
  stage="$INSTALL_ROOT/lib/.memos-desktop.new"

  rm -rf "$stage"
  mkdir -p "$stage" "$INSTALL_ROOT/bin" "$INSTALL_ROOT/share/applications"
  cp -R "$source_dir/." "$stage/"
  [ -f "$stage/memos-desktop" ] || fail "release archive has no memos-desktop binary"
  chmod 755 "$stage/memos-desktop"
  rm -rf "$app_dir"
  mv "$stage" "$app_dir"
  ln -sfn "$app_dir/memos-desktop" "$INSTALL_ROOT/bin/memos-desktop"

  desktop_file="$INSTALL_ROOT/share/applications/com.markbang.MemosDesktop.desktop"
  cat >"$desktop_file" <<EOF
[Desktop Entry]
Type=Application
Name=Memos Desktop
Comment=Native desktop client for Memos
Exec="$app_dir/memos-desktop"
TryExec=$app_dir/memos-desktop
Icon=accessories-text-editor
Terminal=false
Categories=Office;Utility;
StartupNotify=true
EOF

  if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database "$INSTALL_ROOT/share/applications" >/dev/null 2>&1 || true
  fi

  echo "Memos Desktop installed to $app_dir"
  path_instructions
}

install_macos() {
  source_dir="$1"
  app_path="$APPLICATIONS_DIR/Memos Desktop.app"
  stage="$APPLICATIONS_DIR/.Memos Desktop.app.new"

  [ -f "$source_dir/memos-desktop" ] || fail "release archive has no memos-desktop binary"
  rm -rf "$stage"
  mkdir -p "$stage/Contents/MacOS" "$stage/Contents/Resources" "$INSTALL_ROOT/bin"
  cp "$source_dir/memos-desktop" "$stage/Contents/MacOS/memos-desktop"
  chmod 755 "$stage/Contents/MacOS/memos-desktop"
  for document in LICENSE LICENSE-THIRD-PARTY.md NOTICE README.md; do
    if [ -f "$source_dir/$document" ]; then
      cp "$source_dir/$document" "$stage/Contents/Resources/$document"
    fi
  done
  cat >"$stage/Contents/Info.plist" <<'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "https://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleDisplayName</key><string>Memos Desktop</string>
  <key>CFBundleExecutable</key><string>memos-desktop</string>
  <key>CFBundleIdentifier</key><string>com.markbang.MemosDesktop</string>
  <key>CFBundleName</key><string>Memos Desktop</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>LSApplicationCategoryType</key><string>public.app-category.productivity</string>
  <key>NSHighResolutionCapable</key><true/>
</dict>
</plist>
EOF

  rm -rf "$app_path"
  mv "$stage" "$app_path"
  ln -sfn "$app_path/Contents/MacOS/memos-desktop" "$INSTALL_ROOT/bin/memos-desktop"
  echo "Memos Desktop installed to $app_path"
  echo "Open it with: open '$app_path'"
  path_instructions
}

uninstall() {
  platform="$(platform_name)"
  rm -f "$INSTALL_ROOT/bin/memos-desktop"
  if [ "$platform" = "linux" ]; then
    rm -rf "$INSTALL_ROOT/lib/memos-desktop"
    rm -f "$INSTALL_ROOT/share/applications/com.markbang.MemosDesktop.desktop"
  else
    rm -rf "$APPLICATIONS_DIR/Memos Desktop.app"
  fi
  echo "Memos Desktop has been uninstalled. User configuration was preserved."
}

main() {
  action="install"
  while [ "$#" -gt 0 ]; do
    case "$1" in
      --version)
        [ "$#" -ge 2 ] || fail "--version requires a value"
        VERSION="$2"
        shift 2
        ;;
      --uninstall)
        action="uninstall"
        shift
        ;;
      --help | -h)
        usage
        exit 0
        ;;
      *) fail "unknown argument: $1" ;;
    esac
  done

  if [ "$action" = "uninstall" ]; then
    uninstall
    exit 0
  fi

  platform="$(platform_name)"
  architecture="$(architecture_name)"
  case "$platform-$architecture" in
    linux-x86_64) asset="memos-desktop-linux-x86_64.tar.gz" ;;
    macos-x86_64) asset="memos-desktop-macos-x86_64.tar.gz" ;;
    macos-aarch64) asset="memos-desktop-macos-aarch64.tar.gz" ;;
    linux-aarch64) fail "a Linux aarch64 release is not available yet" ;;
    *) fail "unsupported platform and architecture: $platform-$architecture" ;;
  esac

  temp_root="${TMPDIR:-/tmp}"
  temp="$(mktemp -d "$temp_root/memos-desktop.XXXXXX")"
  trap 'rm -rf "$temp"' EXIT HUP INT TERM
  archive="$temp/$asset"
  checksums="$temp/SHA256SUMS"

  if [ -n "${MEMOS_DESKTOP_ARCHIVE_PATH:-}" ]; then
    [ -n "${MEMOS_DESKTOP_CHECKSUMS_PATH:-}" ] || fail "MEMOS_DESKTOP_CHECKSUMS_PATH is required with a local archive"
    cp "$MEMOS_DESKTOP_ARCHIVE_PATH" "$archive"
    cp "$MEMOS_DESKTOP_CHECKSUMS_PATH" "$checksums"
  else
    case "$VERSION" in
      latest) release_base="https://github.com/$REPOSITORY/releases/latest/download" ;;
      v*) release_base="https://github.com/$REPOSITORY/releases/download/$VERSION" ;;
      *) release_base="https://github.com/$REPOSITORY/releases/download/v$VERSION" ;;
    esac
    echo "Downloading Memos Desktop $VERSION for $platform-$architecture..."
    download "$release_base/$asset" "$archive"
    download "$release_base/SHA256SUMS" "$checksums"
  fi

  verify_archive "$archive" "$checksums" "$asset"
  mkdir -p "$temp/extracted"
  tar -xzf "$archive" -C "$temp/extracted"
  if [ "$platform" = "linux" ]; then
    install_linux "$temp/extracted"
  else
    install_macos "$temp/extracted"
  fi
}

main "$@"
