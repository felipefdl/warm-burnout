mod common;

use common::{xcode_color, xcode_syntax_color, xcode_syntax_font};

const DARK: &str = include_str!("../xcode/Warm Burnout Dark.xccolortheme");
const LIGHT: &str = include_str!("../xcode/Warm Burnout Light.xccolortheme");

fn parse_plist(src: &str) -> plist::Value {
  let cursor = std::io::Cursor::new(src.as_bytes());
  plist::from_reader(cursor).expect("invalid plist")
}

// -- Valid plist structure --

#[test]
fn dark_is_valid_plist() {
  parse_plist(DARK);
}

#[test]
fn light_is_valid_plist() {
  parse_plist(LIGHT);
}

#[test]
fn dark_has_version_field() {
  let v = parse_plist(DARK);
  let dict = v.as_dictionary().unwrap();
  assert_eq!(
    dict.get("DVTFontAndColorVersion").and_then(|v| v.as_signed_integer()),
    Some(1)
  );
}

#[test]
fn light_has_version_field() {
  let v = parse_plist(LIGHT);
  let dict = v.as_dictionary().unwrap();
  assert_eq!(
    dict.get("DVTFontAndColorVersion").and_then(|v| v.as_signed_integer()),
    Some(1)
  );
}

// -- Canonical backgrounds --

#[test]
fn dark_background_is_canonical() {
  assert_eq!(xcode_color(DARK, "DVTSourceTextBackground"), "#1a1510");
}

#[test]
fn light_background_is_canonical() {
  assert_eq!(xcode_color(LIGHT, "DVTSourceTextBackground"), "#f5ede0");
}

// -- Canonical foregrounds --

#[test]
fn dark_foreground_is_canonical() {
  assert_eq!(xcode_syntax_color(DARK, "xcode.syntax.plain"), "#bfbdb6");
}

#[test]
fn light_foreground_is_canonical() {
  assert_eq!(xcode_syntax_color(LIGHT, "xcode.syntax.plain"), "#3a3630");
}

// -- Canonical cursor --

#[test]
fn dark_cursor_is_canonical() {
  assert_eq!(xcode_color(DARK, "DVTSourceTextInsertionPointColor"), "#f5c56e");
}

#[test]
fn light_cursor_is_canonical() {
  assert_eq!(xcode_color(LIGHT, "DVTSourceTextInsertionPointColor"), "#8a6600");
}

// -- No pure black/white backgrounds --

#[test]
fn no_pure_black_background() {
  let bg = xcode_color(DARK, "DVTSourceTextBackground");
  assert_ne!(bg, "#000000", "dark background must not be pure black");
}

#[test]
fn no_pure_white_background() {
  let bg = xcode_color(LIGHT, "DVTSourceTextBackground");
  assert_ne!(bg, "#ffffff", "light background must not be pure white");
}

// -- Syntax colors match canonical palette (dark) --

#[test]
fn dark_syntax_colors_match_palette() {
  let expected = [
    ("xcode.syntax.plain", "#bfbdb6"),
    ("xcode.syntax.keyword", "#ff8f40"),
    ("xcode.syntax.identifier.function", "#ffb454"),
    ("xcode.syntax.identifier.function.system", "#ec9878"),
    ("xcode.syntax.identifier.type", "#90aec0"),
    ("xcode.syntax.identifier.type.system", "#90aec0"),
    ("xcode.syntax.identifier.class", "#90aec0"),
    ("xcode.syntax.identifier.class.system", "#90aec0"),
    ("xcode.syntax.string", "#b4bc78"),
    ("xcode.syntax.character", "#b4bc78"),
    ("xcode.syntax.number", "#d4a8b8"),
    ("xcode.syntax.comment", "#b4a89c"),
    ("xcode.syntax.comment.doc", "#b4a89c"),
    ("xcode.syntax.comment.doc.keyword", "#b4a89c"),
    ("xcode.syntax.preprocessor", "#e6c08a"),
    ("xcode.syntax.attribute", "#e6c08a"),
    ("xcode.syntax.identifier.constant", "#d4a8b8"),
    ("xcode.syntax.identifier.constant.system", "#d4a8b8"),
    ("xcode.syntax.identifier.variable", "#ec9878"),
    ("xcode.syntax.identifier.variable.system", "#ec9878"),
    ("xcode.syntax.identifier.macro", "#e6c08a"),
    ("xcode.syntax.identifier.macro.system", "#e6c08a"),
    ("xcode.syntax.url", "#90aec0"),
    ("xcode.syntax.mark", "#b4a89c"),
    ("xcode.syntax.declaration.type", "#90aec0"),
    ("xcode.syntax.declaration.other", "#ffb454"),
    ("xcode.syntax.markup.code", "#bfbdb6"),
    ("xcode.syntax.markup.aside.kind", "#e6c08a"),
  ];
  for (key, color) in expected {
    assert_eq!(xcode_syntax_color(DARK, key), color, "dark syntax {key} mismatch");
  }
}

// -- Syntax colors match canonical palette (light) --

#[test]
fn light_syntax_colors_match_palette() {
  let expected = [
    ("xcode.syntax.plain", "#3a3630"),
    ("xcode.syntax.keyword", "#924800"),
    ("xcode.syntax.identifier.function", "#855700"),
    ("xcode.syntax.identifier.function.system", "#883850"),
    ("xcode.syntax.identifier.type", "#285464"),
    ("xcode.syntax.identifier.type.system", "#285464"),
    ("xcode.syntax.identifier.class", "#285464"),
    ("xcode.syntax.identifier.class.system", "#285464"),
    ("xcode.syntax.string", "#4d5c1a"),
    ("xcode.syntax.character", "#4d5c1a"),
    ("xcode.syntax.number", "#7e4060"),
    ("xcode.syntax.comment", "#544c40"),
    ("xcode.syntax.comment.doc", "#544c40"),
    ("xcode.syntax.comment.doc.keyword", "#544c40"),
    ("xcode.syntax.preprocessor", "#7a5a1c"),
    ("xcode.syntax.attribute", "#7a5a1c"),
    ("xcode.syntax.identifier.constant", "#7e4060"),
    ("xcode.syntax.identifier.constant.system", "#7e4060"),
    ("xcode.syntax.identifier.variable", "#883850"),
    ("xcode.syntax.identifier.variable.system", "#883850"),
    ("xcode.syntax.identifier.macro", "#7a5a1c"),
    ("xcode.syntax.identifier.macro.system", "#7a5a1c"),
    ("xcode.syntax.url", "#285464"),
    ("xcode.syntax.mark", "#544c40"),
    ("xcode.syntax.declaration.type", "#285464"),
    ("xcode.syntax.declaration.other", "#855700"),
    ("xcode.syntax.markup.code", "#3a3630"),
    ("xcode.syntax.markup.aside.kind", "#7a5a1c"),
  ];
  for (key, color) in expected {
    assert_eq!(xcode_syntax_color(LIGHT, key), color, "light syntax {key} mismatch");
  }
}

// -- Font style verification --

const BOLD_KEYS: &[&str] = &[
  "xcode.syntax.keyword",
  "xcode.syntax.comment.doc.keyword",
  "xcode.syntax.mark",
];

const ITALIC_KEYS: &[&str] = &[
  "xcode.syntax.comment",
  "xcode.syntax.comment.doc",
  "xcode.syntax.identifier.type",
  "xcode.syntax.identifier.type.system",
  "xcode.syntax.identifier.class",
  "xcode.syntax.identifier.class.system",
  "xcode.syntax.declaration.type",
];

#[test]
fn dark_bold_keys_use_bold_font() {
  for key in BOLD_KEYS {
    let font = xcode_syntax_font(DARK, key);
    assert!(font.contains("Bold"), "dark {key} should be bold, got: {font}");
  }
}

#[test]
fn dark_italic_keys_use_italic_font() {
  for key in ITALIC_KEYS {
    let font = xcode_syntax_font(DARK, key);
    assert!(font.contains("Italic"), "dark {key} should be italic, got: {font}");
  }
}

#[test]
fn light_bold_keys_use_bold_font() {
  for key in BOLD_KEYS {
    let font = xcode_syntax_font(LIGHT, key);
    assert!(font.contains("Bold"), "light {key} should be bold, got: {font}");
  }
}

#[test]
fn light_italic_keys_use_italic_font() {
  for key in ITALIC_KEYS {
    let font = xcode_syntax_font(LIGHT, key);
    assert!(font.contains("Italic"), "light {key} should be italic, got: {font}");
  }
}

// -- Both variants have same syntax keys (structural parity) --

#[test]
fn both_variants_have_same_syntax_keys() {
  let dark = parse_plist(DARK);
  let light = parse_plist(LIGHT);
  let dark_dict = dark.as_dictionary().unwrap();
  let light_dict = light.as_dictionary().unwrap();

  let dark_syntax = dark_dict
    .get("DVTSourceTextSyntaxColors")
    .and_then(|v| v.as_dictionary())
    .unwrap();
  let light_syntax = light_dict
    .get("DVTSourceTextSyntaxColors")
    .and_then(|v| v.as_dictionary())
    .unwrap();

  let dark_keys: Vec<&str> = dark_syntax.keys().map(String::as_str).collect();
  let light_keys: Vec<&str> = light_syntax.keys().map(String::as_str).collect();

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

#[test]
fn both_variants_have_same_font_keys() {
  let dark = parse_plist(DARK);
  let light = parse_plist(LIGHT);
  let dark_dict = dark.as_dictionary().unwrap();
  let light_dict = light.as_dictionary().unwrap();

  let dark_fonts = dark_dict
    .get("DVTSourceTextSyntaxFonts")
    .and_then(|v| v.as_dictionary())
    .unwrap();
  let light_fonts = light_dict
    .get("DVTSourceTextSyntaxFonts")
    .and_then(|v| v.as_dictionary())
    .unwrap();

  let dark_keys: Vec<&str> = dark_fonts.keys().map(String::as_str).collect();
  let light_keys: Vec<&str> = light_fonts.keys().map(String::as_str).collect();

  for key in &dark_keys {
    assert!(light_keys.contains(key), "dark has font key '{key}' but light does not");
  }
  for key in &light_keys {
    assert!(dark_keys.contains(key), "light has font key '{key}' but dark does not");
  }
}

// -- Console colors present --

#[test]
fn dark_has_console_colors() {
  let v = parse_plist(DARK);
  let dict = v.as_dictionary().unwrap();
  for key in [
    "DVTConsoleTextBackgroundColor",
    "DVTConsoleTextInsertionPointColor",
    "DVTConsoleTextSelectionColor",
    "DVTConsoleDebuggerInputTextColor",
    "DVTConsoleDebuggerOutputTextColor",
    "DVTConsoleDebuggerPromptTextColor",
    "DVTConsoleExectuableInputTextColor",
    "DVTConsoleExectuableOutputTextColor",
  ] {
    assert!(
      dict.get(key).and_then(|v| v.as_string()).is_some(),
      "dark missing console key: {key}"
    );
  }
}

#[test]
fn light_has_console_colors() {
  let v = parse_plist(LIGHT);
  let dict = v.as_dictionary().unwrap();
  for key in [
    "DVTConsoleTextBackgroundColor",
    "DVTConsoleTextInsertionPointColor",
    "DVTConsoleTextSelectionColor",
    "DVTConsoleDebuggerInputTextColor",
    "DVTConsoleDebuggerOutputTextColor",
    "DVTConsoleDebuggerPromptTextColor",
    "DVTConsoleExectuableInputTextColor",
    "DVTConsoleExectuableOutputTextColor",
  ] {
    assert!(
      dict.get(key).and_then(|v| v.as_string()).is_some(),
      "light missing console key: {key}"
    );
  }
}

// -- Console background matches editor background --

#[test]
fn dark_console_background_matches_editor() {
  let editor = xcode_color(DARK, "DVTSourceTextBackground");
  let console = xcode_color(DARK, "DVTConsoleTextBackgroundColor");
  assert_eq!(editor, console, "dark console bg should match editor bg");
}

#[test]
fn light_console_background_matches_editor() {
  let editor = xcode_color(LIGHT, "DVTSourceTextBackground");
  let console = xcode_color(LIGHT, "DVTConsoleTextBackgroundColor");
  assert_eq!(editor, console, "light console bg should match editor bg");
}
