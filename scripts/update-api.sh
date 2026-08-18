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

# Progenitor's stand-alone CLI cannot configure an inner client state. Add a
# clone-shared authorization slot so sessions for the same base URL stay isolated.
perl -0pi -e 's/pub struct Client \{\n    pub\(crate\) baseurl:/pub struct Client {\n    pub(crate) authorization: crate::auth::AuthorizationState,\n    pub(crate) baseurl:/' crates/memos-api/src/generated.rs
perl -0pi -e 's/Self \{\n            baseurl: baseurl\.to_string\(\),\n            client,\n        \}/Self {\n            authorization: Default::default(),\n            baseurl: baseurl.to_string(),\n            client,\n        }/' crates/memos-api/src/generated.rs
if ! grep -q 'authorization: crate::auth::AuthorizationState' crates/memos-api/src/generated.rs \
  || ! grep -q 'authorization: Default::default()' crates/memos-api/src/generated.rs; then
  echo "failed to add generated client authorization state and initializer" >&2
  exit 1
fi

cargo fmt --all
