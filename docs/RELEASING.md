# Releasing

1. Update `workspace.package.version` in the root `Cargo.toml`.
2. Update `CHANGELOG.md` and move completed entries out of `Unreleased`.
3. Run `make ci` on a Linux host with GPUI native dependencies installed.
4. Run `cargo deny check`.
5. Merge the release preparation pull request into `main`.
6. Create and push a signed SemVer tag:

```bash
git tag -s vX.Y.Z -m "memos-desktop vX.Y.Z"
git push origin vX.Y.Z
```

The Release workflow builds platform archives, publishes `install.sh`, generates SHA-256 checksums, and creates a GitHub Release with generated notes. Verify every artifact and run the installer in an isolated user directory before announcing the release.

For a release containing updater changes, install the previous version with `scripts/install.sh`, publish the candidate release, and exercise the complete in-app path: automatic/manual discovery, installer and archive checksum verification, `--update` replacement without integration-file churn, explicit restart into the new binary, and the post-update notification. Also verify that a source build and the Windows archive only open the release page.

The `memos-api` crate version tracks the upstream Memos API contract and is not the desktop application version.
