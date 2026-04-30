# Zellij -- Agent Instructions

## Platform Reference

See the root [`AGENTS.md`](../AGENTS.md) for the canonical palette, design principles, and brand rules. Do not duplicate palette tables here.

## Zellij Theme Format

- Theme files are `.kdl` -- KDL documents loaded from the user's themes folder or inlined into the main config.
- Modern UI-component spec only. The legacy 11-color palette (`fg`, `bg`, `black`, `red`, ...) is intentionally not used; Zellij would map those onto built-in plugin defaults and undo the deliberate component mapping.
- Colors are RGB triplets (0-255), space-separated. A single `0` is the terminal-default sentinel (transparent).
- Comments are `//`. Block comments `/* */` are not used here.

## File Layout

Each `.kdl` file contains a single `themes` block with a single named theme inside it:

```kdl
themes {
  warm-burnout-dark {
    text_unselected { ... }
    ribbon_selected { ... }
    // ... all components
  }
}
```

Theme names match the file stem: `warm-burnout-dark` and `warm-burnout-light`. These are the strings users put in `theme "..."` in their config.

## Required Components

Every variant must define all 15 components. The first 12 each carry 6 attributes (`base`, `background`, `emphasis_0..3`). The two `exit_code_*` components only use `base` per the Zellij spec, but include the full 6 attributes anyway for forward compatibility. `multiplayer_user_colors` uses 10 `player_N` attributes instead.

| Component | Used For |
|---|---|
| `text_unselected` | Bare UI text (Ctrl/Alt modifier hints) |
| `text_selected` | Bare UI text when selected |
| `ribbon_unselected` | Inactive tabs, inactive keybinding modes |
| `ribbon_selected` | Active tab, active keybinding mode |
| `table_title` | Table title row |
| `table_cell_unselected` | Normal table cells |
| `table_cell_selected` | Highlighted table row |
| `list_unselected` | Normal list items |
| `list_selected` | Focused list item |
| `frame_unselected` | Frame around non-focused panes |
| `frame_selected` | Frame around focused pane |
| `frame_highlight` | Frame when in non-base mode (PANE/TAB) |
| `exit_code_success` | Command-pane exit code on success |
| `exit_code_error` | Command-pane exit code on error |
| `multiplayer_user_colors` | 10 player colors for multi-user sessions |

## Color Mapping Rationale

Zellij themes control UI chrome only, not terminal ANSI colors. The mapping is built around two principles:

1. The status bar bg matches the **Ghostty terminal bg** so the bar feels integrated rather than reading as a darker black strip below the terminal. The `tests/canonical.rs` cross-platform test pins this.
2. Interactive chips (active tab, focused pane border, mode highlight) follow the **HA card pattern**: raised surfaces with strong warm accents, three-tier elevation hierarchy.

Component mapping:

- **`text_unselected.background`** (`#1a1510` dark, `#F5EDE0` light): terminal bg. Matches Ghostty so the bar is integrated, not a "black void" below the terminal.
- **`text_unselected.base`** (`#b4a89c` dark, `#544c40` light): comments / warm stone. Subdued informational text.
- **`ribbon_unselected.background`** (`#1f1d17` dark, `#F0E8DC` light): card surface, raised one tone above the bar (HA card pattern). Inactive tabs read as pill-shaped chips above the integrated bar.
- **`ribbon_selected.background`** (`#ff8f40` dark, `#924800` light): keywords / burnt orange. Active tab is the strongest visual landmark.
- **`ribbon_selected.base`** (`#1a1510` dark, `#F5EDE0` light): inverted background for max contrast on the orange.
- **`frame_unselected.base`** (`#3a342a` dark, `#C5BEB2` light): stone tone offset from bg, raised on dark and recessed on light. Targets ~1.5:1 against bg either way: subtle but legible.
- **`frame_selected.base`** (`#b8522e` both): accent / copper rust. Focus indicator.
- **`frame_highlight.base`** (`#ffb454` dark, `#855700` light): functions / amber. Mode-change indicator. Three-tier hierarchy: cursor=gold, focus=copper, mode=amber, all distinct.
- **`exit_code_success.base`** (`#b4bc78` dark, `#4d5c1a` light): strings / sage.
- **`exit_code_error.base`** (`#f49090` dark, `#b03434` light): error tone.
- **`multiplayer_user_colors`**: each `player_N` slot gets a distinct warm hue from the palette, with the copper accent assigned last.

The `0` sentinel is used for `background` wherever a component should let the underlying pane background show through. The `_unselected` variants of tables and lists, `table_title`, all `frame_*` components, and both `exit_code_*` components use `0`. The `_selected` variants of tables and lists override this with `line-highlight` (`#222018` dark, `#ddd6ca` light) so the focused row stands out against the pane bg.

## Emphasis Color Discipline

Each component carries four `emphasis_N` slots that plugins may use to colorize subcomponents (indices in fuzzy results, key hints, mode names in the compact-bar plugin). The compact-bar plugin in particular uses one of these emphases to render the current mode name (NORMAL, PANE, TAB, etc.) in the status bar, so an inappropriate color here shows up as the most prominent text in your bar.

Brand rule: Zellij chrome stays warm. The steel-blue type color (`#90aec0` dark, `#285464` light) is reserved for syntax token coloring only and must not appear in any Zellij UI component, including emphases. Use warm tokens only:

- `emphasis_0`: functions amber (`#ffb454` / `#855700`)
- `emphasis_1`: strings sage (`#b4bc78` / `#4d5c1a`)
- `emphasis_2`: tags terra cotta (`#dc9e92` / `#8e4632`): replaces the steel-blue type color that would otherwise be used
- `emphasis_3`: constants dusty mauve (`#d4a8b8` / `#7e4060`)

Per-context exceptions:

- On the `ribbon_selected` orange background: invert to dark (`#1a1510`) plus cursor gold and accent copper for max contrast on the warm orange.
- On `frame_unselected`: the emphasis stack starts at comments (`#b4a89c` / `#544c40`) instead of functions amber so emphasized indices on unfocused borders stay quiet. Slots 1-3 follow the standard strings/tags/constants order.
- On `frame_selected`: the stack pulls in focus-adjacent warm tokens (functions amber, tags terra cotta, cursor gold) and ends with the `0` sentinel so the most prominent slot defers to the pane bg.
- On `frame_highlight`: the stack collapses to amber across emphasis_1..3 (with keywords burnt orange in slot 0) so the mode-name signal stays monolithic in the compact-bar plugin instead of fanning out into four hues.

The brand-rule test in `tests/zellij.rs` (`dark_no_canonical_steel_blue_in_chrome`, `light_no_canonical_steel_blue_in_chrome`) walks every component and every attribute and rejects the canonical steel-blue hex. The check is narrow by design: it pins the canonical token only, not arbitrary cool hues. Add new components with this constraint in mind.

## File Structure

```
zellij/
  warm-burnout-dark.kdl     # Dark variant
  warm-burnout-light.kdl    # Light variant
  README.md                 # Install instructions
  AGENTS.md                 # This file
```

No plugin entry script exists. Zellij does not have a TPM-equivalent runtime loader; users either drop the files in `~/.config/zellij/themes/` or inline them into the main config.
