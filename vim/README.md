# Warm Burnout for Vim

Your retinas asked nicely. A classic Vim colorscheme -- no Lua, no Treesitter, no runtime dependencies. Just 200+ highlight groups of clinically warm, emotionally cold syntax highlighting.

Requires Vim 8.0 or newer. For Neovim, use the [`nvim/`](../nvim/) platform instead.

## Installation

### vim-plug

```vim
Plug 'felipefdl/warm-burnout', { 'rtp': 'vim' }
```

Then in your `vimrc`:

```vim
set termguicolors
colorscheme warm-burnout-dark
```

### Vim 8 packages (native)

```sh
mkdir -p ~/.vim/pack/themes/start
git clone https://github.com/felipefdl/warm-burnout ~/.vim/pack/themes/start/warm-burnout
```

Then symlink or copy the `vim/colors/` directory into your runtimepath:

```sh
mkdir -p ~/.vim/colors
cp ~/.vim/pack/themes/start/warm-burnout/vim/colors/*.vim ~/.vim/colors/
```

### Manual

Drop `warm-burnout-dark.vim` and `warm-burnout-light.vim` into `~/.vim/colors/`.

## Usage

```vim
set termguicolors
colorscheme warm-burnout-dark
" or
colorscheme warm-burnout-light
```

`termguicolors` is strongly recommended. Without it, Vim falls back to the nearest 256-color approximation, which loses the carefully tuned contrast ratios.

## Variants

- `warm-burnout-dark`: AAA contrast, warm brown-black background (`#1a1510`)
- `warm-burnout-light`: AA contrast, sepia cream background (`#F5EDE0`)

## Supported Plugins

- ALE
- coc.nvim
- vim-gitgutter
- vim-signify
- NERDTree
- netrw

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
