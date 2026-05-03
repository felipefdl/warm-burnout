mod common;

use common::{jetbrains_attribute, jetbrains_color};
use serde_json::Value as JsonValue;

const DARK: &str = include_str!("../jetbrains/Warm-Burnout-Dark.xml");
const LIGHT: &str = include_str!("../jetbrains/Warm-Burnout-Light.xml");
const DARK_THEME: &str = include_str!("../jetbrains/Warm Burnout Islands Dark.theme.json");
const LIGHT_THEME: &str = include_str!("../jetbrains/Warm Burnout Islands Light.theme.json");
const PLUGIN_XML: &str = include_str!("../jetbrains/META-INF/plugin.xml");

// -- Valid XML structure --

#[test]
fn dark_has_xml_declaration() {
  assert!(DARK.starts_with("<?xml"), "dark .xml must start with XML declaration");
}

#[test]
fn light_has_xml_declaration() {
  assert!(LIGHT.starts_with("<?xml"), "light .xml must start with XML declaration");
}

// XML comments may not contain "--" except as the closing "-->". IntelliJ silently rejects
// editor schemes that fail to parse, which removes the theme from the picker and was the
// regression behind v1.1.0 not appearing after install. Catch it before it ships.
fn assert_no_bad_xml_comments(xml: &str, label: &str) {
  let mut idx = 0;
  while let Some(open) = xml[idx..].find("<!--") {
    let start = idx + open + 4;
    let close = xml[start..]
      .find("-->")
      .unwrap_or_else(|| panic!("{label} has unterminated XML comment"));
    let body = &xml[start..start + close];
    assert!(
      !body.contains("--"),
      "{label} editor scheme has invalid XML comment containing '--' at byte offset {start}: {body:?}"
    );
    idx = start + close + 3;
  }
}

#[test]
fn dark_xml_comments_are_well_formed() {
  assert_no_bad_xml_comments(DARK, "dark");
}

#[test]
fn light_xml_comments_are_well_formed() {
  assert_no_bad_xml_comments(LIGHT, "light");
}

#[test]
fn dark_has_scheme_element() {
  assert!(DARK.contains("<scheme"), "dark .xml must have <scheme> element");
  assert!(DARK.contains("</scheme>"), "dark .xml must have closing </scheme>");
}

#[test]
fn light_has_scheme_element() {
  assert!(LIGHT.contains("<scheme"), "light .xml must have <scheme> element");
  assert!(LIGHT.contains("</scheme>"), "light .xml must have closing </scheme>");
}

// -- Scheme names and parent schemes --

#[test]
fn dark_scheme_name() {
  assert!(
    DARK.contains("name=\"Warm Burnout Dark\""),
    "dark scheme name must be 'Warm Burnout Dark'"
  );
}

#[test]
fn light_scheme_name() {
  assert!(
    LIGHT.contains("name=\"Warm Burnout Light\""),
    "light scheme name must be 'Warm Burnout Light'"
  );
}

#[test]
fn dark_parent_scheme() {
  assert!(
    DARK.contains("parent_scheme=\"Darcula\""),
    "dark must use Darcula as parent scheme"
  );
}

#[test]
fn light_parent_scheme() {
  assert!(
    LIGHT.contains("parent_scheme=\"Default\""),
    "light must use Default as parent scheme"
  );
}

// -- Canonical backgrounds --

#[test]
fn dark_background_is_canonical() {
  assert_eq!(jetbrains_attribute(DARK, "TEXT", "BACKGROUND"), "#1a1510");
}

#[test]
fn light_background_is_canonical() {
  assert_eq!(jetbrains_attribute(LIGHT, "TEXT", "BACKGROUND"), "#f5ede0");
}

// -- Canonical foregrounds --

#[test]
fn dark_foreground_is_canonical() {
  assert_eq!(jetbrains_attribute(DARK, "TEXT", "FOREGROUND"), "#bfbdb6");
}

#[test]
fn light_foreground_is_canonical() {
  assert_eq!(jetbrains_attribute(LIGHT, "TEXT", "FOREGROUND"), "#3a3630");
}

// -- Canonical cursor --

#[test]
fn dark_cursor_is_canonical() {
  assert_eq!(jetbrains_color(DARK, "CARET_COLOR"), "#f5c56e");
}

#[test]
fn light_cursor_is_canonical() {
  assert_eq!(jetbrains_color(LIGHT, "CARET_COLOR"), "#8a6600");
}

// -- No pure black/white backgrounds --

#[test]
fn no_pure_black_background() {
  let bg = jetbrains_attribute(DARK, "TEXT", "BACKGROUND");
  assert_ne!(bg, "#000000", "dark background must not be pure black");
}

#[test]
fn no_pure_white_background() {
  let bg = jetbrains_attribute(LIGHT, "TEXT", "BACKGROUND");
  assert_ne!(bg, "#ffffff", "light background must not be pure white");
}

// -- Dark syntax colors match canonical palette --

#[test]
fn dark_syntax_colors_match_palette() {
  let expected: &[(&str, &str, &str)] = &[
    ("TEXT", "FOREGROUND", "#bfbdb6"),
    ("DEFAULT_KEYWORD", "FOREGROUND", "#ff8f40"),
    ("DEFAULT_FUNCTION_DECLARATION", "FOREGROUND", "#ffb454"),
    ("DEFAULT_FUNCTION_CALL", "FOREGROUND", "#ffb454"),
    ("DEFAULT_CLASS_NAME", "FOREGROUND", "#90aec0"),
    ("DEFAULT_INTERFACE_NAME", "FOREGROUND", "#90aec0"),
    ("DEFAULT_STRING", "FOREGROUND", "#b4bc78"),
    ("DEFAULT_NUMBER", "FOREGROUND", "#d4a8b8"),
    ("DEFAULT_CONSTANT", "FOREGROUND", "#d4a8b8"),
    ("DEFAULT_OPERATION_SIGN", "FOREGROUND", "#f29668"),
    ("DEFAULT_LINE_COMMENT", "FOREGROUND", "#b4a89c"),
    ("DEFAULT_BLOCK_COMMENT", "FOREGROUND", "#b4a89c"),
    ("DEFAULT_DOC_COMMENT", "FOREGROUND", "#b4a89c"),
    ("DEFAULT_METADATA", "FOREGROUND", "#e6c08a"),
    ("DEFAULT_MARKUP_TAG", "FOREGROUND", "#dc9e92"),
    ("DEFAULT_INSTANCE_FIELD", "FOREGROUND", "#ec9878"),
    ("DEFAULT_STATIC_METHOD", "FOREGROUND", "#ec9878"),
    ("DEFAULT_VALID_STRING_ESCAPE", "FOREGROUND", "#96b898"),
    ("DEFAULT_INVALID_STRING_ESCAPE", "FOREGROUND", "#f49090"),
    ("CSS.PROPERTY_NAME", "FOREGROUND", "#deb074"),
    ("HTML_TAG_NAME", "FOREGROUND", "#dc9e92"),
  ];
  for (attr, prop, color) in expected {
    assert_eq!(
      jetbrains_attribute(DARK, attr, prop),
      *color,
      "dark {attr}.{prop} mismatch"
    );
  }
}

// -- Light syntax colors match canonical palette --

#[test]
fn light_syntax_colors_match_palette() {
  let expected: &[(&str, &str, &str)] = &[
    ("TEXT", "FOREGROUND", "#3a3630"),
    ("DEFAULT_KEYWORD", "FOREGROUND", "#924800"),
    ("DEFAULT_FUNCTION_DECLARATION", "FOREGROUND", "#855700"),
    ("DEFAULT_FUNCTION_CALL", "FOREGROUND", "#855700"),
    ("DEFAULT_CLASS_NAME", "FOREGROUND", "#285464"),
    ("DEFAULT_INTERFACE_NAME", "FOREGROUND", "#285464"),
    ("DEFAULT_STRING", "FOREGROUND", "#4d5c1a"),
    ("DEFAULT_NUMBER", "FOREGROUND", "#7e4060"),
    ("DEFAULT_CONSTANT", "FOREGROUND", "#7e4060"),
    ("DEFAULT_OPERATION_SIGN", "FOREGROUND", "#8f4418"),
    ("DEFAULT_LINE_COMMENT", "FOREGROUND", "#544c40"),
    ("DEFAULT_BLOCK_COMMENT", "FOREGROUND", "#544c40"),
    ("DEFAULT_DOC_COMMENT", "FOREGROUND", "#544c40"),
    ("DEFAULT_METADATA", "FOREGROUND", "#7a5a1c"),
    ("DEFAULT_MARKUP_TAG", "FOREGROUND", "#8e4632"),
    ("DEFAULT_INSTANCE_FIELD", "FOREGROUND", "#883850"),
    ("DEFAULT_STATIC_METHOD", "FOREGROUND", "#883850"),
    ("DEFAULT_VALID_STRING_ESCAPE", "FOREGROUND", "#286a48"),
    ("DEFAULT_INVALID_STRING_ESCAPE", "FOREGROUND", "#b03434"),
    ("CSS.PROPERTY_NAME", "FOREGROUND", "#74501c"),
    ("HTML_TAG_NAME", "FOREGROUND", "#8e4632"),
  ];
  for (attr, prop, color) in expected {
    assert_eq!(
      jetbrains_attribute(LIGHT, attr, prop),
      *color,
      "light {attr}.{prop} mismatch"
    );
  }
}

// -- Font style verification --

#[test]
fn dark_bold_attributes() {
  let bold_attrs = ["DEFAULT_KEYWORD", "DEFAULT_DOC_COMMENT_TAG"];
  for attr in bold_attrs {
    assert_eq!(
      jetbrains_attribute(DARK, attr, "FONT_TYPE"),
      "1",
      "dark {attr} should be bold (FONT_TYPE=1)"
    );
  }
}

#[test]
fn dark_italic_attributes() {
  let italic_attrs = [
    "DEFAULT_CLASS_NAME",
    "DEFAULT_INTERFACE_NAME",
    "DEFAULT_LINE_COMMENT",
    "DEFAULT_BLOCK_COMMENT",
    "DEFAULT_DOC_COMMENT",
  ];
  for attr in italic_attrs {
    assert_eq!(
      jetbrains_attribute(DARK, attr, "FONT_TYPE"),
      "2",
      "dark {attr} should be italic (FONT_TYPE=2)"
    );
  }
}

#[test]
fn light_bold_attributes() {
  let bold_attrs = ["DEFAULT_KEYWORD", "DEFAULT_DOC_COMMENT_TAG"];
  for attr in bold_attrs {
    assert_eq!(
      jetbrains_attribute(LIGHT, attr, "FONT_TYPE"),
      "1",
      "light {attr} should be bold (FONT_TYPE=1)"
    );
  }
}

#[test]
fn light_italic_attributes() {
  let italic_attrs = [
    "DEFAULT_CLASS_NAME",
    "DEFAULT_INTERFACE_NAME",
    "DEFAULT_LINE_COMMENT",
    "DEFAULT_BLOCK_COMMENT",
    "DEFAULT_DOC_COMMENT",
  ];
  for attr in italic_attrs {
    assert_eq!(
      jetbrains_attribute(LIGHT, attr, "FONT_TYPE"),
      "2",
      "light {attr} should be italic (FONT_TYPE=2)"
    );
  }
}

// -- Structural parity: both variants have the same attribute names --

fn extract_attribute_names(src: &str) -> Vec<String> {
  let attrs_start = src.find("<attributes>").expect("missing <attributes>");
  let attrs_end = src.find("</attributes>").expect("missing </attributes>");
  let section = &src[attrs_start..attrs_end];
  section
    .lines()
    .filter_map(|line| {
      let trimmed = line.trim();
      // Top-level attribute options: <option name="..."> (not self-closing, no value=)
      if trimmed.starts_with("<option name=\"") && !trimmed.contains("value=\"") {
        let start = trimmed.find("name=\"").unwrap() + 6;
        let end = trimmed[start..].find('"').unwrap() + start;
        Some(trimmed[start..end].to_string())
      } else {
        None
      }
    })
    .collect()
}

fn extract_color_names(src: &str) -> Vec<String> {
  let colors_start = src.find("<colors>").expect("missing <colors>");
  let colors_end = src.find("</colors>").expect("missing </colors>");
  let section = &src[colors_start..colors_end];
  section
    .lines()
    .filter_map(|line| {
      let trimmed = line.trim();
      if trimmed.starts_with("<option name=\"") && trimmed.contains("value=\"") {
        let start = trimmed.find("name=\"").unwrap() + 6;
        let end = trimmed[start..].find('"').unwrap() + start;
        Some(trimmed[start..end].to_string())
      } else {
        None
      }
    })
    .collect()
}

#[test]
fn both_variants_have_same_attributes() {
  let dark_attrs = extract_attribute_names(DARK);
  let light_attrs = extract_attribute_names(LIGHT);

  for attr in &dark_attrs {
    assert!(
      light_attrs.contains(attr),
      "dark has attribute '{attr}' but light does not"
    );
  }
  for attr in &light_attrs {
    assert!(
      dark_attrs.contains(attr),
      "light has attribute '{attr}' but dark does not"
    );
  }
}

#[test]
fn both_variants_have_same_colors() {
  let dark_colors = extract_color_names(DARK);
  let light_colors = extract_color_names(LIGHT);

  for color in &dark_colors {
    assert!(
      light_colors.contains(color),
      "dark has color '{color}' but light does not"
    );
  }
  for color in &light_colors {
    assert!(
      dark_colors.contains(color),
      "light has color '{color}' but dark does not"
    );
  }
}

// -- .theme.json validation --

fn parse_theme(src: &str) -> JsonValue {
  serde_json::from_str(src).expect("invalid JSON in .theme.json")
}

#[test]
fn dark_theme_json_is_valid() {
  parse_theme(DARK_THEME);
}

#[test]
fn light_theme_json_is_valid() {
  parse_theme(LIGHT_THEME);
}

#[test]
fn dark_theme_json_name() {
  let v = parse_theme(DARK_THEME);
  assert_eq!(v["name"].as_str(), Some("Warm Burnout Islands Dark"));
}

#[test]
fn light_theme_json_name() {
  let v = parse_theme(LIGHT_THEME);
  assert_eq!(v["name"].as_str(), Some("Warm Burnout Islands Light"));
}

#[test]
fn dark_theme_json_is_dark() {
  let v = parse_theme(DARK_THEME);
  assert_eq!(v["dark"].as_bool(), Some(true));
}

#[test]
fn light_theme_json_is_light() {
  let v = parse_theme(LIGHT_THEME);
  assert_eq!(v["dark"].as_bool(), Some(false));
}

#[test]
fn dark_theme_json_references_editor_scheme() {
  let v = parse_theme(DARK_THEME);
  let scheme = v["editorScheme"].as_str().unwrap();
  assert!(
    scheme.contains("Warm-Burnout-Dark.xml"),
    "dark .theme.json must reference the dark editor scheme .xml file"
  );
}

#[test]
fn light_theme_json_references_editor_scheme() {
  let v = parse_theme(LIGHT_THEME);
  let scheme = v["editorScheme"].as_str().unwrap();
  assert!(
    scheme.contains("Warm-Burnout-Light.xml"),
    "light .theme.json must reference the light editor scheme .xml file"
  );
}

#[test]
fn dark_theme_json_has_ui_section() {
  let v = parse_theme(DARK_THEME);
  assert!(v["ui"].is_object(), "dark .theme.json must have 'ui' object");
}

#[test]
fn light_theme_json_has_ui_section() {
  let v = parse_theme(LIGHT_THEME);
  assert!(v["ui"].is_object(), "light .theme.json must have 'ui' object");
}

#[test]
fn dark_theme_json_has_required_ui_components() {
  let v = parse_theme(DARK_THEME);
  let ui = v["ui"].as_object().unwrap();
  let required = [
    "*",
    "Editor",
    "EditorTabs",
    "Panel",
    "Tree",
    "ToolWindow",
    "StatusBar",
    "Popup",
    "Menu",
  ];
  for key in required {
    assert!(ui.contains_key(key), "dark .theme.json missing UI component: {key}");
  }
}

#[test]
fn light_theme_json_has_required_ui_components() {
  let v = parse_theme(LIGHT_THEME);
  let ui = v["ui"].as_object().unwrap();
  let required = [
    "*",
    "Editor",
    "EditorTabs",
    "Panel",
    "Tree",
    "ToolWindow",
    "StatusBar",
    "Popup",
    "Menu",
  ];
  for key in required {
    assert!(ui.contains_key(key), "light .theme.json missing UI component: {key}");
  }
}

#[test]
fn theme_json_ui_keys_parity() {
  let dark = parse_theme(DARK_THEME);
  let light = parse_theme(LIGHT_THEME);
  let dark_keys: Vec<&str> = dark["ui"].as_object().unwrap().keys().map(String::as_str).collect();
  let light_keys: Vec<&str> = light["ui"].as_object().unwrap().keys().map(String::as_str).collect();

  for key in &dark_keys {
    assert!(
      light_keys.contains(key),
      "dark .theme.json has UI key '{key}' but light does not"
    );
  }
  for key in &light_keys {
    assert!(
      dark_keys.contains(key),
      "light .theme.json has UI key '{key}' but dark does not"
    );
  }
}

// -- Dark .theme.json uses canonical palette colors --

#[test]
fn dark_theme_json_editor_background() {
  let v = parse_theme(DARK_THEME);
  assert_eq!(v["ui"]["Editor"]["background"].as_str(), Some("1a1510"));
}

#[test]
fn dark_theme_json_sidebar_background() {
  let v = parse_theme(DARK_THEME);
  assert_eq!(v["ui"]["Panel"]["background"].as_str(), Some("14120f"));
}

#[test]
fn dark_theme_json_accent_color() {
  let v = parse_theme(DARK_THEME);
  assert_eq!(v["ui"]["EditorTabs"]["underlineColor"].as_str(), Some("b8522e"));
}

// -- Light .theme.json uses canonical palette colors --

#[test]
fn light_theme_json_editor_background() {
  let v = parse_theme(LIGHT_THEME);
  assert_eq!(v["ui"]["Editor"]["background"].as_str(), Some("f5ede0"));
}

#[test]
fn light_theme_json_sidebar_background() {
  let v = parse_theme(LIGHT_THEME);
  assert_eq!(v["ui"]["Panel"]["background"].as_str(), Some("ede6da"));
}

#[test]
fn light_theme_json_accent_color() {
  let v = parse_theme(LIGHT_THEME);
  assert_eq!(v["ui"]["EditorTabs"]["underlineColor"].as_str(), Some("b8522e"));
}

// -- plugin.xml validation --

#[test]
fn plugin_xml_has_id() {
  assert!(
    PLUGIN_XML.contains("<id>com.warm-burnout.theme</id>"),
    "plugin.xml must have plugin ID"
  );
}

#[test]
fn plugin_xml_has_name() {
  assert!(
    PLUGIN_XML.contains("<name>Warm Burnout</name>"),
    "plugin.xml must have plugin name"
  );
}

#[test]
fn plugin_xml_registers_dark_theme() {
  assert!(
    PLUGIN_XML.contains("Warm Burnout Islands Dark.theme.json"),
    "plugin.xml must register dark theme"
  );
}

#[test]
fn plugin_xml_registers_light_theme() {
  assert!(
    PLUGIN_XML.contains("Warm Burnout Islands Light.theme.json"),
    "plugin.xml must register light theme"
  );
}

// -- Editor scheme overrides prevent parent blue bleed --

#[test]
fn dark_search_result_uses_warm_highlight() {
  let bg = jetbrains_attribute(DARK, "SEARCH_RESULT_ATTRIBUTES", "BACKGROUND");
  assert_eq!(bg, "#4c4126", "dark search result should use warm gold highlight");
}

#[test]
fn light_search_result_uses_warm_highlight() {
  let bg = jetbrains_attribute(LIGHT, "SEARCH_RESULT_ATTRIBUTES", "BACKGROUND");
  assert_eq!(bg, "#e0c890", "light search result should use warm gold highlight");
}

#[test]
fn dark_has_breadcrumbs_override() {
  let fg = jetbrains_attribute(DARK, "BREADCRUMBS_DEFAULT", "FOREGROUND");
  assert_eq!(fg, "#b4a89c", "dark breadcrumbs should use warm stone color");
}

#[test]
fn light_has_breadcrumbs_override() {
  let fg = jetbrains_attribute(LIGHT, "BREADCRUMBS_DEFAULT", "FOREGROUND");
  assert_eq!(fg, "#544c40", "light breadcrumbs should use warm stone color");
}

// -- Islands theme validation --

#[test]
fn dark_theme_has_island_keys() {
  let v = parse_theme(DARK_THEME);
  let island = &v["ui"]["Island"];
  assert!(island.is_object(), "dark theme must have Island section");
  assert!(island["arc"].is_string(), "missing Island.arc");
  assert!(island["borderWidth"].is_string(), "missing Island.borderWidth");
  assert!(island["borderColor"].is_string(), "missing Island.borderColor");
}

#[test]
fn light_theme_has_island_keys() {
  let v = parse_theme(LIGHT_THEME);
  let island = &v["ui"]["Island"];
  assert!(island.is_object(), "light theme must have Island section");
  assert!(island["arc"].is_string(), "missing Island.arc");
  assert!(island["borderWidth"].is_string(), "missing Island.borderWidth");
  assert!(island["borderColor"].is_string(), "missing Island.borderColor");
}

#[test]
fn dark_theme_has_main_window_background() {
  let v = parse_theme(DARK_THEME);
  assert!(
    v["ui"]["MainWindow"]["background"].is_string(),
    "dark theme must have MainWindow.background (canvas color)"
  );
}

#[test]
fn light_theme_has_main_window_background() {
  let v = parse_theme(LIGHT_THEME);
  assert!(
    v["ui"]["MainWindow"]["background"].is_string(),
    "light theme must have MainWindow.background (canvas color)"
  );
}

#[test]
fn dark_theme_transparent_stripe_border() {
  let v = parse_theme(DARK_THEME);
  let border = v["ui"]["ToolWindow"]["Stripe"]["borderColor"].as_str().unwrap();
  assert!(
    border.ends_with("00") || border == "00000000",
    "dark ToolWindow.Stripe.borderColor must be transparent"
  );
}

#[test]
fn light_theme_transparent_stripe_border() {
  let v = parse_theme(LIGHT_THEME);
  let border = v["ui"]["ToolWindow"]["Stripe"]["borderColor"].as_str().unwrap();
  assert!(
    border.ends_with("00") || border == "00000000",
    "light ToolWindow.Stripe.borderColor must be transparent"
  );
}

// -- Issue #5: TODO highlight must not render bold (was reported as too loud) --

#[test]
fn dark_todo_not_bold() {
  assert_ne!(
    jetbrains_attribute(DARK, "TODO_DEFAULT_ATTRIBUTES", "FONT_TYPE"),
    "1",
    "TODO_DEFAULT_ATTRIBUTES.FONT_TYPE must not be bold (1); reported as too loud in #5"
  );
}

#[test]
fn light_todo_not_bold() {
  assert_ne!(
    jetbrains_attribute(LIGHT, "TODO_DEFAULT_ATTRIBUTES", "FONT_TYPE"),
    "1",
    "TODO_DEFAULT_ATTRIBUTES.FONT_TYPE must not be bold (1); reported as too loud in #5"
  );
}

// -- Issue #5: explicit overrides for surfaces that misrender under Islands inheritance --

const ISSUE_5_SURFACES: &[(&str, &[&str])] = &[
  ("Editor", &["background"]),
  ("Popup", &["background"]),
  ("CompletionPopup", &["background"]),
  ("ToolTip", &["background", "foreground", "borderColor"]),
  ("Island", &["borderColor"]),
  ("TitlePane", &["background", "inactiveBackground"]),
];

fn assert_issue_5_surfaces(theme_src: &str, label: &str) {
  let v = parse_theme(theme_src);
  for (component, keys) in ISSUE_5_SURFACES {
    for key in *keys {
      assert!(
        v["ui"][component][key].is_string(),
        "{label} theme.json missing override {component}.{key}; required to keep #5 surfaces from drifting under parentTheme inheritance"
      );
    }
  }
}

#[test]
fn dark_overrides_issue_5_surfaces() {
  assert_issue_5_surfaces(DARK_THEME, "dark");
}

#[test]
fn light_overrides_issue_5_surfaces() {
  assert_issue_5_surfaces(LIGHT_THEME, "light");
}

// -- Issue #8: ReSharper / Rider C# attribute coverage sentinel --

const RESHARPER_CSHARP_SENTINEL: &[&str] = &[
  "ReSharper.CSHARP_LOCAL_VARIABLE_IDENTIFIER",
  "ReSharper.CSHARP_PARAMETER_IDENTIFIER",
  "ReSharper.CSHARP_FIELD_IDENTIFIER",
  "ReSharper.CSHARP_PROPERTY_IDENTIFIER",
  "ReSharper.CSHARP_METHOD_IDENTIFIER",
  "ReSharper.CSHARP_CLASS_IDENTIFIER",
  "ReSharper.CSHARP_INTERFACE_IDENTIFIER",
  "ReSharper.CSHARP_STRUCT_IDENTIFIER",
  "ReSharper.CSHARP_ENUM_IDENTIFIER",
  "ReSharper.CSHARP_DELEGATE_IDENTIFIER",
  "ReSharper.CSHARP_RECORD_IDENTIFIER",
  "ReSharper.CSHARP_TYPE_PARAMETER_IDENTIFIER",
  "ReSharper.CSHARP_NAMESPACE_IDENTIFIER",
  "ReSharper.CSHARP_KEYWORD",
  "ReSharper.CSHARP_CONSTANT_IDENTIFIER",
  "ReSharper.CSHARP_EVENT_IDENTIFIER",
  "ReSharper.CSHARP_EXTENSION_METHOD_IDENTIFIER",
];

fn assert_resharper_coverage(scheme: &str, label: &str) {
  for key in RESHARPER_CSHARP_SENTINEL {
    let needle = format!("<option name=\"{key}\">");
    assert!(
      scheme.contains(&needle),
      "{label} editor scheme missing override for {key}; required to fix Rider/C# gray fallback (#8)"
    );
  }
}

#[test]
fn dark_covers_resharper_csharp_attributes() {
  assert_resharper_coverage(DARK, "dark");
}

#[test]
fn light_covers_resharper_csharp_attributes() {
  assert_resharper_coverage(LIGHT, "light");
}

// -- Generic identifier coverage so Kotlin, Java, JS, Python, etc. do not
//    fall through to the parent scheme's teal/blue defaults. --

const IDENTIFIER_COVERAGE_SENTINEL: &[&str] = &[
  "DEFAULT_LOCAL_VARIABLE",
  "DEFAULT_REASSIGNED_LOCAL_VARIABLE",
  "DEFAULT_PARAMETER",
  "DEFAULT_REASSIGNED_PARAMETER",
  "DEFAULT_GLOBAL_VARIABLE",
  "DEFAULT_INSTANCE_METHOD",
  "DEFAULT_PREDEFINED_SYMBOL",
  "DEFAULT_LABEL",
  "DEFAULT_TYPE_PARAMETER_NAME",
  "DEFAULT_ENUM_NAME",
  "DEFAULT_ABSTRACT_CLASS_NAME",
];

fn assert_identifier_coverage(scheme: &str, label: &str) {
  for key in IDENTIFIER_COVERAGE_SENTINEL {
    let needle = format!("<option name=\"{key}\">");
    assert!(
      scheme.contains(&needle),
      "{label} editor scheme missing override for {key}; required so language plugins do not inherit teal/blue defaults"
    );
  }
}

#[test]
fn dark_covers_default_identifiers() {
  assert_identifier_coverage(DARK, "dark");
}

#[test]
fn light_covers_default_identifiers() {
  assert_identifier_coverage(LIGHT, "light");
}

// -- Per-language baseAttributes inheritance, ported from Catppuccin's editor.tera.
//    Each language plugin (Bash, Go, PHP, Python, Rust, Ruby, Swift, JS, TS, Dart, etc.)
//    declares its own attribute keys and falls back to the parent theme's teal/blue defaults
//    unless we explicitly point them at our DEFAULT_* customizations. --

const LANGUAGE_INHERITANCE_SENTINEL: &[(&str, &str)] = &[
  ("BASH.FUNCTION_CALL", "DEFAULT_FUNCTION_CALL"),
  ("BASH.HERE_DOC_END", "DEFAULT_KEYWORD"),
  ("BASH.SHEBANG", "DEFAULT_LINE_COMMENT"),
  ("DART_FAT_ARROW", "DEFAULT_OPERATION_SIGN"),
  ("DART_LOCAL_FUNCTION_REFERENCE", "DEFAULT_FUNCTION_CALL"),
  ("DART_TYPE_NAME_DYNAMIC", "DEFAULT_KEYWORD"),
  ("GO_BUILTIN_FUNCTION_CALL", "DEFAULT_FUNCTION_CALL"),
  ("GO_BUILTIN_TYPE", "DEFAULT_CLASS_NAME"),
  ("GO_BUILTIN_VARIABLE", "DEFAULT_LOCAL_VARIABLE"),
  ("GO_PACKAGE", "DEFAULT_LOCAL_VARIABLE"),
  ("HTML_ATTRIBUTE_VALUE", "DEFAULT_STRING"),
  ("INI.SECTION", "DEFAULT_MARKUP_TAG"),
  ("JS.DECORATOR", "DEFAULT_METADATA"),
  ("JS.GLOBAL_FUNCTION", "DEFAULT_FUNCTION_DECLARATION"),
  ("JS.GLOBAL_VARIABLE", "DEFAULT_LOCAL_VARIABLE"),
  ("JS.LOCAL_VARIABLE", "DEFAULT_LOCAL_VARIABLE"),
  ("JS.PARAMETER", "DEFAULT_PARAMETER"),
  ("JS.REGEXP", "DEFAULT_VALID_STRING_ESCAPE"),
  ("JSON.PROPERTY_KEY", "DEFAULT_INSTANCE_FIELD"),
  ("OC.METHOD_DECLARATION", "DEFAULT_INSTANCE_METHOD"),
  ("OC.PROTOCOL_REFERENCE", "DEFAULT_INTERFACE_NAME"),
  ("PHP_INSTANCE_FIELD", "DEFAULT_INSTANCE_FIELD"),
  ("PHP_INTERFACE", "DEFAULT_INTERFACE_NAME"),
  ("PHP_PARAMETER", "DEFAULT_PARAMETER"),
  ("PHP_VAR", "DEFAULT_LOCAL_VARIABLE"),
  ("PROPERTIES.KEY", "DEFAULT_INSTANCE_FIELD"),
  ("PUPPET_KEYWORD", "DEFAULT_KEYWORD"),
  ("PY.ANNOTATION", "DEFAULT_METADATA"),
  ("PY.BUILTIN_NAME", "DEFAULT_LOCAL_VARIABLE"),
  ("PY.DECORATOR", "DEFAULT_METADATA"),
  ("PY.KEYWORD", "DEFAULT_KEYWORD"),
  ("PY.KEYWORD_ARGUMENT", "DEFAULT_PARAMETER"),
  ("PY.SELF_PARAMETER", "DEFAULT_PARAMETER"),
  ("PY.STRING", "DEFAULT_STRING"),
  ("RUBY_LOCAL_VAR_ID", "DEFAULT_LOCAL_VARIABLE"),
  ("RUBY_METHOD_NAME", "DEFAULT_INSTANCE_METHOD"),
  ("RUBY_PARAMETER_ID", "DEFAULT_PARAMETER"),
  ("SWIFT_EXTERNAL_PARAMETER", "DEFAULT_PARAMETER"),
  ("TS.MODULE_NAME", "DEFAULT_LOCAL_VARIABLE"),
  ("TS.TYPE_GUARD", "DEFAULT_KEYWORD"),
  ("TS.TYPE_PARAMETER", "DEFAULT_TYPE_PARAMETER_NAME"),
  ("XML_ATTRIBUTE_NAME", "DEFAULT_MARKUP_ATTRIBUTE"),
  ("XML_TAG_NAME", "DEFAULT_MARKUP_TAG"),
  ("YAML_ANCHOR", "DEFAULT_LABEL"),
  ("YAML_SCALAR_KEY", "DEFAULT_INSTANCE_FIELD"),
  ("org.rust.FUNCTION", "DEFAULT_FUNCTION_DECLARATION"),
  ("org.rust.FUNCTION_CALL", "DEFAULT_FUNCTION_CALL"),
  ("org.rust.LIFETIME", "DEFAULT_LABEL"),
  ("org.rust.MACRO", "DEFAULT_FUNCTION_CALL"),
  ("org.rust.METHOD", "DEFAULT_INSTANCE_METHOD"),
  ("org.rust.PARAMETER", "DEFAULT_PARAMETER"),
  ("org.rust.SELF_PARAMETER", "DEFAULT_PARAMETER"),
  ("org.rust.STRUCT", "DEFAULT_CLASS_NAME"),
  ("org.rust.TRAIT", "DEFAULT_INTERFACE_NAME"),
  ("org.rust.TYPE_PARAMETER", "DEFAULT_TYPE_PARAMETER_NAME"),
  ("org.rust.VARIABLE", "DEFAULT_LOCAL_VARIABLE"),
  ("org.toml.KEY", "DEFAULT_INSTANCE_FIELD"),
];

fn assert_language_inheritance(scheme: &str, label: &str) {
  for (key, target) in LANGUAGE_INHERITANCE_SENTINEL {
    let needle = format!("<option name=\"{key}\" baseAttributes=\"{target}\"/>");
    assert!(
      scheme.contains(&needle),
      "{label} editor scheme missing inheritance: expected `{needle}`. Without this the language plugin falls back to the parent theme."
    );
  }
}

#[test]
fn dark_covers_language_inheritance() {
  assert_language_inheritance(DARK, "dark");
}

#[test]
fn light_covers_language_inheritance() {
  assert_language_inheritance(LIGHT, "light");
}

// -- Compose / annotation / rainbow overrides. Without these, Compose function calls inherit
//    from Darcula's special Compose attribute, annotations render in Darcula's yellow, and
//    Android Studio's semantic-highlighting rainbow colors leak in for every identifier. --

#[test]
fn dark_overrides_composable_call_attribute() {
  assert!(
    DARK.contains("<option name=\"ComposableCallTextAttributes\" baseAttributes=\"DEFAULT_FUNCTION_CALL\"/>"),
    "dark scheme must override ComposableCallTextAttributes so @Composable calls render as DEFAULT_FUNCTION_CALL"
  );
}

#[test]
fn light_overrides_composable_call_attribute() {
  assert!(
    LIGHT.contains("<option name=\"ComposableCallTextAttributes\" baseAttributes=\"DEFAULT_FUNCTION_CALL\"/>"),
    "light scheme must override ComposableCallTextAttributes so @Composable calls render as DEFAULT_FUNCTION_CALL"
  );
}

#[test]
fn dark_overrides_annotation_name() {
  assert_eq!(
    jetbrains_attribute(DARK, "ANNOTATION_NAME_ATTRIBUTES", "FOREGROUND"),
    "#e6c08a",
    "dark annotation name must use the decorator color"
  );
  assert_eq!(
    jetbrains_attribute(DARK, "KOTLIN_ANNOTATION", "FOREGROUND"),
    "#e6c08a",
    "dark Kotlin annotation must use the decorator color"
  );
}

#[test]
fn light_overrides_annotation_name() {
  assert_eq!(
    jetbrains_attribute(LIGHT, "ANNOTATION_NAME_ATTRIBUTES", "FOREGROUND"),
    "#7a5a1c",
    "light annotation name must use the decorator color"
  );
}

#[test]
fn dark_collapses_rainbow_palette() {
  for slot in 0..5 {
    let key = format!("RAINBOW_COLOR{slot}");
    assert_eq!(
      jetbrains_attribute(DARK, &key, "FOREGROUND"),
      "#bfbdb6",
      "dark {key} must collapse to foreground so semantic highlighting stays consistent"
    );
  }
}

#[test]
fn dark_collapses_round_brackets_rainbow() {
  for slot in 0..5 {
    let key = format!("ROUND_BRACKETS_RAINBOW_COLOR{slot}");
    assert_eq!(
      jetbrains_attribute(DARK, &key, "FOREGROUND"),
      "#bfbdb6",
      "dark {key} must collapse to foreground"
    );
  }
}

#[test]
fn light_collapses_rainbow_palette() {
  for slot in 0..5 {
    let key = format!("RAINBOW_COLOR{slot}");
    assert_eq!(
      jetbrains_attribute(LIGHT, &key, "FOREGROUND"),
      "#3a3630",
      "light {key} must collapse to foreground"
    );
  }
}
