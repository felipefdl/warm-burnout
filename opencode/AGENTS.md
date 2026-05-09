# OpenCode -- Agent Instructions

## Platform Reference

See the root [`AGENTS.md`](../AGENTS.md) for the canonical palette, design principles, and brand rules. Do not duplicate palette tables here.

## OpenCode Theme Format

- Single JSON file with `$schema`, `defs`, and `theme` sections.
- `defs` contains named color definitions (hex values) that can be referenced by name in the `theme` section.
- `theme` contains semantic color roles. Each role is an object with `dark` and `light` keys for variant-specific values, or a single value/reference for both.
- Both variants live in one file: `warm-burnout.json`.
- Placed in `~/.config/opencode/themes/` (user-wide) or `.opencode/themes/` (project-local).
- Schema: `https://opencode.ai/theme.json`.

## Color Mapping

OpenCode uses semantic color roles rather than raw palette indices. The mapping from canonical palette to OpenCode roles:

### Core UI

| Role | Dark | Light | Canonical source |
|------|------|-------|-----------------|
| `primary` | `#ff8f40` | `#924800` | Keywords |
| `secondary` | `#ffb454` | `#855700` | Functions |
| `accent` | `#b8522e` | `#b8522e` | Brand accent |
| `text` | `#bfbdb6` | `#3a3630` | Foreground |
| `textMuted` | `#b4a89c` | `#544c40` | Comments |
| `background` | `#1a1510` | `#F5EDE0` | Editor background |
| `backgroundPanel` | `#14120f` | `#EDE6DA` | Panel/sidebar background |
| `backgroundElement` | `#24201a` | `#EDE6DA` | Interactive element background |

### Status

| Role | Dark | Light | Canonical source |
|------|------|-------|-----------------|
| `error` | `#f49090` | `#b03434` | Error/invalid token |
| `warning` | `#b8522e` | `#b8522e` | Brand accent |
| `success` | `#b4bc78` | `#4d5c1a` | Strings (warm green) |
| `info` | `#90aec0` | `#285464` | Types accent (the one cool color) |

### Syntax

| Role | Dark | Light | Canonical source |
|------|------|-------|-----------------|
| `syntaxKeyword` | `#ff8f40` | `#924800` | Keywords/storage |
| `syntaxFunction` | `#ffb454` | `#855700` | Functions |
| `syntaxVariable` | `#bfbdb6` | `#3a3630` | Foreground |
| `syntaxString` | `#b4bc78` | `#4d5c1a` | Strings |
| `syntaxNumber` | `#d4a8b8` | `#7e4060` | Constants/numbers |
| `syntaxType` | `#90aec0` | `#285464` | Types/classes |
| `syntaxOperator` | `#f29668` | `#8f4418` | Operators |
| `syntaxComment` | `#b4a89c` | `#544c40` | Comments |
| `syntaxPunctuation` | `#bfbdb6` | `#3a3630` | Foreground |

### Diff

Diff foreground colors use ANSI red/green from the terminal palette (programs depend on conventional red/green for diffs). Diff backgrounds are opaque blends of those colors over the editor background at ~12% opacity.

### Markdown

Markdown roles map to syntax tokens for visual consistency: headings and strong text use keywords, links use types, code uses strings, emphasis uses decorators, list markers use operators.

## File Naming

- Single file: `warm-burnout.json`
- No variant suffix needed: both dark and light are in one file.

## Design Decisions

1. `primary` uses keywords (burnt orange) rather than the brand accent (`#b8522e`). Keywords are the most visually prominent token and map well to primary interactive elements.
2. `warning` reuses the brand accent. The accent already reads as a warm caution tone.
3. `info` uses the types accent (steel patina). This is the single cool hue in the palette, reserved for informational/neutral highlights.
4. `syntaxVariable` maps to foreground rather than a distinct color. Variables are the most common token; coloring them differently from base text adds noise without information.
5. Diff backgrounds are subtle tints computed as opaque blends (terminal apps cannot rely on alpha compositing).
