# Obsidian Theme: Design Spec

## Goal

Add Obsidian (the note-taking app) as a new platform target for Warm Burnout. Full community theme with both dark and light variants, publishable to the Obsidian community theme directory via a mirror repo.

## File Structure

```
obsidian/
  theme.css        # Single CSS file, both dark + light variants
  manifest.json    # Obsidian theme manifest
  README.md        # Install instructions in brand voice
  AGENTS.md        # Platform-specific agent rules
```

### manifest.json

```json
{
  "name": "Warm Burnout",
  "version": "1.4.2",
  "minAppVersion": "1.0.0",
  "author": "Felipe Lima",
  "authorUrl": "https://warmburnout.com"
}
```

Version stays in sync with the rest of the suite. Both variants live in one `theme.css`. No build step.

## CSS Architecture

The `theme.css` has four layers, top to bottom.

### Layer 1: Palette variables (`--wb-*`)

Two blocks (`.theme-dark`, `.theme-light`). These are the only place raw hex values appear. The Rust test harness reads values from these declarations.

```css
.theme-dark {
  --wb-bg: #1a1510;
  --wb-fg: #bfbdb6;
  --wb-accent: #b8522e;
  --wb-cursor: #f5c56e;
  --wb-amber: #ffb454;
  --wb-burnt-orange: #ff8f40;
  --wb-operator: #f29668;
  --wb-terra-cotta: #dc9e92;
  --wb-dried-sage: #b4bc78;
  --wb-verdigris: #96b898;
  --wb-dusty-mauve: #d4a8b8;
  --wb-coral: #ec9878;
  --wb-warm-stone: #b4a89c;
  --wb-aged-brass: #deb074;
  --wb-steel-patina: #90aec0;
  --wb-gold: #e6c08a;
  --wb-error: #f49090;
  /* derived UI surface colors below */
}

.theme-light {
  --wb-bg: #F5EDE0;
  --wb-fg: #3a3630;
  --wb-accent: #b8522e;
  --wb-cursor: #8a6600;
  --wb-amber: #855700;
  --wb-burnt-orange: #924800;
  --wb-operator: #8f4418;
  --wb-terra-cotta: #8e4632;
  --wb-dried-sage: #4d5c1a;
  --wb-verdigris: #286a48;
  --wb-dusty-mauve: #7e4060;
  --wb-coral: #883850;
  --wb-warm-stone: #544c40;
  --wb-aged-brass: #74501c;
  --wb-steel-patina: #285464;
  --wb-gold: #7a5a1c;
  --wb-error: #b03434;
  /* derived UI surface colors below */
}
```

`--wb-operator` is added because the canonical operator color (`#f29668` dark, `#8f4418` light) does not map to any named material. This keeps the palette block complete.

### Layer 2: Obsidian base palette mapping

Maps `--wb-*` into Obsidian's `--color-base-*` ramp (00 through 100, 13 steps) and `--color-*` extended colors. The entire app inherits warmth from these, since Obsidian's semantic variables cascade downstream.

Extended color mapping:

| Obsidian variable | Warm Burnout source |
|---|---|
| `--color-red` | `--wb-error` |
| `--color-orange` | `--wb-burnt-orange` |
| `--color-yellow` | `--wb-amber` |
| `--color-green` | `--wb-dried-sage` |
| `--color-cyan` | `--wb-verdigris` |
| `--color-blue` | `--wb-steel-patina` |
| `--color-purple` | `--wb-dusty-mauve` |
| `--color-pink` | `--wb-coral` |

Accent color (`--accent-h`, `--accent-s`, `--accent-l`) set to the HSL decomposition of `#b8522e` (copper rust).

### Layer 3: Code syntax highlighting

Maps to Obsidian's `--code-*` CSS variables. These cover both CodeMirror 6 (editing view) and Prism.js (reading view) for the tokens Obsidian exposes through variables.

| Variable | Warm Burnout source | Canonical role |
|---|---|---|
| `--code-background` | `--wb-bg` | Background |
| `--code-normal` | `--wb-fg` | Foreground |
| `--code-function` | `--wb-amber` | Functions |
| `--code-keyword` | `--wb-burnt-orange` | Keywords/storage |
| `--code-string` | `--wb-dried-sage` | Strings |
| `--code-comment` | `--wb-warm-stone` | Comments |
| `--code-tag` | `--wb-terra-cotta` | Tags (HTML) |
| `--code-value` | `--wb-dusty-mauve` | Constants/numbers |
| `--code-property` | `--wb-aged-brass` | CSS properties |
| `--code-important` | `--wb-verdigris` | Regex, escapes |
| `--code-operator` | `--wb-operator` | Operators |
| `--code-punctuation` | (dimmed foreground) | Punctuation |

Additional Prism.js `.token.*` rules for reading view to get finer control where Obsidian's variables do not reach (e.g., `.token.class-name` mapped to `--wb-steel-patina`, `.token.boolean` to `--wb-dusty-mauve`, `.token.constant` to `--wb-dusty-mauve`).

### Layer 4: Warmth tweaks

Subtle non-color adjustments to reinforce the warm feel. No layout, font, or spacing changes.

- Warm-tinted box shadows (palette-derived, not pure black/gray)
- 1-2px softer border radius on tooltips and modals
- Warm-tinted scrollbar tracks
- Warm selection/highlight backgrounds (palette colors with alpha)

## Surface Hierarchy

A proper 13-step ramp for `--color-base-*`. All intermediates stay on-hue (warm undertone, no neutral grays).

### Dark

| Step | Purpose | Approximate hex |
|---|---|---|
| 00 | Deepest surface (code blocks, embeds) | `#14120f` |
| 05 | Primary background | `#1a1510` (canonical bg) |
| 10 | Slight elevation (hover states) | `#1f1d17` |
| 20 | Borders, dividers | `#2a2520` |
| 25 | Active surface (sidebar selection) | `#302a22` |
| 30 | Muted interactive | `#3a342c` |
| 35 | Scrollbar track | `#443d34` |
| 40 | Placeholder text, disabled | `#5a5348` |
| 50 | Midpoint (icons, muted UI text) | `#6e665c` |
| 60 | Secondary text | `#8a8278` |
| 70 | Strong secondary text | `#9e968c` |
| 100 | Primary text | `#bfbdb6` (canonical fg) |

### Light

Inverted ramp from `#F5EDE0` (lightest, base-00) down to `#3a3630` (darkest, base-100). Same warm undertone throughout. Exact intermediate values finalized during implementation, validated against WCAG thresholds.

## Heading Colors

Headings mapped to palette colors by visual hierarchy, giving notes a natural energy gradient:

| Heading | Dark | Light | Palette material |
|---|---|---|---|
| H1 | `#ffb454` | `#855700` | Amber |
| H2 | `#ff8f40` | `#924800` | Burnt orange |
| H3 | `#deb074` | `#74501c` | Aged brass |
| H4 | `#dc9e92` | `#8e4632` | Terra cotta |
| H5 | `#90aec0` | `#285464` | Steel patina |
| H6 | `#b4a89c` | `#544c40` | Warm stone |

## Mirror Repo and Distribution

### Mirror repo

`felipefdl/warm-burnout-obsidian` on GitHub. Contains only the files Obsidian needs at the repo root: `theme.css`, `manifest.json`, `README.md`, `LICENSE`, `screenshot.png`.

### Sync workflow

A new job in `release-themes.yml`, alongside the existing `package` job. On `v*` tag push, after validation:

1. Checkout the mirror repo using a deploy key or PAT (stored as repo secret)
2. Copy `obsidian/theme.css`, `obsidian/manifest.json`, `obsidian/README.md`, and root `LICENSE` to the mirror
3. Commit and push to mirror's `main`
4. Create a matching `v*` tag on the mirror (Obsidian checks tags for version updates)

### Community submission

One-time PR to `obsidianmd/obsidian-releases`, adding to `community-css-themes.json`:

```json
{
  "name": "Warm Burnout",
  "author": "felipefdl",
  "repo": "felipefdl/warm-burnout-obsidian",
  "screenshot": "screenshot.png",
  "modes": ["dark", "light"]
}
```

After initial submission, updates are automatic via the mirror's tags.

## Test Harness Integration

### CSS color extractor (`common.rs`)

New function:

```rust
pub fn obsidian_color(src: &str, variant: &str, key: &str) -> String
```

Extracts hex values from `--wb-{key}: #hex;` declarations inside `.theme-{variant}` blocks. The `--wb-` prefix makes parsing unambiguous.

### Per-platform tests (`tests/obsidian.rs`)

- All `--wb-*` values are valid hex
- Both `.theme-dark` and `.theme-light` blocks exist
- All canonical palette colors present in both variants
- `manifest.json` is valid JSON with required fields (`name`, `version`, `minAppVersion`, `author`)
- Code variable mappings reference existing `--wb-*` variables (no typos)
- Heading color variables are set for H1 through H6

### Cross-platform tests (`tests/canonical.rs`)

Add Obsidian to the existing comparison chains for background, foreground, accent, and cursor. Same pattern as every other platform: compare `obsidian_color(src, "dark", "bg")` against the VS Code source of truth.

## Release Packaging

Add to `release-themes.yml`:

```yaml
- name: Package Obsidian
  run: |
    cd obsidian
    zip -j ../warm-burnout-obsidian.zip \
      theme.css \
      manifest.json
```

Add `warm-burnout-obsidian.zip` to the `files:` list in the GitHub Release step.

## Monorepo Checklist (per AGENTS.md "Adding a New Platform")

1. Create `obsidian/` at the project root
2. Add `obsidian/README.md` with install instructions in brand voice
3. Add `obsidian/AGENTS.md` referencing root AGENTS.md for palette
4. Map canonical palette to CSS custom properties
5. Add packaging step to `release-themes.yml`
6. No build artifacts to gitignore (no build step)
7. Update platforms table in root `README.md`
8. Update platforms grid in `site/index.html`
9. Create mirror repo `felipefdl/warm-burnout-obsidian`
10. Add mirror sync job to CI
11. Submit PR to `obsidianmd/obsidian-releases`
