# Warm Burnout Dark -- zsh-syntax-highlighting theme
# Source this file AFTER loading zsh-syntax-highlighting.
# Requires a truecolor terminal.
#
# Palette reference: ../AGENTS.md

typeset -gA ZSH_HIGHLIGHT_STYLES

# Functions -- callable things
ZSH_HIGHLIGHT_STYLES[command]='fg=#ffb454'
ZSH_HIGHLIGHT_STYLES[function]='fg=#ffb454'
ZSH_HIGHLIGHT_STYLES[alias]='fg=#ffb454'
ZSH_HIGHLIGHT_STYLES[suffix-alias]='fg=#ffb454'
ZSH_HIGHLIGHT_STYLES[global-alias]='fg=#ffb454'
ZSH_HIGHLIGHT_STYLES[hashed-command]='fg=#ffb454'

# Keywords -- structural
ZSH_HIGHLIGHT_STYLES[reserved-word]='fg=#ff8f40,bold'
ZSH_HIGHLIGHT_STYLES[builtin]='fg=#ff8f40,bold'
ZSH_HIGHLIGHT_STYLES[precommand]='fg=#ff8f40,bold'

# Strings
ZSH_HIGHLIGHT_STYLES[single-quoted-argument]='fg=#b4bc78'
ZSH_HIGHLIGHT_STYLES[double-quoted-argument]='fg=#b4bc78'
ZSH_HIGHLIGHT_STYLES[dollar-quoted-argument]='fg=#b4bc78'

# Error/invalid
ZSH_HIGHLIGHT_STYLES[single-quoted-argument-unclosed]='fg=#f49090'
ZSH_HIGHLIGHT_STYLES[double-quoted-argument-unclosed]='fg=#f49090'
ZSH_HIGHLIGHT_STYLES[dollar-quoted-argument-unclosed]='fg=#f49090'
ZSH_HIGHLIGHT_STYLES[back-quoted-argument-unclosed]='fg=#f49090'
ZSH_HIGHLIGHT_STYLES[unknown-token]='fg=#f49090'

# Operators
ZSH_HIGHLIGHT_STYLES[commandseparator]='fg=#f29668'
ZSH_HIGHLIGHT_STYLES[assign]='fg=#f29668'
ZSH_HIGHLIGHT_STYLES[redirection]='fg=#f29668'
ZSH_HIGHLIGHT_STYLES[path_pathseparator]='fg=#f29668'
ZSH_HIGHLIGHT_STYLES[path_prefix_pathseparator]='fg=#f29668'
ZSH_HIGHLIGHT_STYLES[back-quoted-argument-delimiter]='fg=#f29668'

# Types (cool accent) -- options/flags
ZSH_HIGHLIGHT_STYLES[single-hyphen-option]='fg=#90aec0'
ZSH_HIGHLIGHT_STYLES[double-hyphen-option]='fg=#90aec0'

# Decorators -- filesystem references
ZSH_HIGHLIGHT_STYLES[path]='fg=#e6c08a'
ZSH_HIGHLIGHT_STYLES[autodirectory]='fg=#e6c08a'
ZSH_HIGHLIGHT_STYLES[path_prefix]='fg=#e6c08a,underline'

# Regex/escape -- patterns and interpolation
ZSH_HIGHLIGHT_STYLES[globbing]='fg=#96b898'
ZSH_HIGHLIGHT_STYLES[rc-quote]='fg=#96b898'
ZSH_HIGHLIGHT_STYLES[dollar-double-quoted-argument]='fg=#96b898'
ZSH_HIGHLIGHT_STYLES[back-double-quoted-argument]='fg=#96b898'

# Constants -- value-producing references
ZSH_HIGHLIGHT_STYLES[history-expansion]='fg=#d4a8b8'
ZSH_HIGHLIGHT_STYLES[back-quoted-argument]='fg=#d4a8b8'
ZSH_HIGHLIGHT_STYLES[named-fd]='fg=#d4a8b8'
ZSH_HIGHLIGHT_STYLES[numeric-fd]='fg=#d4a8b8'

# Comments
ZSH_HIGHLIGHT_STYLES[comment]='fg=#b4a89c,italic'

# Foreground
ZSH_HIGHLIGHT_STYLES[default]='fg=#bfbdb6'
ZSH_HIGHLIGHT_STYLES[arg0]='fg=#bfbdb6'

# Cursor
ZSH_HIGHLIGHT_STYLES[cursor]='standout'
