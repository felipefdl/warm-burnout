#!/bin/sh
set -e
cd "$(dirname "$0")"

# Sanity-check generated files. Run `just jetbrains-build` first if any are missing.
for f in \
  "Warm Burnout Islands Dark.theme.json" \
  "Warm Burnout Islands Light.theme.json" \
  "Warm-Burnout-Dark.xml" \
  "Warm-Burnout-Light.xml"; do
  if [ ! -f "$f" ]; then
    echo "error: $f missing. Run 'just jetbrains-build' from the repo root first." >&2
    exit 1
  fi
done

rm -f warm-burnout-theme.jar
zip -q -r warm-burnout-theme.jar \
  META-INF/ \
  "Warm Burnout Islands Dark.theme.json" \
  "Warm Burnout Islands Light.theme.json" \
  Warm-Burnout-Dark.xml \
  Warm-Burnout-Light.xml
echo "Built: jetbrains/warm-burnout-theme.jar"
