mod common;

use common::{extract_hex_colors, is_valid_hex, tmux_option_value, tmux_style_bg, tmux_style_fg};

const DARK: &str = include_str!("../tmux/warm-burnout-dark.conf");
const LIGHT: &str = include_str!("../tmux/warm-burnout-light.conf");
const PLUGIN: &str = include_str!("../tmux/warm-burnout.tmux");

fn validate_tmux_conf(src: &str) {
  for (i, line) in src.lines().enumerate() {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
      continue;
    }
    assert!(
      line.starts_with("set -g "),
      "line {}: expected 'set -g', got: {line}",
      i + 1
    );
  }
}

fn tmux_option_names(src: &str) -> Vec<String> {
  src
    .lines()
    .filter(|l| {
      let l = l.trim();
      !l.is_empty() && !l.starts_with('#')
    })
    .map(|l| {
      let rest = l.trim().strip_prefix("set -g ").unwrap();
      rest.split_whitespace().next().unwrap().to_string()
    })
    .collect()
}

// -- Format validation --

#[test]
fn dark_is_valid_tmux_conf() {
  validate_tmux_conf(DARK);
}

#[test]
fn light_is_valid_tmux_conf() {
  validate_tmux_conf(LIGHT);
}

// -- Hex color validation --

#[test]
fn dark_all_hex_colors_valid() {
  for (line, hex) in extract_hex_colors(DARK) {
    assert!(is_valid_hex(hex), "dark line {line}: invalid hex: {hex}");
  }
}

#[test]
fn light_all_hex_colors_valid() {
  for (line, hex) in extract_hex_colors(LIGHT) {
    assert!(is_valid_hex(hex), "light line {line}: invalid hex: {hex}");
  }
}

// -- Required options --

const REQUIRED_OPTIONS: &[&str] = &[
  "status-position",
  "status-justify",
  "status-style",
  "status-left",
  "status-left-style",
  "status-right",
  "window-status-format",
  "window-status-current-format",
  "window-status-current-style",
  "window-status-style",
  "window-status-separator",
  "pane-border-style",
  "pane-active-border-style",
  "message-style",
  "message-command-style",
  "mode-style",
  "clock-mode-colour",
];

#[test]
fn dark_has_required_options() {
  let options = tmux_option_names(DARK);
  for required in REQUIRED_OPTIONS {
    assert!(
      options.contains(&required.to_string()),
      "dark missing option: {required}"
    );
  }
}

#[test]
fn light_has_required_options() {
  let options = tmux_option_names(LIGHT);
  for required in REQUIRED_OPTIONS {
    assert!(
      options.contains(&required.to_string()),
      "light missing option: {required}"
    );
  }
}

// -- Both variants set the same options --

#[test]
fn dark_and_light_set_same_options() {
  let mut dark_opts = tmux_option_names(DARK);
  let mut light_opts = tmux_option_names(LIGHT);
  dark_opts.sort();
  light_opts.sort();
  assert_eq!(
    dark_opts, light_opts,
    "dark and light must configure identical option sets"
  );
}

// -- Canonical color assertions (dark) --

#[test]
fn dark_status_bg_is_sidebar() {
  let style = tmux_option_value(DARK, "status-style");
  assert_eq!(tmux_style_bg(&style), "#14120f");
}

#[test]
fn dark_status_fg_is_comments() {
  let style = tmux_option_value(DARK, "status-style");
  assert_eq!(tmux_style_fg(&style), "#b4a89c");
}

#[test]
fn dark_session_name_is_functions() {
  let style = tmux_option_value(DARK, "status-left-style");
  assert_eq!(tmux_style_fg(&style), "#ffb454");
}

#[test]
fn dark_active_window_bg_is_keywords() {
  let style = tmux_option_value(DARK, "window-status-current-style");
  assert_eq!(tmux_style_bg(&style), "#ff8f40");
}

#[test]
fn dark_active_pane_border_is_accent() {
  let style = tmux_option_value(DARK, "pane-active-border-style");
  assert_eq!(tmux_style_fg(&style), "#b8522e");
}

#[test]
fn dark_copy_mode_bg_is_cursor_gold() {
  let style = tmux_option_value(DARK, "mode-style");
  assert_eq!(tmux_style_bg(&style), "#f5c56e");
}

#[test]
fn dark_clock_is_cursor_gold() {
  let value = tmux_option_value(DARK, "clock-mode-colour");
  assert_eq!(value, "#f5c56e");
}

// -- Canonical color assertions (light) --

#[test]
fn light_status_bg_is_sidebar() {
  let style = tmux_option_value(LIGHT, "status-style");
  assert_eq!(tmux_style_bg(&style), "#ede6da");
}

#[test]
fn light_status_fg_is_comments() {
  let style = tmux_option_value(LIGHT, "status-style");
  assert_eq!(tmux_style_fg(&style), "#544c40");
}

#[test]
fn light_session_name_is_functions() {
  let style = tmux_option_value(LIGHT, "status-left-style");
  assert_eq!(tmux_style_fg(&style), "#855700");
}

#[test]
fn light_active_window_bg_is_keywords() {
  let style = tmux_option_value(LIGHT, "window-status-current-style");
  assert_eq!(tmux_style_bg(&style), "#924800");
}

#[test]
fn light_active_pane_border_is_accent() {
  let style = tmux_option_value(LIGHT, "pane-active-border-style");
  assert_eq!(tmux_style_fg(&style), "#b8522e");
}

#[test]
fn light_copy_mode_bg_is_cursor_gold() {
  let style = tmux_option_value(LIGHT, "mode-style");
  assert_eq!(tmux_style_bg(&style), "#8a6600");
}

#[test]
fn light_clock_is_cursor_gold() {
  let value = tmux_option_value(LIGHT, "clock-mode-colour");
  assert_eq!(value, "#8a6600");
}

// -- Plugin validation --

#[test]
fn plugin_is_valid_shell_script() {
  assert!(
    PLUGIN.starts_with("#!/usr/bin/env bash"),
    "plugin must have bash shebang"
  );
  assert!(PLUGIN.contains("source-file"), "plugin must reference source-file");
  assert!(
    PLUGIN.contains("warm-burnout-variant"),
    "plugin must read @warm-burnout-variant"
  );
}
