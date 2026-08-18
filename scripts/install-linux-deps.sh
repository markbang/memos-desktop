#!/usr/bin/env bash
set -euo pipefail

if ! command -v apt-get >/dev/null 2>&1; then
  echo "This helper currently supports Debian and Ubuntu runners." >&2
  exit 1
fi

sudo apt-get update
sudo apt-get install --yes --no-install-recommends \
  clang cmake pkg-config \
  libfontconfig1-dev libfreetype6-dev \
  libx11-dev libx11-xcb-dev libxcb1-dev libxcb-render0-dev \
  libxcb-shape0-dev libxcb-xfixes0-dev libxcb-randr0-dev \
  libxcb-sync-dev libxcb-present-dev libxcb-dri3-dev \
  libxcb-xkb-dev libxkbcommon-dev libxkbcommon-x11-dev \
  libwayland-dev
