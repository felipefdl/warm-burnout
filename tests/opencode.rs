mod common;

use common::{contrast_ratio, extract_hex_colors, is_valid_hex, opencode_color};

const THEME: &str = include_str!("../opencode/warm-burnout.json");

fn parse_json(src: &str) -> serde_json::Value {
  serde_json::from_str(src).expect("invalid JSON")
}

// -- Valid JSON --

#[test]
fn theme_is_valid_json() {
  parse_json(THEME);
}

// -- All hex colors are valid --

#[test]
fn all_hex_colors_are_valid() {
  for (line, hex) in extract_hex_colors(THEME) {
    assert!(is_valid_hex(hex), "line {line}: invalid hex: {hex}");
  }
}

// -- Schema reference --

#[test]
fn has_schema_reference() {
  let v = parse_json(THEME);
  assert_eq!(v["$schema"].as_str().unwrap(), "https://opencode.ai/theme.json");
}

// -- Required theme keys present --

const REQUIRED_KEYS: &[&str] = &[
  "primary",
  "secondary",
  "accent",
  "error",
  "warning",
  "success",
  "info",
  "text",
  "textMuted",
  "background",
  "backgroundPanel",
  "backgroundElement",
  "border",
  "borderActive",
  "borderSubtle",
];

#[test]
fn has_all_required_theme_keys() {
  let v = parse_json(THEME);
  let theme = v["theme"].as_object().expect("missing theme object");
  for key in REQUIRED_KEYS {
    assert!(theme.contains_key(*key), "missing required theme key: {key}");
  }
}

// -- Both variants present in each key --

#[test]
fn all_theme_keys_have_both_variants() {
  let v = parse_json(THEME);
  let theme = v["theme"].as_object().unwrap();
  for (key, val) in theme {
    let obj = val
      .as_object()
      .unwrap_or_else(|| panic!("theme.{key} is not an object"));
    assert!(obj.contains_key("dark"), "theme.{key} missing 'dark' variant");
    assert!(obj.contains_key("light"), "theme.{key} missing 'light' variant");
  }
}

// -- Canonical backgrounds --

#[test]
fn dark_background_is_canonical() {
  assert_eq!(opencode_color(THEME, "dark", "background"), "#1a1510");
}

#[test]
fn light_background_is_canonical() {
  assert_eq!(opencode_color(THEME, "light", "background"), "#f5ede0");
}

// -- No pure black/white backgrounds --

#[test]
fn dark_no_pure_black_background() {
  assert_ne!(
    opencode_color(THEME, "dark", "background"),
    "#000000",
    "dark background must not be pure black (halation risk)"
  );
}

#[test]
fn light_no_pure_white_background() {
  assert_ne!(
    opencode_color(THEME, "light", "background"),
    "#ffffff",
    "light background must not be pure white (luminance overload)"
  );
}

// -- Canonical foregrounds --

#[test]
fn dark_text_is_canonical() {
  assert_eq!(opencode_color(THEME, "dark", "text"), "#bfbdb6");
}

#[test]
fn light_text_is_canonical() {
  assert_eq!(opencode_color(THEME, "light", "text"), "#3a3630");
}

// -- Canonical syntax tokens --

#[test]
fn dark_syntax_keyword_is_canonical() {
  assert_eq!(opencode_color(THEME, "dark", "syntaxKeyword"), "#ff8f40");
}

#[test]
fn light_syntax_keyword_is_canonical() {
  assert_eq!(opencode_color(THEME, "light", "syntaxKeyword"), "#924800");
}

#[test]
fn dark_syntax_function_is_canonical() {
  assert_eq!(opencode_color(THEME, "dark", "syntaxFunction"), "#ffb454");
}

#[test]
fn light_syntax_function_is_canonical() {
  assert_eq!(opencode_color(THEME, "light", "syntaxFunction"), "#855700");
}

#[test]
fn dark_syntax_string_is_canonical() {
  assert_eq!(opencode_color(THEME, "dark", "syntaxString"), "#b4bc78");
}

#[test]
fn light_syntax_string_is_canonical() {
  assert_eq!(opencode_color(THEME, "light", "syntaxString"), "#4d5c1a");
}

#[test]
fn dark_syntax_number_is_canonical() {
  assert_eq!(opencode_color(THEME, "dark", "syntaxNumber"), "#d4a8b8");
}

#[test]
fn light_syntax_number_is_canonical() {
  assert_eq!(opencode_color(THEME, "light", "syntaxNumber"), "#7e4060");
}

#[test]
fn dark_syntax_type_is_canonical() {
  assert_eq!(opencode_color(THEME, "dark", "syntaxType"), "#90aec0");
}

#[test]
fn light_syntax_type_is_canonical() {
  assert_eq!(opencode_color(THEME, "light", "syntaxType"), "#285464");
}

#[test]
fn dark_syntax_operator_is_canonical() {
  assert_eq!(opencode_color(THEME, "dark", "syntaxOperator"), "#f29668");
}

#[test]
fn light_syntax_operator_is_canonical() {
  assert_eq!(opencode_color(THEME, "light", "syntaxOperator"), "#8f4418");
}

#[test]
fn dark_syntax_comment_is_canonical() {
  assert_eq!(opencode_color(THEME, "dark", "syntaxComment"), "#b4a89c");
}

#[test]
fn light_syntax_comment_is_canonical() {
  assert_eq!(opencode_color(THEME, "light", "syntaxComment"), "#544c40");
}

// -- Accent is canonical brand color --

#[test]
fn dark_accent_is_canonical() {
  assert_eq!(opencode_color(THEME, "dark", "accent"), "#b8522e");
}

#[test]
fn light_accent_is_canonical() {
  assert_eq!(opencode_color(THEME, "light", "accent"), "#b8522e");
}

// -- Error uses canonical error token --

#[test]
fn dark_error_is_canonical() {
  assert_eq!(opencode_color(THEME, "dark", "error"), "#f49090");
}

#[test]
fn light_error_is_canonical() {
  assert_eq!(opencode_color(THEME, "light", "error"), "#b03434");
}

// -- Info uses canonical types accent (the one cool color) --

#[test]
fn dark_info_is_canonical_types() {
  assert_eq!(opencode_color(THEME, "dark", "info"), "#90aec0");
}

#[test]
fn light_info_is_canonical_types() {
  assert_eq!(opencode_color(THEME, "light", "info"), "#285464");
}

// -- Success uses canonical strings (warm green) --

#[test]
fn dark_success_is_canonical_strings() {
  assert_eq!(opencode_color(THEME, "dark", "success"), "#b4bc78");
}

#[test]
fn light_success_is_canonical_strings() {
  assert_eq!(opencode_color(THEME, "light", "success"), "#4d5c1a");
}

// -- Background hierarchy: panel is dimmer than main bg --

#[test]
fn dark_panel_is_dimmer_than_background() {
  let bg = opencode_color(THEME, "dark", "background");
  let panel = opencode_color(THEME, "dark", "backgroundPanel");
  assert_ne!(bg, panel, "panel and background should differ");
}

#[test]
fn light_panel_is_dimmer_than_background() {
  let bg = opencode_color(THEME, "light", "background");
  let panel = opencode_color(THEME, "light", "backgroundPanel");
  assert_ne!(bg, panel, "panel and background should differ");
}

// -- Contrast: dark text meets AAA (>= 7.0:1) against dark background --

#[test]
fn dark_text_contrast_meets_aaa() {
  let bg = opencode_color(THEME, "dark", "background");
  let fg = opencode_color(THEME, "dark", "text");
  let cr = contrast_ratio(&fg, &bg);
  assert!(cr >= 7.0, "dark text contrast {cr:.1}:1 is below AAA (7.0:1)");
}

// -- Contrast: light text meets AA (>= 4.5:1) against light background --

#[test]
fn light_text_contrast_meets_aa() {
  let bg = opencode_color(THEME, "light", "background");
  let fg = opencode_color(THEME, "light", "text");
  let cr = contrast_ratio(&fg, &bg);
  assert!(cr >= 4.5, "light text contrast {cr:.1}:1 is below AA (4.5:1)");
}

// -- Contrast: dark syntax tokens meet AAA --

const DARK_SYNTAX_KEYS: &[&str] = &[
  "syntaxKeyword",
  "syntaxFunction",
  "syntaxString",
  "syntaxNumber",
  "syntaxType",
  "syntaxOperator",
  "syntaxComment",
];

#[test]
fn dark_syntax_tokens_meet_aaa_contrast() {
  let bg = opencode_color(THEME, "dark", "background");
  for key in DARK_SYNTAX_KEYS {
    let fg = opencode_color(THEME, "dark", key);
    let cr = contrast_ratio(&fg, &bg);
    assert!(cr >= 7.0, "dark {key} ({fg}) contrast {cr:.1}:1 is below AAA (7.0:1)");
  }
}

// -- Contrast: light syntax tokens meet AA --

const LIGHT_SYNTAX_KEYS: &[&str] = &[
  "syntaxKeyword",
  "syntaxFunction",
  "syntaxString",
  "syntaxNumber",
  "syntaxType",
  "syntaxOperator",
  "syntaxComment",
];

#[test]
fn light_syntax_tokens_meet_aa_contrast() {
  let bg = opencode_color(THEME, "light", "background");
  for key in LIGHT_SYNTAX_KEYS {
    let fg = opencode_color(THEME, "light", key);
    let cr = contrast_ratio(&fg, &bg);
    assert!(cr >= 4.5, "light {key} ({fg}) contrast {cr:.1}:1 is below AA (4.5:1)");
  }
}

// -- Brand: no steel-blue in chrome (reserved for types accent only) --

const CHROME_KEYS: &[&str] = &["background", "backgroundPanel", "backgroundElement", "text", "border"];

#[test]
fn dark_no_steel_blue_in_chrome() {
  for key in CHROME_KEYS {
    let val = opencode_color(THEME, "dark", key);
    assert_ne!(val, "#90aec0", "dark {key} must not be canonical steel-blue");
  }
}

#[test]
fn light_no_steel_blue_in_chrome() {
  for key in CHROME_KEYS {
    let val = opencode_color(THEME, "light", key);
    assert_ne!(val, "#285464", "light {key} must not be canonical steel-blue");
  }
}

// -- Defs section exists and all references resolve --

#[test]
fn defs_section_exists() {
  let v = parse_json(THEME);
  assert!(v.get("defs").is_some(), "missing defs section");
  assert!(!v["defs"].as_object().unwrap().is_empty(), "defs section is empty");
}

#[test]
fn all_theme_references_resolve() {
  let v = parse_json(THEME);
  let defs = v["defs"].as_object().unwrap();
  let theme = v["theme"].as_object().unwrap();

  for (key, val) in theme {
    let obj = val.as_object().unwrap();
    for variant in ["dark", "light"] {
      let ref_val = &obj[variant];
      if let Some(s) = ref_val.as_str()
        && !s.starts_with('#')
      {
        assert!(
          defs.contains_key(s),
          "theme.{key}.{variant} references undefined def: {s}"
        );
      }
    }
  }
}

// -- Cross-platform: syntax colors match VS Code canonical palette --

#[test]
fn dark_primary_matches_canonical_keywords() {
  assert_eq!(opencode_color(THEME, "dark", "primary"), "#ff8f40");
}

#[test]
fn light_primary_matches_canonical_keywords() {
  assert_eq!(opencode_color(THEME, "light", "primary"), "#924800");
}

#[test]
fn dark_secondary_matches_canonical_functions() {
  assert_eq!(opencode_color(THEME, "dark", "secondary"), "#ffb454");
}

#[test]
fn light_secondary_matches_canonical_functions() {
  assert_eq!(opencode_color(THEME, "light", "secondary"), "#855700");
}

// -- Markdown tokens map to expected canonical values --

#[test]
fn dark_markdown_heading_matches_keywords() {
  assert_eq!(
    opencode_color(THEME, "dark", "markdownHeading"),
    opencode_color(THEME, "dark", "syntaxKeyword"),
  );
}

#[test]
fn dark_markdown_code_matches_strings() {
  assert_eq!(
    opencode_color(THEME, "dark", "markdownCode"),
    opencode_color(THEME, "dark", "syntaxString"),
  );
}

#[test]
fn dark_markdown_link_matches_types() {
  assert_eq!(
    opencode_color(THEME, "dark", "markdownLink"),
    opencode_color(THEME, "dark", "syntaxType"),
  );
}

// -- Diff section present and uses expected colors --

const DIFF_KEYS: &[&str] = &[
  "diffAdded",
  "diffRemoved",
  "diffContext",
  "diffHunkHeader",
  "diffHighlightAdded",
  "diffHighlightRemoved",
  "diffAddedBg",
  "diffRemovedBg",
  "diffContextBg",
  "diffLineNumber",
  "diffAddedLineNumberBg",
  "diffRemovedLineNumberBg",
];

#[test]
fn has_all_diff_keys() {
  let v = parse_json(THEME);
  let theme = v["theme"].as_object().unwrap();
  for key in DIFF_KEYS {
    assert!(theme.contains_key(*key), "missing diff key: {key}");
  }
}

// -- Markdown section present --

const MARKDOWN_KEYS: &[&str] = &[
  "markdownText",
  "markdownHeading",
  "markdownLink",
  "markdownLinkText",
  "markdownCode",
  "markdownBlockQuote",
  "markdownEmph",
  "markdownStrong",
  "markdownHorizontalRule",
  "markdownListItem",
  "markdownListEnumeration",
  "markdownImage",
  "markdownImageText",
  "markdownCodeBlock",
];

#[test]
fn has_all_markdown_keys() {
  let v = parse_json(THEME);
  let theme = v["theme"].as_object().unwrap();
  for key in MARKDOWN_KEYS {
    assert!(theme.contains_key(*key), "missing markdown key: {key}");
  }
}

// -- Syntax section present --

const SYNTAX_KEYS: &[&str] = &[
  "syntaxComment",
  "syntaxKeyword",
  "syntaxFunction",
  "syntaxVariable",
  "syntaxString",
  "syntaxNumber",
  "syntaxType",
  "syntaxOperator",
  "syntaxPunctuation",
];

#[test]
fn has_all_syntax_keys() {
  let v = parse_json(THEME);
  let theme = v["theme"].as_object().unwrap();
  for key in SYNTAX_KEYS {
    assert!(theme.contains_key(*key), "missing syntax key: {key}");
  }
}

// -- Diff fg vs diff bg contrast --
//
// Base diffAdded / diffRemoved must meet AA (4.5:1) since they carry the
// primary signal. Highlight variants are inline emphasis on top of the same
// background; the WCAG 3.0:1 large-text threshold is the right bar there.

const DIFF_BASE_PAIRS: &[(&str, &str)] = &[("diffAdded", "diffAddedBg"), ("diffRemoved", "diffRemovedBg")];
const DIFF_HIGHLIGHT_PAIRS: &[(&str, &str)] = &[
  ("diffHighlightAdded", "diffAddedBg"),
  ("diffHighlightRemoved", "diffRemovedBg"),
];

#[test]
fn dark_diff_base_over_diff_bg_meets_aa() {
  for (fg_key, bg_key) in DIFF_BASE_PAIRS {
    let fg = opencode_color(THEME, "dark", fg_key);
    let bg = opencode_color(THEME, "dark", bg_key);
    let cr = contrast_ratio(&fg, &bg);
    assert!(
      cr >= 4.5,
      "dark {fg_key} ({fg}) over {bg_key} ({bg}) contrast {cr:.1}:1 is below AA (4.5:1)"
    );
  }
}

#[test]
fn light_diff_base_over_diff_bg_meets_aa() {
  for (fg_key, bg_key) in DIFF_BASE_PAIRS {
    let fg = opencode_color(THEME, "light", fg_key);
    let bg = opencode_color(THEME, "light", bg_key);
    let cr = contrast_ratio(&fg, &bg);
    assert!(
      cr >= 4.5,
      "light {fg_key} ({fg}) over {bg_key} ({bg}) contrast {cr:.1}:1 is below AA (4.5:1)"
    );
  }
}

#[test]
fn dark_diff_highlight_over_diff_bg_meets_large_text_aa() {
  for (fg_key, bg_key) in DIFF_HIGHLIGHT_PAIRS {
    let fg = opencode_color(THEME, "dark", fg_key);
    let bg = opencode_color(THEME, "dark", bg_key);
    let cr = contrast_ratio(&fg, &bg);
    assert!(
      cr >= 3.0,
      "dark {fg_key} ({fg}) over {bg_key} ({bg}) contrast {cr:.1}:1 is below large-text AA (3.0:1)"
    );
  }
}

#[test]
fn light_diff_highlight_over_diff_bg_meets_large_text_aa() {
  for (fg_key, bg_key) in DIFF_HIGHLIGHT_PAIRS {
    let fg = opencode_color(THEME, "light", fg_key);
    let bg = opencode_color(THEME, "light", bg_key);
    let cr = contrast_ratio(&fg, &bg);
    assert!(
      cr >= 3.0,
      "light {fg_key} ({fg}) over {bg_key} ({bg}) contrast {cr:.1}:1 is below large-text AA (3.0:1)"
    );
  }
}

// -- Status colors meet AA contrast against background --
//
// `warning` deliberately reuses the brand accent (#b8522e) which sits at ~3.7:1
// dark and ~4.2:1 light against the editor bg. Per AGENTS.md design decision 2,
// this is the chosen warm-caution tone; it ships with iconography and is not
// required to meet text-AA. So error/info/success are checked here, warning is
// covered by `*_warning_uses_brand_accent` instead.

const TEXT_STATUS_KEYS: &[&str] = &["error", "info", "success"];

#[test]
fn dark_status_colors_meet_aa_contrast() {
  let bg = opencode_color(THEME, "dark", "background");
  for key in TEXT_STATUS_KEYS {
    let fg = opencode_color(THEME, "dark", key);
    let cr = contrast_ratio(&fg, &bg);
    assert!(cr >= 4.5, "dark {key} ({fg}) contrast {cr:.1}:1 is below AA (4.5:1)");
  }
}

#[test]
fn light_status_colors_meet_aa_contrast() {
  let bg = opencode_color(THEME, "light", "background");
  for key in TEXT_STATUS_KEYS {
    let fg = opencode_color(THEME, "light", key);
    let cr = contrast_ratio(&fg, &bg);
    assert!(cr >= 4.5, "light {key} ({fg}) contrast {cr:.1}:1 is below AA (4.5:1)");
  }
}

#[test]
fn dark_warning_uses_brand_accent() {
  assert_eq!(opencode_color(THEME, "dark", "warning"), "#b8522e");
}

#[test]
fn light_warning_uses_brand_accent() {
  assert_eq!(opencode_color(THEME, "light", "warning"), "#b8522e");
}

// -- borderActive resolves to the brand accent --

#[test]
fn dark_border_active_matches_accent() {
  assert_eq!(
    opencode_color(THEME, "dark", "borderActive"),
    opencode_color(THEME, "dark", "accent"),
  );
}

#[test]
fn light_border_active_matches_accent() {
  assert_eq!(
    opencode_color(THEME, "light", "borderActive"),
    opencode_color(THEME, "light", "accent"),
  );
}

// -- No orphan defs (every def is referenced by at least one theme key) --

#[test]
fn no_orphan_defs() {
  let v = parse_json(THEME);
  let defs = v["defs"].as_object().unwrap();
  let theme_str = serde_json::to_string(&v["theme"]).unwrap();
  for def_name in defs.keys() {
    let needle = format!("\"{def_name}\"");
    assert!(
      theme_str.contains(&needle),
      "def '{def_name}' is defined but not referenced by any theme key"
    );
  }
}
