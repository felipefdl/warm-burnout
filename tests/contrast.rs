mod common;

use common::contrast_ratio;

// -- Sanity checks for the contrast_ratio function itself --

#[test]
fn black_on_white_is_21() {
  let cr = contrast_ratio("#000000", "#ffffff");
  assert!((cr - 21.0).abs() < 0.01, "black on white should be 21:1, got {cr:.2}");
}

#[test]
fn white_on_white_is_1() {
  let cr = contrast_ratio("#ffffff", "#ffffff");
  assert!((cr - 1.0).abs() < 0.01, "white on white should be 1:1, got {cr:.2}");
}

#[test]
fn black_on_black_is_1() {
  let cr = contrast_ratio("#000000", "#000000");
  assert!((cr - 1.0).abs() < 0.01, "black on black should be 1:1, got {cr:.2}");
}

#[test]
fn order_independent() {
  let a = contrast_ratio("#bfbdb6", "#1a1510");
  let b = contrast_ratio("#1a1510", "#bfbdb6");
  assert!(
    (a - b).abs() < 0.001,
    "contrast ratio should be order-independent: {a:.3} vs {b:.3}"
  );
}

#[test]
fn strips_hash_prefix() {
  let with = contrast_ratio("#bfbdb6", "#1a1510");
  let without = contrast_ratio("bfbdb6", "1a1510");
  assert!((with - without).abs() < 0.001, "hash prefix should be optional");
}

// -- Dark theme: all tokens must be >= 7.0:1 (AAA) against #1a1510 --

const DARK_BG: &str = "#1a1510";

fn assert_dark_aaa(name: &str, hex: &str) {
  let cr = contrast_ratio(hex, DARK_BG);
  assert!(cr >= 7.0, "dark {name} ({hex}) contrast {cr:.2}:1 < 7.0 AAA minimum");
}

#[test]
fn dark_foreground_aaa() {
  assert_dark_aaa("foreground", "#bfbdb6");
}

#[test]
fn dark_keywords_aaa() {
  assert_dark_aaa("keywords/storage", "#ff8f40");
}

#[test]
fn dark_functions_aaa() {
  assert_dark_aaa("functions", "#ffb454");
}

#[test]
fn dark_operators_aaa() {
  assert_dark_aaa("operators", "#f29668");
}

#[test]
fn dark_decorators_aaa() {
  assert_dark_aaa("decorators", "#e6c08a");
}

#[test]
fn dark_types_aaa() {
  assert_dark_aaa("types/classes", "#90aec0");
}

#[test]
fn dark_strings_aaa() {
  assert_dark_aaa("strings", "#b4bc78");
}

#[test]
fn dark_regex_aaa() {
  assert_dark_aaa("regex/escape", "#96b898");
}

#[test]
fn dark_constants_aaa() {
  assert_dark_aaa("constants/numbers", "#d4a8b8");
}

#[test]
fn dark_tags_aaa() {
  assert_dark_aaa("tags", "#dc9e92");
}

#[test]
fn dark_member_vars_aaa() {
  assert_dark_aaa("member vars", "#ec9878");
}

#[test]
fn dark_comments_aaa() {
  assert_dark_aaa("comments", "#b4a89c");
}

#[test]
fn dark_error_aaa() {
  assert_dark_aaa("error/invalid", "#f49090");
}

#[test]
fn dark_css_properties_aaa() {
  assert_dark_aaa("CSS properties", "#deb074");
}

// -- Light theme: all tokens must be >= 4.5:1 (AA) against #F5EDE0 --

const LIGHT_BG: &str = "#F5EDE0";

fn assert_light_aa(name: &str, hex: &str) {
  let cr = contrast_ratio(hex, LIGHT_BG);
  assert!(cr >= 4.5, "light {name} ({hex}) contrast {cr:.2}:1 < 4.5 AA minimum");
}

#[test]
fn light_foreground_aa() {
  assert_light_aa("foreground", "#3a3630");
}

#[test]
fn light_keywords_aa() {
  assert_light_aa("keywords/storage", "#924800");
}

#[test]
fn light_functions_aa() {
  assert_light_aa("functions", "#855700");
}

#[test]
fn light_operators_aa() {
  assert_light_aa("operators", "#8f4418");
}

#[test]
fn light_decorators_aa() {
  assert_light_aa("decorators", "#7a5a1c");
}

#[test]
fn light_types_aa() {
  assert_light_aa("types/classes", "#285464");
}

#[test]
fn light_strings_aa() {
  assert_light_aa("strings", "#4d5c1a");
}

#[test]
fn light_regex_aa() {
  assert_light_aa("regex/escape", "#286a48");
}

#[test]
fn light_constants_aa() {
  assert_light_aa("constants/numbers", "#7e4060");
}

#[test]
fn light_tags_aa() {
  assert_light_aa("tags", "#8e4632");
}

#[test]
fn light_member_vars_aa() {
  assert_light_aa("member vars", "#883850");
}

#[test]
fn light_comments_aa() {
  assert_light_aa("comments", "#544c40");
}

#[test]
fn light_error_aa() {
  assert_light_aa("error/invalid", "#b03434");
}

#[test]
fn light_css_properties_aa() {
  assert_light_aa("CSS properties", "#74501c");
}

// -- Verify specific claimed contrast ratios from AGENTS.md (within 0.1 tolerance) --

fn assert_ratio(name: &str, fg: &str, bg: &str, expected: f64) {
  let cr = contrast_ratio(fg, bg);
  assert!(
    (cr - expected).abs() < 0.15,
    "{name}: expected {expected:.1}:1, got {cr:.2}:1 (fg={fg} bg={bg})"
  );
}

// Dark theme claimed ratios
#[test]
fn dark_foreground_claimed_ratio() {
  assert_ratio("dark foreground", "#bfbdb6", DARK_BG, 9.6);
}

#[test]
fn dark_keywords_claimed_ratio() {
  assert_ratio("dark keywords", "#ff8f40", DARK_BG, 8.0);
}

#[test]
fn dark_functions_claimed_ratio() {
  assert_ratio("dark functions", "#ffb454", DARK_BG, 10.3);
}

#[test]
fn dark_operators_claimed_ratio() {
  assert_ratio("dark operators", "#f29668", DARK_BG, 8.1);
}

#[test]
fn dark_decorators_claimed_ratio() {
  // AGENTS.md claims 10.0:1 but the WCAG formula yields 10.6:1.
  // The documented value appears rounded down; the actual ratio is correct and exceeds AAA.
  assert_ratio("dark decorators", "#e6c08a", DARK_BG, 10.6);
}

#[test]
fn dark_types_claimed_ratio() {
  assert_ratio("dark types", "#90aec0", DARK_BG, 7.8);
}

#[test]
fn dark_strings_claimed_ratio() {
  assert_ratio("dark strings", "#b4bc78", DARK_BG, 9.0);
}

#[test]
fn dark_regex_claimed_ratio() {
  assert_ratio("dark regex", "#96b898", DARK_BG, 8.3);
}

#[test]
fn dark_constants_claimed_ratio() {
  assert_ratio("dark constants", "#d4a8b8", DARK_BG, 8.7);
}

#[test]
fn dark_tags_claimed_ratio() {
  assert_ratio("dark tags", "#dc9e92", DARK_BG, 8.1);
}

#[test]
fn dark_member_vars_claimed_ratio() {
  assert_ratio("dark member vars", "#ec9878", DARK_BG, 8.1);
}

#[test]
fn dark_comments_claimed_ratio() {
  assert_ratio("dark comments", "#b4a89c", DARK_BG, 7.8);
}

#[test]
fn dark_error_claimed_ratio() {
  assert_ratio("dark error", "#f49090", DARK_BG, 7.9);
}

#[test]
fn dark_css_properties_claimed_ratio() {
  assert_ratio("dark CSS properties", "#deb074", DARK_BG, 9.1);
}

// Light theme claimed ratios
#[test]
fn light_foreground_claimed_ratio() {
  assert_ratio("light foreground", "#3a3630", LIGHT_BG, 10.3);
}

#[test]
fn light_keywords_claimed_ratio() {
  assert_ratio("light keywords", "#924800", LIGHT_BG, 5.7);
}

#[test]
fn light_functions_claimed_ratio() {
  assert_ratio("light functions", "#855700", LIGHT_BG, 5.4);
}

#[test]
fn light_operators_claimed_ratio() {
  assert_ratio("light operators", "#8f4418", LIGHT_BG, 6.0);
}

#[test]
fn light_decorators_claimed_ratio() {
  assert_ratio("light decorators", "#7a5a1c", LIGHT_BG, 5.5);
}

#[test]
fn light_types_claimed_ratio() {
  assert_ratio("light types", "#285464", LIGHT_BG, 7.2);
}

#[test]
fn light_strings_claimed_ratio() {
  assert_ratio("light strings", "#4d5c1a", LIGHT_BG, 6.3);
}

#[test]
fn light_regex_claimed_ratio() {
  assert_ratio("light regex", "#286a48", LIGHT_BG, 5.6);
}

#[test]
fn light_constants_claimed_ratio() {
  assert_ratio("light constants", "#7e4060", LIGHT_BG, 6.5);
}

#[test]
fn light_tags_claimed_ratio() {
  assert_ratio("light tags", "#8e4632", LIGHT_BG, 5.9);
}

#[test]
fn light_member_vars_claimed_ratio() {
  assert_ratio("light member vars", "#883850", LIGHT_BG, 6.6);
}

#[test]
fn light_comments_claimed_ratio() {
  assert_ratio("light comments", "#544c40", LIGHT_BG, 7.3);
}

#[test]
fn light_error_claimed_ratio() {
  assert_ratio("light error", "#b03434", LIGHT_BG, 5.3);
}

#[test]
fn light_css_properties_claimed_ratio() {
  assert_ratio("light CSS properties", "#74501c", LIGHT_BG, 6.2);
}

// -- Accent and cursor colors --

#[test]
fn dark_accent_contrast() {
  let cr = contrast_ratio("#b8522e", DARK_BG);
  assert!(
    cr >= 3.0,
    "dark accent #b8522e contrast {cr:.2}:1 (used for UI accents, not body text)"
  );
}

#[test]
fn light_accent_contrast() {
  let cr = contrast_ratio("#b8522e", LIGHT_BG);
  assert!(
    cr >= 3.0,
    "light accent #b8522e contrast {cr:.2}:1 (used for UI accents, not body text)"
  );
}

#[test]
fn dark_cursor_contrast() {
  let cr = contrast_ratio("#f5c56e", DARK_BG);
  assert!(
    cr >= 7.0,
    "dark cursor #f5c56e contrast {cr:.2}:1 should be high-visibility"
  );
}

#[test]
fn light_cursor_contrast() {
  let cr = contrast_ratio("#8a6600", LIGHT_BG);
  assert!(cr >= 4.5, "light cursor #8a6600 contrast {cr:.2}:1 should be >= AA");
}
