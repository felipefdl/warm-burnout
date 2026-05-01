# Warm Burnout for Helix

Your terminal editor was also burning your retinas. Consistent damage, even in modal editing.

## Install

Copy the theme files to your Helix themes directory:

```sh
mkdir -p ~/.config/helix/themes
cp warm-burnout-dark.toml warm-burnout-light.toml ~/.config/helix/themes/
```

## Configure

Add to the top of your `~/.config/helix/config.toml`:

```toml
theme = "warm-burnout-dark"
```

Or for the light variant:

```toml
theme = "warm-burnout-light"
```

Switch at runtime with `:theme warm-burnout-dark` or `:theme warm-burnout-light`.

## What you get

- Full syntax highlighting for every tree-sitter grammar Helix supports
- Editor UI: statusline, bufferline, popups, menus, picker, gutter
- Mode-aware statusline colors (normal, insert, select)
- Diagnostic underlines (hint, info, warning, error)
- Diff gutter indicators
- Inlay hints styled to stay out of your way
- Cursor matching, jump labels, indent guides

## Palette

Both themes derive from the canonical Warm Burnout palette defined in the root [`AGENTS.md`](../AGENTS.md). Pre-blended opaque values are used where the canonical palette specifies alpha transparency (selection, line numbers), since Helix runs in a terminal.
