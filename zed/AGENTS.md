# Zed -- Agent Instructions

## Platform Reference

See the root [`AGENTS.md`](../AGENTS.md) for the canonical palette, design principles, and brand rules. Do not duplicate palette tables here.

## Zed Theme Format

- Zed uses a **Theme Family** JSON format: a single JSON file containing both dark and light variants.
- Top-level keys: `$schema`, `name`, `author`, `themes` (array).
- Each theme object: `name`, `appearance` (`"dark"` or `"light"`), `style` (all colors and syntax).
- Schema reference: `https://zed.dev/schema/themes/v0.2.0.json`.

## Syntax Highlighting

- Zed uses **Tree-sitter** for syntax highlighting, not TextMate scopes.
- Syntax keys are flat strings like `keyword`, `function`, `type`, `string`, `comment`, etc.
- Dotted keys for sub-categories: `string.regex`, `string.escape`, `variable.member`, `type.builtin`, etc.
- Each syntax entry is an object with `color` (string), optional `font_style` (`"italic"`, `"oblique"`, `"normal"`), and optional `font_weight` (100-900).

## Style Structure

- UI colors are flat keys in `style`: `background`, `text`, `editor.background`, `editor.foreground`, etc.
- `syntax` is a nested object within `style` containing all syntax token definitions.
- `players` is an array of objects with `cursor`, `selection`, and `background` keys.
- Terminal colors: `terminal.background`, `terminal.foreground`, `terminal.ansi.<color>`, `terminal.ansi.bright_<color>`.

## Surface Hierarchy (Dark)

| Surface | Hex | Usage |
|---------|-----|-------|
| `surface.background` / `panel.background` | `#14120f` | Sidebar, panels, status bar, tab bar, title bar |
| `background` / `editor.background` | `#1a1510` | Editor, toolbar |
| `elevated_surface.background` | `#1f1d17` | Widgets, popovers, dropdowns |

## Color Format

- Zed supports 8-digit hex (RRGGBBAA) for alpha transparency.
- Use alpha values directly -- no need to pre-blend against backgrounds.

## File Structure

```
zed/
  extension.toml              # Extension manifest
  LICENSE                     # Required by Zed extension registry
  README.md                   # Install instructions
  AGENTS.md                   # This file
  themes/
    warm-burnout.json         # Theme family (dark + light in one file)
```

## Publishing

Zed extensions are published by adding a git submodule to `zed-industries/extensions`:

1. Fork `zed-industries/extensions`
2. Add submodule: `git submodule add https://github.com/felipefdl/warm-burnout.git extensions/warm-burnout`
3. Add entry to `extensions.toml` pointing to the `zed/` subdirectory
4. Open PR to `zed-industries/extensions`

Each version update requires a PR to `zed-industries/extensions` that bumps the submodule pointer and the version in the central `extensions.toml`. See the release checklist in the root `AGENTS.md`.
