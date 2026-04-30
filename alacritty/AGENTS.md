# Alacritty -- Agent Instructions

## Platform Reference

See the root [`AGENTS.md`](../AGENTS.md) for the canonical palette, design principles, and brand rules. Do not duplicate palette tables here.

## Alacritty Theme Format

- Theme files are TOML (`.toml`). Alacritty switched its config format from YAML to TOML in 0.13 and dropped YAML entirely in 0.14.
- Imported by another config via `general.import` (Alacritty >= 0.15) or top-level `import` (older versions). Both forms accept absolute paths or `~`-prefixed paths.
- Conventional install location: `~/.config/alacritty/themes/`. On Windows: `%APPDATA%\alacritty\themes\`.

## Color Mapping

Core blocks (`primary`, `cursor`, `selection`, `normal`, `bright`) come verbatim from the Ghostty + Windows Terminal themes so all sibling terminal emulators render identically:

| Alacritty Key | Canonical Source |
|---------------|-----------------|
| `colors.primary.background` | Editor background (`#1a1510` dark, `#f5ede0` light) |
| `colors.primary.foreground` | Editor foreground (`#bfbdb6` dark, `#3a3630` light) |
| `colors.primary.bright_foreground` | Same as foreground -- bold must not punch up to harsh white |
| `colors.cursor.cursor` | Canonical cursor (`#f5c56e` dark, `#8a6600` light) |
| `colors.cursor.text` | Same as background |
| `colors.selection.background` | Opaque selection blend (`#33393a` dark, `#e5e8e2` light) |
| `colors.selection.text` | Same as foreground |
| `colors.normal.*` / `colors.bright.*` | Ghostty `palette = 0..15` |

Extended UI uses canonical palette hex values only. No off-palette tints:

| Alacritty Key | Canonical Source |
|---------------|-----------------|
| `colors.vi_mode_cursor.cursor` / `.text` | Canonical cursor / background |
| `colors.search.matches` | Functions token (`#ffb454` dark / `#855700` light) on background |
| `colors.search.focused_match` | Keywords token (`#ff8f40` dark / `#924800` light) on background |
| `colors.footer_bar.foreground` | Comments token (`#b4a89c` dark / `#544c40` light) |
| `colors.footer_bar.background` | Recessed chrome below editor bg (`#14120f` dark / `#ede6da` light) |
| `colors.hints.start` / `.hints.end` | Types token (`#90aec0` dark / `#285464` light) -- the one sanctioned cool accent, matches iTerm2 Link Color |

## Selection Color Note

Alacritty does not honour alpha in `colors.selection.background`. Use the opaque blends listed above (the same values Ghostty uses for the same reason). They are pre-mixed approximations of the steel-patina selection at 25% over the editor background.

## Bright Foreground Note

`bright_foreground` is set explicitly to the same value as `foreground`. When `draw_bold_text_with_bright_colors` is enabled (Alacritty's default for some setups), bold text would otherwise jump to `bright.white`, which is `#ffffff` on dark and a near-white on light. Both punch through the warm palette. Mirroring iTerm2's Bold Color choice keeps bold weight visible without the luminance spike.

## File Naming

- Dark theme: `warm-burnout-dark.toml`
- Light theme: `warm-burnout-light.toml`
- Filenames double as the import path -- keep them stable.
