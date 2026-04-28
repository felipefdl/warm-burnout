# Warm Burnout for Warp

The AI terminal also wants to roast your retinas. Not on our watch.

## Install

Drop the YAML files into Warp's themes directory:

```sh
mkdir -p ~/.warp/themes
cp warm-burnout-dark.yaml warm-burnout-light.yaml ~/.warp/themes/
```

Linux: `${XDG_DATA_HOME:-$HOME/.local/share}/warp-terminal/themes/`. Windows: `%APPDATA%\warp\Warp\data\themes\`.

## Configure

Open Warp, then **Settings** > **Appearance** > **Themes** > **Custom**:

- **Warm Burnout Dark**: for the 3am sessions
- **Warm Burnout Light**: for when someone forces you to open the blinds

## Verify

Run `ls --color` or any ANSI test script. Warm browns instead of searing blues. The accent on the tab bar should be the canonical copper rust.

## Palette

Both themes derive from the canonical Warm Burnout palette defined in the root [`AGENTS.md`](../AGENTS.md). The 16 ANSI colors and the editor background, foreground, and cursor match the Ghostty theme exactly.
