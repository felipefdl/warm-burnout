mod common;

use common::{extract_hex_colors, hex_to_lower, is_valid_hex};
use serde_yml::Value;

const THEME: &str = include_str!("../eza/theme.yml");

fn parse_theme() -> Value {
  serde_yml::from_str(THEME).expect("invalid YAML")
}

fn get_section<'a>(root: &'a Value, name: &str) -> &'a Value {
  root.get(name).unwrap_or_else(|| panic!("missing section: {name}"))
}

fn get_foreground(section: &Value, key: &str, section_name: &str) -> String {
  let entry = section
    .get(key)
    .unwrap_or_else(|| panic!("{section_name}.{key}: missing"));
  hex_to_lower(
    entry
      .get("foreground")
      .and_then(|v| v.as_str())
      .unwrap_or_else(|| panic!("{section_name}.{key}: missing foreground")),
  )
}

fn top_foreground(section: &Value, section_name: &str) -> String {
  hex_to_lower(
    section
      .get("foreground")
      .and_then(|v| v.as_str())
      .unwrap_or_else(|| panic!("{section_name}: missing foreground")),
  )
}

// -- Valid YAML --

#[test]
fn is_valid_yaml() {
  parse_theme();
}

// -- All hex colors in the file are valid --

#[test]
fn all_hex_colors_are_valid() {
  for (line, hex) in extract_hex_colors(THEME) {
    assert!(is_valid_hex(hex), "line {line}: invalid hex: {hex}");
  }
}

// -- colourful flag --

#[test]
fn has_colourful_true() {
  let v = parse_theme();
  assert_eq!(
    v.get("colourful").and_then(|v| v.as_bool()),
    Some(true),
    "eza theme must have 'colourful: true' at top level"
  );
}

// -- Required sections --

const REQUIRED_SECTIONS: &[&str] = &[
  "filekinds",
  "perms",
  "size",
  "users",
  "links",
  "git",
  "git_repo",
  "security_context",
  "file_type",
  "punctuation",
  "date",
  "inode",
  "blocks",
  "header",
  "octal",
  "flags",
  "symlink_path",
  "control_char",
  "broken_symlink",
  "broken_path_overlay",
];

#[test]
fn has_all_required_sections() {
  let v = parse_theme();
  for section in REQUIRED_SECTIONS {
    assert!(v.get(section).is_some(), "missing section: {section}");
  }
}

// -- filekinds section --

#[test]
fn filekinds_has_required_entries() {
  let v = parse_theme();
  let section = get_section(&v, "filekinds");
  for key in [
    "normal",
    "directory",
    "symlink",
    "pipe",
    "block_device",
    "char_device",
    "socket",
    "special",
    "executable",
    "mount_point",
  ] {
    assert!(section.get(key).is_some(), "filekinds.{key}: missing");
    let fg = get_foreground(section, key, "filekinds");
    assert!(is_valid_hex(&fg), "filekinds.{key}: invalid hex: {fg}");
  }
}

// -- perms section --

#[test]
fn perms_has_required_entries() {
  let v = parse_theme();
  let section = get_section(&v, "perms");
  for key in [
    "user_read",
    "user_write",
    "user_execute_file",
    "user_execute_other",
    "group_read",
    "group_write",
    "group_execute",
    "other_read",
    "other_write",
    "other_execute",
    "special_user_file",
    "special_other",
    "attribute",
  ] {
    assert!(section.get(key).is_some(), "perms.{key}: missing");
    let fg = get_foreground(section, key, "perms");
    assert!(is_valid_hex(&fg), "perms.{key}: invalid hex: {fg}");
  }
}

// -- size section --

#[test]
fn size_has_required_entries() {
  let v = parse_theme();
  let section = get_section(&v, "size");
  for key in [
    "major",
    "minor",
    "number_byte",
    "number_kilo",
    "number_mega",
    "number_giga",
    "number_huge",
    "unit_byte",
    "unit_kilo",
    "unit_mega",
    "unit_giga",
    "unit_huge",
  ] {
    assert!(section.get(key).is_some(), "size.{key}: missing");
    let fg = get_foreground(section, key, "size");
    assert!(is_valid_hex(&fg), "size.{key}: invalid hex: {fg}");
  }
}

// -- users section --

#[test]
fn users_has_required_entries() {
  let v = parse_theme();
  let section = get_section(&v, "users");
  for key in ["user_you", "user_root", "user_other", "group_yours", "group_other", "group_root"] {
    assert!(section.get(key).is_some(), "users.{key}: missing");
    let fg = get_foreground(section, key, "users");
    assert!(is_valid_hex(&fg), "users.{key}: invalid hex: {fg}");
  }
}

// -- git section --

#[test]
fn git_has_required_entries() {
  let v = parse_theme();
  let section = get_section(&v, "git");
  for key in ["new", "modified", "deleted", "renamed", "typechange", "ignored", "conflicted"] {
    assert!(section.get(key).is_some(), "git.{key}: missing");
    let fg = get_foreground(section, key, "git");
    assert!(is_valid_hex(&fg), "git.{key}: invalid hex: {fg}");
  }
}

// -- git_repo section --

#[test]
fn git_repo_has_required_entries() {
  let v = parse_theme();
  let section = get_section(&v, "git_repo");
  for key in ["branch_main", "branch_other", "git_clean", "git_dirty"] {
    assert!(section.get(key).is_some(), "git_repo.{key}: missing");
    let fg = get_foreground(section, key, "git_repo");
    assert!(is_valid_hex(&fg), "git_repo.{key}: invalid hex: {fg}");
  }
}

// -- file_type section --

#[test]
fn file_type_has_required_entries() {
  let v = parse_theme();
  let section = get_section(&v, "file_type");
  for key in [
    "image", "video", "music", "lossless", "crypto", "document", "compressed", "temp", "compiled", "build",
    "source",
  ] {
    assert!(section.get(key).is_some(), "file_type.{key}: missing");
    let fg = get_foreground(section, key, "file_type");
    assert!(is_valid_hex(&fg), "file_type.{key}: invalid hex: {fg}");
  }
}

// -- security_context section --

#[test]
fn security_context_has_required_entries() {
  let v = parse_theme();
  let section = get_section(&v, "security_context");
  for key in ["colon", "user", "role", "typ", "range"] {
    assert!(section.get(key).is_some(), "security_context.{key}: missing");
    let fg = get_foreground(section, key, "security_context");
    assert!(is_valid_hex(&fg), "security_context.{key}: invalid hex: {fg}");
  }
}

// -- links section --

#[test]
fn links_has_required_entries() {
  let v = parse_theme();
  let section = get_section(&v, "links");
  for key in ["normal", "multi_link_file"] {
    assert!(section.get(key).is_some(), "links.{key}: missing");
    let fg = get_foreground(section, key, "links");
    assert!(is_valid_hex(&fg), "links.{key}: invalid hex: {fg}");
  }
}

// -- Flat sections have foreground --

#[test]
fn flat_sections_have_foreground() {
  let v = parse_theme();
  for section_name in [
    "punctuation",
    "date",
    "inode",
    "blocks",
    "header",
    "octal",
    "flags",
    "symlink_path",
    "control_char",
    "broken_symlink",
    "broken_path_overlay",
  ] {
    let section = get_section(&v, section_name);
    let fg = top_foreground(section, section_name);
    assert!(is_valid_hex(&fg), "{section_name}: invalid hex: {fg}");
  }
}

// -- header is bold --

#[test]
fn header_is_bold() {
  let v = parse_theme();
  let section = get_section(&v, "header");
  assert_eq!(
    section.get("is_bold").and_then(|v| v.as_bool()),
    Some(true),
    "header should have is_bold: true"
  );
}

// -- Canonical palette colors --

#[test]
fn filekinds_normal_uses_canonical_foreground() {
  let v = parse_theme();
  let fg = get_foreground(get_section(&v, "filekinds"), "normal", "filekinds");
  assert_eq!(fg, "#bfbdb6");
}

#[test]
fn filekinds_directory_uses_canonical_color() {
  let v = parse_theme();
  let fg = get_foreground(get_section(&v, "filekinds"), "directory", "filekinds");
  assert_eq!(fg, "#deb074");
}

#[test]
fn filekinds_symlink_uses_canonical_color() {
  let v = parse_theme();
  let fg = get_foreground(get_section(&v, "filekinds"), "symlink", "filekinds");
  assert_eq!(fg, "#96b898");
}

#[test]
fn filekinds_executable_uses_canonical_color() {
  let v = parse_theme();
  let fg = get_foreground(get_section(&v, "filekinds"), "executable", "filekinds");
  assert_eq!(fg, "#b4bc78");
}

#[test]
fn git_new_uses_canonical_string_color() {
  let v = parse_theme();
  let fg = get_foreground(get_section(&v, "git"), "new", "git");
  assert_eq!(fg, "#b4bc78");
}

#[test]
fn git_modified_uses_canonical_function_color() {
  let v = parse_theme();
  let fg = get_foreground(get_section(&v, "git"), "modified", "git");
  assert_eq!(fg, "#ffb454");
}

#[test]
fn git_deleted_uses_canonical_member_color() {
  let v = parse_theme();
  let fg = get_foreground(get_section(&v, "git"), "deleted", "git");
  assert_eq!(fg, "#f58088");
}

#[test]
fn git_conflicted_uses_canonical_error_color() {
  let v = parse_theme();
  let fg = get_foreground(get_section(&v, "git"), "conflicted", "git");
  assert_eq!(fg, "#f08888");
}

// -- No unexpected top-level keys (catches typos / wrong structure) --

#[test]
fn no_unexpected_top_level_keys() {
  let v = parse_theme();
  let allowed: &[&str] = &[
    "colourful",
    "filekinds",
    "perms",
    "size",
    "users",
    "links",
    "git",
    "git_repo",
    "security_context",
    "file_type",
    "punctuation",
    "date",
    "inode",
    "blocks",
    "header",
    "octal",
    "flags",
    "symlink_path",
    "control_char",
    "broken_symlink",
    "broken_path_overlay",
  ];
  let mapping = v.as_mapping().expect("top level should be a YAML mapping");
  for (key, _) in mapping {
    let key_str = key.as_str().unwrap_or("<non-string key>");
    assert!(
      allowed.contains(&key_str),
      "unexpected top-level key: '{key_str}' (typo or wrong section name?)"
    );
  }
}

// -- Nested sections use map-of-maps structure (catches format errors) --

#[test]
fn nested_sections_are_maps_of_maps() {
  let v = parse_theme();
  for section_name in ["filekinds", "perms", "size", "users", "links", "git", "git_repo", "security_context", "file_type"] {
    let section = get_section(&v, section_name);
    let mapping = section
      .as_mapping()
      .unwrap_or_else(|| panic!("{section_name} should be a YAML mapping"));
    for (key, val) in mapping {
      let key_str = key.as_str().unwrap_or("<non-string>");
      assert!(
        val.is_mapping(),
        "{section_name}.{key_str} should be a mapping with 'foreground', got scalar or sequence"
      );
    }
  }
}

// -- Flat sections are direct maps (not nested) --

#[test]
fn flat_sections_are_direct_maps() {
  let v = parse_theme();
  for section_name in [
    "punctuation",
    "date",
    "inode",
    "blocks",
    "header",
    "octal",
    "flags",
    "symlink_path",
    "control_char",
    "broken_symlink",
    "broken_path_overlay",
  ] {
    let section = get_section(&v, section_name);
    assert!(
      section.is_mapping(),
      "{section_name} should be a mapping"
    );
    assert!(
      section.get("foreground").is_some(),
      "{section_name} should have a direct 'foreground' key"
    );
  }
}
