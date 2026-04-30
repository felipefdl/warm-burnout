mod common;

use common::{extract_hex_colors, hex_to_lower, is_valid_hex};

const DARK: &str = include_str!("../vim/colors/warm-burnout-dark.vim");
const LIGHT: &str = include_str!("../vim/colors/warm-burnout-light.vim");

// -- Helpers --

/// Find a highlight line whose first token (after `hi` or `highlight`) matches `group`.
/// Matches the whole word so `Type` doesn't pick up `Typedef`.
fn highlight_line<'a>(src: &'a str, group: &str) -> Option<&'a str> {
  src.lines().find(|l| {
    let t = l.trim_start();
    let Some(t) = t.strip_prefix("highlight ").or_else(|| t.strip_prefix("hi ")) else {
      return false;
    };
    t.split_whitespace().next() == Some(group)
  })
}

/// Read a key=value attribute from a vim highlight line. Returns the value without whitespace.
fn hl_attr<'a>(line: &'a str, key: &str) -> Option<&'a str> {
  for token in line.split_whitespace() {
    if let Some(rest) = token.strip_prefix(&format!("{key}=")) {
      return Some(rest);
    }
  }
  None
}

// -- Variants declare their colorscheme name --

#[test]
fn dark_sets_colors_name() {
  assert!(
    DARK.contains("let g:colors_name = 'warm-burnout-dark'"),
    "dark variant must set g:colors_name to warm-burnout-dark"
  );
}

#[test]
fn light_sets_colors_name() {
  assert!(
    LIGHT.contains("let g:colors_name = 'warm-burnout-light'"),
    "light variant must set g:colors_name to warm-burnout-light"
  );
}

#[test]
fn dark_sets_background_dark() {
  assert!(
    DARK.contains("set background=dark"),
    "dark variant must set background=dark"
  );
}

#[test]
fn light_sets_background_light() {
  assert!(
    LIGHT.contains("set background=light"),
    "light variant must set background=light"
  );
}

#[test]
fn dark_clears_highlights() {
  assert!(DARK.contains("hi clear"), "dark variant must clear existing highlights");
}

#[test]
fn light_clears_highlights() {
  assert!(
    LIGHT.contains("hi clear"),
    "light variant must clear existing highlights"
  );
}

// -- All hex colors are valid --

#[test]
fn dark_hex_colors_valid() {
  for (line, hex) in extract_hex_colors(DARK) {
    assert!(is_valid_hex(hex), "dark line {line}: invalid hex: {hex}");
  }
}

#[test]
fn light_hex_colors_valid() {
  for (line, hex) in extract_hex_colors(LIGHT) {
    assert!(is_valid_hex(hex), "light line {line}: invalid hex: {hex}");
  }
}

// -- No 8-digit alpha hex (Vim does not support alpha) --

#[test]
fn dark_no_alpha_hex() {
  for (line, hex) in extract_hex_colors(DARK) {
    let raw = hex.trim_start_matches('#');
    assert_ne!(raw.len(), 8, "dark line {line}: 8-digit alpha hex not supported: {hex}");
  }
}

#[test]
fn light_no_alpha_hex() {
  for (line, hex) in extract_hex_colors(LIGHT) {
    let raw = hex.trim_start_matches('#');
    assert_ne!(
      raw.len(),
      8,
      "light line {line}: 8-digit alpha hex not supported: {hex}"
    );
  }
}

// -- Canonical backgrounds --

#[test]
fn dark_normal_background_is_canonical() {
  let line = highlight_line(DARK, "Normal").expect("missing Normal highlight");
  let bg = hl_attr(line, "guibg").expect("Normal missing guibg");
  assert_eq!(hex_to_lower(bg), "#1a1510", "dark Normal background");
}

#[test]
fn light_normal_background_is_canonical() {
  let line = highlight_line(LIGHT, "Normal").expect("missing Normal highlight");
  let bg = hl_attr(line, "guibg").expect("Normal missing guibg");
  assert_eq!(hex_to_lower(bg), "#f5ede0", "light Normal background");
}

// -- Canonical foregrounds --

#[test]
fn dark_normal_foreground_is_canonical() {
  let line = highlight_line(DARK, "Normal").expect("missing Normal highlight");
  let fg = hl_attr(line, "guifg").expect("Normal missing guifg");
  assert_eq!(hex_to_lower(fg), "#bfbdb6", "dark Normal foreground");
}

#[test]
fn light_normal_foreground_is_canonical() {
  let line = highlight_line(LIGHT, "Normal").expect("missing Normal highlight");
  let fg = hl_attr(line, "guifg").expect("Normal missing guifg");
  assert_eq!(hex_to_lower(fg), "#3a3630", "light Normal foreground");
}

// -- Canonical cursor --

#[test]
fn dark_cursor_is_canonical() {
  let line = highlight_line(DARK, "Cursor").expect("missing Cursor highlight");
  let bg = hl_attr(line, "guibg").expect("Cursor missing guibg");
  assert_eq!(hex_to_lower(bg), "#f5c56e", "dark Cursor");
}

#[test]
fn light_cursor_is_canonical() {
  let line = highlight_line(LIGHT, "Cursor").expect("missing Cursor highlight");
  let bg = hl_attr(line, "guibg").expect("Cursor missing guibg");
  assert_eq!(hex_to_lower(bg), "#8a6600", "light Cursor");
}

// -- No pure black/white backgrounds --

#[test]
fn dark_no_pure_black_background() {
  let line = highlight_line(DARK, "Normal").unwrap();
  let bg = hex_to_lower(hl_attr(line, "guibg").unwrap());
  assert_ne!(bg, "#000000", "dark background must not be pure black");
}

#[test]
fn light_no_pure_white_background() {
  let line = highlight_line(LIGHT, "Normal").unwrap();
  let bg = hex_to_lower(hl_attr(line, "guibg").unwrap());
  assert_ne!(bg, "#ffffff", "light background must not be pure white");
}

// -- Canonical syntax colors --

#[test]
fn dark_syntax_colors_match_palette() {
  let expected = [
    ("Keyword", "#ff8f40"),
    ("Function", "#ffb454"),
    ("Type", "#90aec0"),
    ("String", "#b4bc78"),
    ("Comment", "#b4a89c"),
    ("Number", "#d4a8b8"),
    ("Operator", "#f29668"),
    ("Tag", "#dc9e92"),
  ];
  for (group, expected_hex) in expected {
    let line = highlight_line(DARK, group).unwrap_or_else(|| panic!("dark missing highlight: {group}"));
    let fg = hl_attr(line, "guifg").unwrap_or_else(|| panic!("{group} missing guifg"));
    assert_eq!(hex_to_lower(fg), expected_hex, "dark {group} guifg");
  }
}

#[test]
fn light_syntax_colors_match_palette() {
  let expected = [
    ("Keyword", "#924800"),
    ("Function", "#855700"),
    ("Type", "#285464"),
    ("String", "#4d5c1a"),
    ("Comment", "#544c40"),
    ("Number", "#7e4060"),
    ("Operator", "#8f4418"),
    ("Tag", "#8e4632"),
  ];
  for (group, expected_hex) in expected {
    let line = highlight_line(LIGHT, group).unwrap_or_else(|| panic!("light missing highlight: {group}"));
    let fg = hl_attr(line, "guifg").unwrap_or_else(|| panic!("{group} missing guifg"));
    assert_eq!(hex_to_lower(fg), expected_hex, "light {group} guifg");
  }
}

// -- Diagnostic colors --

#[test]
fn dark_diagnostic_colors() {
  let cases = [
    ("DiagnosticError", "#f49090"),
    ("DiagnosticWarn", "#b8522e"),
    ("DiagnosticInfo", "#90aec0"),
    ("DiagnosticHint", "#b4a89c"),
  ];
  for (group, hex) in cases {
    let line = highlight_line(DARK, group).unwrap_or_else(|| panic!("missing {group}"));
    let fg = hl_attr(line, "guifg").unwrap();
    assert_eq!(hex_to_lower(fg), hex, "dark {group}");
  }
}

#[test]
fn light_diagnostic_colors() {
  let cases = [
    ("DiagnosticError", "#b03434"),
    ("DiagnosticWarn", "#b8522e"),
    ("DiagnosticInfo", "#285464"),
    ("DiagnosticHint", "#544c40"),
  ];
  for (group, hex) in cases {
    let line = highlight_line(LIGHT, group).unwrap_or_else(|| panic!("missing {group}"));
    let fg = hl_attr(line, "guifg").unwrap();
    assert_eq!(hex_to_lower(fg), hex, "light {group}");
  }
}

// -- Git colors --

#[test]
fn dark_git_colors() {
  let cases = [
    ("GitGutterAdd", "#70bf56"),
    ("GitGutterChange", "#73b8ff"),
    ("GitGutterDelete", "#f26d78"),
  ];
  for (group, hex) in cases {
    let line = highlight_line(DARK, group).unwrap_or_else(|| panic!("missing {group}"));
    let fg = hl_attr(line, "guifg").unwrap();
    assert_eq!(hex_to_lower(fg), hex, "dark {group}");
  }
}

#[test]
fn light_git_colors() {
  let cases = [
    ("GitGutterAdd", "#226414"),
    ("GitGutterChange", "#2868a0"),
    ("GitGutterDelete", "#c43040"),
  ];
  for (group, hex) in cases {
    let line = highlight_line(LIGHT, group).unwrap_or_else(|| panic!("missing {group}"));
    let fg = hl_attr(line, "guifg").unwrap();
    assert_eq!(hex_to_lower(fg), hex, "light {group}");
  }
}

// -- Required highlight groups --

const REQUIRED_EDITOR_UI: &[&str] = &[
  "Normal",
  "NormalFloat",
  "Visual",
  "CursorLine",
  "CursorLineNr",
  "LineNr",
  "SignColumn",
  "Pmenu",
  "PmenuSel",
  "Search",
  "IncSearch",
  "StatusLine",
  "StatusLineNC",
  "TabLine",
  "TabLineSel",
  "TabLineFill",
  "FloatBorder",
  "WinSeparator",
  "MatchParen",
  "Directory",
];

const REQUIRED_SYNTAX: &[&str] = &[
  "Comment",
  "Constant",
  "String",
  "Number",
  "Boolean",
  "Function",
  "Statement",
  "Keyword",
  "Operator",
  "Type",
  "PreProc",
  "Special",
  "Identifier",
  "Delimiter",
  "Error",
  "Todo",
];

const REQUIRED_DIAGNOSTIC: &[&str] = &[
  "DiagnosticError",
  "DiagnosticWarn",
  "DiagnosticInfo",
  "DiagnosticHint",
  "DiagnosticSignError",
  "DiagnosticUnderlineError",
  "DiagnosticVirtualTextError",
];

const REQUIRED_GIT: &[&str] = &["GitGutterAdd", "GitGutterChange", "GitGutterDelete"];

#[test]
fn dark_has_editor_ui_groups() {
  for group in REQUIRED_EDITOR_UI {
    assert!(
      highlight_line(DARK, group).is_some(),
      "dark missing editor UI group: {group}"
    );
  }
}

#[test]
fn light_has_editor_ui_groups() {
  for group in REQUIRED_EDITOR_UI {
    assert!(
      highlight_line(LIGHT, group).is_some(),
      "light missing editor UI group: {group}"
    );
  }
}

#[test]
fn dark_has_syntax_groups() {
  for group in REQUIRED_SYNTAX {
    assert!(
      highlight_line(DARK, group).is_some(),
      "dark missing syntax group: {group}"
    );
  }
}

#[test]
fn light_has_syntax_groups() {
  for group in REQUIRED_SYNTAX {
    assert!(
      highlight_line(LIGHT, group).is_some(),
      "light missing syntax group: {group}"
    );
  }
}

#[test]
fn dark_has_diagnostic_groups() {
  for group in REQUIRED_DIAGNOSTIC {
    assert!(
      highlight_line(DARK, group).is_some(),
      "dark missing diagnostic group: {group}"
    );
  }
}

#[test]
fn light_has_diagnostic_groups() {
  for group in REQUIRED_DIAGNOSTIC {
    assert!(
      highlight_line(LIGHT, group).is_some(),
      "light missing diagnostic group: {group}"
    );
  }
}

#[test]
fn dark_has_git_groups() {
  for group in REQUIRED_GIT {
    assert!(highlight_line(DARK, group).is_some(), "dark missing git group: {group}");
  }
}

#[test]
fn light_has_git_groups() {
  for group in REQUIRED_GIT {
    assert!(
      highlight_line(LIGHT, group).is_some(),
      "light missing git group: {group}"
    );
  }
}

// -- Font style system: bold keywords, italic types, italic comments --

fn assert_style(src: &str, group: &str, style: &str, variant: &str) {
  let line = highlight_line(src, group).unwrap_or_else(|| panic!("{variant} missing {group}"));
  let gui = hl_attr(line, "gui").unwrap_or_else(|| panic!("{variant} {group} missing gui="));
  let cterm = hl_attr(line, "cterm").unwrap_or_else(|| panic!("{variant} {group} missing cterm="));
  assert!(
    gui.split(',').any(|s| s == style),
    "{variant} {group} gui= should include {style}, got: {gui}"
  );
  assert!(
    cterm.split(',').any(|s| s == style),
    "{variant} {group} cterm= should include {style}, got: {cterm}"
  );
}

#[test]
fn dark_keywords_are_bold() {
  assert_style(DARK, "Keyword", "bold", "dark");
  assert_style(DARK, "Statement", "bold", "dark");
  assert_style(DARK, "Conditional", "bold", "dark");
}

#[test]
fn light_keywords_are_bold() {
  assert_style(LIGHT, "Keyword", "bold", "light");
  assert_style(LIGHT, "Statement", "bold", "light");
  assert_style(LIGHT, "Conditional", "bold", "light");
}

#[test]
fn dark_tags_are_bold() {
  assert_style(DARK, "Tag", "bold", "dark");
}

#[test]
fn light_tags_are_bold() {
  assert_style(LIGHT, "Tag", "bold", "light");
}

#[test]
fn dark_comments_are_italic() {
  assert_style(DARK, "Comment", "italic", "dark");
}

#[test]
fn light_comments_are_italic() {
  assert_style(LIGHT, "Comment", "italic", "light");
}

#[test]
fn dark_types_are_italic() {
  assert_style(DARK, "Type", "italic", "dark");
  assert_style(DARK, "Typedef", "italic", "dark");
  assert_style(DARK, "Structure", "italic", "dark");
}

#[test]
fn light_types_are_italic() {
  assert_style(LIGHT, "Type", "italic", "light");
  assert_style(LIGHT, "Typedef", "italic", "light");
  assert_style(LIGHT, "Structure", "italic", "light");
}

// -- Diagnostics use undercurl --

#[test]
fn dark_diagnostic_underlines_use_undercurl() {
  for group in [
    "DiagnosticUnderlineError",
    "DiagnosticUnderlineWarn",
    "DiagnosticUnderlineInfo",
    "DiagnosticUnderlineHint",
  ] {
    let line = highlight_line(DARK, group).unwrap_or_else(|| panic!("missing {group}"));
    let gui = hl_attr(line, "gui").unwrap();
    assert!(gui.contains("undercurl"), "dark {group} should use undercurl");
  }
}

#[test]
fn light_diagnostic_underlines_use_undercurl() {
  for group in [
    "DiagnosticUnderlineError",
    "DiagnosticUnderlineWarn",
    "DiagnosticUnderlineInfo",
    "DiagnosticUnderlineHint",
  ] {
    let line = highlight_line(LIGHT, group).unwrap_or_else(|| panic!("missing {group}"));
    let gui = hl_attr(line, "gui").unwrap();
    assert!(gui.contains("undercurl"), "light {group} should use undercurl");
  }
}

// -- Terminal ANSI: both variants declare 16 colors --

fn extract_terminal_ansi(src: &str) -> Vec<String> {
  let start = src
    .find("g:terminal_ansi_colors")
    .expect("missing g:terminal_ansi_colors");
  let block = &src[start..];
  let close = block.find(']').expect("missing closing bracket");
  let block = &block[..close];

  block
    .split('\'')
    .filter_map(|s| {
      let s = s.trim();
      if s.starts_with('#') && (s.len() == 7 || s.len() == 4) {
        Some(hex_to_lower(s))
      } else {
        None
      }
    })
    .collect()
}

#[test]
fn dark_terminal_has_16_colors() {
  let colors = extract_terminal_ansi(DARK);
  assert_eq!(colors.len(), 16, "dark terminal_ansi_colors must have 16 entries");
}

#[test]
fn light_terminal_has_16_colors() {
  let colors = extract_terminal_ansi(LIGHT);
  assert_eq!(colors.len(), 16, "light terminal_ansi_colors must have 16 entries");
}

// -- Terminal ANSI matches Ghostty/VS Code --

#[test]
fn dark_terminal_ansi_matches_ghostty() {
  let ghostty = include_str!("../ghostty/warm-burnout-dark");
  let vim_colors = extract_terminal_ansi(DARK);

  let ghostty_colors: Vec<String> = ghostty
    .lines()
    .filter(|l| l.starts_with("palette"))
    .filter_map(|l| {
      l.split_once('=')
        .and_then(|(_, v)| v.trim().split_once('='))
        .map(|(_, hex)| hex_to_lower(hex))
    })
    .collect();

  for (i, expected) in ghostty_colors.iter().enumerate() {
    assert_eq!(
      &vim_colors[i], expected,
      "dark terminal_color_{i}: vim={} ghostty={expected}",
      vim_colors[i]
    );
  }
}

#[test]
fn light_terminal_ansi_matches_vscode() {
  let vscode: serde_json::Value =
    serde_json::from_str(include_str!("../vscode/themes/warm-burnout-light.json")).unwrap();
  let vim_colors = extract_terminal_ansi(LIGHT);

  let ansi_keys = [
    "ansiBlack",
    "ansiRed",
    "ansiGreen",
    "ansiYellow",
    "ansiBlue",
    "ansiMagenta",
    "ansiCyan",
    "ansiWhite",
    "ansiBrightBlack",
    "ansiBrightRed",
    "ansiBrightGreen",
    "ansiBrightYellow",
    "ansiBrightBlue",
    "ansiBrightMagenta",
    "ansiBrightCyan",
    "ansiBrightWhite",
  ];

  for (i, ansi_key) in ansi_keys.iter().enumerate() {
    let vscode_key = format!("terminal.{ansi_key}");
    let vscode_hex = hex_to_lower(vscode["colors"][&vscode_key].as_str().unwrap());
    assert_eq!(
      vim_colors[i], vscode_hex,
      "light terminal_color_{i}: vim={} vscode={vscode_hex}",
      vim_colors[i]
    );
  }
}

// -- Every highlight sets both GUI and cterm attributes (with allowlisted exceptions) --
//
// Style-only groups like Underlined or plain gui=undercurl lines legitimately omit
// guifg/ctermfg. This test only checks foreground color groups.

fn color_highlight_lines(src: &str) -> impl Iterator<Item = &str> {
  src.lines().filter(|l| {
    let t = l.trim_start();
    (t.starts_with("hi ") || t.starts_with("highlight ")) && t.contains("guifg=#") && !t.contains("guifg=NONE")
  })
}

#[test]
fn dark_color_highlights_have_cterm_equivalents() {
  for line in color_highlight_lines(DARK) {
    assert!(
      line.contains("ctermfg="),
      "dark highlight missing ctermfg: {}",
      line.trim()
    );
  }
}

#[test]
fn light_color_highlights_have_cterm_equivalents() {
  for line in color_highlight_lines(LIGHT) {
    assert!(
      line.contains("ctermfg="),
      "light highlight missing ctermfg: {}",
      line.trim()
    );
  }
}
