# Warm Burnout for Neovim

Your retinas asked nicely. Treesitter, LSP, and 200+ highlight groups of clinically warm, emotionally cold syntax highlighting.

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

- `warm-burnout-dark` -- AAA contrast, warm brown-black background (`#1a1510`)
- `warm-burnout-light` -- AA contrast, sepia cream background (`#F5EDE0`)

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
| Terra cotta | `#d49484` | `#8e4632` | HTML tags |
| Dried sage | `#b4bc78` | `#4d5c1a` | Strings |
| Verdigris | `#96b898` | `#286a48` | Regex, escapes |
| Dusty mauve | `#d4a8b8` | `#7e4060` | Numbers, constants |
| Coral | `#f58088` | `#a02838` | Member variables |
| Warm stone | `#aea195` | `#5a5244` | Comments |
| Aged brass | `#deb074` | `#74501c` | CSS properties |
| Steel patina | `#8aa8b8` | `#2a5868` | Types, classes |
| Gold | `#e6c08a` | `#7a5a1c` | Decorators |

The burnout is spreading.
