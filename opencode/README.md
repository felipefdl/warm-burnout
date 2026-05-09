# Warm Burnout for OpenCode

Your AI coding agent was also burning your retinas. Consistent damage, even in the conversation window.

## Install

Copy the theme file to your OpenCode themes directory:

```sh
mkdir -p ~/.config/opencode/themes
cp warm-burnout.json ~/.config/opencode/themes/
```

Or drop it into your project:

```sh
mkdir -p .opencode/themes
cp warm-burnout.json .opencode/themes/
```

## Configure

Set the theme in your `tui.json`:

```json
{
  "$schema": "https://opencode.ai/tui.json",
  "theme": "warm-burnout"
}
```

Or use the `/theme` command inside OpenCode and select `warm-burnout`.

## Both Variants Included

The theme file contains both dark and light variants. OpenCode picks the right one based on your terminal's background color.

## Palette

Both variants derive from the canonical Warm Burnout palette defined in the root [`AGENTS.md`](../AGENTS.md).
