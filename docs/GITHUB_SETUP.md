# GitHub Repository Setup

Files in `.github/` configure CI, releases, dependency updates, issue forms, and ownership. The following repository settings are not stored in Git and should be enabled after the initial push.

## General

- Default branch: `main`
- Description: `A native Memos desktop client built with Rust, GPUI, and GPUI Component.`
- Website: `https://usememos.com`
- Topics: `memos`, `rust`, `gpui`, `desktop`, `markdown`, `self-hosted`
- Enable Issues and Discussions
- Disable Wiki unless it has a clear owner

## Branch Protection

Create a ruleset for `main`:

- require a pull request before merging;
- require one approving review;
- dismiss stale approvals when new commits are pushed;
- require conversation resolution;
- require branches to be up to date;
- require signed commits when all maintainers can support them;
- block force pushes and branch deletion.

Required status checks should include:

- `Format`
- `Linux x86_64`
- `macOS`
- `Windows`
- `Clippy`
- `Dependency policy`
- `Analyze Rust`

## Security

Enable:

- Dependabot alerts and security updates;
- private vulnerability reporting;
- secret scanning and push protection;
- CodeQL default alerts;
- automatic deletion of branches after merge.

## Releases

Push a SemVer tag to create a release:

```bash
git tag -s vX.Y.Z -m "memos-desktop vX.Y.Z"
git push origin vX.Y.Z
```

The release workflow builds Linux, macOS Intel, macOS Apple Silicon, and Windows archives, then publishes checksums with generated release notes.
