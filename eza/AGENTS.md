# eza -- Agent Instructions

## Platform Reference

See the root [`AGENTS.md`](../AGENTS.md) for the canonical palette, design principles, and brand rules. Do not duplicate palette tables here.

## eza Theme Format

- Theme files are YAML (`.yml`). eza reads `theme.yml` from `$EZA_CONFIG_DIR` (typically `~/.config/eza/`).
- Requires eza v0.19.2 or later.
- Must have `colourful: true` at the top level.
- Colors use quoted hex notation (`"#rrggbb"`).
- Comments: lines starting with `#`.

## File Structure

```
eza/
  dark.yml      # Dark variant
  light.yml     # Light variant
  README.md     # Install instructions
  AGENTS.md     # This file
```

Users copy their chosen variant to `~/.config/eza/theme.yml`. Only one variant active at a time.

## Color Mapping

eza themes style file listings (file types, permissions, sizes, git status, etc.), not editor syntax. The mapping uses canonical palette colors by semantic role:

| eza Element | Canonical Role | Dark Hex | Light Hex |
|-------------|---------------|----------|-----------|
| Normal files | Foreground | `#bfbdb6` | `#3a3630` |
| Directories | CSS properties | `#deb074` | `#74501c` |
| Symlinks | Regex/escape | `#96b898` | `#286a48` |
| Executables | Strings | `#b4bc78` | `#4d5c1a` |
| Mount points | Keywords | `#ff8f40` | `#924800` |
| Dates, punctuation | Comments | `#b4a89c` | `#544c40` |
| Permissions read | Decorators | `#e6c08a` | `#7a5a1c` |
| Permissions write | Member vars | `#ec9878` | `#883850` |
| Permissions execute | Strings | `#b4bc78` | `#4d5c1a` |
| Size (byte) | Foreground | `#bfbdb6` | `#3a3630` |
| Size (kilo) | Strings | `#b4bc78` | `#4d5c1a` |
| Size (mega) | Functions | `#ffb454` | `#855700` |
| Size (giga) | Member vars | `#ec9878` | `#883850` |
| Size (huge) | Error | `#f49090` | `#b03434` |
| Size units | Muted stone | `#ada69c` | `#6a6254` |
| User (you) | Decorators | `#e6c08a` | `#7a5a1c` |
| User (root) | Error | `#f49090` | `#b03434` |
| User (other) | Constants | `#d4a8b8` | `#7e4060` |
| Git new | Strings | `#b4bc78` | `#4d5c1a` |
| Git modified | Functions | `#ffb454` | `#855700` |
| Git deleted | Member vars | `#ec9878` | `#883850` |
| Git conflicted | Error | `#f49090` | `#b03434` |
| Source/compiled | Types | `#90aec0` | `#285464` |
| Header | Foreground + bold | `#bfbdb6` | `#3a3630` |

## Derived Colors

The dark theme uses `#ada69c` for muted UI chrome elements (inode, blocks, size units, permission attributes). This is not in the canonical palette but sits between the comment color and foreground. The light equivalent is `#6a6254`, keeping the same warm stone character.

## Section Structure

eza themes have two kinds of sections:

**Nested sections** (map-of-maps): `filekinds`, `perms`, `size`, `users`, `links`, `git`, `git_repo`, `security_context`, `file_type`. Each entry is a key with a `foreground` sub-key.

**Flat sections** (direct map): `punctuation`, `date`, `inode`, `blocks`, `header`, `octal`, `flags`, `symlink_path`, `control_char`, `broken_symlink`, `broken_path_overlay`. These have `foreground` directly. `header` also has `is_bold: true`.

## Rules

1. Both `dark.yml` and `light.yml` must have identical structure (same sections, same keys).
2. Use exact hex values from the canonical palette. No approximations.
3. The `colourful: true` flag is required at the top level.
4. `header` must have `is_bold: true`.
5. No top-level keys beyond the defined sections and `colourful`.
6. Keep the palette reference comment block at the top of each file.
