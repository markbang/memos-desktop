# Architecture

## Layers

```text
GPUI application
  -> workflow state and desktop interaction
  -> ApiSession
  -> generated memos-api client
  -> Memos v1 REST gateway
```

## Desktop state

`MemosDesktop` owns window-level workflow state: active route, selection, search, composer state, current profile, and loaded resources. GPUI entities own text input state and subscriptions.

Network work never executes on the render thread. `ApiSession` dispatches generated-client futures onto a dedicated Tokio multi-thread runtime and returns the result to GPUI entity tasks.

## Automatic updates

The updater follows the lifecycle used by Zed's `crates/auto_update` subsystem, inspected at Zed commit `d9ad6aff67e47de43abb270d22de75dd950f1b48`: explicit status transitions, delayed periodic polling, quiet automatic failures, visible manual failures, install-source detection, verified staging, explicit restart paths, and post-update notification. The implementation is original and scoped to this repository's Apache-2.0 release pipeline.

`auto_update.rs` owns release parsing, SemVer comparison, bounded downloads, SHA-256 verification, installation detection, temporary-directory cleanup, installer execution, and restart-path derivation. `MemosDesktop` owns scheduling and UI-observable status. Network and installer work run on the Tokio runtime; GPUI render code reads state only.

Official installer layouts on Linux and macOS carry an `.installer-managed` marker and use `install.sh --update`, which atomically replaces the existing application while preserving desktop entries and command symlinks. Default-path v0.1.0 installs are recognized once for marker migration. Package-manager installs, source builds, unsupported platforms, and Windows archives are treated as externally managed and only receive a release link. This avoids overwriting files owned by another installer and avoids pretending that the current Windows ZIP can safely update a running executable.

## API contract

`api/openapi.yaml` is pinned to the official Memos `v0.30.0` contract. This matters because later Memos main builds replace the stable `ShortcutService` contract with a different saved-view service. `openapi-normalizer` parses the document structurally, removes generator-incompatible `default` response declarations, and repairs the known v0.30 instance-setting wildcard route. Progenitor then emits `crates/memos-api/src/generated.rs`; `scripts/update-api.sh` reapplies the small client-state hook after regeneration.

Each generated `Client` owns clone-shared authentication state, so two sessions for the same server cannot overwrite one another. Progenitor's request hook injects only the bearer token. Memos v0.30's REST gateway exposes refresh cookies as response metadata and does not forward REST request cookies to gRPC metadata, so the explicit Connect refresh/sign-out requests attach the cookie while normal resource operations use the generated REST client.

## Authentication invariants

- Passwords are persisted only through the operating system credential store and can be forgotten from the account context menu.
- Passwords and access tokens are never written to application configuration.
- Bearer tokens are marked sensitive in HTTP headers.
- Refresh occurs 30 seconds before token expiry.
- Personal access tokens have no client-side expiry assumption.
- Failed PAT validation clears the session's bearer and refresh-cookie state.

## Compatibility

The workspace pins the published GPUI `0.2.2` and GPUI Component `0.5.1` releases. Because that GPUI line pins `taffy 0.9.0` and vulnerable `grid 0.18.0`, `vendor/grid` carries the upstream dimension-overflow fix without changing the GPUI/taffy API contract. `Cargo.lock` is committed because the UI ecosystem remains pre-1.0 and exact dependency resolution is part of the application contract.
