#![allow(dead_code)]

pub fn is_valid_hex(s: &str) -> bool {
  let s = s.strip_prefix('#').unwrap_or(s);
  matches!(s.len(), 3 | 6 | 8) && s.chars().all(|c| c.is_ascii_hexdigit())
}

pub fn hex_to_lower(s: &str) -> String {
  s.trim().to_lowercase()
}

pub fn extract_hex_colors(src: &str) -> Vec<(usize, &str)> {
  src
    .lines()
    .enumerate()
    .flat_map(|(i, line)| {
      let mut colors = Vec::new();
      let mut rest = line;
      while let Some(pos) = rest.find('#') {
        let after = &rest[pos + 1..];
        let hex_len = after.chars().take_while(|c| c.is_ascii_hexdigit()).count();
        if matches!(hex_len, 3 | 6 | 8) {
          colors.push((i + 1, &rest[pos..pos + 1 + hex_len]));
        }
        rest = &rest[pos + 1..];
      }
      colors
    })
    .collect()
}

pub fn ghostty_color(src: &str, key: &str) -> String {
  src
    .lines()
    .find(|l| {
      let l = l.trim();
      l.starts_with(key) && l[key.len()..].trim_start().starts_with('=')
    })
    .and_then(|l| l.split_once('='))
    .map(|(_, v)| hex_to_lower(v))
    .unwrap_or_else(|| panic!("no {key} in ghostty theme"))
}

pub fn starship_palette_color(src: &str, palette: &str, key: &str) -> String {
  let table = src.parse::<toml::Table>().unwrap();
  hex_to_lower(table["palettes"][palette][key].as_str().unwrap())
}

pub fn vscode_color(src: &str, key: &str) -> String {
  let v: serde_json::Value = serde_json::from_str(src).unwrap();
  hex_to_lower(v["colors"][key].as_str().unwrap())
}

pub fn zed_editor_color(src: &str, theme_name: &str, key: &str) -> String {
  let v: serde_json::Value = serde_json::from_str(src).unwrap();
  let themes = v["themes"].as_array().unwrap();
  let theme = themes
    .iter()
    .find(|t| t["name"].as_str() == Some(theme_name))
    .unwrap_or_else(|| panic!("no theme named '{theme_name}'"));
  hex_to_lower(
    theme["style"][key]
      .as_str()
      .unwrap_or_else(|| panic!("missing style key: {key}")),
  )
}

pub fn zed_syntax_color(src: &str, theme_name: &str, key: &str) -> String {
  let v: serde_json::Value = serde_json::from_str(src).unwrap();
  let themes = v["themes"].as_array().unwrap();
  let theme = themes
    .iter()
    .find(|t| t["name"].as_str() == Some(theme_name))
    .unwrap_or_else(|| panic!("no theme named '{theme_name}'"));
  hex_to_lower(
    theme["style"]["syntax"][key]["color"]
      .as_str()
      .unwrap_or_else(|| panic!("missing syntax key: {key}")),
  )
}
