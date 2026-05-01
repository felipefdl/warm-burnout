mod common;

use common::is_valid_hex;
use toml::Table as TomlTable;

const DARK: &str = include_str!("../helix/warm-burnout-dark.toml");
const LIGHT: &str = include_str!("../helix/warm-burnout-light.toml");

fn parse_toml(src: &str) -> TomlTable {
  src.parse::<TomlTable>().expect("invalid TOML")
}

fn get_palette(table: &TomlTable) -> &TomlTable {
  table
    .get("palette")
    .and_then(|p| p.as_table())
    .expect("missing [palette] section")
}

// -- Valid TOML ---------------------------------------------------------

#[test]
fn dark_is_valid_toml() {
  parse_toml(DARK);
}

#[test]
fn light_is_valid_toml() {
  parse_toml(LIGHT);
}

// -- Palette keys -------------------------------------------------------

const EXPECTED_PALETTE_KEYS: &[&str] = &[
  "bg",
  "bg-dim",
  "bg-float",
  "bg-highlight",
  "bg-selection",
  "fg",
  "fg-dim",
  "fg-gutter",
  "fg-gutter-active",
  "comment",
  "cursor",
  "accent",
  "keyword",
  "func",
  "string",
  "type",
  "operator",
  "number",
  "tag",
  "member",
  "regex",
  "decorator",
  "error",
  "warn",
  "info",
  "hint",
  "added",
  "modified",
  "deleted",
];

#[test]
fn dark_palette_has_all_keys() {
  let table = parse_toml(DARK);
  let palette = get_palette(&table);
  for key in EXPECTED_PALETTE_KEYS {
    assert!(palette.contains_key(*key), "dark [palette] missing key: {key}");
  }
}

#[test]
fn light_palette_has_all_keys() {
  let table = parse_toml(LIGHT);
  let palette = get_palette(&table);
  for key in EXPECTED_PALETTE_KEYS {
    assert!(palette.contains_key(*key), "light [palette] missing key: {key}");
  }
}

#[test]
fn dark_palette_colors_are_valid_hex() {
  let table = parse_toml(DARK);
  let palette = get_palette(&table);
  for (key, val) in palette {
    let hex = val.as_str().unwrap_or_else(|| panic!("{key} is not a string"));
    assert!(is_valid_hex(hex), "dark palette {key}: '{hex}' is not valid hex");
  }
}

#[test]
fn light_palette_colors_are_valid_hex() {
  let table = parse_toml(LIGHT);
  let palette = get_palette(&table);
  for (key, val) in palette {
    let hex = val.as_str().unwrap_or_else(|| panic!("{key} is not a string"));
    assert!(is_valid_hex(hex), "light palette {key}: '{hex}' is not valid hex");
  }
}

#[test]
fn dark_and_light_palettes_have_same_keys() {
  let dark_table = parse_toml(DARK);
  let dark = get_palette(&dark_table);
  let light_table = parse_toml(LIGHT);
  let light = get_palette(&light_table);
  let mut dark_keys: Vec<&String> = dark.keys().collect();
  let mut light_keys: Vec<&String> = light.keys().collect();
  dark_keys.sort();
  light_keys.sort();
  assert_eq!(
    dark_keys, light_keys,
    "dark and light palettes should have identical keys"
  );
}

// -- Required scopes present -------------------------------------------

const REQUIRED_SYNTAX_SCOPES: &[&str] = &[
  "keyword",
  "keyword.operator",
  "keyword.function",
  "keyword.storage",
  "function",
  "function.builtin",
  "function.method",
  "function.macro",
  "type",
  "type.builtin",
  "constructor",
  "string",
  "string.regexp",
  "constant",
  "constant.numeric",
  "constant.character.escape",
  "comment",
  "variable",
  "variable.builtin",
  "variable.parameter",
  "variable.other.member",
  "operator",
  "punctuation",
  "punctuation.delimiter",
  "punctuation.bracket",
  "tag",
  "attribute",
  "namespace",
  "label",
];

const REQUIRED_UI_SCOPES: &[&str] = &[
  "ui.background",
  "ui.cursor",
  "ui.cursor.match",
  "ui.linenr",
  "ui.linenr.selected",
  "ui.statusline",
  "ui.statusline.inactive",
  "ui.statusline.normal",
  "ui.statusline.insert",
  "ui.statusline.select",
  "ui.popup",
  "ui.window",
  "ui.text",
  "ui.text.focus",
  "ui.selection",
  "ui.selection.primary",
  "ui.menu",
  "ui.menu.selected",
  "ui.virtual.ruler",
  "ui.virtual.whitespace",
  "ui.virtual.indent-guide",
  "ui.virtual.inlay-hint",
  "ui.cursorline.primary",
  "ui.bufferline",
  "ui.bufferline.active",
  "ui.gutter",
  "ui.highlight",
];

const REQUIRED_DIAGNOSTIC_SCOPES: &[&str] = &[
  "diagnostic",
  "diagnostic.hint",
  "diagnostic.info",
  "diagnostic.warning",
  "diagnostic.error",
  "warning",
  "error",
  "info",
  "hint",
];

fn assert_scope_present(table: &TomlTable, scope: &str) {
  assert!(
    table.contains_key(scope),
    "missing scope: \"{scope}\" -- add it to the theme file"
  );
}

#[test]
fn dark_has_required_syntax_scopes() {
  let table = parse_toml(DARK);
  for scope in REQUIRED_SYNTAX_SCOPES {
    assert_scope_present(&table, scope);
  }
}

#[test]
fn light_has_required_syntax_scopes() {
  let table = parse_toml(LIGHT);
  for scope in REQUIRED_SYNTAX_SCOPES {
    assert_scope_present(&table, scope);
  }
}

#[test]
fn dark_has_required_ui_scopes() {
  let table = parse_toml(DARK);
  for scope in REQUIRED_UI_SCOPES {
    assert_scope_present(&table, scope);
  }
}

#[test]
fn light_has_required_ui_scopes() {
  let table = parse_toml(LIGHT);
  for scope in REQUIRED_UI_SCOPES {
    assert_scope_present(&table, scope);
  }
}

#[test]
fn dark_has_required_diagnostic_scopes() {
  let table = parse_toml(DARK);
  for scope in REQUIRED_DIAGNOSTIC_SCOPES {
    assert_scope_present(&table, scope);
  }
}

#[test]
fn light_has_required_diagnostic_scopes() {
  let table = parse_toml(LIGHT);
  for scope in REQUIRED_DIAGNOSTIC_SCOPES {
    assert_scope_present(&table, scope);
  }
}

// -- Dark and light have identical scope keys --------------------------

#[test]
fn dark_and_light_have_same_scopes() {
  let dark_table = parse_toml(DARK);
  let light_table = parse_toml(LIGHT);
  let mut dark_keys: Vec<&String> = dark_table.keys().filter(|k| *k != "palette").collect();
  let mut light_keys: Vec<&String> = light_table.keys().filter(|k| *k != "palette").collect();
  dark_keys.sort();
  light_keys.sort();
  assert_eq!(
    dark_keys, light_keys,
    "dark and light themes should define identical scope keys"
  );
}

// -- Canonical palette values ------------------------------------------

fn palette_color(table: &TomlTable, key: &str) -> String {
  get_palette(table)[key]
    .as_str()
    .unwrap_or_else(|| panic!("palette.{key} is not a string"))
    .to_lowercase()
}

#[test]
fn dark_background_is_canonical() {
  let table = parse_toml(DARK);
  assert_eq!(palette_color(&table, "bg"), "#1a1510");
}

#[test]
fn light_background_is_canonical() {
  let table = parse_toml(LIGHT);
  assert_eq!(palette_color(&table, "bg"), "#f5ede0");
}

#[test]
fn dark_foreground_is_canonical() {
  let table = parse_toml(DARK);
  assert_eq!(palette_color(&table, "fg"), "#bfbdb6");
}

#[test]
fn light_foreground_is_canonical() {
  let table = parse_toml(LIGHT);
  assert_eq!(palette_color(&table, "fg"), "#3a3630");
}

#[test]
fn dark_cursor_is_canonical() {
  let table = parse_toml(DARK);
  assert_eq!(palette_color(&table, "cursor"), "#f5c56e");
}

#[test]
fn light_cursor_is_canonical() {
  let table = parse_toml(LIGHT);
  assert_eq!(palette_color(&table, "cursor"), "#8a6600");
}

#[test]
fn dark_accent_matches_canonical() {
  let table = parse_toml(DARK);
  assert_eq!(
    palette_color(&table, "accent"),
    "#b8522e",
    "dark accent must be canonical copper rust"
  );
}

#[test]
fn light_accent_matches_canonical() {
  let table = parse_toml(LIGHT);
  assert_eq!(
    palette_color(&table, "accent"),
    "#b8522e",
    "light accent must be canonical copper rust"
  );
}

#[test]
fn no_pure_black_background() {
  let table = parse_toml(DARK);
  assert_ne!(palette_color(&table, "bg"), "#000000", "dark bg must not be pure black");
}

#[test]
fn no_pure_white_background() {
  let table = parse_toml(LIGHT);
  assert_ne!(
    palette_color(&table, "bg"),
    "#ffffff",
    "light bg must not be pure white"
  );
}

#[test]
fn dark_syntax_colors_match_palette() {
  let table = parse_toml(DARK);
  let expected = [
    ("keyword", "#ff8f40"),
    ("func", "#ffb454"),
    ("type", "#90aec0"),
    ("string", "#b4bc78"),
    ("comment", "#b4a89c"),
    ("number", "#d4a8b8"),
    ("operator", "#f29668"),
    ("tag", "#dc9e92"),
    ("regex", "#96b898"),
    ("decorator", "#e6c08a"),
    ("member", "#ec9878"),
  ];
  for (key, color) in expected {
    assert_eq!(palette_color(&table, key), color, "dark palette {key} mismatch");
  }
}

#[test]
fn light_syntax_colors_match_palette() {
  let table = parse_toml(LIGHT);
  let expected = [
    ("keyword", "#924800"),
    ("func", "#855700"),
    ("type", "#285464"),
    ("string", "#4d5c1a"),
    ("comment", "#544c40"),
    ("number", "#7e4060"),
    ("operator", "#8f4418"),
    ("tag", "#8e4632"),
    ("regex", "#286a48"),
    ("decorator", "#7a5a1c"),
    ("member", "#883850"),
  ];
  for (key, color) in expected {
    assert_eq!(palette_color(&table, key), color, "light palette {key} mismatch");
  }
}

#[test]
fn dark_diagnostic_colors() {
  let table = parse_toml(DARK);
  assert_eq!(palette_color(&table, "error"), "#f49090");
  assert_eq!(palette_color(&table, "warn"), "#b8522e");
  assert_eq!(palette_color(&table, "info"), "#90aec0");
  assert_eq!(palette_color(&table, "hint"), "#b4a89c");
}

#[test]
fn light_diagnostic_colors() {
  let table = parse_toml(LIGHT);
  assert_eq!(palette_color(&table, "error"), "#b03434");
  assert_eq!(palette_color(&table, "warn"), "#b8522e");
  assert_eq!(palette_color(&table, "info"), "#285464");
  assert_eq!(palette_color(&table, "hint"), "#544c40");
}

#[test]
fn dark_git_colors() {
  let table = parse_toml(DARK);
  assert_eq!(palette_color(&table, "added"), "#70bf56");
  assert_eq!(palette_color(&table, "modified"), "#73b8ff");
  assert_eq!(palette_color(&table, "deleted"), "#f26d78");
}

#[test]
fn light_git_colors() {
  let table = parse_toml(LIGHT);
  assert_eq!(palette_color(&table, "added"), "#226414");
  assert_eq!(palette_color(&table, "modified"), "#2868a0");
  assert_eq!(palette_color(&table, "deleted"), "#c43040");
}

// -- Three-tier font style system --------------------------------------

fn scope_modifiers(table: &TomlTable, scope: &str) -> Vec<String> {
  let entry = table.get(scope).unwrap_or_else(|| panic!("missing scope: {scope}"));
  entry
    .get("modifiers")
    .and_then(|m| m.as_array())
    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
    .unwrap_or_default()
}

#[test]
fn dark_keywords_are_bold() {
  let table = parse_toml(DARK);
  for scope in [
    "keyword",
    "keyword.control",
    "keyword.function",
    "keyword.storage",
    "tag",
  ] {
    assert!(
      scope_modifiers(&table, scope).contains(&"bold".to_string()),
      "{scope} must be bold"
    );
  }
}

#[test]
fn light_keywords_are_bold() {
  let table = parse_toml(LIGHT);
  for scope in [
    "keyword",
    "keyword.control",
    "keyword.function",
    "keyword.storage",
    "tag",
  ] {
    assert!(
      scope_modifiers(&table, scope).contains(&"bold".to_string()),
      "{scope} must be bold"
    );
  }
}

#[test]
fn dark_types_and_comments_are_italic() {
  let table = parse_toml(DARK);
  for scope in ["type", "type.builtin", "comment", "comment.line", "attribute"] {
    assert!(
      scope_modifiers(&table, scope).contains(&"italic".to_string()),
      "{scope} must be italic"
    );
  }
}

#[test]
fn light_types_and_comments_are_italic() {
  let table = parse_toml(LIGHT);
  for scope in ["type", "type.builtin", "comment", "comment.line", "attribute"] {
    assert!(
      scope_modifiers(&table, scope).contains(&"italic".to_string()),
      "{scope} must be italic"
    );
  }
}
