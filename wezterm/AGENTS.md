# WezTerm Agent Instructions

Follow the root [`AGENTS.md`](../AGENTS.md). The canonical palette lives there; do not duplicate the palette tables here.

## Format

WezTerm standalone color schemes are TOML files with `[colors]` and `[metadata]` sections. Place them in `~/.config/wezterm/colors`; WezTerm loads the display name from `metadata.name` when present.

## Mapping

| WezTerm Key | Canonical Source |
|-------------|-----------------|
| `metadata.name` | `Warm Burnout Dark` / `Warm Burnout Light` |
| `colors.background` | Editor background (`#1a1510` dark, `#F5EDE0` light) |
| `colors.foreground` | Editor foreground (`#bfbdb6` dark, `#3a3630` light) |
| `colors.cursor_bg` / `cursor_border` | Canonical cursor (`#f5c56e` dark, `#8a6600` light) |
| `colors.cursor_fg` | Background, so block cursor text stays readable |
| `colors.selection_bg` / `selection_fg` | Ghostty selection background and foreground |
| `colors.ansi` | Ghostty `palette = 0` through `palette = 7` |
| `colors.brights` | Ghostty `palette = 8` through `palette = 15` |
| `colors.scrollbar_thumb` / `tab_bar.inactive_tab_edge` | Zellij `frame_unselected.base` stone (`#3a342a` dark, `#c5beb2` light) |
| `colors.split` | Canonical copper rust accent (`#b8522e`) |
| `colors.tab_bar.background` | Mirrors `colors.background` so the tab bar reads as part of the terminal |
| `colors.tab_bar.active_tab.bg_color` | Keywords token (`#ff8f40` dark, `#924800` light) |

The `ansi` and `brights` arrays must stay in conventional terminal order: black, red, green, yellow, blue, magenta, cyan, white.

## Metadata

Keep filenames lowercase and hyphenated:

- Dark theme: `warm-burnout-dark.toml`
- Light theme: `warm-burnout-light.toml`

Keep `metadata.name` in title case so users select `Warm Burnout Dark` or `Warm Burnout Light` in `wezterm.lua`.

## Chrome Discipline

WezTerm themes can color native surfaces: tab bar, copy mode, quick select, split lines, and scrollbar thumb. Keep those warm. The steel-blue type accent is reserved for syntax and should not appear in WezTerm chrome. ANSI blue is exempt because terminal programs depend on that slot.
