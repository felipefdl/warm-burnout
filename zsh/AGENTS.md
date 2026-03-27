# Zsh -- Agent Instructions

## Platform Reference

See the root [`AGENTS.md`](../AGENTS.md) for the canonical palette, design principles, and brand rules. Do not duplicate palette tables here.

## Deliverables

Two theme types, each with dark and light variants:

### zsh-syntax-highlighting themes (`.zsh-theme`)

Set `ZSH_HIGHLIGHT_STYLES` entries using `fg=#RRGGBB` truecolor values. Must be sourced **after** the zsh-syntax-highlighting plugin.

### fzf color schemes (`-fzf.zsh`)

Export `FZF_DEFAULT_OPTS` with `--color` flags. Must be sourced **after** fzf initialization.

## File Naming

- `warm-burnout-dark.zsh-theme` -- syntax highlighting, dark
- `warm-burnout-light.zsh-theme` -- syntax highlighting, light
- `warm-burnout-dark-fzf.zsh` -- fzf, dark
- `warm-burnout-light-fzf.zsh` -- fzf, light

## Semantic Color Mapping

Zsh syntax tokens map to the canonical palette roles:

| Token Group | Palette Role | Dark | Light |
|-------------|-------------|------|-------|
| `command`, `function`, `alias`, `suffix-alias`, `global-alias`, `hashed-command` | Functions | `#ffb454` | `#855700` |
| `reserved-word`, `builtin`, `precommand` | Keywords (bold) | `#ff8f40` | `#924800` |
| `single-quoted-argument`, `double-quoted-argument`, `dollar-quoted-argument` | Strings | `#b4bc78` | `#4d5c1a` |
| `*-unclosed` variants, `unknown-token` | Error/invalid | `#f49090` | `#b03434` |
| `commandseparator`, `assign`, `redirection`, `path_pathseparator`, `path_prefix_pathseparator`, `back-quoted-argument-delimiter` | Operators | `#f29668` | `#8f4418` |
| `single-hyphen-option`, `double-hyphen-option` | Types (cool accent) | `#90aec0` | `#285464` |
| `path`, `autodirectory` | Decorators | `#e6c08a` | `#7a5a1c` |
| `path_prefix` | Decorators (underline) | `#e6c08a` | `#7a5a1c` |
| `globbing` | Regex/escape | `#96b898` | `#286a48` |
| `rc-quote`, `dollar-double-quoted-argument`, `back-double-quoted-argument` | Regex/escape | `#96b898` | `#286a48` |
| `history-expansion`, `back-quoted-argument`, `named-fd`, `numeric-fd` | Constants | `#d4a8b8` | `#7e4060` |
| `comment` | Comments (italic) | `#b4a89c` | `#544c40` |
| `default`, `arg0` | Foreground | `#bfbdb6` | `#3a3630` |

## Derived UI Colors (fzf)

Colors not directly in the canonical palette, derived from VS Code UI:

| fzf element | Dark | Light | Source |
|-------------|------|-------|--------|
| `bg` | `#1a1510` | `#F5EDE0` | Editor background |
| `bg+` | `#222018` | `#EDE6DA` | `editor.lineHighlightBackground` / `panel.background` |
| `border`, `separator` | `#302a22` | `#DDD6CA` | Opaque warm gray / `panel.border` |
| `gutter` | `#1a1510` | `#F5EDE0` | Editor background |
| `pointer` | `#f5c56e` | `#8a6600` | Cursor color |

## Source Order Dependency

Both theme types depend on sourcing order in `.zshrc`:

1. zsh-syntax-highlighting themes: source **after** the zsh-syntax-highlighting plugin
2. fzf themes: source **after** fzf initialization (`eval "$(fzf --zsh)"`)

This is because the themes set variables that the tools read at runtime.

## Design Decisions

1. **Options as cool accent**: `--flags` use the Types color (steel-blue). Shell commands are all warm -- the cool accent creates instant visual contrast.
2. **Paths as Decorators**: Warm gold reads as "reference/metadata" -- paths reference the filesystem, not code logic.
3. **No prompt theme**: Too opinionated. Prompt colors inherit ANSI from the terminal theme.
4. **No dircolors/LS_COLORS**: ANSI-based `ls` colors already inherit from the terminal theme.
5. **Truecolor only**: `fg=#RRGGBB` requires a truecolor terminal. Reasonable given the suite targets Ghostty (truecolor native).
