mod common;

use common::{extract_hex_colors, ghostty_ansi_color, ghostty_color, is_valid_hex, moshi_color};

const DARK: &str = include_str!("../moshi/warm-burnout-dark.json");
const LIGHT: &str = include_str!("../moshi/warm-burnout-light.json");
const GHOSTTY_DARK: &str = include_str!("../ghostty/warm-burnout-dark");
const GHOSTTY_LIGHT: &str = include_str!("../ghostty/warm-burnout-light");

const REQUIRED_TOP_LEVEL: &[&str] = &["v", "name", "mode", "colors"];

const REQUIRED_COLOR_KEYS: &[&str] = &[
  "background",
  "foreground",
  "cursor",
  "black",
  "red",
  "green",
  "yellow",
  "blue",
  "magenta",
  "cyan",
  "white",
  "brightBlack",
  "brightRed",
  "brightGreen",
  "brightYellow",
  "brightBlue",
  "brightMagenta",
  "brightCyan",
  "brightWhite",
  "selectionBackground",
];

const ANSI_NORMAL: &[&str] = &["black", "red", "green", "yellow", "blue", "magenta", "cyan", "white"];
const ANSI_BRIGHT: &[&str] = &[
  "brightBlack",
  "brightRed",
  "brightGreen",
  "brightYellow",
  "brightBlue",
  "brightMagenta",
  "brightCyan",
  "brightWhite",
];

fn parse_json(src: &str) -> serde_json::Value {
  serde_json::from_str(src).expect("invalid JSON")
}

// -- Valid JSON --

#[test]
fn dark_is_valid_json() {
  parse_json(DARK);
}

#[test]
fn light_is_valid_json() {
  parse_json(LIGHT);
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

// -- Top-level schema --

#[test]
fn dark_top_level_keys_are_exact() {
  let v = parse_json(DARK);
  let obj = v.as_object().expect("dark top level should be an object");
  assert_eq!(
    obj.len(),
    REQUIRED_TOP_LEVEL.len(),
    "dark unexpected top-level key count"
  );
  for key in REQUIRED_TOP_LEVEL {
    assert!(obj.contains_key(*key), "dark missing required key: {key}");
  }
  for key in obj.keys() {
    assert!(
      REQUIRED_TOP_LEVEL.contains(&key.as_str()),
      "dark: unexpected top-level key: '{key}'"
    );
  }
}

#[test]
fn light_top_level_keys_are_exact() {
  let v = parse_json(LIGHT);
  let obj = v.as_object().expect("light top level should be an object");
  assert_eq!(
    obj.len(),
    REQUIRED_TOP_LEVEL.len(),
    "light unexpected top-level key count"
  );
  for key in REQUIRED_TOP_LEVEL {
    assert!(obj.contains_key(*key), "light missing required key: {key}");
  }
  for key in obj.keys() {
    assert!(
      REQUIRED_TOP_LEVEL.contains(&key.as_str()),
      "light: unexpected top-level key: '{key}'"
    );
  }
}

#[test]
fn dark_v_is_1() {
  let v = parse_json(DARK);
  assert_eq!(v["v"].as_u64(), Some(1), "dark v must be number 1");
}

#[test]
fn light_v_is_1() {
  let v = parse_json(LIGHT);
  assert_eq!(v["v"].as_u64(), Some(1), "light v must be number 1");
}

// -- name and mode --

#[test]
fn dark_name_is_warm_burnout_dark() {
  let v = parse_json(DARK);
  assert_eq!(v["name"].as_str(), Some("Warm Burnout Dark"));
}

#[test]
fn light_name_is_warm_burnout_light() {
  let v = parse_json(LIGHT);
  assert_eq!(v["name"].as_str(), Some("Warm Burnout Light"));
}

#[test]
fn dark_mode_is_dark() {
  let v = parse_json(DARK);
  assert_eq!(v["mode"].as_str(), Some("dark"));
}

#[test]
fn light_mode_is_light() {
  let v = parse_json(LIGHT);
  assert_eq!(v["mode"].as_str(), Some("light"));
}

// -- Required colors keys --

#[test]
fn dark_has_all_required_color_keys() {
  let v = parse_json(DARK);
  let colors = v["colors"].as_object().expect("dark colors should be an object");
  for key in REQUIRED_COLOR_KEYS {
    assert!(colors.contains_key(*key), "dark missing colors.{key}");
  }
}

#[test]
fn light_has_all_required_color_keys() {
  let v = parse_json(LIGHT);
  let colors = v["colors"].as_object().expect("light colors should be an object");
  for key in REQUIRED_COLOR_KEYS {
    assert!(colors.contains_key(*key), "light missing colors.{key}");
  }
}

// -- Chrome matches Ghostty --

#[test]
fn dark_background_matches_ghostty() {
  let moshi = moshi_color(DARK, "background");
  let ghostty = ghostty_color(GHOSTTY_DARK, "background");
  assert_eq!(moshi, ghostty, "dark background: moshi={moshi} ghostty={ghostty}");
}

#[test]
fn light_background_matches_ghostty() {
  let moshi = moshi_color(LIGHT, "background");
  let ghostty = ghostty_color(GHOSTTY_LIGHT, "background");
  assert_eq!(moshi, ghostty, "light background: moshi={moshi} ghostty={ghostty}");
}

#[test]
fn dark_foreground_matches_ghostty() {
  let moshi = moshi_color(DARK, "foreground");
  let ghostty = ghostty_color(GHOSTTY_DARK, "foreground");
  assert_eq!(moshi, ghostty, "dark foreground: moshi={moshi} ghostty={ghostty}");
}

#[test]
fn light_foreground_matches_ghostty() {
  let moshi = moshi_color(LIGHT, "foreground");
  let ghostty = ghostty_color(GHOSTTY_LIGHT, "foreground");
  assert_eq!(moshi, ghostty, "light foreground: moshi={moshi} ghostty={ghostty}");
}

#[test]
fn dark_cursor_matches_ghostty() {
  let moshi = moshi_color(DARK, "cursor");
  let ghostty = ghostty_color(GHOSTTY_DARK, "cursor-color");
  assert_eq!(moshi, ghostty, "dark cursor: moshi={moshi} ghostty={ghostty}");
}

#[test]
fn light_cursor_matches_ghostty() {
  let moshi = moshi_color(LIGHT, "cursor");
  let ghostty = ghostty_color(GHOSTTY_LIGHT, "cursor-color");
  assert_eq!(moshi, ghostty, "light cursor: moshi={moshi} ghostty={ghostty}");
}

#[test]
fn dark_selection_background_matches_ghostty() {
  let moshi = moshi_color(DARK, "selectionBackground");
  let ghostty = ghostty_color(GHOSTTY_DARK, "selection-background");
  assert_eq!(
    moshi, ghostty,
    "dark selectionBackground: moshi={moshi} ghostty={ghostty}"
  );
}

#[test]
fn light_selection_background_matches_ghostty() {
  let moshi = moshi_color(LIGHT, "selectionBackground");
  let ghostty = ghostty_color(GHOSTTY_LIGHT, "selection-background");
  assert_eq!(
    moshi, ghostty,
    "light selectionBackground: moshi={moshi} ghostty={ghostty}"
  );
}

// -- ANSI palette matches Ghostty --

fn assert_ansi_matches_ghostty(moshi_src: &str, ghostty_src: &str, keys: &[&str], base: u8, variant: &str) {
  for (offset, name) in keys.iter().enumerate() {
    let moshi = moshi_color(moshi_src, name);
    let ghostty = ghostty_ansi_color(ghostty_src, base + offset as u8);
    assert_eq!(
      moshi,
      ghostty,
      "{variant} {name} (palette {}): moshi={moshi} ghostty={ghostty}",
      base + offset as u8
    );
  }
}

#[test]
fn dark_normal_ansi_matches_ghostty() {
  assert_ansi_matches_ghostty(DARK, GHOSTTY_DARK, ANSI_NORMAL, 0, "dark");
}

#[test]
fn dark_bright_ansi_matches_ghostty() {
  assert_ansi_matches_ghostty(DARK, GHOSTTY_DARK, ANSI_BRIGHT, 8, "dark");
}

#[test]
fn light_normal_ansi_matches_ghostty() {
  assert_ansi_matches_ghostty(LIGHT, GHOSTTY_LIGHT, ANSI_NORMAL, 0, "light");
}

#[test]
fn light_bright_ansi_matches_ghostty() {
  assert_ansi_matches_ghostty(LIGHT, GHOSTTY_LIGHT, ANSI_BRIGHT, 8, "light");
}

// -- No pure black/white backgrounds --

#[test]
fn no_pure_black_background() {
  let bg = moshi_color(DARK, "background");
  assert_ne!(bg, "#000000", "dark background must not be pure black");
}

#[test]
fn no_pure_white_background() {
  let bg = moshi_color(LIGHT, "background");
  assert_ne!(bg, "#ffffff", "light background must not be pure white");
}

// Brand rule: chrome stays warm; steel type accent is reserved.

#[test]
fn dark_no_canonical_steel_blue_in_chrome() {
  for key in ["background", "foreground", "cursor"] {
    let val = moshi_color(DARK, key);
    assert_ne!(val, "#90aec0", "dark {key} must not be canonical steel-blue");
  }
}

#[test]
fn light_no_canonical_steel_blue_in_chrome() {
  for key in ["background", "foreground", "cursor"] {
    let val = moshi_color(LIGHT, key);
    assert_ne!(val, "#285464", "light {key} must not be canonical steel-blue");
  }
}
