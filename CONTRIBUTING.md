# Contributing

Thanks for helping improve Memos Desktop.

On Debian or Ubuntu, install native GPUI build dependencies with `scripts/install-linux-deps.sh` before the first build.

## Before Opening A Pull Request

1. Keep changes focused on one behavior or maintenance concern.
2. Run `cargo fmt --all -- --check`.
3. Run `cargo check --workspace`.
4. Run `cargo test --workspace --all-targets`.
5. Update the feature matrix or design documentation when behavior or product scope changes.
6. Do not commit passwords, access tokens, Memos instance data, or generated build output.

## API Contract Updates

`api/openapi.yaml` is sourced from the upstream Memos project. Regenerate the typed client with:

```bash
scripts/update-api.sh
```

The script requires Rust stable and a nightly `rustfmt` component because Progenitor formats generated source with unstable formatting options.

## Commit Messages

Use Conventional Commits, for example:

```text
feat: add memo share management
fix: preserve selected memo after refresh
chore: update Memos API contract
```

## Pull Requests

Pull requests should explain the user-visible behavior, include verification commands, and call out platform-specific changes. Keep generated API diffs separate from unrelated UI refactors.
