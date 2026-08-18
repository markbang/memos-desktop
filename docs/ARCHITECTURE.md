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

## API contract

`api/openapi.yaml` is pinned to the official Memos `v0.30.0` contract. This matters because later Memos main builds replace the stable `ShortcutService` contract with a different saved-view service. `openapi-normalizer` parses the document structurally, removes generator-incompatible `default` response declarations, and repairs the known v0.30 instance-setting wildcard route. Progenitor then emits `crates/memos-api/src/generated.rs`; `scripts/update-api.sh` reapplies the small client-state hook after regeneration.

Each generated `Client` owns clone-shared authentication state, so two sessions for the same server cannot overwrite one another. Progenitor's request hook injects only the bearer token. Memos v0.30's REST gateway exposes refresh cookies as response metadata and does not forward REST request cookies to gRPC metadata, so the explicit Connect refresh/sign-out requests attach the cookie while normal resource operations use the generated REST client.

## Authentication invariants

- Passwords are never persisted.
- Access tokens are never written to application configuration.
- Bearer tokens are marked sensitive in HTTP headers.
- Refresh occurs 30 seconds before token expiry.
- Personal access tokens have no client-side expiry assumption.
- Failed PAT validation clears the session's bearer and refresh-cookie state.

## Compatibility

The workspace pins the published GPUI `0.2.2` and GPUI Component `0.5.1` releases. `Cargo.lock` is committed because the UI ecosystem remains pre-1.0 and exact dependency resolution is part of the application contract.
