mod common;

use common::{extract_hex_colors, ghostty_ansi_color, ghostty_color, is_valid_hex, wezterm_ansi_color, wezterm_color};

const DARK: &str = include_str!("../wezterm/warm-burnout-dark.toml");
const LIGHT: &str = include_str!("../wezterm/warm-burnout-light.toml");

const GHOSTTY_DARK: &str = include_str!("../ghostty/warm-burnout-dark");
const GHOSTTY_LIGHT: &str = include_str!("../ghostty/warm-burnout-light");

const REQUIRED_TABLES: &[&str] = &[
  "colors",
  "colors.indexed",
  "colors.tab_bar",
  "colors.tab_bar.active_tab",
  "colors.tab_bar.inactive_tab",
  "colors.tab_bar.inactive_tab_hover",
  "colors.tab_bar.new_tab",
  "colors.tab_bar.new_tab_hover",
  "metadata",
];

const UNSUPPORTED_STABLE_PALETTE_KEYS: &[&str] = &[
  "input_selector_label_bg",
  "input_selector_label_fg",
  "launcher_label_bg",
  "launcher_label_fg",
];

fn parse_toml(src: &str) -> toml::Table {
  src.parse::<toml::Table>().expect("invalid TOML")
}

fn assert_path_exists(src: &str, path: &str, variant: &str) {
  let table = parse_toml(src);
  let mut value = toml::Value::Table(table);
  for part in path.split('.') {
    value = match value.get(part) {
      Some(v) => v.clone(),
      None => panic!("{variant}: missing required table path '{path}' (failed at '{part}')"),
    };
  }
  assert!(value.is_table(), "{variant}: '{path}' exists but is not a table");
}

fn assert_unsupported_stable_palette_keys_absent(src: &str, variant: &str) {
  let table = parse_toml(src);
  let colors = table
    .get("colors")
    .and_then(|v| v.as_table())
    .expect("missing colors table");
  for key in UNSUPPORTED_STABLE_PALETTE_KEYS {
    assert!(
      !colors.contains_key(*key),
      "{variant}: colors.{key} is not accepted by stable WezTerm palette files"
    );
  }
}

#[test]
fn dark_is_valid_toml() {
  parse_toml(DARK);
}

#[test]
fn light_is_valid_toml() {
  parse_toml(LIGHT);
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
fn dark_has_required_tables() {
  for path in REQUIRED_TABLES {
    assert_path_exists(DARK, path, "dark");
  }
}

#[test]
fn light_has_required_tables() {
  for path in REQUIRED_TABLES {
    assert_path_exists(LIGHT, path, "light");
  }
}

#[test]
fn dark_has_no_unsupported_stable_palette_keys() {
  assert_unsupported_stable_palette_keys_absent(DARK, "dark");
}

#[test]
fn light_has_no_unsupported_stable_palette_keys() {
  assert_unsupported_stable_palette_keys_absent(LIGHT, "light");
}

#[test]
fn dark_metadata_name_is_warm_burnout_dark() {
  assert_eq!(wezterm_color(DARK, "metadata.name"), "Warm Burnout Dark");
}

#[test]
fn light_metadata_name_is_warm_burnout_light() {
  assert_eq!(wezterm_color(LIGHT, "metadata.name"), "Warm Burnout Light");
}

#[test]
fn dark_background_matches_ghostty() {
  let wezterm = wezterm_color(DARK, "colors.background");
  let ghostty = ghostty_color(GHOSTTY_DARK, "background");
  assert_eq!(wezterm, ghostty, "dark background: wezterm={wezterm} ghostty={ghostty}");
}

#[test]
fn light_background_matches_ghostty() {
  let wezterm = wezterm_color(LIGHT, "colors.background");
  let ghostty = ghostty_color(GHOSTTY_LIGHT, "background");
  assert_eq!(
    wezterm, ghostty,
    "light background: wezterm={wezterm} ghostty={ghostty}"
  );
}

#[test]
fn dark_foreground_matches_ghostty() {
  let wezterm = wezterm_color(DARK, "colors.foreground");
  let ghostty = ghostty_color(GHOSTTY_DARK, "foreground");
  assert_eq!(wezterm, ghostty, "dark foreground: wezterm={wezterm} ghostty={ghostty}");
}

#[test]
fn light_foreground_matches_ghostty() {
  let wezterm = wezterm_color(LIGHT, "colors.foreground");
  let ghostty = ghostty_color(GHOSTTY_LIGHT, "foreground");
  assert_eq!(
    wezterm, ghostty,
    "light foreground: wezterm={wezterm} ghostty={ghostty}"
  );
}

#[test]
fn dark_cursor_matches_ghostty() {
  let wezterm = wezterm_color(DARK, "colors.cursor_bg");
  let ghostty = ghostty_color(GHOSTTY_DARK, "cursor-color");
  assert_eq!(wezterm, ghostty, "dark cursor: wezterm={wezterm} ghostty={ghostty}");
}

#[test]
fn light_cursor_matches_ghostty() {
  let wezterm = wezterm_color(LIGHT, "colors.cursor_bg");
  let ghostty = ghostty_color(GHOSTTY_LIGHT, "cursor-color");
  assert_eq!(wezterm, ghostty, "light cursor: wezterm={wezterm} ghostty={ghostty}");
}

#[test]
fn dark_cursor_fg_matches_background() {
  assert_eq!(
    wezterm_color(DARK, "colors.cursor_fg"),
    wezterm_color(DARK, "colors.background")
  );
}

#[test]
fn light_cursor_fg_matches_background() {
  assert_eq!(
    wezterm_color(LIGHT, "colors.cursor_fg"),
    wezterm_color(LIGHT, "colors.background")
  );
}

#[test]
fn dark_selection_matches_ghostty() {
  assert_eq!(
    wezterm_color(DARK, "colors.selection_bg"),
    ghostty_color(GHOSTTY_DARK, "selection-background")
  );
  assert_eq!(
    wezterm_color(DARK, "colors.selection_fg"),
    ghostty_color(GHOSTTY_DARK, "selection-foreground")
  );
}

#[test]
fn light_selection_matches_ghostty() {
  assert_eq!(
    wezterm_color(LIGHT, "colors.selection_bg"),
    ghostty_color(GHOSTTY_LIGHT, "selection-background")
  );
  assert_eq!(
    wezterm_color(LIGHT, "colors.selection_fg"),
    ghostty_color(GHOSTTY_LIGHT, "selection-foreground")
  );
}

fn assert_ansi_bank_matches_ghostty(wezterm_src: &str, ghostty_src: &str, bank: &str, base: u8, variant: &str) {
  for index in 0..8 {
    let wezterm = wezterm_ansi_color(wezterm_src, bank, index);
    let ghostty = ghostty_ansi_color(ghostty_src, base + index as u8);
    assert_eq!(
      wezterm,
      ghostty,
      "{variant} {bank}[{index}] (palette {}): wezterm={wezterm} ghostty={ghostty}",
      base + index as u8
    );
  }
}

#[test]
fn dark_ansi_matches_ghostty() {
  assert_ansi_bank_matches_ghostty(DARK, GHOSTTY_DARK, "ansi", 0, "dark");
}

#[test]
fn dark_brights_match_ghostty() {
  assert_ansi_bank_matches_ghostty(DARK, GHOSTTY_DARK, "brights", 8, "dark");
}

#[test]
fn light_ansi_matches_ghostty() {
  assert_ansi_bank_matches_ghostty(LIGHT, GHOSTTY_LIGHT, "ansi", 0, "light");
}

#[test]
fn light_brights_match_ghostty() {
  assert_ansi_bank_matches_ghostty(LIGHT, GHOSTTY_LIGHT, "brights", 8, "light");
}

#[test]
fn dark_split_uses_canonical_accent() {
  assert_eq!(wezterm_color(DARK, "colors.split"), "#b8522e");
}

#[test]
fn light_split_uses_canonical_accent() {
  assert_eq!(wezterm_color(LIGHT, "colors.split"), "#b8522e");
}

#[test]
fn dark_scrollbar_uses_unfocused_frame_stone() {
  assert_eq!(wezterm_color(DARK, "colors.scrollbar_thumb"), "#3a342a");
}

#[test]
fn light_scrollbar_uses_unfocused_frame_stone() {
  assert_eq!(wezterm_color(LIGHT, "colors.scrollbar_thumb"), "#c5beb2");
}

#[test]
fn dark_compose_cursor_matches_cursor_and_bell_uses_amber() {
  assert_eq!(
    wezterm_color(DARK, "colors.compose_cursor"),
    wezterm_color(DARK, "colors.cursor_bg")
  );
  assert_eq!(wezterm_color(DARK, "colors.visual_bell"), "#ffb454");
}

#[test]
fn light_compose_cursor_matches_cursor_and_bell_uses_amber() {
  assert_eq!(
    wezterm_color(LIGHT, "colors.compose_cursor"),
    wezterm_color(LIGHT, "colors.cursor_bg")
  );
  assert_eq!(wezterm_color(LIGHT, "colors.visual_bell"), "#855700");
}

#[test]
fn dark_active_tab_uses_keywords_token() {
  assert_eq!(wezterm_color(DARK, "colors.tab_bar.active_tab.bg_color"), "#ff8f40");
  assert_eq!(wezterm_color(DARK, "colors.tab_bar.active_tab.fg_color"), "#1a1510");
}

#[test]
fn light_active_tab_uses_keywords_token() {
  assert_eq!(wezterm_color(LIGHT, "colors.tab_bar.active_tab.bg_color"), "#924800");
  assert_eq!(wezterm_color(LIGHT, "colors.tab_bar.active_tab.fg_color"), "#f5ede0");
}

#[test]
fn dark_tab_bar_background_matches_terminal_background() {
  assert_eq!(
    wezterm_color(DARK, "colors.tab_bar.background"),
    wezterm_color(DARK, "colors.background")
  );
}

#[test]
fn light_tab_bar_background_matches_terminal_background() {
  assert_eq!(
    wezterm_color(LIGHT, "colors.tab_bar.background"),
    wezterm_color(LIGHT, "colors.background")
  );
}

#[test]
fn dark_inactive_tabs_use_raised_surface_and_comments() {
  assert_eq!(wezterm_color(DARK, "colors.tab_bar.inactive_tab.bg_color"), "#1f1d17");
  assert_eq!(wezterm_color(DARK, "colors.tab_bar.inactive_tab.fg_color"), "#b4a89c");
  assert_eq!(wezterm_color(DARK, "colors.tab_bar.new_tab.bg_color"), "#1f1d17");
  assert_eq!(wezterm_color(DARK, "colors.tab_bar.new_tab.fg_color"), "#b4a89c");
}

#[test]
fn light_inactive_tabs_use_raised_surface_and_comments() {
  assert_eq!(wezterm_color(LIGHT, "colors.tab_bar.inactive_tab.bg_color"), "#f0e8dc");
  assert_eq!(wezterm_color(LIGHT, "colors.tab_bar.inactive_tab.fg_color"), "#544c40");
  assert_eq!(wezterm_color(LIGHT, "colors.tab_bar.new_tab.bg_color"), "#f0e8dc");
  assert_eq!(wezterm_color(LIGHT, "colors.tab_bar.new_tab.fg_color"), "#544c40");
}

#[test]
fn dark_copy_and_quick_select_use_terminal_ui_roles() {
  assert_eq!(wezterm_color(DARK, "colors.copy_mode_active_highlight_bg"), "#ff8f40");
  assert_eq!(wezterm_color(DARK, "colors.copy_mode_inactive_highlight_bg"), "#33393a");
  assert_eq!(wezterm_color(DARK, "colors.quick_select_label_bg"), "#ffb454");
  assert_eq!(wezterm_color(DARK, "colors.quick_select_match_bg"), "#33393a");
}

#[test]
fn light_copy_and_quick_select_use_terminal_ui_roles() {
  assert_eq!(wezterm_color(LIGHT, "colors.copy_mode_active_highlight_bg"), "#924800");
  assert_eq!(
    wezterm_color(LIGHT, "colors.copy_mode_inactive_highlight_bg"),
    "#e5e8e2"
  );
  assert_eq!(wezterm_color(LIGHT, "colors.quick_select_label_bg"), "#855700");
  assert_eq!(wezterm_color(LIGHT, "colors.quick_select_match_bg"), "#e5e8e2");
}

#[test]
fn dark_indexed_palette_exposes_accent_and_amber() {
  assert_eq!(wezterm_color(DARK, "colors.indexed.16"), "#b8522e");
  assert_eq!(wezterm_color(DARK, "colors.indexed.17"), "#ffb454");
}

#[test]
fn light_indexed_palette_exposes_accent_and_amber() {
  assert_eq!(wezterm_color(LIGHT, "colors.indexed.16"), "#b8522e");
  assert_eq!(wezterm_color(LIGHT, "colors.indexed.17"), "#855700");
}

#[test]
fn dark_no_canonical_steel_blue_in_chrome() {
  for path in [
    "colors.foreground",
    "colors.background",
    "colors.cursor_bg",
    "colors.selection_bg",
    "colors.split",
    "colors.scrollbar_thumb",
    "colors.tab_bar.active_tab.bg_color",
    "colors.tab_bar.inactive_tab.bg_color",
  ] {
    let value = wezterm_color(DARK, path);
    assert_ne!(value, "#90aec0", "dark {path} must not be canonical steel-blue");
  }
}

#[test]
fn light_no_canonical_steel_blue_in_chrome() {
  for path in [
    "colors.foreground",
    "colors.background",
    "colors.cursor_bg",
    "colors.selection_bg",
    "colors.split",
    "colors.scrollbar_thumb",
    "colors.tab_bar.active_tab.bg_color",
    "colors.tab_bar.inactive_tab.bg_color",
  ] {
    let value = wezterm_color(LIGHT, path);
    assert_ne!(value, "#285464", "light {path} must not be canonical steel-blue");
  }
}
