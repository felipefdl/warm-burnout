# Neovim -- Agent Instructions

## Platform Reference

See the root [`AGENTS.md`](../AGENTS.md) for the canonical palette, design principles, and brand rules. Do not duplicate palette tables here.

## Neovim Colorscheme Format

- Neovim colorschemes use Lua files in `colors/` and `lua/` directories.
- Color files in `colors/` are entry points that call `require("warm-burnout").load(variant)`.
- Highlight groups are set via `vim.api.nvim_set_hl(0, name, opts)`.
- Terminal ANSI colors use `vim.g.terminal_color_N` (0-15).

## Structure

```
nvim/
  colors/
    warm-burnout-dark.lua     -- Entry point for dark variant
    warm-burnout-light.lua    -- Entry point for light variant
  lua/
    warm-burnout/
      init.lua                -- setup(), load(variant)
      palette.lua             -- Dark + light palette tables
      highlights.lua          -- All highlight group definitions
      terminal.lua            -- Terminal ANSI colors (16 colors)
  README.md                   -- Install instructions
  AGENTS.md                   -- This file
```

## Installation

Since the repo is a monorepo (not a pure Neovim plugin), users add the `nvim/` subdirectory to their runtimepath. With lazy.nvim:

```lua
{
  'felipefdl/warm-burnout',
  priority = 1000,
  config = function(plugin)
    vim.opt.rtp:append(plugin.dir .. '/nvim')
    vim.cmd.colorscheme 'warm-burnout-dark'
  end,
}
```

## Palette Mapping

- `palette.lua` contains `M.dark` and `M.light` tables mapping semantic names to hex values.
- All hex values come directly from the canonical palette in root `AGENTS.md`.
- UI surface hierarchy: `bg_dim` (panels) < `bg` (editor) < `bg_float` (floating windows).

## Highlight Groups

- `highlights.lua` takes a palette table and returns a flat table of `{ GroupName = { opts } }`.
- Groups cover: editor UI, legacy syntax, treesitter captures, LSP diagnostics, LSP semantic tokens, and plugin integrations.
- Font style system: `bold` = keywords/storage, `italic` = types/comments, normal = everything else.

## Adding Plugin Support

1. Add highlight groups to `highlights.lua` in the appropriate section.
2. Use palette semantic names, not raw hex values.
3. Follow existing patterns for foreground/background assignment.

## Color Rules

1. Use exact hex values from the canonical palette. No approximations.
2. Selection uses `#8aa8b840` (steel patina with alpha) matching VS Code and Zed.
3. Terminal ANSI colors match the Ghostty/VS Code terminal colors exactly.
4. Alpha transparency in hex (e.g., `#rrggbbaa`) is supported by Neovim's `termguicolors`.
