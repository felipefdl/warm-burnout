# Warm Burnout for Home Assistant -- Agent Instructions

Refer to the root [`AGENTS.md`](../AGENTS.md) for the canonical palette, design principles, brand rules, and writing style guide. Everything below is specific to the Home Assistant platform.

## Format

Home Assistant themes are single YAML files containing CSS custom properties. The file uses HA's `modes:` syntax to define dark and light variants under one theme name.

```yaml
Warm Burnout:
  modes:
    dark:
      primary-color: "#b8522e"
    light:
      primary-color: "#b8522e"
```

All hex values must be quoted (YAML treats `#` as a comment character without quotes).

## File Layout

The theme YAML lives at `home-assistant/warm-burnout.yaml`. A symlink at `themes/warm-burnout.yaml` points to it so HACS can find it (HACS requires a root-level `themes/` directory).

Supporting files:
- `themes/warm-burnout.yaml` (symlink to `../home-assistant/warm-burnout.yaml`)
- `hacs.json` at repo root (HACS manifest)
- `home-assistant/README.md` (install instructions)
- `home-assistant/AGENTS.md` (this file)

## Variable Reference

The theme overrides CSS custom properties from the official HA default theme (`home-assistant/frontend` repo, `src/resources/styles.ts` and `ha-style.ts`). Categories covered:

- **Text**: `primary-text-color`, `secondary-text-color`, `text-primary-color`, `disabled-text-color`
- **Interface**: `primary-color`, `accent-color`, `divider-color`, `scrollbar-thumb-color`
- **Feedback**: `error-color`, `warning-color`, `success-color`, `info-color`
- **Backgrounds**: `primary-background-color`, `secondary-background-color`, `card-background-color`
- **RGB tokens**: `rgb-primary-color`, `rgb-accent-color`, `rgb-primary-text-color`, etc.
- **Sidebar**: `sidebar-*` colors
- **Switches/sliders**: `switch-*`, `slider-*` colors
- **MDC (Material Design Components)**: `mdc-theme-*`, `mdc-checkbox-*`, `mdc-radio-*`
- **Tables**: `table-row-*`, `data-table-*`
- **Header**: `app-header-*`
- **Code editor (CodeMirror)**: `codemirror-*` syntax colors
- **Badges/chips**: `label-badge-*`, `chip-background-color`
- **Energy dashboard**: `energy-grid-*`, `energy-solar-*`, `energy-battery-*`
- **Graph colors**: 14 warm-spectrum graph colors
- **Extensions**: Mushroom card and Mini Graph Card token overrides

## Color Mapping Rationale

- **Primary/accent**: Copper rust `#b8522e` on both modes. Gives the UI a burnished metal character.
- **State colors**: Warm spectrum only. On/success = sage green, off/error = coral/warm red, active = copper accent.
- **Graphs**: All 14 colors distributed across warm hues. No blues. Charts look like a warm sunset.
- **Energy**: Grid = amber, solar = sage, battery = dusty mauve. All warm, all readable.
- **CodeMirror**: Maps directly from the canonical syntax palette (keywords = `#ff8f40`, strings = `#b4bc78`, etc.).
- **Disabled states**: RGBA with warm grey tint, keeping everything within the warm family.

## Rules

1. Every hex value in the theme must come from the canonical palette or be a derived variant (opacity, slight shade shift for UI chrome). No arbitrary colors.
2. Do not add blues. The only cool-toned values allowed are the steel patina type accent (`#8aa8b8` dark / `#2a5868` light) and `info-color` (which maps to that same accent).
3. When HA updates its default theme with new variables, add them to the theme with appropriate warm mappings.
4. Keep both `dark` and `light` modes in sync: same variable set, same structure, different values.
5. Test changes with `cargo test --test home_assistant`.
