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

## Role to Canonical Token Mapping

OpenCode uses semantic color roles. Each role maps to a canonical palette token defined in the root `AGENTS.md`. Hex values live in `warm-burnout.json` and the canonical palette tables, not here.

### Core UI

- `primary`, `syntaxKeyword`, `markdownHeading`, `markdownStrong` <- Keywords/storage
- `secondary`, `syntaxFunction`, `markdownLinkText`, `markdownImageText` <- Functions
- `accent`, `borderActive`, `warning` <- Brand accent
- `text`, `syntaxVariable`, `syntaxPunctuation`, `markdownText`, `markdownCodeBlock` <- Foreground
- `textMuted`, `syntaxComment`, `markdownBlockQuote`, `diffContext`, `diffHunkHeader` <- Comments
- `background` <- Editor background
- `backgroundPanel`, `diffContextBg` <- Panel/sidebar background (dimmer than editor)
- `backgroundElement` <- Interactive element background

### Status

- `error` <- Error/invalid token
- `success`, `markdownCode` <- Strings (warm green)
- `info`, `markdownLink`, `markdownImage` <- Types accent (the one cool color)

### Syntax

`syntaxKeyword`, `syntaxFunction`, `syntaxString`, `syntaxNumber`, `syntaxType`, `syntaxOperator`, `syntaxComment` map directly to the canonical token of the same name. `syntaxVariable` and `syntaxPunctuation` map to foreground (see design decision 4).

### Diff

Diff foreground colors use ANSI red/green from the terminal palette (programs depend on conventional red/green for diffs); these match the values in `ghostty/warm-burnout-{dark,light}` palette indices 1, 2, 9, 10. Diff backgrounds are subtle ~12% tints of the diff foreground blended over the editor background -- terminal apps cannot rely on alpha compositing.

### Markdown

- `markdownEmph` <- Decorators
- `markdownListItem` <- Operators
- `markdownListEnumeration` <- Constants/numbers
- `markdownHorizontalRule` <- Border (derived chrome)

## File Naming

- Single file: `warm-burnout.json`
- No variant suffix needed: both dark and light are in one file.

## Design Decisions

1. `primary` uses keywords (burnt orange) rather than the brand accent. Keywords are the most visually prominent token and map well to primary interactive elements.
2. `warning` reuses the brand accent. The accent already reads as a warm caution tone.
3. `info` uses the types accent (steel patina). This is the single cool hue in the palette, reserved for informational/neutral highlights.
4. `syntaxVariable` and `syntaxPunctuation` map to foreground rather than a distinct color. They are the most common tokens; coloring them differently from base text adds noise without information.
5. Diff backgrounds are subtle tints computed as opaque blends (terminal apps cannot rely on alpha compositing). Light theme `backgroundElement` and `backgroundPanel` collapse to the same value because the light palette has less luminance headroom for separating multiple chrome levels.
