# Releasing

1. Update `workspace.package.version` in the root `Cargo.toml`.
2. Update `CHANGELOG.md` and move completed entries out of `Unreleased`.
3. Run `make ci` on a Linux host with GPUI native dependencies installed.
4. Run `cargo deny check`.
5. Merge the release preparation pull request into `main`.
6. Create and push a signed SemVer tag:

```bash
git tag -s v0.1.0 -m "memos-desktop v0.1.0"
git push origin v0.1.0
```

The Release workflow builds platform archives, publishes `install.sh`, generates SHA-256 checksums, and creates a GitHub Release with generated notes. Verify every artifact and run the installer in an isolated user directory before announcing the release.

The `memos-api` crate version tracks the upstream Memos API contract and is not the desktop application version.
