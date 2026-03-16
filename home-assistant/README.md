# Warm Burnout for Home Assistant

Your smart home dashboard was the last screen not burning warm. That changes now.

Fully warm palette for the entire HA UI: cards, sidebar, graphs, energy dashboard, code editor, everything. Zero blues in the chrome. Your 2am automation debugging sessions just got 40% less hostile to your retinas.

## Install

### HACS (recommended)

1. Open HACS in your Home Assistant instance
2. Go to Frontend > three-dot menu > Custom repositories
3. Add this repo URL, category: **Theme**
4. Search "Warm Burnout" in HACS and download it
5. Restart Home Assistant (or reload themes via Developer Tools > YAML > Themes)

### Manual

1. Copy `themes/warm-burnout.yaml` from this repo into your HA `config/themes/` directory
2. Make sure your `configuration.yaml` includes:

```yaml
frontend:
  themes: !include_dir_merge_named themes
```

3. Restart Home Assistant

### Activate

1. Go to your Profile (bottom-left user icon)
2. Under Theme, select **Warm Burnout**
3. Pick Dark or Light mode (or leave it on Auto)

## What This Themes

The theme covers every CSS variable in HA's default theme surface:

- **Cards and sections**: warm backgrounds, warm shadows, 16px radius
- **Sidebar**: deep warm tones, copper accent on active items
- **Header bar**: matches the deepest background
- **Buttons, toggles, sliders**: copper rust accent on all interactive elements
- **State icons**: sage green for on, coral for error/off, warm stone for unavailable
- **Graphs**: 14 colors across the warm spectrum (amber, terra cotta, sage, mauve, coral, gold, burnt orange, warm stone). No blues anywhere in your charts.
- **Energy dashboard**: grid = amber, solar = sage, battery = dusty mauve
- **Code editor**: full syntax palette from the canonical Warm Burnout theme. YAML editing in warm tones.
- **Badges, chips, tables, dialogs**: all warm
- **Mushroom cards and Mini Graph Card**: token overrides included for popular HACS frontend cards

## Palette

Both variants derive from the canonical Warm Burnout palette defined in the root [`AGENTS.md`](../AGENTS.md).

Dark background: `#1a1510`. Light background: `#F5EDE0`. Accent: `#b8522e` (copper rust).

## Requirements

Home Assistant 2024.1.0 or later. The theme uses the `modes:` syntax for dark/light, which requires a relatively recent HA version.
