# JetBrains theme rebuild

Date: 2026-05-01
Status: Draft, awaiting review
Replaces: issues #5 and #8

## Problem

Two open bugs against the JetBrains plugin keep resurfacing:

- **#5**: dark UI contrast regressions. Light dialog titles in the dark theme, dark-on-dark popups, loud TODO highlights. Caused by overrides in `*.theme.json` that conflict with the `Islands Dark` parent theme on surfaces we should not have repainted. Mirrors JetBrains youtrack issue IJPL-226200.
- **#8**: Rider/C# syntax falls back to gray. Caused by an editor scheme that covers only ~20 `DEFAULT_*` attributes. ReSharper, Rider, and language plugins use a separate attribute namespace (`ReSharper.CSHARP_*`, `CPP_*`, `IL_*`, etc.) that does not inherit from `DEFAULT_*`.

Cost so far: every JetBrains release surfaces a new variant of one of these bugs. Hand-editing four files with overlapping concerns is the wrong shape for a multi-platform theme suite.

## Goals

1. Stop the recurring time sink. Theme correctness should be a build artifact, not a maintenance task.
2. Fix #5 by aligning with the JetBrains-published override patterns that are known to work under Islands inheritance.
3. Fix #8 by extending coverage to ReSharper, Rider, and language-specific attribute keys.
4. Keep `parentTheme: "Islands Dark/Light"` and `parent_scheme="Darcula"/"Default"` for forward compatibility. Parent inheritance stays as a safety net for keys we do not override.
5. Upstream sync stays manual but mechanical: a `just` recipe surfaces what changed; the maintainer reviews.

## Non-goals

- No JetBrains AI Assistant or Copilot color coverage in this iteration. Plugin-specific keys are out of scope until JetBrains bundles them.
- No automated upstream sync (cron, bot, scheduled PRs). Maintainer triggers the sync command.
- No drop of `parentTheme` or `parent_scheme`.
- No marketing rename. Files keep the `Warm Burnout Islands Dark/Light` naming.

## Approach

Move from hand-edited theme files to palette-driven codegen.

- **Whiskers** ([catppuccin/whiskers](https://github.com/catppuccin/whiskers)) for templating. Rust binary, Tera engine. Catppuccin uses it for their JetBrains, vim, and VS Code themes; the JetBrains pipeline is documented in their public repo.
- **Forked override content** from `JetBrains/rider-theme-pack` (Apache 2.0). Specifically `ReSharperDark.xml` (442 attributes, 105 ReSharper keys covering C#, ASP.NET, C++, IL viewer, stack traces) and `RiderDark.theme.json` (~600 UI keys with the override patterns that work under Islands).
- **Palette source** in `jetbrains/palette.yaml`, mirroring the canonical palette in root `AGENTS.md`.
- **Generated artifacts** as gitignored build outputs.

We hard-fork the *content* of the override sets, not the inheritance chain.

## File layout

```
jetbrains/
  palette.yaml                              # Source palette, both variants
  templates/
    theme.json.tera                         # Single template with matrix frontmatter for dark/light
    editor.xml.tera                         # Single template with matrix frontmatter for dark/light
  UPSTREAM_REVISION.txt                     # Pinned commit SHA of JetBrains/rider-theme-pack
  NOTICE                                    # Apache 2.0 attribution for forked content
  META-INF/plugin.xml                       # Version bumped, content otherwise unchanged
  build.sh                                  # Zip-only; invoked after whiskers by the justfile
  AGENTS.md                                 # Updated: documents codegen pipeline
  README.md                                 # Updated where install/build steps reference filenames
  # Generated, gitignored:
  Warm Burnout Islands Dark.theme.json
  Warm Burnout Islands Light.theme.json
  Warm-Burnout-Dark.xml
  Warm-Burnout-Light.xml
  warm-burnout-theme.jar
```

Two templates, each rendered twice (once per variant). Whether the variant switch is a Whiskers matrix frontmatter, two `whiskers --palette` invocations from the justfile, or four flat templates is implementation detail; the spec only requires that both variants share a single source of palette truth.

## palette.yaml shape

YAML conforming to the Whiskers v2 palette schema. Token names mirror the canonical palette table in root `AGENTS.md`. Both variants live in one file, selected by matrix frontmatter at template time.

Conceptual shape (exact key names finalized during implementation against Whiskers' schema):

```yaml
flavors:
  dark:
    colors:
      background: "#1a1510"
      foreground: "#bfbdb6"
      keyword: "#ff8f40"
      function: "#ffb454"
      operator: "#f29668"
      decorator: "#e6c08a"
      type: "#90aec0"
      string: "#b4bc78"
      regex_escape: "#96b898"
      number: "#d4a8b8"
      tag: "#dc9e92"
      member_var: "#ec9878"
      library_function: "#ec9878"
      comment: "#b4a89c"
      error: "#f49090"
      css_property: "#deb074"
      accent: "#b8522e"
      cursor: "#f5c56e"
      ui_sidebar: "#14120f"
      ui_popup: "#1f1d17"
      ui_hover: "#222018"
      ui_border: "#2a2620"
      ui_muted: "#4a4438"
      ui_canvas: "#0e0c09"
  light:
    colors:
      # mirrored token set with light hex values from AGENTS.md
```

Templates reference tokens via `{{ keyword.hex }}`, `{{ ui_sidebar.hex }}`. Token names stay flat to match Whiskers' default palette resolution.

## Build pipeline

Add a `justfile` at the project root (project does not have one yet). Three JetBrains recipes plus shared helpers:

```just
jetbrains-build:
    cd jetbrains && whiskers templates/theme.json.tera
    cd jetbrains && whiskers templates/editor.xml.tera
    cd jetbrains && ./build.sh

jetbrains-sync:
    bash scripts/jetbrains-sync.sh

jetbrains-clean:
    cd jetbrains && rm -f \
      "Warm Burnout Islands Dark.theme.json" \
      "Warm Burnout Islands Light.theme.json" \
      Warm-Burnout-Dark.xml Warm-Burnout-Light.xml \
      warm-burnout-theme.jar
```

Each `whiskers` invocation reads matrix frontmatter from the template and writes both the dark and light output files. `build.sh` becomes zip-only and runs after generation, with a guard that fails fast if any of the four expected output files are missing.

`scripts/jetbrains-sync.sh` clones `JetBrains/rider-theme-pack` into a temp dir at the pinned revision, then diffs `colorSchemes/ReSharperDark.xml`, `colorSchemes/ReSharperLight.xml`, `RiderDark.theme.json`, and `RiderLight.theme.json` against `master`. Output is a human-readable list of attribute keys added or removed. The maintainer reviews, edits templates, bumps `UPSTREAM_REVISION.txt`, commits.

## CI changes

`.github/workflows/validate.yml`: insert two steps before `cargo test`:
1. `uses: catppuccin/setup-whiskers@v2`
2. `run: just jetbrains-build`

`.github/workflows/release-themes.yml`: same insertion before the zip/attach step.

`.github/workflows/release-vscode.yml`: unchanged.

## Testing

`tests/jetbrains.rs` keeps its current shape. The `include_str!` paths still resolve since generated files land at the existing names. New checks to add:

- **Coverage sentinel**: assert presence of a curated list of `ReSharper.*` C# attributes (`CSHARP_LOCAL_VARIABLE_IDENTIFIER`, `CSHARP_FIELD_IDENTIFIER`, `CSHARP_PROPERTY_IDENTIFIER`, `CSHARP_PARAMETER_IDENTIFIER`, `CSHARP_METHOD_IDENTIFIER`, `CSHARP_CLASS_IDENTIFIER`, `CSHARP_INTERFACE_IDENTIFIER`, `CSHARP_TYPE_PARAMETER_IDENTIFIER`, plus a few more). Catches accidental template regressions of the #8 bug class.
- **Issue #5 surfaces**: assert explicit overrides exist for `Editor.background`, `CompletionPopup.background`, `Popup.background`, `ToolTip.background`, `Island.borderColor`, `TitlePane.background`, `TitlePane.inactiveBackground`. These are the surfaces called out in the JetBrains youtrack pattern that produces #5.
- **TODO loudness**: assert `TODO_DEFAULT_ATTRIBUTES` does not have `FONT_TYPE=1` (bold). Per the issue #5 note, the current bold-yellow TODO is too loud. Correct value is `FONT_TYPE=0` plus the existing foreground.

Generated files are deterministic; same inputs produce same outputs. No additional integration tests beyond the existing structural and palette checks.

Cargo tests depend on generated files via `include_str!`. Local workflow is `just jetbrains-build` then `cargo test`. CI orders these explicitly. A `build.rs` that auto-invokes Whiskers is possible but adds a Rust-to-shell dependency; not adopting it for this iteration. If the manual order trips up contributors, a `cargo test` wrapper recipe in the justfile can chain them.

## Sync workflow

When JetBrains releases a new IDE version:
1. `just jetbrains-sync` outputs added/removed attribute keys against the pinned upstream rev.
2. Maintainer adds new keys to templates, mapped to existing palette tokens (or adds new tokens if needed).
3. `cargo test` validates structural and palette compliance.
4. Bump `UPSTREAM_REVISION.txt` to the upstream commit reviewed.
5. Commit, ship.

No bot, no cron, no auto-merge. A single command surfaces what changed; the human decides what to do.

## Migration plan

Single PR. Squash on merge.

1. Add `jetbrains/palette.yaml`, `jetbrains/templates/*`, `jetbrains/UPSTREAM_REVISION.txt`, `jetbrains/NOTICE`, `scripts/jetbrains-sync.sh`, `justfile`.
2. Generate files locally via Whiskers. The output will not byte-match the current committed files; we forked from `rider-theme-pack`, which has a different override set than what we hand-wrote. Reconcile by:
   - Mapping our existing color choices into the new template (palette tokens already match by design).
   - Identifying any UI keys we currently override that the rider-theme-pack base does not. Add them as template additions.
   - Identifying any rider-theme-pack overrides that conflict with our brand (e.g., upstream uses orange for keywords; ours is also orange, no conflict expected, but verify).
   - Iterating until `cargo test jetbrains` passes and a manual visual check in IntelliJ matches the current 1.0.2 plugin.
3. Delete the four committed theme files: `Warm Burnout Islands Dark.theme.json`, `Warm Burnout Islands Light.theme.json`, `Warm-Burnout-Dark.xml`, `Warm-Burnout-Light.xml`.
4. Add the four generated filenames to `.gitignore`.
5. Update `jetbrains/build.sh` to invoke Whiskers first.
6. Add CI steps to install Whiskers and run codegen before tests.
7. Update `jetbrains/AGENTS.md` and `jetbrains/README.md` to describe the codegen pipeline and the maintainer sync workflow.
8. Bump `META-INF/plugin.xml` version to `1.1.0`. Editor coverage and UI fixes are user-visible; minor bump is correct.
9. Update root `AGENTS.md` to mention `jetbrains/palette.yaml` as the JetBrains source of truth.

Manual verification in IntelliJ IDEA, Rider, and at least one other JetBrains IDE before tagging the release. Surfaces to check:
- Settings dialog title bar in dark mode
- Search Everywhere completion popup
- Project tree, tool windows
- C# file in Rider with classes, methods, fields, parameters, properties, generic types
- TODO highlights in any source file

## Risks and unknowns

- **Whiskers version pinning**. Whiskers v2 (Tera) is the version Catppuccin uses for JetBrains. Pin the major version in CI via `catppuccin/setup-whiskers@v2`. Local installs use `cargo install whiskers` or homebrew.
- **Light editor scheme upstream gap**. No `IslandSchemeLight.xml` exists in `intellij-community`. Light variant bases on `parent_scheme="Default"` and overrides aggressively, mirroring `ReSharperLight.xml`. Acceptable; matches what every Islands Light theme does.
- **Apache 2.0 attribution**. Forked content from `rider-theme-pack` requires keeping a NOTICE. Add `jetbrains/NOTICE` with the upstream copyright header. Apache 2.0 to MIT redistribution is fine provided NOTICE is kept.
- **Whiskers as a build-time dependency**. Adds one toolchain requirement. CI installs via the action; local dev uses `cargo install` or brew.
- **`parentTheme` resolver fallback**. If a user has a non-Islands-capable IDE (older than build 242), `parentTheme: "Islands Dark"` falls back to base Darcula. `plugin.xml` already pins `since-build="242"`, so this is not reachable.
- **Palette drift between platforms**. Other platforms (vscode, zed, etc.) keep their handwritten color files. The JetBrains `palette.yaml` and the canonical palette in root `AGENTS.md` must stay in sync. A future iteration could promote `palette.yaml` to a project-wide source consumed by every platform; out of scope here.
