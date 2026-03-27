mod common;

use common::iterm2_color;

const DARK: &str = include_str!("../iterm2/Warm Burnout Dark.itermcolors");
const LIGHT: &str = include_str!("../iterm2/Warm Burnout Light.itermcolors");

fn parse_plist(src: &str) -> plist::Value {
  let cursor = std::io::Cursor::new(src.as_bytes());
  plist::from_reader(cursor).expect("invalid plist")
}

// -- Valid plist structure --

#[test]
fn dark_is_valid_plist() {
  parse_plist(DARK);
}

#[test]
fn light_is_valid_plist() {
  parse_plist(LIGHT);
}

// -- Canonical backgrounds --

#[test]
fn dark_background_is_canonical() {
  assert_eq!(iterm2_color(DARK, "Background Color"), "#1a1510");
}

#[test]
fn light_background_is_canonical() {
  assert_eq!(iterm2_color(LIGHT, "Background Color"), "#f5ede0");
}

// -- Canonical foregrounds --

#[test]
fn dark_foreground_is_canonical() {
  assert_eq!(iterm2_color(DARK, "Foreground Color"), "#bfbdb6");
}

#[test]
fn light_foreground_is_canonical() {
  assert_eq!(iterm2_color(LIGHT, "Foreground Color"), "#3a3630");
}

// -- Canonical cursors --

#[test]
fn dark_cursor_is_canonical() {
  assert_eq!(iterm2_color(DARK, "Cursor Color"), "#f5c56e");
}

#[test]
fn light_cursor_is_canonical() {
  assert_eq!(iterm2_color(LIGHT, "Cursor Color"), "#8a6600");
}

// -- No pure black/white backgrounds --

#[test]
fn no_pure_black_background() {
  let bg = iterm2_color(DARK, "Background Color");
  assert_ne!(bg, "#000000", "dark background must not be pure black");
}

#[test]
fn no_pure_white_background() {
  let bg = iterm2_color(LIGHT, "Background Color");
  assert_ne!(bg, "#ffffff", "light background must not be pure white");
}

// -- All required keys present --

const REQUIRED_KEYS: &[&str] = &[
  "Ansi 0 Color",
  "Ansi 1 Color",
  "Ansi 2 Color",
  "Ansi 3 Color",
  "Ansi 4 Color",
  "Ansi 5 Color",
  "Ansi 6 Color",
  "Ansi 7 Color",
  "Ansi 8 Color",
  "Ansi 9 Color",
  "Ansi 10 Color",
  "Ansi 11 Color",
  "Ansi 12 Color",
  "Ansi 13 Color",
  "Ansi 14 Color",
  "Ansi 15 Color",
  "Background Color",
  "Foreground Color",
  "Bold Color",
  "Cursor Color",
  "Cursor Text Color",
  "Selection Color",
  "Selected Text Color",
];

#[test]
fn dark_has_all_required_keys() {
  let v = parse_plist(DARK);
  let dict = v.as_dictionary().unwrap();
  for key in REQUIRED_KEYS {
    assert!(
      dict.get(*key).and_then(|v| v.as_dictionary()).is_some(),
      "dark missing required key: {key}"
    );
  }
}

#[test]
fn light_has_all_required_keys() {
  let v = parse_plist(LIGHT);
  let dict = v.as_dictionary().unwrap();
  for key in REQUIRED_KEYS {
    assert!(
      dict.get(*key).and_then(|v| v.as_dictionary()).is_some(),
      "light missing required key: {key}"
    );
  }
}

// -- ANSI colors match Ghostty --

#[test]
fn dark_ansi_colors_match_ghostty() {
  let expected = [
    ("Ansi 0 Color", "#23211b"),
    ("Ansi 1 Color", "#f06b73"),
    ("Ansi 2 Color", "#70bf56"),
    ("Ansi 3 Color", "#fdb04c"),
    ("Ansi 4 Color", "#4fbfff"),
    ("Ansi 5 Color", "#d0a1ff"),
    ("Ansi 6 Color", "#93e2c8"),
    ("Ansi 7 Color", "#c7c7c7"),
    ("Ansi 8 Color", "#686868"),
    ("Ansi 9 Color", "#f07178"),
    ("Ansi 10 Color", "#aad94c"),
    ("Ansi 11 Color", "#ffb454"),
    ("Ansi 12 Color", "#59c2ff"),
    ("Ansi 13 Color", "#d2a6ff"),
    ("Ansi 14 Color", "#95e6cb"),
    ("Ansi 15 Color", "#ffffff"),
  ];
  for (key, color) in expected {
    assert_eq!(iterm2_color(DARK, key), color, "dark {key} mismatch");
  }
}

#[test]
fn light_ansi_colors_match_ghostty() {
  let expected = [
    ("Ansi 0 Color", "#3a3630"),
    ("Ansi 1 Color", "#b82820"),
    ("Ansi 2 Color", "#2d6a14"),
    ("Ansi 3 Color", "#8a6000"),
    ("Ansi 4 Color", "#2060a0"),
    ("Ansi 5 Color", "#8a3090"),
    ("Ansi 6 Color", "#146858"),
    ("Ansi 7 Color", "#c0b8aa"),
    ("Ansi 8 Color", "#686868"),
    ("Ansi 9 Color", "#c83028"),
    ("Ansi 10 Color", "#3a7a20"),
    ("Ansi 11 Color", "#9a7008"),
    ("Ansi 12 Color", "#2870b0"),
    ("Ansi 13 Color", "#9a38a0"),
    ("Ansi 14 Color", "#208870"),
    ("Ansi 15 Color", "#faf6f0"),
  ];
  for (key, color) in expected {
    assert_eq!(iterm2_color(LIGHT, key), color, "light {key} mismatch");
  }
}

// -- Both variants have same keys (structural parity) --

#[test]
fn both_variants_have_same_keys() {
  let dark = parse_plist(DARK);
  let light = parse_plist(LIGHT);
  let dark_dict = dark.as_dictionary().unwrap();
  let light_dict = light.as_dictionary().unwrap();

  let dark_keys: Vec<&str> = dark_dict.keys().map(String::as_str).collect();
  let light_keys: Vec<&str> = light_dict.keys().map(String::as_str).collect();

  for key in &dark_keys {
    assert!(light_keys.contains(key), "dark has key '{key}' but light does not");
  }
  for key in &light_keys {
    assert!(dark_keys.contains(key), "light has key '{key}' but dark does not");
  }
}

// -- All color dicts have sRGB color space --

#[test]
fn dark_all_colors_use_srgb() {
  let v = parse_plist(DARK);
  let dict = v.as_dictionary().unwrap();
  for (key, value) in dict.iter() {
    if let Some(color_dict) = value.as_dictionary() {
      let cs = color_dict
        .get("Color Space")
        .and_then(|v| v.as_string())
        .unwrap_or_else(|| panic!("dark {key}: missing Color Space"));
      assert_eq!(cs, "sRGB", "dark {key}: expected sRGB, got {cs}");
    }
  }
}

#[test]
fn light_all_colors_use_srgb() {
  let v = parse_plist(LIGHT);
  let dict = v.as_dictionary().unwrap();
  for (key, value) in dict.iter() {
    if let Some(color_dict) = value.as_dictionary() {
      let cs = color_dict
        .get("Color Space")
        .and_then(|v| v.as_string())
        .unwrap_or_else(|| panic!("light {key}: missing Color Space"));
      assert_eq!(cs, "sRGB", "light {key}: expected sRGB, got {cs}");
    }
  }
}
