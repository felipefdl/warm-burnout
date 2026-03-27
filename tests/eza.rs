mod common;

use common::{extract_hex_colors, hex_to_lower, is_valid_hex};
use serde_yml::Value;

const DARK: &str = include_str!("../eza/dark.yml");
const LIGHT: &str = include_str!("../eza/light.yml");

fn parse_theme(src: &str) -> Value {
  serde_yml::from_str(src).expect("invalid YAML")
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
fn dark_is_valid_yaml() {
  parse_theme(DARK);
}

#[test]
fn light_is_valid_yaml() {
  parse_theme(LIGHT);
}

// -- All hex colors in the file are valid --

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

// -- colourful flag --

#[test]
fn dark_has_colourful_true() {
  let v = parse_theme(DARK);
  assert_eq!(
    v.get("colourful").and_then(|v| v.as_bool()),
    Some(true),
    "dark eza theme must have 'colourful: true' at top level"
  );
}

#[test]
fn light_has_colourful_true() {
  let v = parse_theme(LIGHT);
  assert_eq!(
    v.get("colourful").and_then(|v| v.as_bool()),
    Some(true),
    "light eza theme must have 'colourful: true' at top level"
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
fn dark_has_all_required_sections() {
  let v = parse_theme(DARK);
  for section in REQUIRED_SECTIONS {
    assert!(v.get(section).is_some(), "dark missing section: {section}");
  }
}

#[test]
fn light_has_all_required_sections() {
  let v = parse_theme(LIGHT);
  for section in REQUIRED_SECTIONS {
    assert!(v.get(section).is_some(), "light missing section: {section}");
  }
}

// -- Both variants have identical structure --

#[test]
fn dark_and_light_have_same_top_level_keys() {
  let dark = parse_theme(DARK);
  let light = parse_theme(LIGHT);
  let dark_keys: Vec<&str> = dark.as_mapping().unwrap().keys().map(|k| k.as_str().unwrap()).collect();
  let light_keys: Vec<&str> = light
    .as_mapping()
    .unwrap()
    .keys()
    .map(|k| k.as_str().unwrap())
    .collect();
  assert_eq!(
    dark_keys, light_keys,
    "dark and light must have identical top-level keys"
  );
}

// -- filekinds section --

const FILEKINDS_KEYS: &[&str] = &[
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
];

#[test]
fn dark_filekinds_has_required_entries() {
  let v = parse_theme(DARK);
  let section = get_section(&v, "filekinds");
  for key in FILEKINDS_KEYS {
    assert!(section.get(key).is_some(), "dark filekinds.{key}: missing");
    let fg = get_foreground(section, key, "filekinds");
    assert!(is_valid_hex(&fg), "dark filekinds.{key}: invalid hex: {fg}");
  }
}

#[test]
fn light_filekinds_has_required_entries() {
  let v = parse_theme(LIGHT);
  let section = get_section(&v, "filekinds");
  for key in FILEKINDS_KEYS {
    assert!(section.get(key).is_some(), "light filekinds.{key}: missing");
    let fg = get_foreground(section, key, "filekinds");
    assert!(is_valid_hex(&fg), "light filekinds.{key}: invalid hex: {fg}");
  }
}

// -- perms section --

const PERMS_KEYS: &[&str] = &[
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
];

#[test]
fn dark_perms_has_required_entries() {
  let v = parse_theme(DARK);
  let section = get_section(&v, "perms");
  for key in PERMS_KEYS {
    assert!(section.get(key).is_some(), "dark perms.{key}: missing");
    let fg = get_foreground(section, key, "perms");
    assert!(is_valid_hex(&fg), "dark perms.{key}: invalid hex: {fg}");
  }
}

#[test]
fn light_perms_has_required_entries() {
  let v = parse_theme(LIGHT);
  let section = get_section(&v, "perms");
  for key in PERMS_KEYS {
    assert!(section.get(key).is_some(), "light perms.{key}: missing");
    let fg = get_foreground(section, key, "perms");
    assert!(is_valid_hex(&fg), "light perms.{key}: invalid hex: {fg}");
  }
}

// -- size section --

const SIZE_KEYS: &[&str] = &[
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
];

#[test]
fn dark_size_has_required_entries() {
  let v = parse_theme(DARK);
  let section = get_section(&v, "size");
  for key in SIZE_KEYS {
    assert!(section.get(key).is_some(), "dark size.{key}: missing");
    let fg = get_foreground(section, key, "size");
    assert!(is_valid_hex(&fg), "dark size.{key}: invalid hex: {fg}");
  }
}

#[test]
fn light_size_has_required_entries() {
  let v = parse_theme(LIGHT);
  let section = get_section(&v, "size");
  for key in SIZE_KEYS {
    assert!(section.get(key).is_some(), "light size.{key}: missing");
    let fg = get_foreground(section, key, "size");
    assert!(is_valid_hex(&fg), "light size.{key}: invalid hex: {fg}");
  }
}

// -- users section --

const USERS_KEYS: &[&str] = &[
  "user_you",
  "user_root",
  "user_other",
  "group_yours",
  "group_other",
  "group_root",
];

#[test]
fn dark_users_has_required_entries() {
  let v = parse_theme(DARK);
  let section = get_section(&v, "users");
  for key in USERS_KEYS {
    assert!(section.get(key).is_some(), "dark users.{key}: missing");
    let fg = get_foreground(section, key, "users");
    assert!(is_valid_hex(&fg), "dark users.{key}: invalid hex: {fg}");
  }
}

#[test]
fn light_users_has_required_entries() {
  let v = parse_theme(LIGHT);
  let section = get_section(&v, "users");
  for key in USERS_KEYS {
    assert!(section.get(key).is_some(), "light users.{key}: missing");
    let fg = get_foreground(section, key, "users");
    assert!(is_valid_hex(&fg), "light users.{key}: invalid hex: {fg}");
  }
}

// -- git section --

const GIT_KEYS: &[&str] = &[
  "new",
  "modified",
  "deleted",
  "renamed",
  "typechange",
  "ignored",
  "conflicted",
];

#[test]
fn dark_git_has_required_entries() {
  let v = parse_theme(DARK);
  let section = get_section(&v, "git");
  for key in GIT_KEYS {
    assert!(section.get(key).is_some(), "dark git.{key}: missing");
    let fg = get_foreground(section, key, "git");
    assert!(is_valid_hex(&fg), "dark git.{key}: invalid hex: {fg}");
  }
}

#[test]
fn light_git_has_required_entries() {
  let v = parse_theme(LIGHT);
  let section = get_section(&v, "git");
  for key in GIT_KEYS {
    assert!(section.get(key).is_some(), "light git.{key}: missing");
    let fg = get_foreground(section, key, "git");
    assert!(is_valid_hex(&fg), "light git.{key}: invalid hex: {fg}");
  }
}

// -- git_repo section --

const GIT_REPO_KEYS: &[&str] = &["branch_main", "branch_other", "git_clean", "git_dirty"];

#[test]
fn dark_git_repo_has_required_entries() {
  let v = parse_theme(DARK);
  let section = get_section(&v, "git_repo");
  for key in GIT_REPO_KEYS {
    assert!(section.get(key).is_some(), "dark git_repo.{key}: missing");
    let fg = get_foreground(section, key, "git_repo");
    assert!(is_valid_hex(&fg), "dark git_repo.{key}: invalid hex: {fg}");
  }
}

#[test]
fn light_git_repo_has_required_entries() {
  let v = parse_theme(LIGHT);
  let section = get_section(&v, "git_repo");
  for key in GIT_REPO_KEYS {
    assert!(section.get(key).is_some(), "light git_repo.{key}: missing");
    let fg = get_foreground(section, key, "git_repo");
    assert!(is_valid_hex(&fg), "light git_repo.{key}: invalid hex: {fg}");
  }
}

// -- file_type section --

const FILE_TYPE_KEYS: &[&str] = &[
  "image",
  "video",
  "music",
  "lossless",
  "crypto",
  "document",
  "compressed",
  "temp",
  "compiled",
  "build",
  "source",
];

#[test]
fn dark_file_type_has_required_entries() {
  let v = parse_theme(DARK);
  let section = get_section(&v, "file_type");
  for key in FILE_TYPE_KEYS {
    assert!(section.get(key).is_some(), "dark file_type.{key}: missing");
    let fg = get_foreground(section, key, "file_type");
    assert!(is_valid_hex(&fg), "dark file_type.{key}: invalid hex: {fg}");
  }
}

#[test]
fn light_file_type_has_required_entries() {
  let v = parse_theme(LIGHT);
  let section = get_section(&v, "file_type");
  for key in FILE_TYPE_KEYS {
    assert!(section.get(key).is_some(), "light file_type.{key}: missing");
    let fg = get_foreground(section, key, "file_type");
    assert!(is_valid_hex(&fg), "light file_type.{key}: invalid hex: {fg}");
  }
}

// -- security_context section --

const SECURITY_CONTEXT_KEYS: &[&str] = &["colon", "user", "role", "typ", "range"];

#[test]
fn dark_security_context_has_required_entries() {
  let v = parse_theme(DARK);
  let section = get_section(&v, "security_context");
  for key in SECURITY_CONTEXT_KEYS {
    assert!(section.get(key).is_some(), "dark security_context.{key}: missing");
    let fg = get_foreground(section, key, "security_context");
    assert!(is_valid_hex(&fg), "dark security_context.{key}: invalid hex: {fg}");
  }
}

#[test]
fn light_security_context_has_required_entries() {
  let v = parse_theme(LIGHT);
  let section = get_section(&v, "security_context");
  for key in SECURITY_CONTEXT_KEYS {
    assert!(section.get(key).is_some(), "light security_context.{key}: missing");
    let fg = get_foreground(section, key, "security_context");
    assert!(is_valid_hex(&fg), "light security_context.{key}: invalid hex: {fg}");
  }
}

// -- links section --

const LINKS_KEYS: &[&str] = &["normal", "multi_link_file"];

#[test]
fn dark_links_has_required_entries() {
  let v = parse_theme(DARK);
  let section = get_section(&v, "links");
  for key in LINKS_KEYS {
    assert!(section.get(key).is_some(), "dark links.{key}: missing");
    let fg = get_foreground(section, key, "links");
    assert!(is_valid_hex(&fg), "dark links.{key}: invalid hex: {fg}");
  }
}

#[test]
fn light_links_has_required_entries() {
  let v = parse_theme(LIGHT);
  let section = get_section(&v, "links");
  for key in LINKS_KEYS {
    assert!(section.get(key).is_some(), "light links.{key}: missing");
    let fg = get_foreground(section, key, "links");
    assert!(is_valid_hex(&fg), "light links.{key}: invalid hex: {fg}");
  }
}

// -- Flat sections have foreground --

const FLAT_SECTIONS: &[&str] = &[
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
fn dark_flat_sections_have_foreground() {
  let v = parse_theme(DARK);
  for section_name in FLAT_SECTIONS {
    let section = get_section(&v, section_name);
    let fg = top_foreground(section, section_name);
    assert!(is_valid_hex(&fg), "dark {section_name}: invalid hex: {fg}");
  }
}

#[test]
fn light_flat_sections_have_foreground() {
  let v = parse_theme(LIGHT);
  for section_name in FLAT_SECTIONS {
    let section = get_section(&v, section_name);
    let fg = top_foreground(section, section_name);
    assert!(is_valid_hex(&fg), "light {section_name}: invalid hex: {fg}");
  }
}

// -- header is bold --

#[test]
fn dark_header_is_bold() {
  let v = parse_theme(DARK);
  let section = get_section(&v, "header");
  assert_eq!(
    section.get("is_bold").and_then(|v| v.as_bool()),
    Some(true),
    "dark header should have is_bold: true"
  );
}

#[test]
fn light_header_is_bold() {
  let v = parse_theme(LIGHT);
  let section = get_section(&v, "header");
  assert_eq!(
    section.get("is_bold").and_then(|v| v.as_bool()),
    Some(true),
    "light header should have is_bold: true"
  );
}

// -- Dark canonical palette colors --

#[test]
fn dark_filekinds_normal_uses_canonical_foreground() {
  let v = parse_theme(DARK);
  let fg = get_foreground(get_section(&v, "filekinds"), "normal", "filekinds");
  assert_eq!(fg, "#bfbdb6");
}

#[test]
fn dark_filekinds_directory_uses_canonical_color() {
  let v = parse_theme(DARK);
  let fg = get_foreground(get_section(&v, "filekinds"), "directory", "filekinds");
  assert_eq!(fg, "#deb074");
}

#[test]
fn dark_filekinds_symlink_uses_canonical_color() {
  let v = parse_theme(DARK);
  let fg = get_foreground(get_section(&v, "filekinds"), "symlink", "filekinds");
  assert_eq!(fg, "#96b898");
}

#[test]
fn dark_filekinds_executable_uses_canonical_color() {
  let v = parse_theme(DARK);
  let fg = get_foreground(get_section(&v, "filekinds"), "executable", "filekinds");
  assert_eq!(fg, "#b4bc78");
}

#[test]
fn dark_git_new_uses_canonical_string_color() {
  let v = parse_theme(DARK);
  let fg = get_foreground(get_section(&v, "git"), "new", "git");
  assert_eq!(fg, "#b4bc78");
}

#[test]
fn dark_git_modified_uses_canonical_function_color() {
  let v = parse_theme(DARK);
  let fg = get_foreground(get_section(&v, "git"), "modified", "git");
  assert_eq!(fg, "#ffb454");
}

#[test]
fn dark_git_deleted_uses_canonical_member_color() {
  let v = parse_theme(DARK);
  let fg = get_foreground(get_section(&v, "git"), "deleted", "git");
  assert_eq!(fg, "#ec9878");
}

#[test]
fn dark_git_conflicted_uses_canonical_error_color() {
  let v = parse_theme(DARK);
  let fg = get_foreground(get_section(&v, "git"), "conflicted", "git");
  assert_eq!(fg, "#f49090");
}

// -- Light canonical palette colors --

#[test]
fn light_filekinds_normal_uses_canonical_foreground() {
  let v = parse_theme(LIGHT);
  let fg = get_foreground(get_section(&v, "filekinds"), "normal", "filekinds");
  assert_eq!(fg, "#3a3630");
}

#[test]
fn light_filekinds_directory_uses_canonical_color() {
  let v = parse_theme(LIGHT);
  let fg = get_foreground(get_section(&v, "filekinds"), "directory", "filekinds");
  assert_eq!(fg, "#74501c");
}

#[test]
fn light_filekinds_symlink_uses_canonical_color() {
  let v = parse_theme(LIGHT);
  let fg = get_foreground(get_section(&v, "filekinds"), "symlink", "filekinds");
  assert_eq!(fg, "#286a48");
}

#[test]
fn light_filekinds_executable_uses_canonical_color() {
  let v = parse_theme(LIGHT);
  let fg = get_foreground(get_section(&v, "filekinds"), "executable", "filekinds");
  assert_eq!(fg, "#4d5c1a");
}

#[test]
fn light_git_new_uses_canonical_string_color() {
  let v = parse_theme(LIGHT);
  let fg = get_foreground(get_section(&v, "git"), "new", "git");
  assert_eq!(fg, "#4d5c1a");
}

#[test]
fn light_git_modified_uses_canonical_function_color() {
  let v = parse_theme(LIGHT);
  let fg = get_foreground(get_section(&v, "git"), "modified", "git");
  assert_eq!(fg, "#855700");
}

#[test]
fn light_git_deleted_uses_canonical_member_color() {
  let v = parse_theme(LIGHT);
  let fg = get_foreground(get_section(&v, "git"), "deleted", "git");
  assert_eq!(fg, "#883850");
}

#[test]
fn light_git_conflicted_uses_canonical_error_color() {
  let v = parse_theme(LIGHT);
  let fg = get_foreground(get_section(&v, "git"), "conflicted", "git");
  assert_eq!(fg, "#b03434");
}

// -- No unexpected top-level keys (catches typos / wrong structure) --

const ALLOWED_TOP_LEVEL: &[&str] = &[
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

#[test]
fn dark_no_unexpected_top_level_keys() {
  let v = parse_theme(DARK);
  let mapping = v.as_mapping().expect("top level should be a YAML mapping");
  for (key, _) in mapping {
    let key_str = key.as_str().unwrap_or("<non-string key>");
    assert!(
      ALLOWED_TOP_LEVEL.contains(&key_str),
      "dark: unexpected top-level key: '{key_str}' (typo or wrong section name?)"
    );
  }
}

#[test]
fn light_no_unexpected_top_level_keys() {
  let v = parse_theme(LIGHT);
  let mapping = v.as_mapping().expect("top level should be a YAML mapping");
  for (key, _) in mapping {
    let key_str = key.as_str().unwrap_or("<non-string key>");
    assert!(
      ALLOWED_TOP_LEVEL.contains(&key_str),
      "light: unexpected top-level key: '{key_str}' (typo or wrong section name?)"
    );
  }
}

// -- Nested sections use map-of-maps structure (catches format errors) --

const NESTED_SECTIONS: &[&str] = &[
  "filekinds",
  "perms",
  "size",
  "users",
  "links",
  "git",
  "git_repo",
  "security_context",
  "file_type",
];

#[test]
fn dark_nested_sections_are_maps_of_maps() {
  let v = parse_theme(DARK);
  for section_name in NESTED_SECTIONS {
    let section = get_section(&v, section_name);
    let mapping = section
      .as_mapping()
      .unwrap_or_else(|| panic!("dark {section_name} should be a YAML mapping"));
    for (key, val) in mapping {
      let key_str = key.as_str().unwrap_or("<non-string>");
      assert!(
        val.is_mapping(),
        "dark {section_name}.{key_str} should be a mapping with 'foreground', got scalar or sequence"
      );
    }
  }
}

#[test]
fn light_nested_sections_are_maps_of_maps() {
  let v = parse_theme(LIGHT);
  for section_name in NESTED_SECTIONS {
    let section = get_section(&v, section_name);
    let mapping = section
      .as_mapping()
      .unwrap_or_else(|| panic!("light {section_name} should be a YAML mapping"));
    for (key, val) in mapping {
      let key_str = key.as_str().unwrap_or("<non-string>");
      assert!(
        val.is_mapping(),
        "light {section_name}.{key_str} should be a mapping with 'foreground', got scalar or sequence"
      );
    }
  }
}

// -- Flat sections are direct maps (not nested) --

#[test]
fn dark_flat_sections_are_direct_maps() {
  let v = parse_theme(DARK);
  for section_name in FLAT_SECTIONS {
    let section = get_section(&v, section_name);
    assert!(section.is_mapping(), "dark {section_name} should be a mapping");
    assert!(
      section.get("foreground").is_some(),
      "dark {section_name} should have a direct 'foreground' key"
    );
  }
}

#[test]
fn light_flat_sections_are_direct_maps() {
  let v = parse_theme(LIGHT);
  for section_name in FLAT_SECTIONS {
    let section = get_section(&v, section_name);
    assert!(section.is_mapping(), "light {section_name} should be a mapping");
    assert!(
      section.get("foreground").is_some(),
      "light {section_name} should have a direct 'foreground' key"
    );
  }
}
