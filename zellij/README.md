# Warm Burnout for Zellij

Your panes were the last frontier of cool gray. Conquered.

## Requirements

Zellij 0.40+ (modern theme spec with UI components). Older versions ignore most components and silently fall back to defaults.

## Install

### Themes folder (recommended)

Drop both files into your Zellij themes directory and pick a variant in your config:

```sh
cp warm-burnout-dark.kdl warm-burnout-light.kdl ~/.config/zellij/themes/
```

Then in `~/.config/zellij/config.kdl`:

```kdl
theme "warm-burnout-dark"  // or "warm-burnout-light"
```

Find your themes directory with `zellij setup --check` if the default path is wrong.

### Inline (for live editing)

Paste the contents of either file directly into `~/.config/zellij/config.kdl` and set:

```kdl
theme "warm-burnout-dark"
```

Zellij picks up changes in real time when the theme is in the main config.

### CLI

```sh
zellij options --theme warm-burnout-dark
```

## Variant Switching

Edit `theme "..."` in your config and reload with `Ctrl + o, w` (session manager) or restart the session.

## What It Covers

Zellij themes control UI chrome only: tabs, status bar ribbons, pane frames, command-pane exit codes, tables, lists, and multi-user player colors. Terminal ANSI colors come from your terminal emulator (Ghostty, Alacritty, etc.), not from Zellij.

The mapping mirrors the tmux theme: active tab in burnt orange, focused pane border in copper rust, mode highlight in functions amber. See [`AGENTS.md`](AGENTS.md) for the full chrome rationale.

## Verify

After loading, the active tab should be burnt orange with dark text, the focused pane frame should be copper, and entering PANE or TAB mode should turn the frame gold. If everything is default magenta-and-cyan, your Zellij is too old or the theme name is wrong.

## Palette

Both variants derive from the canonical Warm Burnout palette defined in the root [`AGENTS.md`](../AGENTS.md).
