mod common;

use common::{emacs_palette_color, extract_hex_colors, is_valid_hex};

const DARK: &str = include_str!("../emacs/warm-burnout-dark-theme.el");
const LIGHT: &str = include_str!("../emacs/warm-burnout-light-theme.el");
const SHARED: &str = include_str!("../emacs/warm-burnout.el");

#[test]
fn dark_all_hex_colors_valid() {
  for (line, hex) in extract_hex_colors(DARK) {
    assert!(is_valid_hex(hex), "line {line}: invalid hex: {hex}");
  }
}

#[test]
fn light_all_hex_colors_valid() {
  for (line, hex) in extract_hex_colors(LIGHT) {
    assert!(is_valid_hex(hex), "line {line}: invalid hex: {hex}");
  }
}

#[test]
fn shared_all_hex_colors_valid() {
  for (line, hex) in extract_hex_colors(SHARED) {
    assert!(is_valid_hex(hex), "line {line}: invalid hex: {hex}");
  }
}

#[test]
fn shared_dark_uses_canonical_background() {
  assert!(
    SHARED.contains("#1a1510"),
    "shared palette must contain canonical dark background #1a1510"
  );
}

#[test]
fn shared_light_uses_canonical_background() {
  assert!(
    SHARED.contains("#F5EDE0"),
    "shared palette must contain canonical light background #F5EDE0"
  );
}

#[test]
fn shared_dark_uses_canonical_foreground() {
  assert!(
    SHARED.contains("#bfbdb6"),
    "shared palette must contain canonical dark foreground #bfbdb6"
  );
}

#[test]
fn shared_light_uses_canonical_foreground() {
  assert!(
    SHARED.contains("#3a3630"),
    "shared palette must contain canonical light foreground #3a3630"
  );
}

#[test]
fn dark_provides_theme() {
  assert!(
    DARK.contains("(provide-theme 'warm-burnout-dark)"),
    "dark theme must provide-theme"
  );
}

#[test]
fn light_provides_theme() {
  assert!(
    LIGHT.contains("(provide-theme 'warm-burnout-light)"),
    "light theme must provide-theme"
  );
}

#[test]
fn dark_has_font_lock_faces() {
  let required = [
    "font-lock-keyword-face",
    "font-lock-function-name-face",
    "font-lock-type-face",
    "font-lock-string-face",
    "font-lock-comment-face",
    "font-lock-constant-face",
  ];
  for face in required {
    assert!(DARK.contains(face), "dark theme missing face: {face}");
  }
}

#[test]
fn light_has_font_lock_faces() {
  let required = [
    "font-lock-keyword-face",
    "font-lock-function-name-face",
    "font-lock-type-face",
    "font-lock-string-face",
    "font-lock-comment-face",
    "font-lock-constant-face",
  ];
  for face in required {
    assert!(LIGHT.contains(face), "light theme missing face: {face}");
  }
}

fn assert_face_style(src: &str, label: &str, face: &str, palette_key: &str, style: &str) {
  let needle = format!("{face} ((t (:foreground ,{palette_key} :{style})");
  assert!(
    src.contains(&needle),
    "{label} {face} should bind {palette_key} with :{style}; not found"
  );
}

#[test]
fn dark_keywords_are_bold() {
  assert_face_style(DARK, "dark", "font-lock-keyword-face", "keyword", "weight bold");
}

#[test]
fn light_keywords_are_bold() {
  assert_face_style(LIGHT, "light", "font-lock-keyword-face", "keyword", "weight bold");
}

#[test]
fn dark_types_are_italic() {
  assert_face_style(DARK, "dark", "font-lock-type-face", "type", "slant italic");
}

#[test]
fn light_types_are_italic() {
  assert_face_style(LIGHT, "light", "font-lock-type-face", "type", "slant italic");
}

#[test]
fn dark_comments_are_italic() {
  assert_face_style(DARK, "dark", "font-lock-comment-face", "comment", "slant italic");
}

#[test]
fn light_comments_are_italic() {
  assert_face_style(LIGHT, "light", "font-lock-comment-face", "comment", "slant italic");
}

#[test]
fn dark_tags_are_bold() {
  assert_face_style(DARK, "dark", "tree-sitter-hl-face:tag", "tag", "weight bold");
}

#[test]
fn light_tags_are_bold() {
  assert_face_style(LIGHT, "light", "tree-sitter-hl-face:tag", "tag", "weight bold");
}

#[test]
fn shared_defines_both_palettes() {
  assert!(
    SHARED.contains("warm-burnout-dark-palette"),
    "shared must define dark palette"
  );
  assert!(
    SHARED.contains("warm-burnout-light-palette"),
    "shared must define light palette"
  );
}

#[test]
fn brand_name_in_themes() {
  assert!(DARK.contains("Warm Burnout"), "dark theme must contain brand name");
  assert!(LIGHT.contains("Warm Burnout"), "light theme must contain brand name");
}

#[test]
fn dark_palette_parses() {
  // Sanity: the parser can read every documented role from the dark palette.
  for key in [
    "bg",
    "fg",
    "comment",
    "cursor",
    "keyword",
    "func",
    "string",
    "type",
    "operator",
    "number",
    "constant",
    "tag",
    "property",
    "member",
    "regex",
    "decorator",
    "error",
    "warn",
    "info",
    "added",
    "modified",
    "deleted",
    "border",
  ] {
    let _ = emacs_palette_color(SHARED, "dark", key);
  }
}

#[test]
fn light_palette_parses() {
  for key in [
    "bg",
    "fg",
    "comment",
    "cursor",
    "keyword",
    "func",
    "string",
    "type",
    "operator",
    "number",
    "constant",
    "tag",
    "property",
    "member",
    "regex",
    "decorator",
    "error",
    "warn",
    "info",
    "added",
    "modified",
    "deleted",
    "border",
  ] {
    let _ = emacs_palette_color(SHARED, "light", key);
  }
}

#[test]
fn dark_canonical_accent_present() {
  // Canonical accent (`#b8522e`) is carried by the `warn` palette key in emacs;
  // make sure it never drifts.
  assert_eq!(emacs_palette_color(SHARED, "dark", "warn"), "#b8522e");
}

#[test]
fn light_canonical_accent_present() {
  assert_eq!(emacs_palette_color(SHARED, "light", "warn"), "#b8522e");
}

#[test]
fn dark_no_alpha_hex() {
  // Emacs only supports 3/6/9/12-digit hex; `#RRGGBBAA` is invalid as a face color.
  for (line, hex) in extract_hex_colors(DARK) {
    let body = hex.strip_prefix('#').unwrap_or(hex);
    assert_ne!(body.len(), 8, "line {line}: 8-digit alpha hex not allowed: {hex}");
  }
  for (line, hex) in extract_hex_colors(SHARED) {
    let body = hex.strip_prefix('#').unwrap_or(hex);
    assert_ne!(body.len(), 8, "line {line}: 8-digit alpha hex not allowed: {hex}");
  }
}

#[test]
fn light_no_alpha_hex() {
  for (line, hex) in extract_hex_colors(LIGHT) {
    let body = hex.strip_prefix('#').unwrap_or(hex);
    assert_ne!(body.len(), 8, "line {line}: 8-digit alpha hex not allowed: {hex}");
  }
}
