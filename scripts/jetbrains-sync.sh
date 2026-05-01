#!/usr/bin/env bash
set -euo pipefail

PINNED_FILE="jetbrains/UPSTREAM_REVISION.txt"
if [[ ! -f "$PINNED_FILE" ]]; then
  echo "error: $PINNED_FILE not found. Run from the repo root." >&2
  exit 1
fi
PINNED=$(cat "$PINNED_FILE")
REPO="https://github.com/JetBrains/rider-theme-pack"

WATCHED=(
  "src/main/resources/RiderDark.theme.json"
  "src/main/resources/RiderLight.theme.json"
  "src/main/resources/colorSchemes/ReSharperDark.xml"
  "src/main/resources/colorSchemes/ReSharperLight.xml"
)

TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

echo "Cloning $REPO ..."
git clone --quiet "$REPO" "$TMP/upstream"
cd "$TMP/upstream"

UPSTREAM_HEAD=$(git rev-parse HEAD)

cat <<INFO

Pinned revision: $PINNED
Upstream HEAD:   $UPSTREAM_HEAD
Watched files:
INFO
for f in "${WATCHED[@]}"; do
  echo "  $f"
done

if [[ "$PINNED" == "$UPSTREAM_HEAD" ]]; then
  echo
  echo "Pinned == HEAD; nothing to sync."
  exit 0
fi

echo
echo "=== Diff stat ==="
git diff --stat "$PINNED" -- "${WATCHED[@]}" || true

echo
echo "=== Attribute keys added since pinned revision ==="
git diff "$PINNED" -- 'src/main/resources/colorSchemes/*.xml' \
  | grep -E '^\+\s*<option name="[A-Z_][^"]*"' \
  | sed -E 's/.*name="([^"]+)".*/\1/' \
  | sort -u \
  | uniq \
  || echo "(none)"

echo
echo "=== UI keys added since pinned revision ==="
git diff "$PINNED" -- 'src/main/resources/Rider*.theme.json' \
  | grep -E '^\+\s*"[A-Za-z][A-Za-z0-9.]*":' \
  | sed -E 's/^\+\s*"([^"]+)":.*/\1/' \
  | sort -u \
  | uniq \
  || echo "(none)"

cat <<NEXT

Review the diffs above. To accept upstream changes:
  1. Add or update template entries in jetbrains/templates/.
  2. Update jetbrains/UPSTREAM_REVISION.txt to:
     $UPSTREAM_HEAD
  3. Run \`just jetbrains-build\` and \`cargo test jetbrains\`.
  4. Commit.
NEXT
