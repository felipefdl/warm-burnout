# Warm Burnout Light -- zsh-syntax-highlighting theme
# Source this file AFTER loading zsh-syntax-highlighting.
# Requires a truecolor terminal.
#
# Palette reference: ../AGENTS.md

typeset -gA ZSH_HIGHLIGHT_STYLES

# Functions -- callable things
ZSH_HIGHLIGHT_STYLES[command]='fg=#855700'
ZSH_HIGHLIGHT_STYLES[function]='fg=#855700'
ZSH_HIGHLIGHT_STYLES[alias]='fg=#855700'
ZSH_HIGHLIGHT_STYLES[suffix-alias]='fg=#855700'
ZSH_HIGHLIGHT_STYLES[global-alias]='fg=#855700'
ZSH_HIGHLIGHT_STYLES[hashed-command]='fg=#855700'

# Keywords -- structural
ZSH_HIGHLIGHT_STYLES[reserved-word]='fg=#924800,bold'
ZSH_HIGHLIGHT_STYLES[builtin]='fg=#924800,bold'
ZSH_HIGHLIGHT_STYLES[precommand]='fg=#924800,bold'

# Strings
ZSH_HIGHLIGHT_STYLES[single-quoted-argument]='fg=#4d5c1a'
ZSH_HIGHLIGHT_STYLES[double-quoted-argument]='fg=#4d5c1a'
ZSH_HIGHLIGHT_STYLES[dollar-quoted-argument]='fg=#4d5c1a'

# Error/invalid
ZSH_HIGHLIGHT_STYLES[single-quoted-argument-unclosed]='fg=#b03434'
ZSH_HIGHLIGHT_STYLES[double-quoted-argument-unclosed]='fg=#b03434'
ZSH_HIGHLIGHT_STYLES[dollar-quoted-argument-unclosed]='fg=#b03434'
ZSH_HIGHLIGHT_STYLES[back-quoted-argument-unclosed]='fg=#b03434'
ZSH_HIGHLIGHT_STYLES[unknown-token]='fg=#b03434'

# Operators
ZSH_HIGHLIGHT_STYLES[commandseparator]='fg=#8f4418'
ZSH_HIGHLIGHT_STYLES[assign]='fg=#8f4418'
ZSH_HIGHLIGHT_STYLES[redirection]='fg=#8f4418'
ZSH_HIGHLIGHT_STYLES[path_pathseparator]='fg=#8f4418'
ZSH_HIGHLIGHT_STYLES[path_prefix_pathseparator]='fg=#8f4418'
ZSH_HIGHLIGHT_STYLES[back-quoted-argument-delimiter]='fg=#8f4418'

# Types (cool accent) -- options/flags
ZSH_HIGHLIGHT_STYLES[single-hyphen-option]='fg=#285464'
ZSH_HIGHLIGHT_STYLES[double-hyphen-option]='fg=#285464'

# Decorators -- filesystem references
ZSH_HIGHLIGHT_STYLES[path]='fg=#7a5a1c'
ZSH_HIGHLIGHT_STYLES[autodirectory]='fg=#7a5a1c'
ZSH_HIGHLIGHT_STYLES[path_prefix]='fg=#7a5a1c,underline'

# Regex/escape -- patterns and interpolation
ZSH_HIGHLIGHT_STYLES[globbing]='fg=#286a48'
ZSH_HIGHLIGHT_STYLES[rc-quote]='fg=#286a48'
ZSH_HIGHLIGHT_STYLES[dollar-double-quoted-argument]='fg=#286a48'
ZSH_HIGHLIGHT_STYLES[back-double-quoted-argument]='fg=#286a48'

# Constants -- value-producing references
ZSH_HIGHLIGHT_STYLES[history-expansion]='fg=#7e4060'
ZSH_HIGHLIGHT_STYLES[back-quoted-argument]='fg=#7e4060'
ZSH_HIGHLIGHT_STYLES[named-fd]='fg=#7e4060'
ZSH_HIGHLIGHT_STYLES[numeric-fd]='fg=#7e4060'

# Comments
ZSH_HIGHLIGHT_STYLES[comment]='fg=#544c40,italic'

# Foreground
ZSH_HIGHLIGHT_STYLES[default]='fg=#3a3630'
ZSH_HIGHLIGHT_STYLES[arg0]='fg=#3a3630'

# Cursor
ZSH_HIGHLIGHT_STYLES[cursor]='standout'
