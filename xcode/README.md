# Warm Burnout for Xcode

Your IDE was also cooking your retinas. Now the damage is consistent across platforms.

## Install

Copy the theme files to your Xcode themes directory:

```sh
mkdir -p ~/Library/Developer/Xcode/UserData/FontAndColorThemes
cp "Warm Burnout Dark.xccolortheme" "Warm Burnout Light.xccolortheme" \
  ~/Library/Developer/Xcode/UserData/FontAndColorThemes/
```

Restart Xcode.

## Configure

Open **Settings** > **Themes**, then select **Warm Burnout Dark** or **Warm Burnout Light**.

## Verify

Open any Swift file. Keywords should be bold burnt orange, types should be italic steel patina, and your comments should actually be readable.

## Palette

Both themes derive from the canonical Warm Burnout palette defined in the root [`AGENTS.md`](../AGENTS.md). RGBA float values map directly from the hex palette.
