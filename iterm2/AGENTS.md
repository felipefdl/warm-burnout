# iTerm2 -- Agent Instructions

## Platform Reference

See the root [`AGENTS.md`](../AGENTS.md) for the canonical palette, design principles, and brand rules. Do not duplicate palette tables here.

## iTerm2 Theme Format

- iTerm2 uses `.itermcolors` files -- XML plists conforming to Apple's PropertyList 1.0 DTD.
- Imported via double-click or **Settings** > **Profiles** > **Colors** > **Color Presets...** > **Import...**.
- Each color entry is a nested `<dict>` with `Red Component`, `Green Component`, `Blue Component`, `Alpha Component` (floats 0.0-1.0), and `Color Space` (`sRGB`).

## Color Representation

- Colors use floating-point RGB components in separate dict keys, not space-delimited strings.
- Convert from hex: divide each 8-bit component by 255.0, round to 6 decimal places.
- Example: `#ff8f40` -> Red: `1.000000`, Green: `0.560784`, Blue: `0.250980`
- Alpha is a separate key. Use `1` for fully opaque, fractional for transparency (e.g., cursor guide).

## Color Key Mapping

| iTerm2 Key | Canonical Source |
|------------|-----------------|
| `Ansi 0 Color` through `Ansi 15 Color` | Ghostty `palette = 0` through `palette = 15` |
| `Background Color` | Editor background (`#1a1510` dark, `#F5EDE0` light) |
| `Foreground Color` | Editor foreground (`#bfbdb6` dark, `#3a3630` light) |
| `Bold Color` | Same as foreground |
| `Cursor Color` | Canonical cursor (`#f5c56e` dark, `#8a6600` light) |
| `Cursor Text Color` | Same as background (readable cursor character) |
| `Selection Color` | Opaque selection blend (`#33393a` dark, `#e5e8e2` light) |
| `Selected Text Color` | Same as foreground |
| `Link Color` | Types color (`#8aa8b8` dark, `#2a5868` light) |
| `Cursor Guide Color` | Selection color with 0.25 alpha |

## File Naming

- Dark theme: `Warm Burnout Dark.itermcolors`
- Light theme: `Warm Burnout Light.itermcolors`
- Spaces in filenames are intentional -- iTerm2 derives the preset name from the filename.
