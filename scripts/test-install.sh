#!/usr/bin/env sh
set -eu

ROOT="$(CDPATH='' cd -- "$(dirname -- "$0")/.." && pwd)"
TEMP="$(mktemp -d "${TMPDIR:-/tmp}/memos-desktop-installer-test.XXXXXX")"
trap 'rm -rf "$TEMP"' EXIT HUP INT TERM

sha256() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
  else
    shasum -a 256 "$1" | awk '{print $1}'
  fi
}

mkdir -p "$TEMP/package"
printf '#!/usr/bin/env sh\nexit 0\n' >"$TEMP/package/memos-desktop"
chmod 755 "$TEMP/package/memos-desktop"
for document in LICENSE LICENSE-THIRD-PARTY.md NOTICE README.md; do
  cp "$ROOT/$document" "$TEMP/package/$document"
done

archive="$TEMP/memos-desktop-linux-x86_64.tar.gz"
tar -czf "$archive" -C "$TEMP/package" .
checksum="$(sha256 "$archive")"
printf '%s  %s\n' "$checksum" "memos-desktop-linux-x86_64.tar.gz" >"$TEMP/SHA256SUMS"

if HOME="$TEMP/missing-home" \
  MEMOS_DESKTOP_INSTALL_ROOT="$TEMP/missing-home/.local" \
  XDG_DATA_HOME="$TEMP/missing-home/.xdg/share" \
  MEMOS_DESKTOP_ARCHIVE_PATH="$archive" \
  MEMOS_DESKTOP_CHECKSUMS_PATH="$TEMP/SHA256SUMS" \
  MEMOS_DESKTOP_PLATFORM=linux \
  MEMOS_DESKTOP_ARCH=x86_64 \
  sh "$ROOT/scripts/install.sh" --update >/dev/null 2>&1; then
  echo "update unexpectedly succeeded without an existing installation" >&2
  exit 1
fi

HOME="$TEMP/linux-home" \
MEMOS_DESKTOP_INSTALL_ROOT="$TEMP/linux-home/.local" \
XDG_DATA_HOME="$TEMP/linux-home/.xdg/share" \
MEMOS_DESKTOP_ARCHIVE_PATH="$archive" \
MEMOS_DESKTOP_CHECKSUMS_PATH="$TEMP/SHA256SUMS" \
MEMOS_DESKTOP_PLATFORM=linux \
MEMOS_DESKTOP_ARCH=x86_64 \
  sh "$ROOT/scripts/install.sh"

linux_app="$TEMP/linux-home/.local/lib/memos-desktop"
test -x "$linux_app/memos-desktop"
grep -q '^0.1.0$' "$linux_app/.installer-managed"
test -L "$TEMP/linux-home/.local/bin/memos-desktop"
test -f "$TEMP/linux-home/.xdg/share/applications/com.markbang.MemosDesktop.desktop"

mkdir -p "$TEMP/update-package"
printf '#!/usr/bin/env sh\n# update-v2\nexit 0\n' >"$TEMP/update-package/memos-desktop"
chmod 755 "$TEMP/update-package/memos-desktop"
for document in LICENSE LICENSE-THIRD-PARTY.md NOTICE README.md; do
  cp "$ROOT/$document" "$TEMP/update-package/$document"
done
linux_update_archive="$TEMP/linux-update.tar.gz"
tar -czf "$linux_update_archive" -C "$TEMP/update-package" .
linux_update_checksum="$(sha256 "$linux_update_archive")"
printf '%s  %s\n' "$linux_update_checksum" "memos-desktop-linux-x86_64.tar.gz" >"$TEMP/LINUX_UPDATE_SHA256SUMS"
printf '%064d  %s\n' 0 "memos-desktop-linux-x86_64.tar.gz" >"$TEMP/BAD_UPDATE_SHA256SUMS"

if HOME="$TEMP/linux-home" \
  MEMOS_DESKTOP_INSTALL_ROOT="$TEMP/linux-home/.local" \
  XDG_DATA_HOME="$TEMP/linux-home/.xdg/share" \
  MEMOS_DESKTOP_ARCHIVE_PATH="$linux_update_archive" \
  MEMOS_DESKTOP_CHECKSUMS_PATH="$TEMP/BAD_UPDATE_SHA256SUMS" \
  MEMOS_DESKTOP_RESOLVED_VERSION=0.2.0 \
  MEMOS_DESKTOP_PLATFORM=linux \
  MEMOS_DESKTOP_ARCH=x86_64 \
  sh "$ROOT/scripts/install.sh" --update >/dev/null 2>&1; then
  echo "update unexpectedly accepted a mismatched checksum" >&2
  exit 1
fi
if grep -q 'update-v2' "$linux_app/memos-desktop"; then
  echo "failed update replaced the existing application" >&2
  exit 1
fi
grep -q '^0.1.0$' "$linux_app/.installer-managed"

rm "$TEMP/linux-home/.xdg/share/applications/com.markbang.MemosDesktop.desktop"
HOME="$TEMP/linux-home" \
MEMOS_DESKTOP_INSTALL_ROOT="$TEMP/linux-home/.local" \
XDG_DATA_HOME="$TEMP/linux-home/.xdg/share" \
MEMOS_DESKTOP_ARCHIVE_PATH="$linux_update_archive" \
MEMOS_DESKTOP_CHECKSUMS_PATH="$TEMP/LINUX_UPDATE_SHA256SUMS" \
MEMOS_DESKTOP_RESOLVED_VERSION=0.2.0 \
MEMOS_DESKTOP_PLATFORM=linux \
MEMOS_DESKTOP_ARCH=x86_64 \
  sh "$ROOT/scripts/install.sh" --update

grep -q 'update-v2' "$linux_app/memos-desktop"
grep -q '^0.2.0$' "$linux_app/.installer-managed"
test -L "$TEMP/linux-home/.local/bin/memos-desktop"
test ! -e "$TEMP/linux-home/.xdg/share/applications/com.markbang.MemosDesktop.desktop"

HOME="$TEMP/linux-home" \
MEMOS_DESKTOP_INSTALL_ROOT="$TEMP/linux-home/.local" \
XDG_DATA_HOME="$TEMP/linux-home/.xdg/share" \
MEMOS_DESKTOP_PLATFORM=linux \
  sh "$ROOT/scripts/install.sh" --uninstall

test ! -e "$linux_app"
test ! -e "$TEMP/linux-home/.local/bin/memos-desktop"

mac_archive="$TEMP/memos-desktop-macos-x86_64.tar.gz"
cp "$archive" "$mac_archive"
mac_checksum="$(sha256 "$mac_archive")"
printf '%s  %s\n' "$mac_checksum" "memos-desktop-macos-x86_64.tar.gz" >"$TEMP/MAC_SHA256SUMS"

HOME="$TEMP/mac-home" \
MEMOS_DESKTOP_INSTALL_ROOT="$TEMP/mac-home/.local" \
MEMOS_DESKTOP_APPLICATIONS_DIR="$TEMP/mac-home/Applications" \
MEMOS_DESKTOP_ARCHIVE_PATH="$mac_archive" \
MEMOS_DESKTOP_CHECKSUMS_PATH="$TEMP/MAC_SHA256SUMS" \
MEMOS_DESKTOP_PLATFORM=macos \
MEMOS_DESKTOP_ARCH=x86_64 \
  sh "$ROOT/scripts/install.sh"

app="$TEMP/mac-home/Applications/Memos Desktop.app"
test -x "$app/Contents/MacOS/memos-desktop"
grep -q '^0.1.0$' "$app/Contents/Resources/.installer-managed"
test -f "$app/Contents/Info.plist"
grep -q '<key>CFBundleShortVersionString</key><string>0.1.0</string>' "$app/Contents/Info.plist"
test -L "$TEMP/mac-home/.local/bin/memos-desktop"

mac_update_archive="$TEMP/macos-update.tar.gz"
tar -czf "$mac_update_archive" -C "$TEMP/update-package" .
mac_update_checksum="$(sha256 "$mac_update_archive")"
printf '%s  %s\n' "$mac_update_checksum" "memos-desktop-macos-x86_64.tar.gz" >"$TEMP/MAC_UPDATE_SHA256SUMS"
rm "$TEMP/mac-home/.local/bin/memos-desktop"
HOME="$TEMP/mac-home" \
MEMOS_DESKTOP_INSTALL_ROOT="$TEMP/mac-home/.local" \
MEMOS_DESKTOP_APPLICATIONS_DIR="$TEMP/mac-home/Applications" \
MEMOS_DESKTOP_ARCHIVE_PATH="$mac_update_archive" \
MEMOS_DESKTOP_CHECKSUMS_PATH="$TEMP/MAC_UPDATE_SHA256SUMS" \
MEMOS_DESKTOP_RESOLVED_VERSION=0.2.0 \
MEMOS_DESKTOP_PLATFORM=macos \
MEMOS_DESKTOP_ARCH=x86_64 \
  sh "$ROOT/scripts/install.sh" --update

grep -q 'update-v2' "$app/Contents/MacOS/memos-desktop"
grep -q '^0.2.0$' "$app/Contents/Resources/.installer-managed"
grep -q '<key>CFBundleShortVersionString</key><string>0.2.0</string>' "$app/Contents/Info.plist"
test ! -e "$TEMP/mac-home/.local/bin/memos-desktop"

HOME="$TEMP/mac-home" \
MEMOS_DESKTOP_INSTALL_ROOT="$TEMP/mac-home/.local" \
MEMOS_DESKTOP_APPLICATIONS_DIR="$TEMP/mac-home/Applications" \
MEMOS_DESKTOP_PLATFORM=macos \
  sh "$ROOT/scripts/install.sh" --uninstall

test ! -e "$app"
test ! -e "$TEMP/mac-home/.local/bin/memos-desktop"

echo "Installer tests passed."
