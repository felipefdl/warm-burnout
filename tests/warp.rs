mod common;

use common::{extract_hex_colors, ghostty_ansi_color, ghostty_color, is_valid_hex, warp_ansi_color, warp_color};
use serde_yml::Value;

const DARK: &str = include_str!("../warp/warm-burnout-dark.yaml");
const LIGHT: &str = include_str!("../warp/warm-burnout-light.yaml");

const GHOSTTY_DARK: &str = include_str!("../ghostty/warm-burnout-dark");
const GHOSTTY_LIGHT: &str = include_str!("../ghostty/warm-burnout-light");

const ANSI_KEYS: &[&str] = &["black", "red", "green", "yellow", "blue", "magenta", "cyan", "white"];

const REQUIRED_TOP_LEVEL: &[&str] = &[
  "name",
  "accent",
  "background",
  "foreground",
  "cursor",
  "details",
  "terminal_colors",
];

fn parse_theme(src: &str) -> Value {
  serde_yml::from_str(src).expect("invalid YAML")
}

// -- Valid YAML --

#[test]
fn dark_is_valid_yaml() {
  parse_theme(DARK);
}

#[test]
fn light_is_valid_yaml() {
  parse_theme(LIGHT);
}

// -- All hex colors are valid --

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

// -- Required top-level keys --

#[test]
fn dark_has_required_top_level_keys() {
  let v = parse_theme(DARK);
  for key in REQUIRED_TOP_LEVEL {
    assert!(v.get(key).is_some(), "dark missing required key: {key}");
  }
}

#[test]
fn light_has_required_top_level_keys() {
  let v = parse_theme(LIGHT);
  for key in REQUIRED_TOP_LEVEL {
    assert!(v.get(key).is_some(), "light missing required key: {key}");
  }
}

#[test]
fn dark_no_unexpected_top_level_keys() {
  let v = parse_theme(DARK);
  let mapping = v.as_mapping().expect("dark top level should be a YAML mapping");
  for (key, _) in mapping {
    let key_str = key.as_str().unwrap_or("<non-string>");
    assert!(
      REQUIRED_TOP_LEVEL.contains(&key_str),
      "dark: unexpected top-level key: '{key_str}'"
    );
  }
}

#[test]
fn light_no_unexpected_top_level_keys() {
  let v = parse_theme(LIGHT);
  let mapping = v.as_mapping().expect("light top level should be a YAML mapping");
  for (key, _) in mapping {
    let key_str = key.as_str().unwrap_or("<non-string>");
    assert!(
      REQUIRED_TOP_LEVEL.contains(&key_str),
      "light: unexpected top-level key: '{key_str}'"
    );
  }
}

// -- name and details --

#[test]
fn dark_name_is_warm_burnout_dark() {
  let v = parse_theme(DARK);
  assert_eq!(v.get("name").and_then(|x| x.as_str()), Some("Warm Burnout Dark"));
}

#[test]
fn light_name_is_warm_burnout_light() {
  let v = parse_theme(LIGHT);
  assert_eq!(v.get("name").and_then(|x| x.as_str()), Some("Warm Burnout Light"));
}

#[test]
fn dark_details_is_darker() {
  let v = parse_theme(DARK);
  assert_eq!(v.get("details").and_then(|x| x.as_str()), Some("darker"));
}

#[test]
fn light_details_is_lighter() {
  let v = parse_theme(LIGHT);
  assert_eq!(v.get("details").and_then(|x| x.as_str()), Some("lighter"));
}

// -- terminal_colors structure --

#[test]
fn dark_terminal_colors_have_normal_and_bright() {
  let v = parse_theme(DARK);
  assert!(v["terminal_colors"]["normal"].is_mapping(), "dark missing normal block");
  assert!(v["terminal_colors"]["bright"].is_mapping(), "dark missing bright block");
}

#[test]
fn light_terminal_colors_have_normal_and_bright() {
  let v = parse_theme(LIGHT);
  assert!(
    v["terminal_colors"]["normal"].is_mapping(),
    "light missing normal block"
  );
  assert!(
    v["terminal_colors"]["bright"].is_mapping(),
    "light missing bright block"
  );
}

fn assert_ansi_block_keys(v: &Value, variant: &str, bank: &str) {
  let mapping = v["terminal_colors"][bank]
    .as_mapping()
    .unwrap_or_else(|| panic!("{variant} terminal_colors.{bank} not a mapping"));
  let actual: Vec<&str> = mapping.keys().filter_map(|k| k.as_str()).collect();
  assert_eq!(
    actual.len(),
    8,
    "{variant} {bank}: expected 8 keys, got {} ({actual:?})",
    actual.len()
  );
  for key in ANSI_KEYS {
    assert!(
      actual.contains(key),
      "{variant} {bank}: missing ANSI slot '{key}' (keys: {actual:?})"
    );
  }
}

#[test]
fn dark_normal_has_all_8_ansi_keys() {
  assert_ansi_block_keys(&parse_theme(DARK), "dark", "normal");
}

#[test]
fn dark_bright_has_all_8_ansi_keys() {
  assert_ansi_block_keys(&parse_theme(DARK), "dark", "bright");
}

#[test]
fn light_normal_has_all_8_ansi_keys() {
  assert_ansi_block_keys(&parse_theme(LIGHT), "light", "normal");
}

#[test]
fn light_bright_has_all_8_ansi_keys() {
  assert_ansi_block_keys(&parse_theme(LIGHT), "light", "bright");
}

// -- Canonical chrome --

#[test]
fn dark_accent_is_canonical_copper_rust() {
  assert_eq!(warp_color(DARK, "accent"), "#b8522e");
}

#[test]
fn light_accent_is_canonical_copper_rust() {
  assert_eq!(warp_color(LIGHT, "accent"), "#b8522e");
}

#[test]
fn dark_background_matches_ghostty() {
  let warp = warp_color(DARK, "background");
  let ghostty = ghostty_color(GHOSTTY_DARK, "background");
  assert_eq!(warp, ghostty, "dark background: warp={warp} ghostty={ghostty}");
}

#[test]
fn light_background_matches_ghostty() {
  let warp = warp_color(LIGHT, "background");
  let ghostty = ghostty_color(GHOSTTY_LIGHT, "background");
  assert_eq!(warp, ghostty, "light background: warp={warp} ghostty={ghostty}");
}

#[test]
fn dark_foreground_matches_ghostty() {
  let warp = warp_color(DARK, "foreground");
  let ghostty = ghostty_color(GHOSTTY_DARK, "foreground");
  assert_eq!(warp, ghostty, "dark foreground: warp={warp} ghostty={ghostty}");
}

#[test]
fn light_foreground_matches_ghostty() {
  let warp = warp_color(LIGHT, "foreground");
  let ghostty = ghostty_color(GHOSTTY_LIGHT, "foreground");
  assert_eq!(warp, ghostty, "light foreground: warp={warp} ghostty={ghostty}");
}

#[test]
fn dark_cursor_matches_ghostty() {
  let warp = warp_color(DARK, "cursor");
  let ghostty = ghostty_color(GHOSTTY_DARK, "cursor-color");
  assert_eq!(warp, ghostty, "dark cursor: warp={warp} ghostty={ghostty}");
}

#[test]
fn light_cursor_matches_ghostty() {
  let warp = warp_color(LIGHT, "cursor");
  let ghostty = ghostty_color(GHOSTTY_LIGHT, "cursor-color");
  assert_eq!(warp, ghostty, "light cursor: warp={warp} ghostty={ghostty}");
}

// -- ANSI palette matches Ghostty --

fn assert_ansi_bank_matches_ghostty(warp_src: &str, ghostty_src: &str, bank: &str, base: u8, variant: &str) {
  for (offset, name) in ANSI_KEYS.iter().enumerate() {
    let warp = warp_ansi_color(warp_src, bank, name);
    let ghostty = ghostty_ansi_color(ghostty_src, base + offset as u8);
    assert_eq!(
      warp,
      ghostty,
      "{variant} {bank}.{name} (palette {}): warp={warp} ghostty={ghostty}",
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

// -- Brand rule: warm everywhere, blue nowhere (chrome only). --
// ANSI blue must literally exist in terminal_colors, so we exclude that block.

#[test]
fn dark_no_canonical_steel_blue_in_chrome() {
  for key in ["accent", "background", "foreground", "cursor"] {
    let val = warp_color(DARK, key);
    assert_ne!(val, "#90aec0", "dark {key} must not be canonical steel-blue");
  }
}

#[test]
fn light_no_canonical_steel_blue_in_chrome() {
  for key in ["accent", "background", "foreground", "cursor"] {
    let val = warp_color(LIGHT, key);
    assert_ne!(val, "#285464", "light {key} must not be canonical steel-blue");
  }
}
