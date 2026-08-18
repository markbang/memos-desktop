# Memos Desktop

[![CI](https://github.com/markbang/memos-desktop/actions/workflows/ci.yml/badge.svg)](https://github.com/markbang/memos-desktop/actions/workflows/ci.yml)
[![CodeQL](https://github.com/markbang/memos-desktop/actions/workflows/codeql.yml/badge.svg)](https://github.com/markbang/memos-desktop/actions/workflows/codeql.yml)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

A native desktop client for Memos built with Rust, GPUI, and GPUI Component.

The application uses Memos' generated v1 OpenAPI contract instead of maintaining a parallel handwritten protocol. It targets Memos v0.30 and keeps a distinct, desktop-first visual and interaction model while covering the server's core workflows.

## Current milestone

The current build is an alpha desktop client with a complete v0.30 transport contract and broad end-to-end workflow coverage. The UI remains intentionally independent from the web application.

The current build includes:

- Instance discovery and validation
- Password, personal access token, and anonymous sessions
- Refresh-cookie based access token renewal
- Timeline loading, local search, and quick filters
- Memo creation with Private, Protected, and Public visibility
- Pin, archive, restore, and permanent delete actions
- Markdown detail inspector with activity, links, shares, and files tabs
- Memo editing, creation-time/location updates, task actions, CEL saved shortcuts, and pagination
- Comment/reaction/relation activity, share-link expiry/revocation, and authenticated attachment previews/opening
- Inbox notification navigation, archive/delete mutations, account/PAT/webhook settings, and admin resources
- Password registration, SSO/OAuth2 PKCE, shared memo links, and SSE live refresh
- Demo mode for visual development without a server
- A generated client covering every Memos v0.30 API operation

See `docs/FEATURE_MATRIX.md` for the parity plan and current status.

## Run

Rust 1.97.1 is pinned by `rust-toolchain.toml`. Debian and Ubuntu users can install GPUI native dependencies with `scripts/install-linux-deps.sh`.

```bash
cargo run -p memos-desktop
```

Use the visual demo workspace without a Memos server:

```bash
cargo run -p memos-desktop -- --demo
```

## Validation

Run the normal workspace checks with:

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

The ignored integration test must target a disposable official Memos v0.30 instance because it creates and deletes users and resources:

```bash
MEMOS_LIVE_URL=http://127.0.0.1:5231 \
MEMOS_LIVE_USERNAME=admin \
MEMOS_LIVE_PASSWORD=integration-test \
cargo test -p memos-desktop live_v030_core_round_trip -- --ignored
```

## Security

Passwords, access tokens, and refresh cookies are kept in process memory only. The local configuration persists only the last server URL and username. Refresh cookies are held in the active session's clone-shared authentication state.

## Structure

```text
crates/app                 GPUI desktop application
crates/memos-api           Generated Memos v1 Rust client
api/openapi.yaml           Upstream Memos OpenAPI contract
api/openapi.codegen.json   Normalized generator input
tools/openapi-normalizer   Structured OpenAPI normalization tool
docs                       Product, design, and parity documentation
```

## License

Memos Desktop is licensed under [Apache-2.0](LICENSE). Third-party attribution is recorded in [LICENSE-THIRD-PARTY.md](LICENSE-THIRD-PARTY.md). Memos itself is an independent MIT-licensed project.

## GitHub Maintenance

- CI checks formatting, workspace compilation, tests, Clippy, and release builds on Linux, macOS, and Windows.
- Version tags such as `v0.1.0` produce platform archives and `SHA256SUMS` through the release workflow documented in [docs/RELEASING.md](docs/RELEASING.md).
- Dependabot updates Cargo and GitHub Actions dependencies weekly.
- CodeQL runs on the default branch, pull requests, and a weekly schedule.

Repository rulesets and security switches that cannot be stored in Git are listed in [docs/GITHUB_SETUP.md](docs/GITHUB_SETUP.md). Release history is maintained in [CHANGELOG.md](CHANGELOG.md).
