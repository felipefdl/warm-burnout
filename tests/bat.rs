mod common;

use common::{
  bat_tmtheme_global_setting, bat_tmtheme_name, bat_tmtheme_root, bat_tmtheme_scope_font_style,
  bat_tmtheme_scope_foreground, extract_hex_colors, is_valid_hex,
};

const DARK: &str = include_str!("../bat/Warm Burnout Dark.tmTheme");
const LIGHT: &str = include_str!("../bat/Warm Burnout Light.tmTheme");

#[test]
fn dark_is_valid_tmtheme_plist() {
  bat_tmtheme_root(DARK);
}

#[test]
fn light_is_valid_tmtheme_plist() {
  bat_tmtheme_root(LIGHT);
}

#[test]
fn dark_name_is_warm_burnout_dark() {
  assert_eq!(bat_tmtheme_name(DARK), "Warm Burnout Dark");
}

#[test]
fn light_name_is_warm_burnout_light() {
  assert_eq!(bat_tmtheme_name(LIGHT), "Warm Burnout Light");
}

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

#[test]
fn dark_global_colors_match_palette() {
  assert_eq!(bat_tmtheme_global_setting(DARK, "background"), "#1a1510");
  assert_eq!(bat_tmtheme_global_setting(DARK, "foreground"), "#bfbdb6");
  assert_eq!(bat_tmtheme_global_setting(DARK, "caret"), "#f5c56e");
  assert_eq!(bat_tmtheme_global_setting(DARK, "selection"), "#33393a");
  assert_eq!(bat_tmtheme_global_setting(DARK, "lineHighlight"), "#222018");
}

#[test]
fn light_global_colors_match_palette() {
  assert_eq!(bat_tmtheme_global_setting(LIGHT, "background"), "#f5ede0");
  assert_eq!(bat_tmtheme_global_setting(LIGHT, "foreground"), "#3a3630");
  assert_eq!(bat_tmtheme_global_setting(LIGHT, "caret"), "#8a6600");
  assert_eq!(bat_tmtheme_global_setting(LIGHT, "selection"), "#e5e8e2");
  assert_eq!(bat_tmtheme_global_setting(LIGHT, "lineHighlight"), "#e2dace");
}

#[test]
fn dark_syntax_colors_match_palette() {
  for (scope, expected) in [
    ("keyword", "#ff8f40"),
    ("keyword.operator", "#f29668"),
    ("entity.name.function", "#ffb454"),
    ("support.function.builtin", "#ec9878"),
    ("entity.name.type", "#90aec0"),
    ("string", "#b4bc78"),
    ("string.regexp", "#96b898"),
    ("constant", "#d4a8b8"),
    ("entity.name.tag", "#dc9e92"),
    ("variable.other.member", "#ec9878"),
    ("comment", "#b4a89c"),
    ("invalid", "#f49090"),
    ("meta.decorator", "#e6c08a"),
    ("support.type.property-name.css", "#deb074"),
  ] {
    assert_eq!(bat_tmtheme_scope_foreground(DARK, scope), expected, "dark {scope}");
  }
}

#[test]
fn light_syntax_colors_match_palette() {
  for (scope, expected) in [
    ("keyword", "#924800"),
    ("keyword.operator", "#8f4418"),
    ("entity.name.function", "#855700"),
    ("support.function.builtin", "#883850"),
    ("entity.name.type", "#285464"),
    ("string", "#4d5c1a"),
    ("string.regexp", "#286a48"),
    ("constant", "#7e4060"),
    ("entity.name.tag", "#8e4632"),
    ("variable.other.member", "#883850"),
    ("comment", "#544c40"),
    ("invalid", "#b03434"),
    ("meta.decorator", "#7a5a1c"),
    ("support.type.property-name.css", "#74501c"),
  ] {
    assert_eq!(bat_tmtheme_scope_foreground(LIGHT, scope), expected, "light {scope}");
  }
}

#[test]
fn dark_font_styles_preserve_three_tiers() {
  assert_eq!(bat_tmtheme_scope_font_style(DARK, "keyword"), "bold");
  assert_eq!(bat_tmtheme_scope_font_style(DARK, "entity.name.tag"), "bold");
  assert_eq!(bat_tmtheme_scope_font_style(DARK, "entity.name.type"), "italic");
  assert_eq!(bat_tmtheme_scope_font_style(DARK, "comment"), "italic");
  assert_eq!(bat_tmtheme_scope_font_style(DARK, "meta.decorator"), "italic");
  assert_eq!(
    bat_tmtheme_scope_font_style(DARK, "support.type.property-name.css"),
    "italic"
  );
  assert_eq!(bat_tmtheme_scope_font_style(DARK, "keyword.operator"), "");
}

#[test]
fn light_font_styles_preserve_three_tiers() {
  assert_eq!(bat_tmtheme_scope_font_style(LIGHT, "keyword"), "bold");
  assert_eq!(bat_tmtheme_scope_font_style(LIGHT, "entity.name.tag"), "bold");
  assert_eq!(bat_tmtheme_scope_font_style(LIGHT, "entity.name.type"), "italic");
  assert_eq!(bat_tmtheme_scope_font_style(LIGHT, "comment"), "italic");
  assert_eq!(bat_tmtheme_scope_font_style(LIGHT, "meta.decorator"), "italic");
  assert_eq!(
    bat_tmtheme_scope_font_style(LIGHT, "support.type.property-name.css"),
    "italic"
  );
  assert_eq!(bat_tmtheme_scope_font_style(LIGHT, "keyword.operator"), "");
}

#[test]
fn dark_has_no_pure_black_background() {
  assert_ne!(bat_tmtheme_global_setting(DARK, "background"), "#000000");
}

#[test]
fn light_has_no_pure_white_background() {
  assert_ne!(bat_tmtheme_global_setting(LIGHT, "background"), "#ffffff");
}
