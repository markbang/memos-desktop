# Changelog

All notable changes to this project will be documented in this file.

The project follows [Semantic Versioning](https://semver.org/) and uses GitHub Releases for signed release notes and platform artifacts.

## Unreleased

## 0.1.1 - 2026-08-24

### Added

- Add Zed-style automatic update state, hourly and manual checks, verified in-place Linux/macOS installer updates, explicit restart, and external-install notifications.
- Add a repository instruction requiring Zed-first GPUI research while preserving the Apache-2.0/GPL licensing boundary.

### Changed

- Align the native workspace with the Memos v0.30 Web navigation, centered feed, Markdown cards, composer hierarchy, and contextual detail presentation.
- Add representative anonymized native-client screenshots and document Web UI parity as the product direction.
- Stream cached attachments to disk with explicit avatar, preview, and original-file size limits.
- Limit concurrent avatar and attachment downloads per authenticated session.

### Fixed

- Validate image responses before using them as avatars or previews and write every cache entry atomically.
- Display the package version in the sign-in screen instead of a hard-coded release number.
- Replace existing configuration files reliably on Windows, report persistence failures, and recover an interrupted replacement.
- Remove the previously configured password when switching to PAT, anonymous, or SSO authentication.

## 0.1.0 - 2026-08-19

### Added

- Native GPUI desktop application foundation.
- Generated client for all 73 operations in the pinned Memos v0.30.0 OpenAPI contract.
- Password/PAT/anonymous/shared-link sessions, registration, OAuth2 PKCE, linked identities, refresh rotation, and sign-out.
- Paginated timelines, Explore, profiles, saved CEL shortcuts, full Memo editing, comments, reactions, relations, shares, attachments, Inbox, account settings, and administration workflows.
- Authenticated attachment caching, image thumbnails, AI transcription, and SSE live refresh.
- Disposable official-v0.30 live integration test covering core resource round trips.
- Cross-platform CI, CodeQL, dependency policy checks, and release automation.

### Fixed

- Isolated authentication state between separate sessions connected to the same server.
- Used Connect RPC for v0.30 refresh and sign-out because its REST gateway does not forward request cookies.
- Pinned API generation to the stable v0.30 `ShortcutService` instead of the post-release Memos main contract.
- Backported the upstream `grid` dimension-overflow fix while GPUI remains pinned to 0.2.2.
