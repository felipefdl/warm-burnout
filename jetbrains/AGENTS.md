# JetBrains -- Agent Instructions

## Platform Reference

See the root [`AGENTS.md`](../AGENTS.md) for the canonical palette, design principles, and brand rules. Do not duplicate palette tables here.

## Theme Architecture

Warm Burnout for JetBrains uses two layers:

1. **`.theme.json`** -- full UI theme with Islands support (sidebar, tabs, toolbar, popups, buttons, etc.)
2. **`.xml`** -- editor color scheme (syntax highlighting, gutter, caret, selection)

The `.theme.json` references the `.xml` via `editorScheme`. Both are packaged together as a plugin (`.jar`).

Theme files are **generated**, not hand-edited. The codegen binary at `tools/warm-burnout-codegen/` reads `palette.yaml` and renders Tera templates in `templates/` into the four output files. The committed repo does not contain the generated `.theme.json` and `.xml` files; they appear after `just jetbrains-build`.

## Plugin Structure

```
jetbrains/
  META-INF/
    plugin.xml                              # Plugin manifest
  palette.yaml                              # Codegen source of truth (both variants)
  templates/
    theme.json.tera                         # UI theme template, both variants
    editor.xml.tera                         # Editor scheme template, both variants
  UPSTREAM_REVISION.txt                     # Pinned JetBrains/rider-theme-pack revision
  build.sh                                  # Zip generated files into the JAR
  README.md                                 # Install instructions
  AGENTS.md                                 # This file
  # Generated, gitignored:
  Warm Burnout Islands Dark.theme.json
  Warm Burnout Islands Light.theme.json
  Warm-Burnout-Dark.xml
  Warm-Burnout-Light.xml
  warm-burnout-theme.jar
```

## Codegen Workflow

To regenerate after a palette or template change:

```sh
just jetbrains-build
```

This runs the codegen binary four times (two templates x two variants) and then `build.sh` to zip the outputs into `warm-burnout-theme.jar`. CI runs the same recipe before `cargo test`.

To clean generated files:

```sh
just jetbrains-clean
```

## Upstream Sync

Rider/ReSharper attribute coverage in `editor.xml.tera` was modeled on `JetBrains/rider-theme-pack` (Apache 2.0). When JetBrains releases a new IDE major version and adds new attribute keys upstream, run:

```sh
just jetbrains-sync
```

The script clones `rider-theme-pack`, diffs against the SHA pinned in `UPSTREAM_REVISION.txt`, and prints attribute keys added or removed. Review the output, add new keys to the templates if they affect visible code surfaces, bump `UPSTREAM_REVISION.txt`, regenerate, run `cargo test`, commit.

## `.theme.json` Format

- JSON file with `name`, `dark` (boolean), `author`, `parentTheme`, `editorScheme` (path to `.xml`), and `ui` (component tree).
- UI keys follow `ComponentName.propertyName` pattern with nesting support.
- Colors are 6-digit hex without `#` prefix. 8-digit hex includes alpha.
- The `*` wildcard sets defaults inherited by all components.
- Islands themes require `parentTheme` (`Islands Dark` / `Islands Light`), `Island` section, and `MainWindow.background`.

## `.xml` Format (Editor Scheme)

- XML with `<scheme>` root, `<colors>` section, and `<attributes>` section.
- Colors are 6-digit hex without `#` prefix.
- `parent_scheme="Darcula"` (dark) / `parent_scheme="Default"` (light) for inheritance.
- Language-specific attributes inherit from `DEFAULT_*` automatically.
- Additional editor attributes (search results, breadcrumbs, folded text, TODOs) override parent scheme defaults to prevent blue bleed-through.

## `FONT_TYPE` Values

- `0` = normal
- `1` = bold
- `2` = italic
- `3` = bold italic

## UI Color Hierarchy (Dark)

- `#14120f` -- sidebar, panels, toolbar, status bar
- `#1a1510` -- editor background, selected tab
- `#1f1d17` -- popups, menus, widgets, text fields
- `#222018` -- hover, line highlight, active selections
- `#2a2620` -- borders, separators, indent guides
- `#4a4438` -- muted elements (line numbers, scrollbar thumb)
- `#b8522e` -- accent (buttons, underlines, focus rings)

## UI Color Hierarchy (Light)

- `#ede6da` -- sidebar, panels, toolbar, status bar
- `#f5ede0` -- editor background, selected tab
- `#f0e8dc` -- popups, menus, widgets, text fields
- `#e3dbd0` -- hover, active selections
- `#DDD6CA` -- borders, separators
- `#a89880` -- muted elements (line numbers, scrollbar thumb)
- `#b8522e` -- accent (buttons, underlines, focus rings)
