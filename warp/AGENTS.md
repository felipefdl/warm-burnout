# Warp -- Agent Instructions

## Platform Reference

See the root [`AGENTS.md`](../AGENTS.md) for the canonical palette, design principles, and brand rules. Do not duplicate palette tables here.

## Warp Theme Format

- Warp custom themes are YAML files, one per variant. Spec: <https://docs.warp.dev/terminal/appearance/custom-themes>.
- Install path: `~/.warp/themes/` (macOS), `${XDG_DATA_HOME:-$HOME/.local/share}/warp-terminal/themes/` (Linux), `%APPDATA%\warp\Warp\data\themes\` (Windows).
- Themes are picked at runtime via **Settings** > **Appearance** > **Themes** > **Custom**.
- Hex values must include the `#` prefix. Both 6-digit hex and quoted strings are accepted; we quote everything for YAML safety.

## Color Key Mapping

| Warp key | Canonical Source |
|----------|-----------------|
| `accent` | Canonical copper rust `#b8522e` |
| `background` | Editor background (`#1a1510` dark, `#F5EDE0` light) |
| `foreground` | Editor foreground (`#bfbdb6` dark, `#3a3630` light) |
| `cursor` | Canonical cursor (`#f5c56e` dark, `#8a6600` light) |
| `details` | `darker` for dark, `lighter` for light |
| `terminal_colors.normal.{black..white}` | Ghostty `palette = 0` through `palette = 7` |
| `terminal_colors.bright.{black..white}` | Ghostty `palette = 8` through `palette = 15` |

The 8-key set in each ANSI block must be exactly `black, red, green, yellow, blue, magenta, cyan, white`. Order in the YAML file is irrelevant to Warp but kept conventional for readability.

## File Naming

- Dark theme: `warm-burnout-dark.yaml`
- Light theme: `warm-burnout-light.yaml`
- Lowercase hyphenated. Warp derives the display name from the `name:` key inside the file, not from the filename.

## Emphasis Color Discipline

Warp themes are pure terminal colors plus a single chrome accent. The chrome fields (`accent`, `background`, `foreground`, `cursor`) must stay warm. Steel-blue `#90aec0` (dark types) and `#285464` (light types) are reserved for editor types and must not appear in chrome.

ANSI blue (`terminal_colors.normal.blue` and `.bright.blue`) is exempt: programs depend on the standard 16-color slot for ANSI blue, and terminal programs may use this color in syntax-highlighted prompts and file listings. Drop the canonical ghostty values verbatim.

## Out of Scope

- `background_image`: not used in any Warm Burnout platform.
- `gradients`: not used.
- Submission to the public `warpdotdev/themes` community repo: separate effort.
