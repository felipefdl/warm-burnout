mod common;

use common::is_valid_hex;
use toml::Table as TomlTable;

const DARK: &str = include_str!("../helix/warm_burnout_dark.toml");
const LIGHT: &str = include_str!("../helix/warm_burnout_light.toml");

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
  "property",
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

// -- Accent matches canonical ------------------------------------------

#[test]
fn dark_accent_matches_canonical() {
  let table = parse_toml(DARK);
  let palette = get_palette(&table);
  let accent = palette["accent"].as_str().unwrap().to_lowercase();
  assert_eq!(accent, "#b8522e", "dark accent must be canonical copper rust");
}

#[test]
fn light_accent_matches_canonical() {
  let table = parse_toml(LIGHT);
  let palette = get_palette(&table);
  let accent = palette["accent"].as_str().unwrap().to_lowercase();
  assert_eq!(accent, "#b8522e", "light accent must be canonical copper rust");
}
