# Warm Burnout for WezTerm

The GPU terminal gets a calibrated dose too. Your eyes deserved this.

## Install

Drop the TOML files into WezTerm's color scheme directory:

```sh
mkdir -p ~/.config/wezterm/colors
cp warm-burnout-dark.toml warm-burnout-light.toml ~/.config/wezterm/colors/
```

Windows portable installs look for a `colors` directory beside `wezterm.exe`. For a custom location, set `color_scheme_dirs` in your `wezterm.lua`.

## Configure

Set the scheme in `~/.config/wezterm/wezterm.lua`:

```lua
local wezterm = require 'wezterm'
local config = wezterm.config_builder()

config.color_scheme = 'Warm Burnout Dark'

return config
```

Swap to `Warm Burnout Light` when daylight wins.

### Auto-switching

```lua
local wezterm = require 'wezterm'
local config = wezterm.config_builder()

local function scheme_for_appearance(appearance)
  if appearance:find 'Dark' then
    return 'Warm Burnout Dark'
  end
  return 'Warm Burnout Light'
end

config.color_scheme = scheme_for_appearance(wezterm.gui.get_appearance())

return config
```

WezTerm reloads config automatically. Use **Ctrl+Shift+R** if you enjoy pressing the panic button politely.

## Verify

Run `ls --color` or an ANSI color test script. Tabs should use burnt orange for the active tab, the cursor should land on amber gold (or oxidized brass on light), selection should stay muted, splits should land on copper rust instead of default gray despair, and copy mode (default `Ctrl+Shift+X`) should highlight the active match in the same burnt orange as the active tab.

## Palette

Both themes derive from the canonical Warm Burnout palette defined in the root [`AGENTS.md`](../AGENTS.md). The 16 ANSI colors and terminal background, foreground, cursor, and selection colors match the Ghostty theme exactly.
