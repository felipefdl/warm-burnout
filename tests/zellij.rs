mod common;

use common::{extract_hex_colors, is_valid_hex, zellij_color, zellij_component_attrs, zellij_component_names};

const DARK: &str = include_str!("../zellij/warm-burnout-dark.kdl");
const LIGHT: &str = include_str!("../zellij/warm-burnout-light.kdl");

const REQUIRED_COMPONENTS: &[&str] = &[
  "text_unselected",
  "text_selected",
  "ribbon_unselected",
  "ribbon_selected",
  "table_title",
  "table_cell_unselected",
  "table_cell_selected",
  "list_unselected",
  "list_selected",
  "frame_unselected",
  "frame_selected",
  "frame_highlight",
  "exit_code_success",
  "exit_code_error",
  "multiplayer_user_colors",
];

const STANDARD_ATTRS: &[&str] = &[
  "base",
  "background",
  "emphasis_0",
  "emphasis_1",
  "emphasis_2",
  "emphasis_3",
];
const PLAYER_ATTRS: &[&str] = &[
  "player_1",
  "player_2",
  "player_3",
  "player_4",
  "player_5",
  "player_6",
  "player_7",
  "player_8",
  "player_9",
  "player_10",
];

// -- Format validation --

fn validate_zellij_kdl(src: &str, theme: &str) {
  assert!(src.contains("themes {"), "missing top-level themes block");
  assert!(
    src.contains(&format!("{theme} {{")),
    "missing named theme block: {theme}"
  );
}

#[test]
fn dark_is_valid_zellij_kdl() {
  validate_zellij_kdl(DARK, "warm-burnout-dark");
}

#[test]
fn light_is_valid_zellij_kdl() {
  validate_zellij_kdl(LIGHT, "warm-burnout-light");
}

// -- Hex color helpers should not appear in source (Zellij uses RGB, not hex) --

#[test]
fn dark_has_no_hex_colors() {
  let hexes = extract_hex_colors(DARK);
  assert!(
    hexes.is_empty(),
    "zellij dark theme uses RGB triplets, found hex literals: {hexes:?}"
  );
}

#[test]
fn light_has_no_hex_colors() {
  let hexes = extract_hex_colors(LIGHT);
  assert!(
    hexes.is_empty(),
    "zellij light theme uses RGB triplets, found hex literals: {hexes:?}"
  );
}

// -- Required components present --

#[test]
fn dark_has_required_components() {
  let names = zellij_component_names(DARK, "warm-burnout-dark");
  for required in REQUIRED_COMPONENTS {
    assert!(
      names.iter().any(|n| n == required),
      "dark missing component: {required}"
    );
  }
}

#[test]
fn light_has_required_components() {
  let names = zellij_component_names(LIGHT, "warm-burnout-light");
  for required in REQUIRED_COMPONENTS {
    assert!(
      names.iter().any(|n| n == required),
      "light missing component: {required}"
    );
  }
}

#[test]
fn dark_and_light_define_same_components() {
  let mut dark_names = zellij_component_names(DARK, "warm-burnout-dark");
  let mut light_names = zellij_component_names(LIGHT, "warm-burnout-light");
  dark_names.sort();
  light_names.sort();
  assert_eq!(
    dark_names, light_names,
    "dark and light must define identical component sets"
  );
}

// -- Required attrs per component --

fn assert_component_attrs(src: &str, theme: &str, component: &str, expected: &[&str]) {
  let attrs = zellij_component_attrs(src, theme, component);
  for expected_attr in expected {
    assert!(
      attrs.iter().any(|a| a == expected_attr),
      "{theme} > {component} missing attr: {expected_attr}"
    );
  }
}

#[test]
fn dark_components_have_required_attrs() {
  for component in REQUIRED_COMPONENTS {
    if *component == "multiplayer_user_colors" {
      assert_component_attrs(DARK, "warm-burnout-dark", component, PLAYER_ATTRS);
    } else {
      assert_component_attrs(DARK, "warm-burnout-dark", component, STANDARD_ATTRS);
    }
  }
}

#[test]
fn light_components_have_required_attrs() {
  for component in REQUIRED_COMPONENTS {
    if *component == "multiplayer_user_colors" {
      assert_component_attrs(LIGHT, "warm-burnout-light", component, PLAYER_ATTRS);
    } else {
      assert_component_attrs(LIGHT, "warm-burnout-light", component, STANDARD_ATTRS);
    }
  }
}

// -- Color values are valid (every parsed RGB triplet round-trips through hex) --

#[test]
fn dark_all_color_values_parse() {
  for component in REQUIRED_COMPONENTS {
    let attrs = if *component == "multiplayer_user_colors" {
      PLAYER_ATTRS
    } else {
      STANDARD_ATTRS
    };
    for attr in attrs {
      let value = zellij_color(DARK, "warm-burnout-dark", component, attr);
      assert!(
        value == "0" || is_valid_hex(&value),
        "dark {component}.{attr} parsed as: {value}"
      );
    }
  }
}

#[test]
fn light_all_color_values_parse() {
  for component in REQUIRED_COMPONENTS {
    let attrs = if *component == "multiplayer_user_colors" {
      PLAYER_ATTRS
    } else {
      STANDARD_ATTRS
    };
    for attr in attrs {
      let value = zellij_color(LIGHT, "warm-burnout-light", component, attr);
      assert!(
        value == "0" || is_valid_hex(&value),
        "light {component}.{attr} parsed as: {value}"
      );
    }
  }
}

// -- Canonical color assertions (dark) --

#[test]
fn dark_text_background_matches_terminal_bg() {
  assert_eq!(
    zellij_color(DARK, "warm-burnout-dark", "text_unselected", "background"),
    "#1a1510"
  );
}

#[test]
fn dark_text_base_is_comments() {
  assert_eq!(
    zellij_color(DARK, "warm-burnout-dark", "text_unselected", "base"),
    "#b4a89c"
  );
}

#[test]
fn dark_text_selected_pins() {
  assert_eq!(
    zellij_color(DARK, "warm-burnout-dark", "text_selected", "base"),
    "#bfbdb6"
  );
  assert_eq!(
    zellij_color(DARK, "warm-burnout-dark", "text_selected", "background"),
    "#1f1d17"
  );
}

#[test]
fn dark_inactive_ribbon_bg_is_raised() {
  let inactive_bg = zellij_color(DARK, "warm-burnout-dark", "ribbon_unselected", "background");
  let bar_bg = zellij_color(DARK, "warm-burnout-dark", "text_unselected", "background");
  assert_ne!(
    inactive_bg, bar_bg,
    "inactive ribbon must be raised above bar bg so chips are visible"
  );
  assert_eq!(inactive_bg, "#1f1d17");
}

#[test]
fn dark_active_ribbon_bg_is_keywords() {
  assert_eq!(
    zellij_color(DARK, "warm-burnout-dark", "ribbon_selected", "background"),
    "#ff8f40"
  );
}

#[test]
fn dark_active_ribbon_base_is_bg() {
  assert_eq!(
    zellij_color(DARK, "warm-burnout-dark", "ribbon_selected", "base"),
    "#1a1510"
  );
}

#[test]
fn dark_focused_frame_is_accent() {
  assert_eq!(
    zellij_color(DARK, "warm-burnout-dark", "frame_selected", "base"),
    "#b8522e"
  );
}

#[test]
fn dark_unfocused_frame_is_raised_stone() {
  assert_eq!(
    zellij_color(DARK, "warm-burnout-dark", "frame_unselected", "base"),
    "#3a342a"
  );
}

#[test]
fn dark_mode_frame_is_functions_amber() {
  assert_eq!(
    zellij_color(DARK, "warm-burnout-dark", "frame_highlight", "base"),
    "#ffb454"
  );
}

#[test]
fn dark_exit_code_success_is_strings() {
  assert_eq!(
    zellij_color(DARK, "warm-burnout-dark", "exit_code_success", "base"),
    "#b4bc78"
  );
}

#[test]
fn dark_exit_code_error_is_error_pink() {
  assert_eq!(
    zellij_color(DARK, "warm-burnout-dark", "exit_code_error", "base"),
    "#f49090"
  );
}

#[test]
fn dark_player_1_is_keywords() {
  assert_eq!(
    zellij_color(DARK, "warm-burnout-dark", "multiplayer_user_colors", "player_1"),
    "#ff8f40"
  );
}

#[test]
fn dark_player_10_is_accent() {
  assert_eq!(
    zellij_color(DARK, "warm-burnout-dark", "multiplayer_user_colors", "player_10"),
    "#b8522e"
  );
}

// -- Canonical color assertions (light) --

#[test]
fn light_text_background_matches_terminal_bg() {
  assert_eq!(
    zellij_color(LIGHT, "warm-burnout-light", "text_unselected", "background"),
    "#f5ede0"
  );
}

#[test]
fn light_text_base_is_comments() {
  assert_eq!(
    zellij_color(LIGHT, "warm-burnout-light", "text_unselected", "base"),
    "#544c40"
  );
}

#[test]
fn light_text_selected_pins() {
  assert_eq!(
    zellij_color(LIGHT, "warm-burnout-light", "text_selected", "base"),
    "#3a3630"
  );
  assert_eq!(
    zellij_color(LIGHT, "warm-burnout-light", "text_selected", "background"),
    "#f0e8dc"
  );
}

#[test]
fn light_inactive_ribbon_bg_is_raised() {
  let inactive_bg = zellij_color(LIGHT, "warm-burnout-light", "ribbon_unselected", "background");
  let bar_bg = zellij_color(LIGHT, "warm-burnout-light", "text_unselected", "background");
  assert_ne!(
    inactive_bg, bar_bg,
    "inactive ribbon must be raised above bar bg so chips are visible"
  );
  assert_eq!(inactive_bg, "#f0e8dc");
}

#[test]
fn light_active_ribbon_bg_is_keywords() {
  assert_eq!(
    zellij_color(LIGHT, "warm-burnout-light", "ribbon_selected", "background"),
    "#924800"
  );
}

#[test]
fn light_active_ribbon_base_is_bg() {
  assert_eq!(
    zellij_color(LIGHT, "warm-burnout-light", "ribbon_selected", "base"),
    "#f5ede0"
  );
}

#[test]
fn light_focused_frame_is_accent() {
  assert_eq!(
    zellij_color(LIGHT, "warm-burnout-light", "frame_selected", "base"),
    "#b8522e"
  );
}

#[test]
fn light_unfocused_frame_is_recessed_stone() {
  assert_eq!(
    zellij_color(LIGHT, "warm-burnout-light", "frame_unselected", "base"),
    "#c5beb2"
  );
}

#[test]
fn light_mode_frame_is_functions_amber() {
  assert_eq!(
    zellij_color(LIGHT, "warm-burnout-light", "frame_highlight", "base"),
    "#855700"
  );
}

#[test]
fn light_exit_code_success_is_strings() {
  assert_eq!(
    zellij_color(LIGHT, "warm-burnout-light", "exit_code_success", "base"),
    "#4d5c1a"
  );
}

#[test]
fn light_exit_code_error_is_error_red() {
  assert_eq!(
    zellij_color(LIGHT, "warm-burnout-light", "exit_code_error", "base"),
    "#b03434"
  );
}

#[test]
fn light_player_1_is_keywords() {
  assert_eq!(
    zellij_color(LIGHT, "warm-burnout-light", "multiplayer_user_colors", "player_1"),
    "#924800"
  );
}

#[test]
fn light_player_10_is_accent() {
  assert_eq!(
    zellij_color(LIGHT, "warm-burnout-light", "multiplayer_user_colors", "player_10"),
    "#b8522e"
  );
}

// -- Brand rule: warm everywhere, blue nowhere --

#[test]
fn dark_no_canonical_steel_blue_in_chrome() {
  let forbidden = "#90aec0";
  for component in REQUIRED_COMPONENTS {
    let attrs = if *component == "multiplayer_user_colors" {
      PLAYER_ATTRS
    } else {
      STANDARD_ATTRS
    };
    for attr in attrs {
      let value = zellij_color(DARK, "warm-burnout-dark", component, attr);
      assert_ne!(
        value, forbidden,
        "dark {component}.{attr} uses steel blue {forbidden}: brand rule says warm only in zellij chrome"
      );
    }
  }
}

#[test]
fn light_no_canonical_steel_blue_in_chrome() {
  let forbidden = "#285464";
  for component in REQUIRED_COMPONENTS {
    let attrs = if *component == "multiplayer_user_colors" {
      PLAYER_ATTRS
    } else {
      STANDARD_ATTRS
    };
    for attr in attrs {
      let value = zellij_color(LIGHT, "warm-burnout-light", component, attr);
      assert_ne!(
        value, forbidden,
        "light {component}.{attr} uses steel blue {forbidden}: brand rule says warm only in zellij chrome"
      );
    }
  }
}

// -- Cross-variant: accent stays the same --

#[test]
fn accent_is_identical_across_variants() {
  let dark_accent = zellij_color(DARK, "warm-burnout-dark", "frame_selected", "base");
  let light_accent = zellij_color(LIGHT, "warm-burnout-light", "frame_selected", "base");
  assert_eq!(
    dark_accent, light_accent,
    "frame_selected accent must be identical in both variants"
  );
}

// -- Sentinel `0` background must stay terminal-default for components that show through --

const TRANSPARENT_BG_COMPONENTS: &[&str] = &[
  "table_title",
  "table_cell_unselected",
  "list_unselected",
  "frame_unselected",
  "frame_selected",
  "frame_highlight",
  "exit_code_success",
  "exit_code_error",
];

#[test]
fn dark_transparent_bg_components_keep_zero_sentinel() {
  for component in TRANSPARENT_BG_COMPONENTS {
    let bg = zellij_color(DARK, "warm-burnout-dark", component, "background");
    assert_eq!(
      bg, "0",
      "dark {component}.background must stay `0` so the pane bg shows through"
    );
  }
}

#[test]
fn light_transparent_bg_components_keep_zero_sentinel() {
  for component in TRANSPARENT_BG_COMPONENTS {
    let bg = zellij_color(LIGHT, "warm-burnout-light", component, "background");
    assert_eq!(
      bg, "0",
      "light {component}.background must stay `0` so the pane bg shows through"
    );
  }
}
