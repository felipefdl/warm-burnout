# Helix -- Agent Instructions

## Platform Reference

See the root [`AGENTS.md`](../AGENTS.md) for the canonical palette, design principles, and brand rules. Do not duplicate palette tables here.

## Helix Theme Format

- Theme files are TOML: `warm-burnout-dark.toml` and `warm-burnout-light.toml`.
- Placed in `~/.config/helix/themes/` (or `%AppData%\helix\themes` on Windows).
- A `[palette]` table at the bottom defines named colors referenced by scope assignments above.
- Scope keys use tree-sitter capture names (e.g., `keyword.control.repeat`).
- The longest matching key wins: `function.builtin` takes precedence over `function`.
- Modifiers: `bold`, `italic`, `underlined`, `crossed_out`, `dim`, `reversed`.
- Underline styles: `line`, `curl`, `dashed`, `dotted`, `double_line`.

## Files

```
helix/
  warm-burnout-dark.toml   -- Dark variant (self-contained)
  warm-burnout-light.toml  -- Light variant (self-contained)
  README.md                -- Install instructions
  AGENTS.md                -- This file
```

## Alpha Transparency

Helix runs in a terminal and does not support alpha hex values (`#rrggbbaa`). Where the canonical palette uses alpha (selection, gutter line numbers), pre-blended opaque values are used:

| Semantic | Source | Dark (over #1a1510) | Light (over #F5EDE0) |
|---|---|---|---|
| Selection bg | `#8aa8b840` | `#363a3a` | `#dadcd6` |
| Line highlight | `#222018` / `#00000014` | `#222018` | `#e2dace` |
| Line numbers | `#a59f96a6` / opaque | `#746f67` | `#8a8070` |

## Scope Mapping

Scope assignments mirror the Neovim highlight groups and VS Code textmate scopes:

- **bold** = keywords (`keyword.*`), storage, tags
- *italic* = types, comments, attributes (HTML), decorator-class scopes (function.macro, special), variable.builtin, inlay-hint.type
- normal = everything else

## Color Rules

1. Use exact hex values from the canonical palette. No approximations.
2. Selection uses the pre-blended neutral values listed above.
3. Both dark and light files are self-contained (no `inherits`).
4. Dark tokens >= 7.0:1 (AAA) against `#1a1510`. Light tokens >= 4.5:1 (AA) against `#F5EDE0`.
