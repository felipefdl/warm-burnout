# Ghostty -- Agent Instructions

## Platform Reference

See the root [`AGENTS.md`](../AGENTS.md) for the canonical palette, design principles, and brand rules. Do not duplicate palette tables here.

## Ghostty Theme Format

- Theme files have **no file extension**. The filename is the theme name.
- Syntax: `key = value`, one per line.
- Comments: lines starting with `#`.
- Placed in `~/.config/ghostty/themes/`.

## Color Mapping

ANSI palette colors (`palette = 0` through `palette = 15`) map directly from the VS Code terminal ANSI values:

- `palette = 0` -- `terminal.ansiBlack`
- `palette = 1` -- `terminal.ansiRed`
- `palette = 2` -- `terminal.ansiGreen`
- `palette = 3` -- `terminal.ansiYellow`
- `palette = 4` -- `terminal.ansiBlue`
- `palette = 5` -- `terminal.ansiMagenta`
- `palette = 6` -- `terminal.ansiCyan`
- `palette = 7` -- `terminal.ansiWhite`
- `palette = 8` through `palette = 15` -- the bright variants in the same order.

Non-ANSI colors:

- `background` -- uses the **editor background** (`#1a1510` dark, `#F5EDE0` light), not the VS Code terminal background. Ghostty is a standalone app; the editor surface is the canonical brand color.
- `foreground` -- `editor.foreground`.
- `cursor-color` -- the canonical cursor color from the root palette.
- `cursor-text` -- matches `background` so the cursor character is readable.
- `selection-background` / `selection-foreground` -- from VS Code `editor.selectionBackground`.

## Selection Color Note

Ghostty may not support alpha channels in selection colors (e.g., `#8aa8b840`). If alpha is not supported, use opaque blends:

- Dark: `#33393a` (result of `#8aa8b8` at 25% over `#1a1510`)
- Light: `#e5e8e2` (result of `#8aa8b8` at 25% over `#F5EDE0`)

Test after changes. If the selection looks wrong, switch to the opaque values.

## File Naming

- Dark theme: `warm-burnout-dark`
- Light theme: `warm-burnout-light`
- No extensions. Filenames are the theme identifiers used in Ghostty config.
