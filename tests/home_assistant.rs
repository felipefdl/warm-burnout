mod common;

use common::{extract_hex_colors, home_assistant_color, is_valid_hex};
use serde_yml::Value;

const THEME: &str = include_str!("../home-assistant/warm-burnout.yaml");

fn parse_theme() -> Value {
  serde_yml::from_str(THEME).expect("invalid YAML")
}

// -- Valid YAML --

#[test]
fn is_valid_yaml() {
  parse_theme();
}

// -- Theme name --

#[test]
fn theme_name_is_warm_burnout() {
  let v = parse_theme();
  assert!(
    v.get("Warm Burnout").is_some(),
    "theme must have top-level key 'Warm Burnout'"
  );
}

// -- Both modes present --

#[test]
fn has_dark_mode() {
  let v = parse_theme();
  assert!(
    v["Warm Burnout"]["modes"]["dark"].is_mapping(),
    "theme must have modes > dark"
  );
}

#[test]
fn has_light_mode() {
  let v = parse_theme();
  assert!(
    v["Warm Burnout"]["modes"]["light"].is_mapping(),
    "theme must have modes > light"
  );
}

// -- All hex colors are valid --

#[test]
fn all_hex_colors_are_valid() {
  for (line, hex) in extract_hex_colors(THEME) {
    assert!(is_valid_hex(hex), "line {line}: invalid hex: {hex}");
  }
}

// -- Required keys exist in both modes --

const REQUIRED_KEYS: &[&str] = &[
  "primary-color",
  "accent-color",
  "primary-text-color",
  "secondary-text-color",
  "disabled-text-color",
  "primary-background-color",
  "secondary-background-color",
  "card-background-color",
  "error-color",
  "warning-color",
  "success-color",
  "info-color",
  "state-icon-color",
  "state-icon-active-color",
  "divider-color",
  "sidebar-text-color",
  "sidebar-background-color",
  "sidebar-selected-text-color",
  "sidebar-selected-icon-color",
  "sidebar-icon-color",
  "app-header-text-color",
  "app-header-background-color",
  "switch-checked-color",
  "switch-unchecked-button-color",
  "slider-color",
  "mdc-theme-primary",
  "mdc-theme-surface",
  "codemirror-keyword",
  "codemirror-string",
  "codemirror-comment",
  "codemirror-number",
  "codemirror-tag",
  "codemirror-property",
  "codemirror-type",
  "graph-color-1",
  "graph-color-14",
  "energy-grid-consumption-color",
  "energy-solar-color",
  "energy-battery-in-color",
];

#[test]
fn dark_mode_has_required_keys() {
  let v = parse_theme();
  let dark = v["Warm Burnout"]["modes"]["dark"]
    .as_mapping()
    .expect("dark mode should be a mapping");
  for key in REQUIRED_KEYS {
    assert!(
      dark.contains_key(&Value::String((*key).to_string())),
      "dark mode missing required key: {key}"
    );
  }
}

#[test]
fn light_mode_has_required_keys() {
  let v = parse_theme();
  let light = v["Warm Burnout"]["modes"]["light"]
    .as_mapping()
    .expect("light mode should be a mapping");
  for key in REQUIRED_KEYS {
    assert!(
      light.contains_key(&Value::String((*key).to_string())),
      "light mode missing required key: {key}"
    );
  }
}

// -- Both modes have the same set of keys --

#[test]
fn dark_and_light_have_same_keys() {
  let v = parse_theme();
  let dark = v["Warm Burnout"]["modes"]["dark"]
    .as_mapping()
    .expect("dark mode should be a mapping");
  let light = v["Warm Burnout"]["modes"]["light"]
    .as_mapping()
    .expect("light mode should be a mapping");

  let dark_keys: Vec<String> = dark.keys().filter_map(|k| k.as_str().map(String::from)).collect();
  let light_keys: Vec<String> = light.keys().filter_map(|k| k.as_str().map(String::from)).collect();

  for key in &dark_keys {
    assert!(light_keys.contains(key), "key '{key}' exists in dark but not light");
  }
  for key in &light_keys {
    assert!(dark_keys.contains(key), "key '{key}' exists in light but not dark");
  }
}

// -- Canonical palette values --

#[test]
fn dark_primary_color_is_copper_rust() {
  assert_eq!(
    home_assistant_color(THEME, "Warm Burnout", "dark", "primary-color"),
    "#b8522e"
  );
}

#[test]
fn light_primary_color_is_copper_rust() {
  assert_eq!(
    home_assistant_color(THEME, "Warm Burnout", "light", "primary-color"),
    "#b8522e"
  );
}

#[test]
fn dark_accent_is_copper_rust() {
  assert_eq!(
    home_assistant_color(THEME, "Warm Burnout", "dark", "accent-color"),
    "#b8522e"
  );
}

#[test]
fn light_accent_is_copper_rust() {
  assert_eq!(
    home_assistant_color(THEME, "Warm Burnout", "light", "accent-color"),
    "#b8522e"
  );
}

#[test]
fn dark_primary_background() {
  assert_eq!(
    home_assistant_color(THEME, "Warm Burnout", "dark", "primary-background-color"),
    "#1a1510"
  );
}

#[test]
fn light_primary_background() {
  assert_eq!(
    home_assistant_color(THEME, "Warm Burnout", "light", "primary-background-color"),
    "#f5ede0"
  );
}

#[test]
fn dark_card_background() {
  assert_eq!(
    home_assistant_color(THEME, "Warm Burnout", "dark", "card-background-color"),
    "#1f1d17"
  );
}

#[test]
fn light_card_background() {
  assert_eq!(
    home_assistant_color(THEME, "Warm Burnout", "light", "card-background-color"),
    "#faf6f0"
  );
}

#[test]
fn dark_primary_text() {
  assert_eq!(
    home_assistant_color(THEME, "Warm Burnout", "dark", "primary-text-color"),
    "#bfbdb6"
  );
}

#[test]
fn light_primary_text() {
  assert_eq!(
    home_assistant_color(THEME, "Warm Burnout", "light", "primary-text-color"),
    "#3a3630"
  );
}

#[test]
fn dark_error_color() {
  assert_eq!(
    home_assistant_color(THEME, "Warm Burnout", "dark", "error-color"),
    "#f49090"
  );
}

#[test]
fn light_error_color() {
  assert_eq!(
    home_assistant_color(THEME, "Warm Burnout", "light", "error-color"),
    "#b03434"
  );
}

#[test]
fn dark_success_color() {
  assert_eq!(
    home_assistant_color(THEME, "Warm Burnout", "dark", "success-color"),
    "#b4bc78"
  );
}

#[test]
fn light_success_color() {
  assert_eq!(
    home_assistant_color(THEME, "Warm Burnout", "light", "success-color"),
    "#4d5c1a"
  );
}

#[test]
fn dark_codemirror_keyword() {
  assert_eq!(
    home_assistant_color(THEME, "Warm Burnout", "dark", "codemirror-keyword"),
    "#ff8f40"
  );
}

#[test]
fn light_codemirror_keyword() {
  assert_eq!(
    home_assistant_color(THEME, "Warm Burnout", "light", "codemirror-keyword"),
    "#924800"
  );
}

#[test]
fn dark_codemirror_string() {
  assert_eq!(
    home_assistant_color(THEME, "Warm Burnout", "dark", "codemirror-string"),
    "#b4bc78"
  );
}

#[test]
fn light_codemirror_string() {
  assert_eq!(
    home_assistant_color(THEME, "Warm Burnout", "light", "codemirror-string"),
    "#4d5c1a"
  );
}
