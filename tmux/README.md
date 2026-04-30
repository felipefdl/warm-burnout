# Warm Burnout for tmux

Your status bar was the last thing not burning in warm tones. Fixed that.

## Requirements

tmux 3.2+ (hex color support). Older versions silently ignore hex values and you get default colors, which defeats the point.

## Install

### Standalone (recommended)

Add to your `~/.tmux.conf`:

```sh
# Dark
source-file /path/to/warm-burnout/tmux/warm-burnout-dark.conf

# Light
source-file /path/to/warm-burnout/tmux/warm-burnout-light.conf
```

Reload with `tmux source-file ~/.tmux.conf` or restart tmux.

### TPM

Since this is a monorepo, point TPM at the subdirectory:

```sh
# In ~/.tmux.conf
set -g @warm-burnout-variant "dark"  # or "light"
run-shell /path/to/warm-burnout/tmux/warm-burnout.tmux
```

If you cloned the full repo, that path is wherever you put it. TPM's `set -g @plugin` syntax expects a standalone repo. Use `run-shell` directly for monorepo setups.

## Variant Switching

Change the source line or set the TPM option:

```sh
# Switch to light
set -g @warm-burnout-variant "light"
run-shell /path/to/warm-burnout/tmux/warm-burnout.tmux
```

## Verify

After loading, your status bar should show warm amber session names and burnt orange active windows. If everything is default gray, your tmux is too old or the path is wrong.

## What It Covers

tmux themes control UI chrome only: status bar, pane borders, messages, copy-mode highlights, and the clock. Terminal ANSI colors come from your terminal emulator (Ghostty, Alacritty, etc.), not from tmux.

## Palette

Both variants derive from the canonical Warm Burnout palette defined in the root [`AGENTS.md`](../AGENTS.md).
