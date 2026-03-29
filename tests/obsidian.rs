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
  assert!(
    THEME.contains(".theme-light"),
    "theme.css must have a .theme-light block"
  );
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
    assert_eq!(
      actual, *expected,
      "dark --wb-{key}: expected={expected} actual={actual}"
    );
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
    assert_eq!(
      actual, *expected,
      "light --wb-{key}: expected={expected} actual={actual}"
    );
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
