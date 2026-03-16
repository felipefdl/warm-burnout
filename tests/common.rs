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

/// Extract a color value from a Lua palette table.
/// Parses lines like `  bg = "#1a1510",` and returns the hex value for the given key.
pub fn nvim_palette_color(src: &str, key: &str) -> String {
  src
    .lines()
    .find(|l| {
      let l = l.trim();
      l.starts_with(key) && l[key.len()..].trim_start().starts_with('=')
    })
    .and_then(|l| {
      l.split_once('=').and_then(|(_, v)| {
        let v = v.trim().trim_end_matches(',');
        if v.starts_with('"') && v.ends_with('"') {
          Some(hex_to_lower(&v[1..v.len() - 1]))
        } else {
          None
        }
      })
    })
    .unwrap_or_else(|| panic!("no key '{key}' in nvim palette"))
}

pub fn xcode_color(src: &str, key: &str) -> String {
  let cursor = std::io::Cursor::new(src.as_bytes());
  let value: plist::Value = plist::from_reader(cursor).expect("invalid plist");
  let dict = value.as_dictionary().expect("plist root is not a dict");
  let rgba_str = dict
    .get(key)
    .and_then(|v| v.as_string())
    .unwrap_or_else(|| panic!("missing key: {key}"));
  rgba_float_to_hex(rgba_str)
}

pub fn xcode_syntax_color(src: &str, key: &str) -> String {
  let cursor = std::io::Cursor::new(src.as_bytes());
  let value: plist::Value = plist::from_reader(cursor).expect("invalid plist");
  let dict = value.as_dictionary().expect("plist root is not a dict");
  let syntax = dict
    .get("DVTSourceTextSyntaxColors")
    .and_then(|v| v.as_dictionary())
    .expect("missing DVTSourceTextSyntaxColors dict");
  let rgba_str = syntax
    .get(key)
    .and_then(|v| v.as_string())
    .unwrap_or_else(|| panic!("missing syntax key: {key}"));
  rgba_float_to_hex(rgba_str)
}

pub fn xcode_syntax_font(src: &str, key: &str) -> String {
  let cursor = std::io::Cursor::new(src.as_bytes());
  let value: plist::Value = plist::from_reader(cursor).expect("invalid plist");
  let dict = value.as_dictionary().expect("plist root is not a dict");
  let fonts = dict
    .get("DVTSourceTextSyntaxFonts")
    .and_then(|v| v.as_dictionary())
    .expect("missing DVTSourceTextSyntaxFonts dict");
  fonts
    .get(key)
    .and_then(|v| v.as_string())
    .unwrap_or_else(|| panic!("missing font key: {key}"))
    .to_string()
}

fn rgba_float_to_hex(rgba: &str) -> String {
  let parts: Vec<f64> = rgba.split_whitespace().map(|s| s.parse().unwrap()).collect();
  assert!(parts.len() >= 3, "rgba string must have at least 3 components");
  let r = (parts[0] * 255.0).round() as u8;
  let g = (parts[1] * 255.0).round() as u8;
  let b = (parts[2] * 255.0).round() as u8;
  format!("#{r:02x}{g:02x}{b:02x}")
}

/// Extract the value of a tmux `set -g option-name "value"` line.
pub fn tmux_option_value(src: &str, option: &str) -> String {
  src
    .lines()
    .find(|l| {
      let l = l.trim();
      l.starts_with("set -g") && l.contains(option)
    })
    .map(|l| {
      let after_option = l.split(option).nth(1).unwrap().trim();
      after_option.trim_matches('"').to_string()
    })
    .unwrap_or_else(|| panic!("no tmux option '{option}'"))
}

/// Extract the fg color from a tmux style string like "fg=#rrggbb,bg=#rrggbb,bold".
pub fn tmux_style_fg(style: &str) -> String {
  style
    .split(',')
    .find(|s| s.starts_with("fg="))
    .map(|s| hex_to_lower(&s[3..]))
    .unwrap_or_else(|| panic!("no fg in style: {style}"))
}

/// Extract the bg color from a tmux style string like "fg=#rrggbb,bg=#rrggbb,bold".
pub fn tmux_style_bg(style: &str) -> String {
  style
    .split(',')
    .find(|s| s.starts_with("bg="))
    .map(|s| hex_to_lower(&s[3..]))
    .unwrap_or_else(|| panic!("no bg in style: {style}"))
}

pub fn windows_terminal_color(src: &str, key: &str) -> String {
  let v: serde_json::Value = serde_json::from_str(src).unwrap();
  hex_to_lower(v[key].as_str().unwrap_or_else(|| panic!("missing key: {key}")))
}

pub fn iterm2_color(src: &str, key: &str) -> String {
  let cursor = std::io::Cursor::new(src.as_bytes());
  let value: plist::Value = plist::from_reader(cursor).expect("invalid plist");
  let dict = value.as_dictionary().expect("plist root is not a dict");
  let color_dict = dict
    .get(key)
    .and_then(|v| v.as_dictionary())
    .unwrap_or_else(|| panic!("missing key: {key}"));
  let r = color_dict.get("Red Component").and_then(|v| v.as_real()).unwrap();
  let g = color_dict.get("Green Component").and_then(|v| v.as_real()).unwrap();
  let b = color_dict.get("Blue Component").and_then(|v| v.as_real()).unwrap();
  let r = (r * 255.0).round() as u8;
  let g = (g * 255.0).round() as u8;
  let b = (b * 255.0).round() as u8;
  format!("#{r:02x}{g:02x}{b:02x}")
}

/// Extract a color value from the `<colors>` section of a JetBrains `.xml` file.
/// Parses lines like `<option name="CARET_COLOR" value="f5c56e"/>` and returns `#f5c56e`.
pub fn jetbrains_color(src: &str, key: &str) -> String {
  let colors_start = src.find("<colors>").expect("missing <colors> section");
  let colors_end = src.find("</colors>").expect("missing </colors> section");
  let colors = &src[colors_start..colors_end];
  let pattern = format!("name=\"{key}\"");
  let line = colors
    .lines()
    .find(|l| l.contains(&pattern))
    .unwrap_or_else(|| panic!("no color key '{key}' in jetbrains theme"));
  let val_start = line.find("value=\"").expect("no value in line") + 7;
  let val_end = line[val_start..].find('"').expect("no closing quote") + val_start;
  format!("#{}", line[val_start..val_end].to_lowercase())
}

/// Extract a property from an attribute in the `<attributes>` section of a JetBrains `.xml` file.
/// `prop` is one of: FOREGROUND, BACKGROUND, FONT_TYPE.
/// For FOREGROUND/BACKGROUND returns `#hexval`; for FONT_TYPE returns the raw digit string.
pub fn jetbrains_attribute(src: &str, attr: &str, prop: &str) -> String {
  let attrs_start = src.find("<attributes>").expect("missing <attributes> section");
  let attrs_section = &src[attrs_start..];
  let attr_pattern = format!("name=\"{attr}\"");
  let attr_pos = attrs_section
    .find(&attr_pattern)
    .unwrap_or_else(|| panic!("no attribute '{attr}' in jetbrains theme"));
  let rest = &attrs_section[attr_pos..];
  let prop_pattern = format!("name=\"{prop}\"");
  let prop_pos = rest
    .find(&prop_pattern)
    .unwrap_or_else(|| panic!("no property '{prop}' in attribute '{attr}'"));
  let prop_rest = &rest[prop_pos..];
  let val_start = prop_rest.find("value=\"").expect("no value") + 7;
  let val_end = prop_rest[val_start..].find('"').expect("no closing quote") + val_start;
  let raw = &prop_rest[val_start..val_end];
  if prop == "FONT_TYPE" {
    raw.to_string()
  } else {
    format!("#{}", raw.to_lowercase())
  }
}

/// Extract a color value from a Home Assistant theme YAML file.
/// Navigates to `theme_name > modes > mode > key` and returns the lowercase hex value.
pub fn home_assistant_color(src: &str, theme_name: &str, mode: &str, key: &str) -> String {
  let v: serde_yml::Value = serde_yml::from_str(src).expect("invalid YAML");
  let val = v[theme_name]["modes"][mode][key]
    .as_str()
    .unwrap_or_else(|| panic!("missing key '{key}' in {theme_name} > modes > {mode}"));
  hex_to_lower(val)
}

/// Extract all key-value pairs from a Lua palette table block (M.dark or M.light).
/// Returns vec of (key, hex_value) pairs.
pub fn nvim_palette_keys(src: &str) -> Vec<String> {
  src
    .lines()
    .filter_map(|l| {
      let l = l.trim();
      if l.contains('=') && !l.starts_with("--") && !l.starts_with("M.") && !l.starts_with("return") {
        l.split('=').next().map(|k| k.trim().to_string())
      } else {
        None
      }
    })
    .collect()
}
