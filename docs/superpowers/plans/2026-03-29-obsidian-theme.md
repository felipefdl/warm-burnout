# Obsidian Theme Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add Obsidian as the 15th platform target for Warm Burnout, with a full community theme (dark + light), Rust test coverage, and CI packaging.

**Architecture:** Single `theme.css` using CSS custom properties as an internal palette layer (`--wb-*`). Both variants in one file via `.theme-dark` / `.theme-light` selectors. Obsidian's base ramp and extended color variables mapped from the palette. Rust tests extract hex values from `--wb-*` declarations.

**Tech Stack:** CSS (theme), JSON (manifest), Rust (tests), GitHub Actions (CI)

**Spec:** `docs/superpowers/specs/2026-03-29-obsidian-theme-design.md`

---

### Task 1: Test Infrastructure

**Files:**
- Modify: `tests/common.rs`

- [ ] **Step 1: Add obsidian_color extractor to common.rs**

Add this function at the end of `tests/common.rs`, before the closing of the file:

```rust
/// Extract a color from an Obsidian theme CSS file.
/// Finds `--wb-{key}: #hex;` inside the `.theme-{variant}` block.
pub fn obsidian_color(src: &str, variant: &str, key: &str) -> String {
  let selector = format!(".theme-{variant}");
  let var_decl = format!("--wb-{}:", key);

  let sel_pos = src
    .find(&selector)
    .unwrap_or_else(|| panic!("no {selector} block in obsidian theme"));
  let rest = &src[sel_pos..];
  let brace_pos = rest
    .find('{')
    .unwrap_or_else(|| panic!("no opening brace after {selector}"));
  let block_start = &rest[brace_pos + 1..];

  let mut depth = 1;
  let mut block_end = 0;
  for (i, c) in block_start.char_indices() {
    match c {
      '{' => depth += 1,
      '}' => {
        depth -= 1;
        if depth == 0 {
          block_end = i;
          break;
        }
      }
      _ => {}
    }
  }
  let block = &block_start[..block_end];

  block
    .lines()
    .find(|l| l.trim().starts_with(&var_decl))
    .and_then(|l| {
      l.split_once(':').map(|(_, v)| {
        let v = v.trim().trim_end_matches(';').trim();
        hex_to_lower(v)
      })
    })
    .unwrap_or_else(|| panic!("no --wb-{key} in {selector} block"))
}
```

- [ ] **Step 2: Add obsidian_color to the import list at the top of canonical.rs**

In `tests/canonical.rs` line 1-7, add `obsidian_color` to the `use common::` import:

```rust
use common::{
  ghostty_ansi_color, ghostty_color, hex_to_lower, home_assistant_color, iterm2_color, jetbrains_attribute,
  jetbrains_color, nvim_palette_color, obsidian_color, starship_palette_color, tmux_option_value, tmux_style_fg,
  vscode_color, windows_terminal_color, xcode_color, xcode_syntax_color, zed_editor_color,
};
```

- [ ] **Step 3: Verify it compiles**

Run: `cargo check --tests 2>&1 | head -5`

Expected: warning about unused import `obsidian_color` (no tests use it yet), but no errors.

- [ ] **Step 4: Commit**

```bash
git add tests/common.rs tests/canonical.rs
git commit -m "Add obsidian_color CSS extractor to test harness"
```

---

### Task 2: Per-Platform Tests (Failing)

**Files:**
- Create: `obsidian/theme.css` (stub)
- Create: `obsidian/manifest.json` (stub)
- Create: `tests/obsidian.rs`

- [ ] **Step 1: Create stub files so include_str compiles**

Create `obsidian/theme.css`:

```css
/* stub */
.theme-dark {}
.theme-light {}
```

Create `obsidian/manifest.json`:

```json
{}
```

- [ ] **Step 2: Write tests/obsidian.rs**

```rust
mod common;

use common::{extract_hex_colors, is_valid_hex, obsidian_color};

const THEME: &str = include_str!("../obsidian/theme.css");
const MANIFEST: &str = include_str!("../obsidian/manifest.json");

// -- Structure --

#[test]
fn has_theme_dark_block() {
  assert!(THEME.contains(".theme-dark"), "theme.css must have a .theme-dark block");
}

#[test]
fn has_theme_light_block() {
  assert!(THEME.contains(".theme-light"), "theme.css must have a .theme-light block");
}

// -- All hex colors valid --

#[test]
fn all_hex_colors_are_valid() {
  for (line, hex) in extract_hex_colors(THEME) {
    assert!(is_valid_hex(hex), "line {line}: invalid hex: {hex}");
  }
}

// -- Dark palette values match canonical --

const DARK_PALETTE: &[(&str, &str)] = &[
  ("bg", "#1a1510"),
  ("fg", "#bfbdb6"),
  ("accent", "#b8522e"),
  ("cursor", "#f5c56e"),
  ("amber", "#ffb454"),
  ("burnt-orange", "#ff8f40"),
  ("operator", "#f29668"),
  ("terra-cotta", "#dc9e92"),
  ("dried-sage", "#b4bc78"),
  ("verdigris", "#96b898"),
  ("dusty-mauve", "#d4a8b8"),
  ("coral", "#ec9878"),
  ("warm-stone", "#b4a89c"),
  ("aged-brass", "#deb074"),
  ("steel-patina", "#90aec0"),
  ("gold", "#e6c08a"),
  ("error", "#f49090"),
];

#[test]
fn dark_palette_values() {
  for (key, expected) in DARK_PALETTE {
    let actual = obsidian_color(THEME, "dark", key);
    assert_eq!(actual, *expected, "dark --wb-{key}: expected={expected} actual={actual}");
  }
}

// -- Light palette values match canonical --

const LIGHT_PALETTE: &[(&str, &str)] = &[
  ("bg", "#f5ede0"),
  ("fg", "#3a3630"),
  ("accent", "#b8522e"),
  ("cursor", "#8a6600"),
  ("amber", "#855700"),
  ("burnt-orange", "#924800"),
  ("operator", "#8f4418"),
  ("terra-cotta", "#8e4632"),
  ("dried-sage", "#4d5c1a"),
  ("verdigris", "#286a48"),
  ("dusty-mauve", "#7e4060"),
  ("coral", "#883850"),
  ("warm-stone", "#544c40"),
  ("aged-brass", "#74501c"),
  ("steel-patina", "#285464"),
  ("gold", "#7a5a1c"),
  ("error", "#b03434"),
];

#[test]
fn light_palette_values() {
  for (key, expected) in LIGHT_PALETTE {
    let actual = obsidian_color(THEME, "light", key);
    assert_eq!(actual, *expected, "light --wb-{key}: expected={expected} actual={actual}");
  }
}

// -- Both variants have the same palette keys --

#[test]
fn dark_and_light_have_same_palette_keys() {
  let dark_keys: Vec<&str> = DARK_PALETTE.iter().map(|(k, _)| *k).collect();
  let light_keys: Vec<&str> = LIGHT_PALETTE.iter().map(|(k, _)| *k).collect();
  assert_eq!(dark_keys, light_keys, "dark and light palette key sets must match");
}

// -- Manifest --

#[test]
fn manifest_is_valid_json() {
  let _: serde_json::Value = serde_json::from_str(MANIFEST).expect("manifest.json is not valid JSON");
}

#[test]
fn manifest_has_required_fields() {
  let v: serde_json::Value = serde_json::from_str(MANIFEST).unwrap();
  assert!(v["name"].is_string(), "manifest missing 'name'");
  assert!(v["version"].is_string(), "manifest missing 'version'");
  assert!(v["minAppVersion"].is_string(), "manifest missing 'minAppVersion'");
  assert!(v["author"].is_string(), "manifest missing 'author'");
}

#[test]
fn manifest_name_is_warm_burnout() {
  let v: serde_json::Value = serde_json::from_str(MANIFEST).unwrap();
  assert_eq!(v["name"].as_str().unwrap(), "Warm Burnout");
}

// -- Code syntax variables reference palette --

const CODE_VARS: &[&str] = &[
  "--code-background",
  "--code-normal",
  "--code-function",
  "--code-keyword",
  "--code-string",
  "--code-comment",
  "--code-tag",
  "--code-value",
  "--code-property",
  "--code-important",
  "--code-operator",
  "--code-punctuation",
];

#[test]
fn has_all_code_syntax_variables() {
  for var in CODE_VARS {
    assert!(
      THEME.contains(&format!("{var}:")),
      "theme.css missing code syntax variable: {var}"
    );
  }
}

// -- Heading colors set for H1-H6 --

#[test]
fn has_heading_color_variables() {
  for n in 1..=6 {
    let var = format!("--h{n}-color");
    assert!(THEME.contains(&format!("{var}:")), "theme.css missing {var}");
  }
}
```

- [ ] **Step 3: Run tests to verify they fail**

Run: `cargo test --test obsidian 2>&1 | tail -20`

Expected: Multiple test failures. `dark_palette_values` and `light_palette_values` should panic with "no --wb-bg in .theme-dark block" since the stub has empty blocks. `manifest_has_required_fields` should fail since the stub is `{}`.

- [ ] **Step 4: Commit**

```bash
git add tests/obsidian.rs obsidian/theme.css obsidian/manifest.json
git commit -m "Add failing Obsidian platform tests"
```

---

### Task 3: Theme CSS

**Files:**
- Modify: `obsidian/theme.css`

- [ ] **Step 1: Write the complete theme.css**

Replace the stub `obsidian/theme.css` with the full theme:

```css
/* ==========================================================================
   Warm Burnout for Obsidian
   Warm, contrast-audited color theme. Dark + Light.
   https://warmburnout.com
   ========================================================================== */

/* -- Dark variant --------------------------------------------------------- */
.theme-dark {
  /* Palette (canonical hex values, test harness reads from these) */
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

  /* Surface ramp (warm undertone throughout, no neutral grays) */
  --color-base-00: #14120f;
  --color-base-05: #1a1510;
  --color-base-10: #1f1d17;
  --color-base-20: #2a2520;
  --color-base-25: #302a22;
  --color-base-30: #3a342c;
  --color-base-35: #443d34;
  --color-base-40: #5a5348;
  --color-base-50: #6e665c;
  --color-base-60: #8a8278;
  --color-base-70: #9e968c;
  --color-base-100: #bfbdb6;

  /* Extended colors */
  --color-red: var(--wb-error);
  --color-red-rgb: 244, 144, 144;
  --color-orange: var(--wb-burnt-orange);
  --color-orange-rgb: 255, 143, 64;
  --color-yellow: var(--wb-amber);
  --color-yellow-rgb: 255, 180, 84;
  --color-green: var(--wb-dried-sage);
  --color-green-rgb: 180, 188, 120;
  --color-cyan: var(--wb-verdigris);
  --color-cyan-rgb: 150, 184, 152;
  --color-blue: var(--wb-steel-patina);
  --color-blue-rgb: 144, 174, 192;
  --color-purple: var(--wb-dusty-mauve);
  --color-purple-rgb: 212, 168, 184;
  --color-pink: var(--wb-coral);
  --color-pink-rgb: 236, 152, 120;

  /* Headings */
  --h1-color: var(--wb-amber);
  --h2-color: var(--wb-burnt-orange);
  --h3-color: var(--wb-aged-brass);
  --h4-color: var(--wb-terra-cotta);
  --h5-color: var(--wb-steel-patina);
  --h6-color: var(--wb-warm-stone);

  /* Selection and highlights */
  --text-selection: rgba(245, 197, 110, 0.2);
  --text-highlight-bg: rgba(245, 197, 110, 0.15);
}

/* -- Light variant -------------------------------------------------------- */
.theme-light {
  /* Palette (canonical hex values, test harness reads from these) */
  --wb-bg: #f5ede0;
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

  /* Surface ramp (warm undertone throughout, no neutral grays) */
  --color-base-00: #f5ede0;
  --color-base-05: #ede5d8;
  --color-base-10: #e5ddd0;
  --color-base-20: #d5cbbd;
  --color-base-25: #c8bfb0;
  --color-base-30: #b8ae9e;
  --color-base-35: #a89e8e;
  --color-base-40: #968c7c;
  --color-base-50: #7e7568;
  --color-base-60: #665e52;
  --color-base-70: #524a40;
  --color-base-100: #3a3630;

  /* Extended colors */
  --color-red: var(--wb-error);
  --color-red-rgb: 176, 52, 52;
  --color-orange: var(--wb-burnt-orange);
  --color-orange-rgb: 146, 72, 0;
  --color-yellow: var(--wb-amber);
  --color-yellow-rgb: 133, 87, 0;
  --color-green: var(--wb-dried-sage);
  --color-green-rgb: 77, 92, 26;
  --color-cyan: var(--wb-verdigris);
  --color-cyan-rgb: 40, 106, 72;
  --color-blue: var(--wb-steel-patina);
  --color-blue-rgb: 40, 84, 100;
  --color-purple: var(--wb-dusty-mauve);
  --color-purple-rgb: 126, 64, 96;
  --color-pink: var(--wb-coral);
  --color-pink-rgb: 136, 56, 80;

  /* Headings */
  --h1-color: var(--wb-amber);
  --h2-color: var(--wb-burnt-orange);
  --h3-color: var(--wb-aged-brass);
  --h4-color: var(--wb-terra-cotta);
  --h5-color: var(--wb-steel-patina);
  --h6-color: var(--wb-warm-stone);

  /* Selection and highlights */
  --text-selection: rgba(138, 102, 0, 0.15);
  --text-highlight-bg: rgba(138, 102, 0, 0.12);
}

/* -- Shared (resolves per-variant via cascade) ---------------------------- */
body {
  /* Accent: copper rust HSL decomposition of #b8522e */
  --accent-h: 16;
  --accent-s: 60%;
  --accent-l: 45%;

  /* Code syntax highlighting (CodeMirror 6 + Prism.js shared tokens) */
  --code-background: var(--color-base-00);
  --code-normal: var(--wb-fg);
  --code-function: var(--wb-amber);
  --code-keyword: var(--wb-burnt-orange);
  --code-string: var(--wb-dried-sage);
  --code-comment: var(--wb-warm-stone);
  --code-tag: var(--wb-terra-cotta);
  --code-value: var(--wb-dusty-mauve);
  --code-property: var(--wb-aged-brass);
  --code-important: var(--wb-verdigris);
  --code-operator: var(--wb-operator);
  --code-punctuation: var(--color-base-60);

  /* Warm shadows */
  --background-modifier-box-shadow: rgba(26, 21, 16, 0.3);
  --input-shadow: 0 1px 2px rgba(26, 21, 16, 0.15);
  --shadow-s: 0px 1px 2px rgba(26, 21, 16, 0.12), 0px 3px 6px rgba(26, 21, 16, 0.08);
  --shadow-l: 0px 2px 4px rgba(26, 21, 16, 0.12), 0px 8px 24px rgba(26, 21, 16, 0.16);

  /* Warm scrollbars */
  --scrollbar-bg: var(--color-base-10);
  --scrollbar-thumb-bg: var(--color-base-35);
  --scrollbar-active-thumb-bg: var(--color-base-40);

  /* Softer radii */
  --radius-s: 4px;
  --radius-m: 6px;
  --radius-l: 10px;
}

/* -- Prism.js token overrides (reading view) ------------------------------ */
.markdown-reading-view .token.keyword {
  color: var(--wb-burnt-orange);
  font-weight: bold;
}

.markdown-reading-view .token.function {
  color: var(--wb-amber);
}

.markdown-reading-view .token.string,
.markdown-reading-view .token.char,
.markdown-reading-view .token.attr-value {
  color: var(--wb-dried-sage);
}

.markdown-reading-view .token.number,
.markdown-reading-view .token.boolean,
.markdown-reading-view .token.constant {
  color: var(--wb-dusty-mauve);
}

.markdown-reading-view .token.comment {
  color: var(--wb-warm-stone);
  font-style: italic;
}

.markdown-reading-view .token.operator {
  color: var(--wb-operator);
}

.markdown-reading-view .token.punctuation {
  color: var(--color-base-60);
}

.markdown-reading-view .token.property {
  color: var(--wb-aged-brass);
  font-style: italic;
}

.markdown-reading-view .token.class-name {
  color: var(--wb-steel-patina);
  font-style: italic;
}

.markdown-reading-view .token.tag {
  color: var(--wb-terra-cotta);
  font-weight: bold;
}

.markdown-reading-view .token.regex {
  color: var(--wb-verdigris);
}

.markdown-reading-view .token.symbol {
  color: var(--wb-terra-cotta);
}

.markdown-reading-view .token.selector {
  color: var(--wb-coral);
}
```

- [ ] **Step 2: Run obsidian tests**

Run: `cargo test --test obsidian 2>&1`

Expected: All tests pass.

- [ ] **Step 3: Commit**

```bash
git add obsidian/theme.css
git commit -m "Implement Obsidian theme CSS with warm palette and surface ramp"
```

---

### Task 4: Manifest

**Files:**
- Modify: `obsidian/manifest.json`

- [ ] **Step 1: Write manifest.json**

Replace the stub with:

```json
{
  "name": "Warm Burnout",
  "version": "1.4.2",
  "minAppVersion": "1.0.0",
  "author": "Felipe Lima",
  "authorUrl": "https://warmburnout.com"
}
```

- [ ] **Step 2: Run obsidian tests**

Run: `cargo test --test obsidian 2>&1`

Expected: All tests pass (including manifest tests).

- [ ] **Step 3: Commit**

```bash
git add obsidian/manifest.json
git commit -m "Add Obsidian theme manifest"
```

---

### Task 5: Cross-Platform Consistency Tests

**Files:**
- Modify: `tests/canonical.rs`

- [ ] **Step 1: Add Obsidian cross-platform tests to canonical.rs**

Append at the end of `tests/canonical.rs`:

```rust
// -- Obsidian cross-platform consistency --

const OBSIDIAN_THEME: &str = include_str!("../obsidian/theme.css");

#[test]
fn dark_background_obsidian_matches_vscode() {
  let obsidian = obsidian_color(OBSIDIAN_THEME, "dark", "bg");
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-dark.json"),
    "editor.background",
  );
  assert_eq!(
    obsidian, vscode,
    "dark background: obsidian={obsidian} vscode={vscode}"
  );
}

#[test]
fn light_background_obsidian_matches_vscode() {
  let obsidian = obsidian_color(OBSIDIAN_THEME, "light", "bg");
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-light.json"),
    "editor.background",
  );
  assert_eq!(
    obsidian, vscode,
    "light background: obsidian={obsidian} vscode={vscode}"
  );
}

#[test]
fn dark_foreground_obsidian_matches_vscode() {
  let obsidian = obsidian_color(OBSIDIAN_THEME, "dark", "fg");
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-dark.json"),
    "editor.foreground",
  );
  assert_eq!(
    obsidian, vscode,
    "dark foreground: obsidian={obsidian} vscode={vscode}"
  );
}

#[test]
fn light_foreground_obsidian_matches_vscode() {
  let obsidian = obsidian_color(OBSIDIAN_THEME, "light", "fg");
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-light.json"),
    "editor.foreground",
  );
  assert_eq!(
    obsidian, vscode,
    "light foreground: obsidian={obsidian} vscode={vscode}"
  );
}

#[test]
fn dark_accent_obsidian_matches_canonical() {
  let obsidian = obsidian_color(OBSIDIAN_THEME, "dark", "accent");
  assert_eq!(obsidian, "#b8522e", "dark accent should be canonical copper rust");
}

#[test]
fn light_accent_obsidian_matches_canonical() {
  let obsidian = obsidian_color(OBSIDIAN_THEME, "light", "accent");
  assert_eq!(obsidian, "#b8522e", "light accent should be canonical copper rust");
}

#[test]
fn dark_background_obsidian_matches_ghostty() {
  let obsidian = obsidian_color(OBSIDIAN_THEME, "dark", "bg");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "background");
  assert_eq!(
    obsidian, ghostty,
    "dark background: obsidian={obsidian} ghostty={ghostty}"
  );
}

#[test]
fn light_background_obsidian_matches_ghostty() {
  let obsidian = obsidian_color(OBSIDIAN_THEME, "light", "bg");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "background");
  assert_eq!(
    obsidian, ghostty,
    "light background: obsidian={obsidian} ghostty={ghostty}"
  );
}

#[test]
fn dark_foreground_obsidian_matches_ghostty() {
  let obsidian = obsidian_color(OBSIDIAN_THEME, "dark", "fg");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "foreground");
  assert_eq!(
    obsidian, ghostty,
    "dark foreground: obsidian={obsidian} ghostty={ghostty}"
  );
}

#[test]
fn light_foreground_obsidian_matches_ghostty() {
  let obsidian = obsidian_color(OBSIDIAN_THEME, "light", "fg");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "foreground");
  assert_eq!(
    obsidian, ghostty,
    "light foreground: obsidian={obsidian} ghostty={ghostty}"
  );
}

#[test]
fn dark_cursor_obsidian_matches_ghostty() {
  let obsidian = obsidian_color(OBSIDIAN_THEME, "dark", "cursor");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "cursor-color");
  assert_eq!(
    obsidian, ghostty,
    "dark cursor: obsidian={obsidian} ghostty={ghostty}"
  );
}

#[test]
fn light_cursor_obsidian_matches_ghostty() {
  let obsidian = obsidian_color(OBSIDIAN_THEME, "light", "cursor");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "cursor-color");
  assert_eq!(
    obsidian, ghostty,
    "light cursor: obsidian={obsidian} ghostty={ghostty}"
  );
}
```

- [ ] **Step 2: Run canonical tests**

Run: `cargo test --test canonical obsidian 2>&1`

Expected: All 12 new tests pass.

- [ ] **Step 3: Commit**

```bash
git add tests/canonical.rs
git commit -m "Add Obsidian to cross-platform consistency tests"
```

---

### Task 6: Brand Tests

**Files:**
- Modify: `tests/brand.rs`

- [ ] **Step 1: Add Obsidian README to brand test list**

In `tests/brand.rs`, add to the `READMES` array (after the last entry, before `];`):

```rust
  ("obsidian", include_str!("../obsidian/README.md")),
```

- [ ] **Step 2: Add Obsidian theme to the patina check list**

In `tests/brand.rs`, add to the `theme_files` array inside `no_theme_file_uses_patina_as_label` (after the last entry, before `];`):

```rust
    ("obsidian/theme", include_str!("../obsidian/theme.css")),
```

Note: This will fail until Task 7 creates the README. If working sequentially, do Task 7 first or create a minimal README stub:

```markdown
# Warm Burnout for Obsidian
```

- [ ] **Step 3: Run brand tests**

Run: `cargo test --test brand 2>&1`

Expected: All pass (theme.css contains "Warm Burnout" in its header comment, README will contain it too).

- [ ] **Step 4: Commit**

```bash
git add tests/brand.rs
git commit -m "Add Obsidian to brand enforcement tests"
```

---

### Task 7: Documentation

**Files:**
- Create: `obsidian/README.md`
- Create: `obsidian/AGENTS.md`

- [ ] **Step 1: Write obsidian/README.md**

```markdown
# Warm Burnout for Obsidian

Your second brain was running on factory-default colors. Cold blues, harsh whites, zero consideration for 2am rabbit holes through your note graph. Fixed.

Full community theme for Obsidian with dark and light variants. Warm palette, contrast-audited, zero blues in the chrome. Headings follow a natural energy gradient from amber down to warm stone.

## Install

### Community Themes (recommended)

1. Open Settings > Appearance > Themes
2. Click "Manage" and search for **Warm Burnout**
3. Install and activate

### Manual

1. Download `theme.css` and `manifest.json` from the [latest release](https://github.com/felipefdl/warm-burnout/releases)
2. Create a folder called `Warm Burnout` inside your vault's `.obsidian/themes/` directory
3. Place both files in that folder
4. Open Settings > Appearance > Themes and select **Warm Burnout**

## What This Themes

- **Surface hierarchy**: 13-step warm ramp from deep brown-black to warm cream. No neutral grays anywhere.
- **Headings**: H1 through H6 mapped to palette materials (amber, burnt orange, aged brass, terra cotta, steel patina, warm stone). Visual weight decreases naturally.
- **Code blocks**: Full syntax palette in both editing and reading views. Functions = amber, keywords = burnt orange, strings = dried sage, types = steel patina.
- **Accent**: Copper rust on all interactive elements, links, and active states.
- **Warm tweaks**: Tinted shadows, warm scrollbar tracks, soft selection highlights. No layout changes.

## Palette

Both variants derive from the canonical Warm Burnout palette defined in the root [`AGENTS.md`](../AGENTS.md).

Dark background: `#1a1510`. Light background: `#F5EDE0`. Accent: `#b8522e` (copper rust).

## Requirements

Obsidian 1.0.0 or later.
```

- [ ] **Step 2: Write obsidian/AGENTS.md**

```markdown
# Obsidian -- Agent Instructions

## Platform Reference

See the root [`AGENTS.md`](../AGENTS.md) for the canonical palette, design principles, and brand rules. Do not duplicate palette tables here.

## Obsidian Theme Format

- Obsidian community themes consist of `theme.css` and `manifest.json` at the repo/directory root.
- Both dark and light variants live in a single `theme.css`, switched via `.theme-dark` / `.theme-light` body class selectors.
- `manifest.json` contains name, version, minAppVersion, author, and optional authorUrl.

## CSS Architecture

The theme uses a four-layer CSS custom property system:

1. **Palette layer** (`--wb-*`): Canonical hex values inside `.theme-dark` / `.theme-light`. Only place raw hex appears. Test harness reads from these.
2. **Base mapping**: Maps `--wb-*` into Obsidian's `--color-base-*` ramp (13 steps) and `--color-*` extended colors.
3. **Code syntax**: `--code-*` variables mapped to `--wb-*` for both CodeMirror 6 and Prism.js. Additional `.token.*` rules for reading view.
4. **Warmth tweaks**: Warm shadows, scrollbar tints, softer radii. No layout changes.

## Color Variable Extraction

The test harness uses `obsidian_color(src, variant, key)` to extract `--wb-{key}: #hex;` from inside the `.theme-{variant}` block. When adding new palette colors, add them as `--wb-*` declarations in both variant blocks.

## Surface Ramp

The `--color-base-*` scale uses 13 steps (00, 05, 10, 20, 25, 30, 35, 40, 50, 60, 70, 100). All intermediates carry a warm undertone. In dark mode, 00 = deepest surface, 100 = primary text. In light mode, 00 = lightest surface, 100 = primary text.

## Heading Colors

H1 through H6 are mapped to palette materials by visual weight: amber, burnt orange, aged brass, terra cotta, steel patina, warm stone. Set via `--h1-color` through `--h6-color` in each variant block.

## Distribution

The theme is distributed via a mirror repo (`felipefdl/warm-burnout-obsidian`) that CI syncs on tag push. The Obsidian community directory pulls from that mirror. Source of truth is always this monorepo.

## File Structure

```
obsidian/
  theme.css        # Single CSS file, both dark + light variants
  manifest.json    # Obsidian theme manifest
  README.md        # Install instructions
  AGENTS.md        # This file
```

## Rules

1. Every hex value in `--wb-*` declarations must come from the canonical palette. No approximations.
2. Surface ramp intermediates must carry warm undertone (R > G > B in hex channels).
3. Do not add blues outside of the steel patina type accent.
4. Keep both `.theme-dark` and `.theme-light` blocks in sync: same `--wb-*` variable set, different values.
5. Test changes with `cargo test --test obsidian`.
```

- [ ] **Step 3: Run full test suite**

Run: `cargo test 2>&1 | tail -5`

Expected: All tests pass, including the brand test for the new README.

- [ ] **Step 4: Commit**

```bash
git add obsidian/README.md obsidian/AGENTS.md
git commit -m "Add Obsidian platform documentation"
```

---

### Task 8: Release Workflow and CI

**Files:**
- Modify: `.github/workflows/release-themes.yml`

- [ ] **Step 1: Add Obsidian packaging step**

In `.github/workflows/release-themes.yml`, add after the "Package Home Assistant" step (before the "Package JetBrains" step):

```yaml
      - name: Package Obsidian
        run: |
          cd obsidian
          zip -j ../warm-burnout-obsidian.zip \
            theme.css \
            manifest.json
```

- [ ] **Step 2: Add zip to release attachment list**

In the `files:` block of the "Attach to GitHub Release" step, add `warm-burnout-obsidian.zip` after `warm-burnout-home-assistant.yaml`:

```yaml
            warm-burnout-home-assistant.yaml
            warm-burnout-obsidian.zip
            jetbrains/warm-burnout-theme.jar
```

- [ ] **Step 3: Add mirror sync job**

Add a new job at the end of the file, after the `package` job:

```yaml
  sync-obsidian-mirror:
    needs: [validate]
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6

      - name: Sync to mirror repo
        env:
          MIRROR_DEPLOY_KEY: ${{ secrets.OBSIDIAN_MIRROR_DEPLOY_KEY }}
        run: |
          mkdir -p ~/.ssh
          echo "$MIRROR_DEPLOY_KEY" > ~/.ssh/deploy_key
          chmod 600 ~/.ssh/deploy_key
          export GIT_SSH_COMMAND="ssh -i ~/.ssh/deploy_key -o StrictHostKeyChecking=no"

          git clone git@github.com:felipefdl/warm-burnout-obsidian.git mirror
          cp obsidian/theme.css mirror/
          cp obsidian/manifest.json mirror/
          cp obsidian/README.md mirror/
          cp LICENSE mirror/

          cd mirror
          git config user.name "github-actions[bot]"
          git config user.email "github-actions[bot]@users.noreply.github.com"
          git add -A
          if git diff --cached --quiet; then
            echo "No changes to sync"
          else
            git commit -m "Sync from warm-burnout ${GITHUB_REF_NAME}"
            git tag "${GITHUB_REF_NAME}"
            git push origin main --tags
          fi
```

- [ ] **Step 4: Commit**

```bash
git add .github/workflows/release-themes.yml
git commit -m "Add Obsidian packaging and mirror sync to release workflow"
```

---

### Task 9: Monorepo Integration

**Files:**
- Modify: `README.md` (root)
- Modify: `site/index.html`
- Modify: `AGENTS.md` (root)

- [ ] **Step 1: Add Obsidian to root README platforms table**

In `README.md`, add a new row after the eza row (before the empty line after the table):

```markdown
| Obsidian | Available | [`obsidian/`](obsidian/) |
```

- [ ] **Step 2: Add Obsidian to site platforms grid**

In `site/index.html`, add after the eza card (before the closing `</div>` of the grid, around line 464):

```html
          <a href="https://github.com/felipefdl/warm-burnout/tree/main/obsidian" class="retro-card rounded-md p-4 no-underline block">
            <p class="text-sm font-bold theme-fg">Obsidian</p>
            <p class="text-xs theme-fg-muted mt-1">Notes</p>
            <span class="inline-block mt-2 text-xs font-bold px-2 py-0.5 rounded" style="background: rgba(184, 82, 46, 0.15); color: var(--theme-accent);">Available</span>
          </a>
```

- [ ] **Step 3: Update platform count in site**

In `site/index.html`, update all instances of "15 platforms" to "16 platforms". There are four locations:

1. `<meta name="description">` tag (line 8)
2. `<meta property="og:description">` tag (line 15)
3. `<meta name="twitter:description">` tag (line 28)
4. JSON-LD `"description"` (line 38)
5. The platforms section subtitle (line 377): change `15 platforms. The burnout is spreading.` to `16 platforms. The burnout is spreading.`

- [ ] **Step 4: Add Obsidian to root AGENTS.md project structure**

In `AGENTS.md`, add to the project structure tree (after the eza block, around line 135):

```
  obsidian/                    # Obsidian community theme
    theme.css                  # Dark + light variants (CSS custom properties)
    manifest.json              # Community theme manifest
    README.md                  # Obsidian install instructions
    AGENTS.md                  # Obsidian-specific agent rules
```

Also add `obsidian.rs` to the tests listing:

```
    obsidian.rs               # Obsidian theme validation tests
```

- [ ] **Step 5: Run full test suite**

Run: `cargo test 2>&1 | tail -5`

Expected: All tests pass.

- [ ] **Step 6: Run formatting and linting**

Run: `cargo fmt -- --check && cargo clippy -- -D warnings`

Expected: No issues.

- [ ] **Step 7: Commit**

```bash
git add README.md site/index.html AGENTS.md
git commit -m "Add Obsidian to monorepo platforms list and site"
```

---

### Task 10: Final Verification

- [ ] **Step 1: Run the complete test suite**

Run: `cargo test 2>&1`

Expected: All tests pass. The output should show the new obsidian and canonical tests.

- [ ] **Step 2: Verify test count increased**

Run: `cargo test 2>&1 | grep "test result"`

Note the total test count. It should have increased by roughly 28 tests (16 obsidian.rs + 12 canonical.rs).

- [ ] **Step 3: Verify file structure**

Run: `ls -la obsidian/`

Expected:
```
AGENTS.md
README.md
manifest.json
theme.css
```

- [ ] **Step 4: Spot-check the CSS**

Run: `head -20 obsidian/theme.css`

Verify the header comment and the start of `.theme-dark` look correct.
