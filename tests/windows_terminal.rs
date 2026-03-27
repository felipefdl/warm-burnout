mod common;

use common::windows_terminal_color;

const DARK: &str = include_str!("../windows-terminal/warm-burnout-dark.json");
const LIGHT: &str = include_str!("../windows-terminal/warm-burnout-light.json");

fn parse_json(src: &str) -> serde_json::Value {
  serde_json::from_str(src).expect("invalid JSON")
}

// -- Valid JSON structure --

#[test]
fn dark_is_valid_json() {
  parse_json(DARK);
}

#[test]
fn light_is_valid_json() {
  parse_json(LIGHT);
}

// -- Canonical backgrounds --

#[test]
fn dark_background_is_canonical() {
  assert_eq!(windows_terminal_color(DARK, "background"), "#1a1510");
}

#[test]
fn light_background_is_canonical() {
  assert_eq!(windows_terminal_color(LIGHT, "background"), "#f5ede0");
}

// -- Canonical foregrounds --

#[test]
fn dark_foreground_is_canonical() {
  assert_eq!(windows_terminal_color(DARK, "foreground"), "#bfbdb6");
}

#[test]
fn light_foreground_is_canonical() {
  assert_eq!(windows_terminal_color(LIGHT, "foreground"), "#3a3630");
}

// -- Canonical cursors --

#[test]
fn dark_cursor_is_canonical() {
  assert_eq!(windows_terminal_color(DARK, "cursorColor"), "#f5c56e");
}

#[test]
fn light_cursor_is_canonical() {
  assert_eq!(windows_terminal_color(LIGHT, "cursorColor"), "#8a6600");
}

// -- No pure black/white backgrounds --

#[test]
fn no_pure_black_background() {
  let bg = windows_terminal_color(DARK, "background");
  assert_ne!(bg, "#000000", "dark background must not be pure black");
}

#[test]
fn no_pure_white_background() {
  let bg = windows_terminal_color(LIGHT, "background");
  assert_ne!(bg, "#ffffff", "light background must not be pure white");
}

// -- All required keys present --

const REQUIRED_KEYS: &[&str] = &[
  "name",
  "background",
  "foreground",
  "cursorColor",
  "selectionBackground",
  "black",
  "red",
  "green",
  "yellow",
  "blue",
  "purple",
  "cyan",
  "white",
  "brightBlack",
  "brightRed",
  "brightGreen",
  "brightYellow",
  "brightBlue",
  "brightPurple",
  "brightCyan",
  "brightWhite",
];

#[test]
fn dark_has_all_required_keys() {
  let v = parse_json(DARK);
  let obj = v.as_object().unwrap();
  for key in REQUIRED_KEYS {
    assert!(obj.contains_key(*key), "dark missing required key: {key}");
  }
}

#[test]
fn light_has_all_required_keys() {
  let v = parse_json(LIGHT);
  let obj = v.as_object().unwrap();
  for key in REQUIRED_KEYS {
    assert!(obj.contains_key(*key), "light missing required key: {key}");
  }
}

// -- Name field matches expected values --

#[test]
fn dark_name_is_correct() {
  let v = parse_json(DARK);
  assert_eq!(v["name"].as_str().unwrap(), "Warm Burnout Dark");
}

#[test]
fn light_name_is_correct() {
  let v = parse_json(LIGHT);
  assert_eq!(v["name"].as_str().unwrap(), "Warm Burnout Light");
}

// -- ANSI colors match Ghostty --

#[test]
fn dark_ansi_colors_match_ghostty() {
  let expected = [
    ("black", "#23211b"),
    ("red", "#f06b73"),
    ("green", "#70bf56"),
    ("yellow", "#fdb04c"),
    ("blue", "#4fbfff"),
    ("purple", "#d0a1ff"),
    ("cyan", "#93e2c8"),
    ("white", "#c7c7c7"),
    ("brightBlack", "#686868"),
    ("brightRed", "#f07178"),
    ("brightGreen", "#aad94c"),
    ("brightYellow", "#ffb454"),
    ("brightBlue", "#59c2ff"),
    ("brightPurple", "#d2a6ff"),
    ("brightCyan", "#95e6cb"),
    ("brightWhite", "#ffffff"),
  ];
  for (key, color) in expected {
    assert_eq!(windows_terminal_color(DARK, key), color, "dark {key} mismatch");
  }
}

#[test]
fn light_ansi_colors_match_ghostty() {
  let expected = [
    ("black", "#3a3630"),
    ("red", "#b82820"),
    ("green", "#2d6a14"),
    ("yellow", "#8a6000"),
    ("blue", "#2060a0"),
    ("purple", "#8a3090"),
    ("cyan", "#146858"),
    ("white", "#c0b8aa"),
    ("brightBlack", "#686868"),
    ("brightRed", "#c83028"),
    ("brightGreen", "#3a7a20"),
    ("brightYellow", "#9a7008"),
    ("brightBlue", "#2870b0"),
    ("brightPurple", "#9a38a0"),
    ("brightCyan", "#208870"),
    ("brightWhite", "#faf6f0"),
  ];
  for (key, color) in expected {
    assert_eq!(windows_terminal_color(LIGHT, key), color, "light {key} mismatch");
  }
}

// -- Both variants have same keys (structural parity) --

#[test]
fn both_variants_have_same_keys() {
  let dark = parse_json(DARK);
  let light = parse_json(LIGHT);
  let dark_obj = dark.as_object().unwrap();
  let light_obj = light.as_object().unwrap();

  let dark_keys: Vec<&String> = dark_obj.keys().collect();
  let light_keys: Vec<&String> = light_obj.keys().collect();

  for key in &dark_keys {
    assert!(
      light_obj.contains_key(key.as_str()),
      "dark has key '{key}' but light does not"
    );
  }
  for key in &light_keys {
    assert!(
      dark_obj.contains_key(key.as_str()),
      "light has key '{key}' but dark does not"
    );
  }
}
