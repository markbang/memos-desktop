# Changelog

All notable changes to this project will be documented in this file.

The project follows [Semantic Versioning](https://semver.org/) and uses GitHub Releases for signed release notes and platform artifacts.

## Unreleased

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
