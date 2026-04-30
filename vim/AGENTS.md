# Vim -- Agent Instructions

## Platform Reference

See the root [`AGENTS.md`](../AGENTS.md) for the canonical palette, design principles, and brand rules. Do not duplicate palette tables here.

## Scope

This platform is for classic Vim (Vim 8+). Neovim users should use [`nvim/`](../nvim/) instead, which supports Treesitter, LSP semantic tokens, and Lua-based plugin integrations.

## Vim Colorscheme Format

- Vim colorschemes are single `.vim` files loaded from `colors/` on the runtimepath.
- Each variant is self-contained: it resets highlights, sets `g:colors_name`, and issues `highlight` commands.
- Both `guifg`/`guibg` (for `termguicolors`) and `ctermfg`/`ctermbg` (for 256-color terminals) are set on every group.
- Font style attributes (`bold`, `italic`, `underline`, `undercurl`) are set via `gui=` and `cterm=`.
- Terminal ANSI colors are set via `g:terminal_ansi_colors` (a 16-element list).

## Structure

```
vim/
  colors/
    warm-burnout-dark.vim     -- Self-contained dark variant
    warm-burnout-light.vim    -- Self-contained light variant
  README.md                   -- Install instructions
  AGENTS.md                   -- This file
```

## Dual-Color Convention

Every `highlight` command sets both:

- `guifg=#rrggbb` for truecolor terminals (`set termguicolors`)
- `ctermfg=N` (0-255) for 256-color terminals

The 256-color indices are perceptual approximations of the canonical hex values. They lose some fidelity (256 colors cannot reproduce the exact WCAG-tuned ratios), but they keep the overall warm palette coherent.

## Font Style System

The three-tier non-color discrimination channel from root `AGENTS.md` is preserved:

- `gui=bold cterm=bold` for keywords, storage classes, HTML tags
- `gui=italic cterm=italic` for types, comments, CSS properties, decorators
- No style (normal) for everything else

This must apply to both the GUI and cterm variants.

## Color Rules

1. Use exact hex values from the canonical palette for all `guifg`/`guibg`. No approximations.
2. 256-color (`cterm*`) indices are nearest-neighbor approximations. Changing them is acceptable as long as the perceptual role is preserved.
3. Terminal ANSI colors (`g:terminal_ansi_colors`) match the Ghostty/VS Code terminal palette exactly.
4. No alpha transparency: Vim does not support it. Use pre-blended opaque equivalents.

## Adding Plugin Support

1. Add `highlight` commands for the plugin's groups in both variant files.
2. Use the same semantic color (keyword = `#ff8f40` dark / `#924800` light, etc.) as other platforms.
3. Always set both GUI and cterm attributes.
4. Keep plugin sections ordered by plugin name for searchability.

## Testing

Run `cargo test --test vim` to validate the colorscheme files. Tests check for:

- Canonical palette colors present in both variants
- Required highlight groups (editor UI, syntax, diagnostics, git)
- Font style system preserved (keywords bold, types italic, comments italic)
- Terminal ANSI arrays have 16 entries
- `g:colors_name` set correctly per variant
