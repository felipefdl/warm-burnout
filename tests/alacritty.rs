mod common;

use common::{alacritty_color, extract_hex_colors, ghostty_ansi_color, ghostty_color, is_valid_hex};

const DARK: &str = include_str!("../alacritty/warm-burnout-dark.toml");
const LIGHT: &str = include_str!("../alacritty/warm-burnout-light.toml");

const GHOSTTY_DARK: &str = include_str!("../ghostty/warm-burnout-dark");
const GHOSTTY_LIGHT: &str = include_str!("../ghostty/warm-burnout-light");

const ANSI_KEYS: &[&str] = &["black", "red", "green", "yellow", "blue", "magenta", "cyan", "white"];

const REQUIRED_TABLES: &[&str] = &[
  "colors.primary",
  "colors.cursor",
  "colors.selection",
  "colors.normal",
  "colors.bright",
  "colors.vi_mode_cursor",
  "colors.search.matches",
  "colors.search.focused_match",
  "colors.footer_bar",
  "colors.hints.start",
  "colors.hints.end",
];

fn parse_toml(src: &str) -> toml::Table {
  src.parse::<toml::Table>().expect("invalid TOML")
}

// -- Valid TOML --

#[test]
fn dark_is_valid_toml() {
  parse_toml(DARK);
}

#[test]
fn light_is_valid_toml() {
  parse_toml(LIGHT);
}

// -- All hex colors in the file are valid --

#[test]
fn dark_all_hex_colors_are_valid() {
  for (line, hex) in extract_hex_colors(DARK) {
    assert!(is_valid_hex(hex), "dark line {line}: invalid hex: {hex}");
  }
}

#[test]
fn light_all_hex_colors_are_valid() {
  for (line, hex) in extract_hex_colors(LIGHT) {
    assert!(is_valid_hex(hex), "light line {line}: invalid hex: {hex}");
  }
}

// -- Required tables present --

fn assert_path_exists(src: &str, path: &str, variant: &str) {
  let table = parse_toml(src);
  let mut value: toml::Value = toml::Value::Table(table);
  for part in path.split('.') {
    value = match value.get(part) {
      Some(v) => v.clone(),
      None => panic!("{variant}: missing required table path '{path}' (failed at '{part}')"),
    };
  }
  assert!(value.is_table(), "{variant}: '{path}' exists but is not a table");
}

#[test]
fn dark_has_all_required_tables() {
  for path in REQUIRED_TABLES {
    assert_path_exists(DARK, path, "dark");
  }
}

#[test]
fn light_has_all_required_tables() {
  for path in REQUIRED_TABLES {
    assert_path_exists(LIGHT, path, "light");
  }
}

// -- Canonical chrome --

#[test]
fn dark_background_is_canonical() {
  assert_eq!(alacritty_color(DARK, "colors.primary.background"), "#1a1510");
}

#[test]
fn light_background_is_canonical() {
  assert_eq!(alacritty_color(LIGHT, "colors.primary.background"), "#f5ede0");
}

#[test]
fn dark_foreground_is_canonical() {
  assert_eq!(alacritty_color(DARK, "colors.primary.foreground"), "#bfbdb6");
}

#[test]
fn light_foreground_is_canonical() {
  assert_eq!(alacritty_color(LIGHT, "colors.primary.foreground"), "#3a3630");
}

#[test]
fn dark_cursor_is_canonical() {
  assert_eq!(alacritty_color(DARK, "colors.cursor.cursor"), "#f5c56e");
}

#[test]
fn light_cursor_is_canonical() {
  assert_eq!(alacritty_color(LIGHT, "colors.cursor.cursor"), "#8a6600");
}

// -- Cross-platform: chrome matches Ghostty --

#[test]
fn dark_background_matches_ghostty() {
  let alac = alacritty_color(DARK, "colors.primary.background");
  let ghostty = ghostty_color(GHOSTTY_DARK, "background");
  assert_eq!(alac, ghostty, "dark background: alacritty={alac} ghostty={ghostty}");
}

#[test]
fn light_background_matches_ghostty() {
  let alac = alacritty_color(LIGHT, "colors.primary.background");
  let ghostty = ghostty_color(GHOSTTY_LIGHT, "background");
  assert_eq!(alac, ghostty, "light background: alacritty={alac} ghostty={ghostty}");
}

#[test]
fn dark_foreground_matches_ghostty() {
  let alac = alacritty_color(DARK, "colors.primary.foreground");
  let ghostty = ghostty_color(GHOSTTY_DARK, "foreground");
  assert_eq!(alac, ghostty, "dark foreground: alacritty={alac} ghostty={ghostty}");
}

#[test]
fn light_foreground_matches_ghostty() {
  let alac = alacritty_color(LIGHT, "colors.primary.foreground");
  let ghostty = ghostty_color(GHOSTTY_LIGHT, "foreground");
  assert_eq!(alac, ghostty, "light foreground: alacritty={alac} ghostty={ghostty}");
}

#[test]
fn dark_cursor_matches_ghostty() {
  let alac = alacritty_color(DARK, "colors.cursor.cursor");
  let ghostty = ghostty_color(GHOSTTY_DARK, "cursor-color");
  assert_eq!(alac, ghostty, "dark cursor: alacritty={alac} ghostty={ghostty}");
}

#[test]
fn light_cursor_matches_ghostty() {
  let alac = alacritty_color(LIGHT, "colors.cursor.cursor");
  let ghostty = ghostty_color(GHOSTTY_LIGHT, "cursor-color");
  assert_eq!(alac, ghostty, "light cursor: alacritty={alac} ghostty={ghostty}");
}

// -- Cursor text matches background (readable cursor character) --

#[test]
fn dark_cursor_text_matches_background() {
  assert_eq!(
    alacritty_color(DARK, "colors.cursor.text"),
    alacritty_color(DARK, "colors.primary.background"),
    "dark cursor.text should match background for readability"
  );
}

#[test]
fn light_cursor_text_matches_background() {
  assert_eq!(
    alacritty_color(LIGHT, "colors.cursor.text"),
    alacritty_color(LIGHT, "colors.primary.background"),
    "light cursor.text should match background for readability"
  );
}

// -- bright_foreground equals foreground --
// Bold text must not punch up to harsh white. See alacritty/AGENTS.md.

#[test]
fn dark_bright_foreground_matches_foreground() {
  assert_eq!(
    alacritty_color(DARK, "colors.primary.bright_foreground"),
    alacritty_color(DARK, "colors.primary.foreground"),
    "dark bright_foreground must equal foreground (no luminance spike on bold)"
  );
}

#[test]
fn light_bright_foreground_matches_foreground() {
  assert_eq!(
    alacritty_color(LIGHT, "colors.primary.bright_foreground"),
    alacritty_color(LIGHT, "colors.primary.foreground"),
    "light bright_foreground must equal foreground (no luminance spike on bold)"
  );
}

// -- Selection: opaque blend (alpha not honoured by Alacritty) --

#[test]
fn dark_selection_background_is_canonical_opaque_blend() {
  assert_eq!(alacritty_color(DARK, "colors.selection.background"), "#33393a");
}

#[test]
fn light_selection_background_is_canonical_opaque_blend() {
  assert_eq!(alacritty_color(LIGHT, "colors.selection.background"), "#e5e8e2");
}

#[test]
fn dark_selection_text_matches_foreground() {
  assert_eq!(
    alacritty_color(DARK, "colors.selection.text"),
    alacritty_color(DARK, "colors.primary.foreground"),
  );
}

#[test]
fn light_selection_text_matches_foreground() {
  assert_eq!(
    alacritty_color(LIGHT, "colors.selection.text"),
    alacritty_color(LIGHT, "colors.primary.foreground"),
  );
}

// -- ANSI palette matches Ghostty --

fn assert_ansi_bank_matches_ghostty(src: &str, ghostty_src: &str, bank: &str, base: u8, variant: &str) {
  for (offset, name) in ANSI_KEYS.iter().enumerate() {
    let alac = alacritty_color(src, &format!("colors.{bank}.{name}"));
    let ghostty = ghostty_ansi_color(ghostty_src, base + offset as u8);
    assert_eq!(
      alac,
      ghostty,
      "{variant} {bank}.{name} (palette {}): alacritty={alac} ghostty={ghostty}",
      base + offset as u8
    );
  }
}

#[test]
fn dark_normal_ansi_matches_ghostty() {
  assert_ansi_bank_matches_ghostty(DARK, GHOSTTY_DARK, "normal", 0, "dark");
}

#[test]
fn dark_bright_ansi_matches_ghostty() {
  assert_ansi_bank_matches_ghostty(DARK, GHOSTTY_DARK, "bright", 8, "dark");
}

#[test]
fn light_normal_ansi_matches_ghostty() {
  assert_ansi_bank_matches_ghostty(LIGHT, GHOSTTY_LIGHT, "normal", 0, "light");
}

#[test]
fn light_bright_ansi_matches_ghostty() {
  assert_ansi_bank_matches_ghostty(LIGHT, GHOSTTY_LIGHT, "bright", 8, "light");
}

// -- vi_mode cursor matches canonical cursor --

#[test]
fn dark_vi_mode_cursor_matches_canonical() {
  assert_eq!(alacritty_color(DARK, "colors.vi_mode_cursor.cursor"), "#f5c56e");
  assert_eq!(alacritty_color(DARK, "colors.vi_mode_cursor.text"), "#1a1510");
}

#[test]
fn light_vi_mode_cursor_matches_canonical() {
  assert_eq!(alacritty_color(LIGHT, "colors.vi_mode_cursor.cursor"), "#8a6600");
  assert_eq!(alacritty_color(LIGHT, "colors.vi_mode_cursor.text"), "#f5ede0");
}

// -- Search highlights use canonical functions / keywords tokens --

#[test]
fn dark_search_matches_use_canonical_functions_token() {
  assert_eq!(alacritty_color(DARK, "colors.search.matches.background"), "#ffb454");
  assert_eq!(alacritty_color(DARK, "colors.search.matches.foreground"), "#1a1510");
}

#[test]
fn light_search_matches_use_canonical_functions_token() {
  assert_eq!(alacritty_color(LIGHT, "colors.search.matches.background"), "#855700");
  assert_eq!(alacritty_color(LIGHT, "colors.search.matches.foreground"), "#f5ede0");
}

#[test]
fn dark_search_focused_match_uses_canonical_keywords_token() {
  assert_eq!(
    alacritty_color(DARK, "colors.search.focused_match.background"),
    "#ff8f40"
  );
  assert_eq!(
    alacritty_color(DARK, "colors.search.focused_match.foreground"),
    "#1a1510"
  );
}

#[test]
fn light_search_focused_match_uses_canonical_keywords_token() {
  assert_eq!(
    alacritty_color(LIGHT, "colors.search.focused_match.background"),
    "#924800"
  );
  assert_eq!(
    alacritty_color(LIGHT, "colors.search.focused_match.foreground"),
    "#f5ede0"
  );
}

// -- Footer bar uses canonical comments token + bg_dim chrome --

#[test]
fn dark_footer_bar_foreground_is_canonical_comments_token() {
  assert_eq!(alacritty_color(DARK, "colors.footer_bar.foreground"), "#b4a89c");
}

#[test]
fn light_footer_bar_foreground_is_canonical_comments_token() {
  assert_eq!(alacritty_color(LIGHT, "colors.footer_bar.foreground"), "#544c40");
}

#[test]
fn dark_footer_bar_background_is_canonical_bg_dim() {
  assert_eq!(alacritty_color(DARK, "colors.footer_bar.background"), "#14120f");
}

#[test]
fn light_footer_bar_background_is_canonical_bg_dim() {
  assert_eq!(alacritty_color(LIGHT, "colors.footer_bar.background"), "#ede6da");
}

// -- Hints use the one sanctioned cool accent (canonical types token) --

#[test]
fn dark_hints_start_uses_canonical_types_token() {
  assert_eq!(alacritty_color(DARK, "colors.hints.start.background"), "#90aec0");
  assert_eq!(alacritty_color(DARK, "colors.hints.start.foreground"), "#1a1510");
}

#[test]
fn light_hints_start_uses_canonical_types_token() {
  assert_eq!(alacritty_color(LIGHT, "colors.hints.start.background"), "#285464");
  assert_eq!(alacritty_color(LIGHT, "colors.hints.start.foreground"), "#f5ede0");
}

#[test]
fn dark_hints_end_uses_canonical_types_token() {
  assert_eq!(alacritty_color(DARK, "colors.hints.end.foreground"), "#90aec0");
}

#[test]
fn light_hints_end_uses_canonical_types_token() {
  assert_eq!(alacritty_color(LIGHT, "colors.hints.end.foreground"), "#285464");
}

#[test]
fn dark_hints_end_background_matches_primary_background() {
  assert_eq!(
    alacritty_color(DARK, "colors.hints.end.background"),
    alacritty_color(DARK, "colors.primary.background"),
    "dark hints.end background should match editor background"
  );
}

#[test]
fn light_hints_end_background_matches_primary_background() {
  assert_eq!(
    alacritty_color(LIGHT, "colors.hints.end.background"),
    alacritty_color(LIGHT, "colors.primary.background"),
    "light hints.end background should match editor background"
  );
}

// -- Background sanity: no pure black, no pure white --

#[test]
fn dark_no_pure_black_background() {
  assert_ne!(
    alacritty_color(DARK, "colors.primary.background"),
    "#000000",
    "dark background must not be pure black (halation risk)"
  );
}

#[test]
fn light_no_pure_white_background() {
  assert_ne!(
    alacritty_color(LIGHT, "colors.primary.background"),
    "#ffffff",
    "light background must not be pure white (luminance overload)"
  );
}

// Brand rule: chrome stays warm; steel type accent is reserved.
// ANSI blue must literally exist in colors.normal/bright, so we exclude those.

#[test]
fn dark_no_canonical_steel_blue_in_chrome() {
  for path in [
    "colors.primary.background",
    "colors.primary.foreground",
    "colors.cursor.cursor",
    "colors.footer_bar.background",
    "colors.footer_bar.foreground",
  ] {
    let val = alacritty_color(DARK, path);
    assert_ne!(val, "#90aec0", "dark {path} must not be canonical steel-blue");
  }
}

#[test]
fn light_no_canonical_steel_blue_in_chrome() {
  for path in [
    "colors.primary.background",
    "colors.primary.foreground",
    "colors.cursor.cursor",
    "colors.footer_bar.background",
    "colors.footer_bar.foreground",
  ] {
    let val = alacritty_color(LIGHT, path);
    assert_ne!(val, "#285464", "light {path} must not be canonical steel-blue");
  }
}
