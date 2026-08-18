#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if [[ -f "$HOME/.cargo/env" ]]; then
  # shellcheck disable=SC1091
  source "$HOME/.cargo/env"
fi
cd "$ROOT"

cargo run -p openapi-normalizer -- api/openapi.yaml api/openapi.codegen.json
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

RUSTFMT="$(rustup which --toolchain nightly rustfmt)" \
  cargo progenitor \
    --input api/openapi.codegen.json \
    --output "$TMP_DIR/memos-api" \
    --name memos-api \
    --version 0.30.0

mv "$TMP_DIR/memos-api/src/lib.rs" crates/memos-api/src/generated.rs
cargo fmt --all
