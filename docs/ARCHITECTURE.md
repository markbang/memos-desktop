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

`api/openapi.yaml` is copied from the upstream Memos repository. `openapi-normalizer` parses it structurally and removes only the generator-incompatible `default` response declarations. Progenitor then emits `crates/memos-api/src/generated.rs`; the stable crate wrapper and authentication hook remain outside generated output. Run `scripts/update-api.sh` when the upstream contract changes.

The generated crate is extended with a per-base-URL bearer-token registry through Progenitor's request hook. A shared reqwest cookie jar keeps refresh-token cookies for the active process.

## Authentication invariants

- Passwords are never persisted.
- Access tokens are never written to application configuration.
- Bearer tokens are marked sensitive in HTTP headers.
- Refresh occurs 30 seconds before token expiry.
- Personal access tokens have no client-side expiry assumption.
- Failed PAT validation clears the token registry entry.

## Compatibility

The workspace pins the published GPUI `0.2.2` and GPUI Component `0.5.1` releases. `Cargo.lock` is committed because the UI ecosystem remains pre-1.0 and exact dependency resolution is part of the application contract.
