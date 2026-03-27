mod common;

use common::{extract_hex_colors, is_valid_hex, zed_editor_color, zed_syntax_color};
use serde_json::Value as JsonValue;

const THEME: &str = include_str!("../zed/themes/warm-burnout.json");

fn parse_theme() -> JsonValue {
  serde_json::from_str(THEME).expect("invalid JSON")
}

fn get_theme_style<'a>(v: &'a JsonValue, name: &str) -> &'a JsonValue {
  v["themes"]
    .as_array()
    .unwrap()
    .iter()
    .find(|t| t["name"].as_str() == Some(name))
    .unwrap_or_else(|| panic!("no theme named '{name}'"))
}

// -- Valid JSON --

#[test]
fn is_valid_json() {
  parse_theme();
}

// -- Schema structure --

#[test]
fn has_schema_field() {
  let v = parse_theme();
  assert!(v.get("$schema").is_some(), "missing '$schema'");
}

#[test]
fn has_name_field() {
  let v = parse_theme();
  assert_eq!(v["name"].as_str(), Some("Warm Burnout"));
}

#[test]
fn has_author_field() {
  let v = parse_theme();
  assert_eq!(v["author"].as_str(), Some("Felipe Lima"));
}

#[test]
fn has_themes_array() {
  let v = parse_theme();
  assert!(v["themes"].as_array().is_some(), "missing 'themes' array");
}

// -- Both variants present --

#[test]
fn has_dark_variant() {
  let v = parse_theme();
  let themes = v["themes"].as_array().unwrap();
  let dark = themes.iter().find(|t| t["name"].as_str() == Some("Warm Burnout Dark"));
  assert!(dark.is_some(), "missing 'Warm Burnout Dark' variant");
  assert_eq!(dark.unwrap()["appearance"].as_str(), Some("dark"));
}

#[test]
fn has_light_variant() {
  let v = parse_theme();
  let themes = v["themes"].as_array().unwrap();
  let light = themes.iter().find(|t| t["name"].as_str() == Some("Warm Burnout Light"));
  assert!(light.is_some(), "missing 'Warm Burnout Light' variant");
  assert_eq!(light.unwrap()["appearance"].as_str(), Some("light"));
}

// -- Canonical backgrounds --

#[test]
fn dark_background_is_canonical() {
  assert_eq!(
    zed_editor_color(THEME, "Warm Burnout Dark", "editor.background"),
    "#1a1510"
  );
}

#[test]
fn light_background_is_canonical() {
  assert_eq!(
    zed_editor_color(THEME, "Warm Burnout Light", "editor.background"),
    "#f5ede0"
  );
}

// -- Canonical foregrounds --

#[test]
fn dark_foreground_is_canonical() {
  assert_eq!(
    zed_editor_color(THEME, "Warm Burnout Dark", "editor.foreground"),
    "#bfbdb6"
  );
}

#[test]
fn light_foreground_is_canonical() {
  assert_eq!(
    zed_editor_color(THEME, "Warm Burnout Light", "editor.foreground"),
    "#3a3630"
  );
}

// -- All hex colors valid --

#[test]
fn all_hex_colors_are_valid() {
  for (line, hex) in extract_hex_colors(THEME) {
    assert!(is_valid_hex(hex), "line {line}: invalid hex: {hex}");
  }
}

// -- No pure black/white backgrounds --

#[test]
fn no_pure_black_background() {
  let bg = zed_editor_color(THEME, "Warm Burnout Dark", "editor.background");
  assert_ne!(bg, "#000000", "dark background must not be pure black");
}

#[test]
fn no_pure_white_background() {
  let bg = zed_editor_color(THEME, "Warm Burnout Light", "editor.background");
  assert_ne!(bg, "#ffffff", "light background must not be pure white");
}

// -- Syntax section exists with required keys --

#[test]
fn dark_syntax_has_required_keys() {
  let v = parse_theme();
  let theme = get_theme_style(&v, "Warm Burnout Dark");
  let syntax = theme["style"]["syntax"].as_object().expect("missing syntax object");
  for key in [
    "keyword", "function", "type", "string", "comment", "variable", "constant", "number", "operator",
  ] {
    assert!(syntax.contains_key(key), "dark syntax missing key: {key}");
  }
}

#[test]
fn light_syntax_has_required_keys() {
  let v = parse_theme();
  let theme = get_theme_style(&v, "Warm Burnout Light");
  let syntax = theme["style"]["syntax"].as_object().expect("missing syntax object");
  for key in [
    "keyword", "function", "type", "string", "comment", "variable", "constant", "number", "operator",
  ] {
    assert!(syntax.contains_key(key), "light syntax missing key: {key}");
  }
}

// -- Syntax colors match canonical palette --

#[test]
fn dark_syntax_colors_match_palette() {
  let expected = [
    ("keyword", "#ff8f40"),
    ("function", "#ffb454"),
    ("type", "#90aec0"),
    ("string", "#b4bc78"),
    ("comment", "#b4a89c"),
    ("variable", "#bfbdb6"),
    ("constant", "#d4a8b8"),
    ("number", "#d4a8b8"),
    ("operator", "#f29668"),
  ];
  for (key, color) in expected {
    assert_eq!(
      zed_syntax_color(THEME, "Warm Burnout Dark", key),
      color,
      "dark syntax {key} mismatch"
    );
  }
}

#[test]
fn light_syntax_colors_match_palette() {
  let expected = [
    ("keyword", "#924800"),
    ("function", "#855700"),
    ("type", "#285464"),
    ("string", "#4d5c1a"),
    ("comment", "#544c40"),
    ("variable", "#3a3630"),
    ("constant", "#7e4060"),
    ("number", "#7e4060"),
    ("operator", "#8f4418"),
  ];
  for (key, color) in expected {
    assert_eq!(
      zed_syntax_color(THEME, "Warm Burnout Light", key),
      color,
      "light syntax {key} mismatch"
    );
  }
}

// -- Font styles preserved --

#[test]
fn dark_keyword_is_bold() {
  let v = parse_theme();
  let theme = get_theme_style(&v, "Warm Burnout Dark");
  assert_eq!(theme["style"]["syntax"]["keyword"]["font_weight"].as_u64(), Some(700));
}

#[test]
fn dark_type_is_italic() {
  let v = parse_theme();
  let theme = get_theme_style(&v, "Warm Burnout Dark");
  assert_eq!(theme["style"]["syntax"]["type"]["font_style"].as_str(), Some("italic"));
}

#[test]
fn dark_comment_is_italic() {
  let v = parse_theme();
  let theme = get_theme_style(&v, "Warm Burnout Dark");
  assert_eq!(
    theme["style"]["syntax"]["comment"]["font_style"].as_str(),
    Some("italic")
  );
}

#[test]
fn light_keyword_is_bold() {
  let v = parse_theme();
  let theme = get_theme_style(&v, "Warm Burnout Light");
  assert_eq!(theme["style"]["syntax"]["keyword"]["font_weight"].as_u64(), Some(700));
}

#[test]
fn light_type_is_italic() {
  let v = parse_theme();
  let theme = get_theme_style(&v, "Warm Burnout Light");
  assert_eq!(theme["style"]["syntax"]["type"]["font_style"].as_str(), Some("italic"));
}

#[test]
fn light_comment_is_italic() {
  let v = parse_theme();
  let theme = get_theme_style(&v, "Warm Burnout Light");
  assert_eq!(
    theme["style"]["syntax"]["comment"]["font_style"].as_str(),
    Some("italic")
  );
}

// -- Terminal ANSI colors present --

#[test]
fn dark_has_terminal_ansi_colors() {
  let v = parse_theme();
  let theme = get_theme_style(&v, "Warm Burnout Dark");
  let style = &theme["style"];
  for key in [
    "terminal.background",
    "terminal.foreground",
    "terminal.ansi.black",
    "terminal.ansi.red",
    "terminal.ansi.green",
    "terminal.ansi.yellow",
    "terminal.ansi.blue",
    "terminal.ansi.magenta",
    "terminal.ansi.cyan",
    "terminal.ansi.white",
    "terminal.ansi.bright_black",
    "terminal.ansi.bright_red",
    "terminal.ansi.bright_green",
    "terminal.ansi.bright_yellow",
    "terminal.ansi.bright_blue",
    "terminal.ansi.bright_magenta",
    "terminal.ansi.bright_cyan",
    "terminal.ansi.bright_white",
  ] {
    assert!(style[key].as_str().is_some(), "dark missing terminal key: {key}");
  }
}

#[test]
fn light_has_terminal_ansi_colors() {
  let v = parse_theme();
  let theme = get_theme_style(&v, "Warm Burnout Light");
  let style = &theme["style"];
  for key in [
    "terminal.background",
    "terminal.foreground",
    "terminal.ansi.black",
    "terminal.ansi.red",
    "terminal.ansi.green",
    "terminal.ansi.yellow",
    "terminal.ansi.blue",
    "terminal.ansi.magenta",
    "terminal.ansi.cyan",
    "terminal.ansi.white",
    "terminal.ansi.bright_black",
    "terminal.ansi.bright_red",
    "terminal.ansi.bright_green",
    "terminal.ansi.bright_yellow",
    "terminal.ansi.bright_blue",
    "terminal.ansi.bright_magenta",
    "terminal.ansi.bright_cyan",
    "terminal.ansi.bright_white",
  ] {
    assert!(style[key].as_str().is_some(), "light missing terminal key: {key}");
  }
}

// -- Both variants have same style keys (structural parity) --

#[test]
fn both_variants_have_same_style_keys() {
  let v = parse_theme();
  let dark = get_theme_style(&v, "Warm Burnout Dark");
  let light = get_theme_style(&v, "Warm Burnout Light");

  let dark_keys: Vec<&str> = dark["style"].as_object().unwrap().keys().map(String::as_str).collect();
  let light_keys: Vec<&str> = light["style"].as_object().unwrap().keys().map(String::as_str).collect();

  for key in &dark_keys {
    assert!(
      light_keys.contains(key),
      "dark has style key '{key}' but light does not"
    );
  }
  for key in &light_keys {
    assert!(dark_keys.contains(key), "light has style key '{key}' but dark does not");
  }
}

#[test]
fn both_variants_have_same_syntax_keys() {
  let v = parse_theme();
  let dark = get_theme_style(&v, "Warm Burnout Dark");
  let light = get_theme_style(&v, "Warm Burnout Light");

  let dark_keys: Vec<&str> = dark["style"]["syntax"]
    .as_object()
    .unwrap()
    .keys()
    .map(|k| k.as_str())
    .collect();
  let light_keys: Vec<&str> = light["style"]["syntax"]
    .as_object()
    .unwrap()
    .keys()
    .map(|k| k.as_str())
    .collect();

  for key in &dark_keys {
    assert!(
      light_keys.contains(key),
      "dark has syntax key '{key}' but light does not"
    );
  }
  for key in &light_keys {
    assert!(
      dark_keys.contains(key),
      "light has syntax key '{key}' but dark does not"
    );
  }
}

// -- extension.toml validation --

const EXTENSION_TOML: &str = include_str!("../zed/extension.toml");

#[test]
fn extension_toml_is_valid() {
  EXTENSION_TOML
    .parse::<toml::Table>()
    .expect("extension.toml is not valid TOML");
}

#[test]
fn extension_toml_has_required_fields() {
  let table: toml::Table = EXTENSION_TOML.parse().unwrap();
  for field in [
    "id",
    "name",
    "version",
    "schema_version",
    "authors",
    "description",
    "repository",
  ] {
    assert!(
      table.contains_key(field),
      "extension.toml missing required field: {field}"
    );
  }
}

#[test]
fn extension_toml_id_is_string() {
  let table: toml::Table = EXTENSION_TOML.parse().unwrap();
  assert!(table["id"].is_str(), "extension.toml 'id' should be a string");
}

#[test]
fn extension_toml_name_is_string() {
  let table: toml::Table = EXTENSION_TOML.parse().unwrap();
  assert!(table["name"].is_str(), "extension.toml 'name' should be a string");
}

#[test]
fn extension_toml_version_is_semver() {
  let table: toml::Table = EXTENSION_TOML.parse().unwrap();
  let version = table["version"].as_str().expect("version should be a string");
  let parts: Vec<&str> = version.split('.').collect();
  assert_eq!(parts.len(), 3, "version should have 3 parts (semver), got: {version}");
  for part in &parts {
    assert!(
      part.parse::<u32>().is_ok(),
      "version part '{part}' is not a number in: {version}"
    );
  }
}

#[test]
fn extension_toml_schema_version_is_integer() {
  let table: toml::Table = EXTENSION_TOML.parse().unwrap();
  assert!(
    table["schema_version"].is_integer(),
    "schema_version should be an integer"
  );
}

#[test]
fn extension_toml_authors_is_nonempty_array() {
  let table: toml::Table = EXTENSION_TOML.parse().unwrap();
  let authors = table["authors"].as_array().expect("authors should be an array");
  assert!(!authors.is_empty(), "authors should not be empty");
}

#[test]
fn extension_toml_repository_is_url() {
  let table: toml::Table = EXTENSION_TOML.parse().unwrap();
  let repo = table["repository"].as_str().expect("repository should be a string");
  assert!(
    repo.starts_with("https://"),
    "repository should be an https URL, got: {repo}"
  );
}
