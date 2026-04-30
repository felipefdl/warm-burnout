# Warm Burnout for Neovim

Your retinas asked nicely. Treesitter, LSP, and 200+ mostly warm highlight groups.

## Installation

### lazy.nvim

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

### Manual

Clone the repo and add the `nvim/` directory to your runtimepath:

```lua
vim.opt.rtp:append('/path/to/warm-burnout/nvim')
vim.cmd.colorscheme 'warm-burnout-dark'
```

## Variants

- `warm-burnout-dark`: AAA contrast, warm brown-black background (`#1a1510`)
- `warm-burnout-light`: AA contrast, sepia cream background (`#F5EDE0`)

## Lua API

```lua
require('warm-burnout').setup({ variant = 'dark' })
```

Or load directly:

```lua
require('warm-burnout').load('light')
```

## Supported Plugins

- Telescope
- Gitsigns
- Neo-tree
- Barbar
- Mini.statusline
- Which-key
- Trouble
- Flash
- Fidget
- Illuminate
- Indent-blankline
- Lazy
- Notify

## The Palette

Inspired by materials that age well. Unlike your eyes.

| Material | Dark | Light | Used for |
|----------|------|-------|----------|
| Amber | `#ffb454` | `#855700` | Functions |
| Burnt orange | `#ff8f40` | `#924800` | Keywords |
| Terra cotta | `#dc9e92` | `#8e4632` | HTML tags |
| Dried sage | `#b4bc78` | `#4d5c1a` | Strings |
| Verdigris | `#96b898` | `#286a48` | Regex, escapes |
| Dusty mauve | `#d4a8b8` | `#7e4060` | Numbers, constants |
| Coral | `#ec9878` | `#883850` | Member variables |
| Warm stone | `#b4a89c` | `#544c40` | Comments |
| Aged brass | `#deb074` | `#74501c` | CSS properties |
| Steel patina | `#90aec0` | `#285464` | Types, classes |
| Gold | `#e6c08a` | `#7a5a1c` | Decorators |

The burnout is spreading.
