# Warm Burnout for Alacritty

Even the GPU-rendered terminal was burning your retinas. The burnout is spreading.

## Install

Drop the TOML files into Alacritty's themes directory:

```sh
mkdir -p ~/.config/alacritty/themes
cp warm-burnout-dark.toml warm-burnout-light.toml ~/.config/alacritty/themes/
```

Windows: `%APPDATA%\alacritty\themes\`.

## Configure

Add an import to your `~/.config/alacritty/alacritty.toml`.

Alacritty 0.15 and newer:

```toml
[general]
import = ["~/.config/alacritty/themes/warm-burnout-dark.toml"]
```

Alacritty 0.13 and 0.14:

```toml
import = ["~/.config/alacritty/themes/warm-burnout-dark.toml"]
```

Swap `warm-burnout-dark.toml` for `warm-burnout-light.toml` when you give in to daylight. Alacritty live-reloads the config on save, so no restart needed.

## Verify

Run `ls --color` or any ANSI color test script. Warm browns instead of searing blues. Vi-mode search highlights should glow amber, hints should land on the canonical copper-patina type accent.

## Palette

Both themes derive from the canonical Warm Burnout palette defined in the root [`AGENTS.md`](../AGENTS.md). The 16 ANSI colors and the editor background, foreground, and cursor match the Ghostty theme exactly. The extended UI block (`vi_mode_cursor`, `search`, `footer_bar`, `hints`) uses canonical palette hex values only, with no off-palette tints.
