# tmux -- Agent Instructions

## Platform Reference

See the root [`AGENTS.md`](../AGENTS.md) for the canonical palette, design principles, and brand rules. Do not duplicate palette tables here.

## tmux Conf Format

- Theme files are `.conf` -- plain tmux configuration fragments sourced via `source-file`.
- Every non-comment, non-empty line must be a `set -g` command.
- Comments: lines starting with `#`.
- Colors use hex notation (`#rrggbb`). Requires tmux 3.2+.

## Color Mapping Rationale

tmux themes control UI chrome (status bar, pane borders, messages, copy mode) -- not terminal ANSI colors. The mapping is:

- **Status bar bg** (`#14120f` dark, `#EDE6DA` light): sidebar surface -- the status bar is a panel, not the editor.
- **Status bar fg** (`#aea195` dark, `#5a5244` light): comments/warm stone -- subdued informational text.
- **Session name** (`#ffb454` dark, `#855700` light): functions/amber -- prominent but not jarring.
- **Active window** (`#ff8f40`/`#924800` bg): keywords/burnt orange -- strongest visual landmark.
- **Pane border** (`#222018` dark, `#DDD6CA` light): line-highlight -- subtle, architectural.
- **Active pane border** (`#b8522e`): accent/copper rust -- focus indicator, same both variants.
- **Messages** (`#bfbdb6`/`#3a3630` fg, `#1f1d17`/`#F0E8DC` bg): foreground on widget surface.
- **Copy mode** (`#f5c56e`/`#8a6600` bg): cursor/gold -- matches cursor highlight.
- **Clock** (`#f5c56e`/`#8a6600`): cursor/gold -- consistent with cursor.

## Status Bar Design

- Position: bottom. Centre-justified window list.
- Left: session name only. Right: empty (user can override).
- Active window uses inverted fg/bg with bold for maximum visibility.
- Window separator is empty string -- spacing is in the format strings.

## TPM Integration

- `warm-burnout.tmux` is the TPM entry point. Must be executable.
- Reads `@warm-burnout-variant` option (default: `dark`), sources the matching `.conf`.
- Uses `$BASH_SOURCE` to resolve its own directory -- works regardless of where the repo is cloned.

## File Structure

```
tmux/
  warm-burnout-dark.conf    # Dark variant
  warm-burnout-light.conf   # Light variant
  warm-burnout.tmux         # TPM plugin entry point (executable)
  README.md                 # Install instructions
  AGENTS.md                 # This file
```
