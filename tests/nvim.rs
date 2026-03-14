mod common;

use std::process::Command;

use common::{extract_hex_colors, hex_to_lower, is_valid_hex, nvim_palette_color};

const PALETTE: &str = include_str!("../nvim/lua/warm-burnout/palette.lua");
const HIGHLIGHTS: &str = include_str!("../nvim/lua/warm-burnout/highlights.lua");
const TERMINAL: &str = include_str!("../nvim/lua/warm-burnout/terminal.lua");
const INIT: &str = include_str!("../nvim/lua/warm-burnout/init.lua");
const COLOR_DARK: &str = include_str!("../nvim/colors/warm-burnout-dark.lua");
const COLOR_LIGHT: &str = include_str!("../nvim/colors/warm-burnout-light.lua");

// -- Extract dark/light palette blocks --

fn dark_block() -> &'static str {
  let start = PALETTE.find("M.dark = {").expect("missing M.dark");
  let block = &PALETTE[start..];
  let end = block.find("\n}\n").expect("missing closing brace for M.dark");
  &block[..end + 2]
}

fn light_block() -> &'static str {
  let start = PALETTE.find("M.light = {").expect("missing M.light");
  let block = &PALETTE[start..];
  let end = block.find("\n}\n").expect("missing closing brace for M.light");
  &block[..end + 2]
}

fn extract_palette_keys(block: &str) -> Vec<String> {
  block
    .lines()
    .filter_map(|l| {
      let l = l.trim();
      if l.contains('=') && !l.starts_with("--") && !l.starts_with("M.") {
        l.split('=').next().map(|k| k.trim().to_string())
      } else {
        None
      }
    })
    .collect()
}

// -- Both palette variants exist --

#[test]
fn palette_has_dark_variant() {
  assert!(PALETTE.contains("M.dark = {"), "palette.lua missing M.dark");
}

#[test]
fn palette_has_light_variant() {
  assert!(PALETTE.contains("M.light = {"), "palette.lua missing M.light");
}

// -- All hex colors valid --

#[test]
fn dark_palette_hex_colors_valid() {
  for (line, hex) in extract_hex_colors(dark_block()) {
    assert!(is_valid_hex(hex), "dark palette line {line}: invalid hex: {hex}");
  }
}

#[test]
fn light_palette_hex_colors_valid() {
  for (line, hex) in extract_hex_colors(light_block()) {
    assert!(is_valid_hex(hex), "light palette line {line}: invalid hex: {hex}");
  }
}

#[test]
fn terminal_hex_colors_valid() {
  for (line, hex) in extract_hex_colors(TERMINAL) {
    assert!(is_valid_hex(hex), "terminal.lua line {line}: invalid hex: {hex}");
  }
}

// -- Canonical backgrounds --

#[test]
fn dark_background_is_canonical() {
  assert_eq!(nvim_palette_color(dark_block(), "bg"), "#1a1510");
}

#[test]
fn light_background_is_canonical() {
  assert_eq!(nvim_palette_color(light_block(), "bg"), "#f5ede0");
}

// -- Canonical foregrounds --

#[test]
fn dark_foreground_is_canonical() {
  assert_eq!(nvim_palette_color(dark_block(), "fg"), "#bfbdb6");
}

#[test]
fn light_foreground_is_canonical() {
  assert_eq!(nvim_palette_color(light_block(), "fg"), "#3a3630");
}

// -- Canonical cursor --

#[test]
fn dark_cursor_is_canonical() {
  assert_eq!(nvim_palette_color(dark_block(), "cursor"), "#f5c56e");
}

#[test]
fn light_cursor_is_canonical() {
  assert_eq!(nvim_palette_color(light_block(), "cursor"), "#8a6600");
}

// -- Canonical accent --

#[test]
fn dark_accent_is_canonical() {
  assert_eq!(nvim_palette_color(dark_block(), "accent"), "#b8522e");
}

#[test]
fn light_accent_is_canonical() {
  assert_eq!(nvim_palette_color(light_block(), "accent"), "#b8522e");
}

// -- No pure black/white backgrounds --

#[test]
fn no_pure_black_background() {
  let bg = nvim_palette_color(dark_block(), "bg");
  assert_ne!(bg, "#000000", "dark background must not be pure black");
}

#[test]
fn no_pure_white_background() {
  let bg = nvim_palette_color(light_block(), "bg");
  assert_ne!(bg, "#ffffff", "light background must not be pure white");
}

// -- Syntax colors match canonical palette --

#[test]
fn dark_syntax_colors_match_palette() {
  let block = dark_block();
  let expected = [
    ("keyword", "#ff8f40"),
    ("func", "#ffb454"),
    ("type", "#8aa8b8"),
    ("string", "#b4bc78"),
    ("comment", "#aea195"),
    ("number", "#d4a8b8"),
    ("operator", "#f29668"),
    ("tag", "#d49484"),
    ("regex", "#96b898"),
    ("decorator", "#e6c08a"),
    ("property", "#deb074"),
    ("member", "#f58088"),
  ];
  for (key, color) in expected {
    assert_eq!(
      nvim_palette_color(block, key),
      hex_to_lower(color),
      "dark palette {key} mismatch"
    );
  }
}

#[test]
fn light_syntax_colors_match_palette() {
  let block = light_block();
  let expected = [
    ("keyword", "#924800"),
    ("func", "#855700"),
    ("type", "#2a5868"),
    ("string", "#4d5c1a"),
    ("comment", "#5a5244"),
    ("number", "#7e4060"),
    ("operator", "#8f4418"),
    ("tag", "#8e4632"),
    ("regex", "#286a48"),
    ("decorator", "#7a5a1c"),
    ("property", "#74501c"),
    ("member", "#a02838"),
  ];
  for (key, color) in expected {
    assert_eq!(
      nvim_palette_color(block, key),
      hex_to_lower(color),
      "light palette {key} mismatch"
    );
  }
}

// -- Diagnostic colors --

#[test]
fn dark_diagnostic_colors() {
  let block = dark_block();
  assert_eq!(nvim_palette_color(block, "error"), "#f08888");
  assert_eq!(nvim_palette_color(block, "warn"), "#b8522e");
  assert_eq!(nvim_palette_color(block, "info"), "#8aa8b8");
  assert_eq!(nvim_palette_color(block, "hint"), "#aea195");
}

#[test]
fn light_diagnostic_colors() {
  let block = light_block();
  assert_eq!(nvim_palette_color(block, "error"), "#b03434");
  assert_eq!(nvim_palette_color(block, "warn"), "#b8522e");
  assert_eq!(nvim_palette_color(block, "info"), "#2a5868");
  assert_eq!(nvim_palette_color(block, "hint"), "#5a5244");
}

// -- Git diff colors --

#[test]
fn dark_git_colors() {
  let block = dark_block();
  assert_eq!(nvim_palette_color(block, "added"), "#70bf56");
  assert_eq!(nvim_palette_color(block, "modified"), "#73b8ff");
  assert_eq!(nvim_palette_color(block, "deleted"), "#f26d78");
}

#[test]
fn light_git_colors() {
  let block = light_block();
  assert_eq!(nvim_palette_color(block, "added"), "#226414");
  assert_eq!(nvim_palette_color(block, "modified"), "#2868a0");
  assert_eq!(nvim_palette_color(block, "deleted"), "#c43040");
}

// -- Both palettes have the same keys (structural parity) --

#[test]
fn dark_and_light_have_same_palette_keys() {
  let dark_keys = extract_palette_keys(dark_block());
  let light_keys = extract_palette_keys(light_block());
  assert_eq!(
    dark_keys, light_keys,
    "dark and light palettes should have identical keys"
  );
}

// -- Required palette keys present --

#[test]
fn dark_has_required_palette_keys() {
  let keys = extract_palette_keys(dark_block());
  for required in [
    "bg",
    "bg_dim",
    "bg_float",
    "bg_highlight",
    "bg_visual",
    "fg",
    "fg_dim",
    "fg_gutter",
    "comment",
    "cursor",
    "accent",
    "keyword",
    "func",
    "string",
    "type",
    "operator",
    "number",
    "error",
    "warn",
    "info",
    "hint",
    "added",
    "modified",
    "deleted",
  ] {
    assert!(
      keys.contains(&required.to_string()),
      "dark palette missing required key: {required}"
    );
  }
}

#[test]
fn light_has_required_palette_keys() {
  let keys = extract_palette_keys(light_block());
  for required in [
    "bg",
    "bg_dim",
    "bg_float",
    "bg_highlight",
    "bg_visual",
    "fg",
    "fg_dim",
    "fg_gutter",
    "comment",
    "cursor",
    "accent",
    "keyword",
    "func",
    "string",
    "type",
    "operator",
    "number",
    "error",
    "warn",
    "info",
    "hint",
    "added",
    "modified",
    "deleted",
  ] {
    assert!(
      keys.contains(&required.to_string()),
      "light palette missing required key: {required}"
    );
  }
}

// -- Terminal ANSI colors: 16 per variant --

#[test]
fn dark_terminal_has_16_colors() {
  let count = TERMINAL
    .lines()
    .filter(|l| {
      let l = l.trim();
      l.starts_with("vim.g.terminal_color_")
    })
    .count();
  // 16 colors in dark block + 16 in light block = 32 total
  assert_eq!(
    count, 32,
    "expected 32 terminal color assignments (16 per variant), got {count}"
  );
}

#[test]
fn dark_terminal_indices_complete() {
  // Find the dark block (first set of terminal_color assignments)
  let dark_section = TERMINAL.split("else").next().expect("missing dark terminal section");
  for i in 0..16 {
    let key = format!("vim.g.terminal_color_{i}");
    assert!(dark_section.contains(&key), "dark terminal missing {key}");
  }
}

#[test]
fn light_terminal_indices_complete() {
  let light_section = TERMINAL.split("else").nth(1).expect("missing light terminal section");
  for i in 0..16 {
    let key = format!("vim.g.terminal_color_{i}");
    assert!(light_section.contains(&key), "light terminal missing {key}");
  }
}

// -- Terminal ANSI colors match VS Code/Ghostty --

#[test]
fn dark_terminal_ansi_matches_ghostty() {
  let ghostty = include_str!("../ghostty/warm-burnout-dark");
  let dark_section = TERMINAL.split("else").next().unwrap();

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
    let key = format!("terminal_color_{i}");
    let nvim_line = dark_section
      .lines()
      .find(|l| l.contains(&key))
      .unwrap_or_else(|| panic!("missing {key} in dark terminal"));
    let nvim_hex = nvim_line
      .split('"')
      .nth(1)
      .map(|s| hex_to_lower(s))
      .unwrap_or_else(|| panic!("no hex value in {key}"));
    assert_eq!(
      nvim_hex, *expected,
      "dark terminal_color_{i}: nvim={nvim_hex} ghostty={expected}"
    );
  }
}

#[test]
fn light_terminal_ansi_matches_vscode() {
  let vscode: serde_json::Value =
    serde_json::from_str(include_str!("../vscode/themes/warm-burnout-light.json")).unwrap();
  let light_section = TERMINAL.split("else").nth(1).unwrap();

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

    let nvim_key = format!("terminal_color_{i}");
    let nvim_line = light_section
      .lines()
      .find(|l| l.contains(&nvim_key))
      .unwrap_or_else(|| panic!("missing {nvim_key} in light terminal"));
    let nvim_hex = nvim_line
      .split('"')
      .nth(1)
      .map(|s| hex_to_lower(s))
      .unwrap_or_else(|| panic!("no hex value in {nvim_key}"));
    assert_eq!(
      nvim_hex, vscode_hex,
      "light terminal_color_{i}: nvim={nvim_hex} vscode={vscode_hex}"
    );
  }
}

// -- Highlights file structure --

#[test]
fn highlights_has_editor_ui_groups() {
  for group in [
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
  ] {
    assert!(
      HIGHLIGHTS.contains(group),
      "highlights.lua missing editor UI group: {group}"
    );
  }
}

#[test]
fn highlights_has_legacy_syntax_groups() {
  for group in [
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
  ] {
    assert!(
      HIGHLIGHTS.contains(group),
      "highlights.lua missing syntax group: {group}"
    );
  }
}

#[test]
fn highlights_has_treesitter_captures() {
  for capture in [
    "@comment",
    "@string",
    "@keyword",
    "@function",
    "@type",
    "@variable",
    "@constant",
    "@number",
    "@boolean",
    "@operator",
    "@punctuation.bracket",
    "@punctuation.delimiter",
    "@tag",
    "@property",
    "@constructor",
    "@module",
  ] {
    assert!(
      HIGHLIGHTS.contains(&format!("[\"{capture}\"]")),
      "highlights.lua missing treesitter capture: {capture}"
    );
  }
}

#[test]
fn highlights_has_lsp_diagnostics() {
  for group in [
    "DiagnosticError",
    "DiagnosticWarn",
    "DiagnosticInfo",
    "DiagnosticHint",
    "DiagnosticVirtualTextError",
    "DiagnosticUnderlineError",
    "DiagnosticSignError",
  ] {
    assert!(
      HIGHLIGHTS.contains(group),
      "highlights.lua missing diagnostic group: {group}"
    );
  }
}

#[test]
fn highlights_has_lsp_semantic_tokens() {
  for group in [
    "@lsp.type.class",
    "@lsp.type.function",
    "@lsp.type.keyword",
    "@lsp.type.method",
    "@lsp.type.type",
    "@lsp.type.interface",
  ] {
    assert!(
      HIGHLIGHTS.contains(&format!("[\"{group}\"]")),
      "highlights.lua missing semantic token group: {group}"
    );
  }
}

#[test]
fn highlights_has_telescope_groups() {
  for group in [
    "TelescopeNormal",
    "TelescopeBorder",
    "TelescopeSelection",
    "TelescopeMatching",
    "TelescopePromptNormal",
  ] {
    assert!(
      HIGHLIGHTS.contains(group),
      "highlights.lua missing Telescope group: {group}"
    );
  }
}

#[test]
fn highlights_has_gitsigns_groups() {
  for group in ["GitSignsAdd", "GitSignsChange", "GitSignsDelete"] {
    assert!(
      HIGHLIGHTS.contains(group),
      "highlights.lua missing Gitsigns group: {group}"
    );
  }
}

#[test]
fn highlights_has_neotree_groups() {
  for group in [
    "NeoTreeNormal",
    "NeoTreeDirectoryName",
    "NeoTreeGitModified",
    "NeoTreeGitAdded",
    "NeoTreeGitDeleted",
  ] {
    assert!(
      HIGHLIGHTS.contains(group),
      "highlights.lua missing Neo-tree group: {group}"
    );
  }
}

#[test]
fn highlights_has_barbar_groups() {
  for group in ["BufferCurrent", "BufferVisible", "BufferInactive"] {
    assert!(
      HIGHLIGHTS.contains(group),
      "highlights.lua missing Barbar group: {group}"
    );
  }
}

#[test]
fn highlights_has_mini_statusline_groups() {
  for group in [
    "MiniStatuslineModeNormal",
    "MiniStatuslineModeInsert",
    "MiniStatuslineModeVisual",
  ] {
    assert!(
      HIGHLIGHTS.contains(group),
      "highlights.lua missing Mini.statusline group: {group}"
    );
  }
}

#[test]
fn highlights_has_whichkey_groups() {
  for group in ["WhichKey", "WhichKeyGroup", "WhichKeyDesc"] {
    assert!(
      HIGHLIGHTS.contains(group),
      "highlights.lua missing Which-key group: {group}"
    );
  }
}

#[test]
fn highlights_has_trouble_groups() {
  for group in ["TroubleNormal", "TroubleText", "TroubleCount"] {
    assert!(
      HIGHLIGHTS.contains(group),
      "highlights.lua missing Trouble group: {group}"
    );
  }
}

#[test]
fn highlights_has_flash_groups() {
  for group in ["FlashLabel", "FlashMatch", "FlashCurrent"] {
    assert!(
      HIGHLIGHTS.contains(group),
      "highlights.lua missing Flash group: {group}"
    );
  }
}

#[test]
fn highlights_has_fidget_groups() {
  for group in ["FidgetTitle", "FidgetTask"] {
    assert!(
      HIGHLIGHTS.contains(group),
      "highlights.lua missing Fidget group: {group}"
    );
  }
}

#[test]
fn highlights_has_illuminate_groups() {
  for group in ["IlluminatedWordText", "IlluminatedWordRead", "IlluminatedWordWrite"] {
    assert!(
      HIGHLIGHTS.contains(group),
      "highlights.lua missing Illuminate group: {group}"
    );
  }
}

// -- Font style system preserved --

#[test]
fn highlights_comments_are_italic() {
  let comment_line = HIGHLIGHTS
    .lines()
    .find(|l| l.trim().starts_with("Comment"))
    .expect("missing Comment highlight");
  assert!(comment_line.contains("italic = true"), "Comment should be italic");
}

#[test]
fn highlights_keywords_are_bold() {
  let keyword_line = HIGHLIGHTS
    .lines()
    .find(|l| l.trim().starts_with("Keyword"))
    .expect("missing Keyword highlight");
  assert!(keyword_line.contains("bold = true"), "Keyword should be bold");
}

#[test]
fn highlights_types_are_italic() {
  let type_line = HIGHLIGHTS
    .lines()
    .find(|l| {
      let l = l.trim();
      l.starts_with("Type ") || l.starts_with("Type=")
    })
    .expect("missing Type highlight");
  assert!(type_line.contains("italic = true"), "Type should be italic");
}

#[test]
fn highlights_treesitter_comment_is_italic() {
  // Find the @comment treesitter line (not @comment.documentation etc.)
  let lines: Vec<&str> = HIGHLIGHTS.lines().collect();
  let idx = lines
    .iter()
    .position(|l| l.contains("[\"@comment\"]"))
    .expect("missing @comment treesitter capture");
  assert!(
    lines[idx].contains("italic = true"),
    "@comment treesitter capture should be italic"
  );
}

#[test]
fn highlights_treesitter_keyword_is_bold() {
  let lines: Vec<&str> = HIGHLIGHTS.lines().collect();
  let idx = lines
    .iter()
    .position(|l| l.contains("[\"@keyword\"]") && !l.contains("@keyword."))
    .expect("missing @keyword treesitter capture");
  assert!(
    lines[idx].contains("bold = true"),
    "@keyword treesitter capture should be bold"
  );
}

#[test]
fn highlights_treesitter_type_is_italic() {
  let lines: Vec<&str> = HIGHLIGHTS.lines().collect();
  let idx = lines
    .iter()
    .position(|l| l.contains("[\"@type\"]") && !l.contains("@type."))
    .expect("missing @type treesitter capture");
  assert!(
    lines[idx].contains("italic = true"),
    "@type treesitter capture should be italic"
  );
}

// -- Diagnostics use undercurl --

#[test]
fn diagnostic_underlines_use_undercurl() {
  for group in [
    "DiagnosticUnderlineError",
    "DiagnosticUnderlineWarn",
    "DiagnosticUnderlineInfo",
    "DiagnosticUnderlineHint",
  ] {
    let line = HIGHLIGHTS
      .lines()
      .find(|l| l.contains(group))
      .unwrap_or_else(|| panic!("missing {group}"));
    assert!(line.contains("undercurl = true"), "{group} should use undercurl");
  }
}

// -- Init file structure --

#[test]
fn init_sets_termguicolors() {
  assert!(
    INIT.contains("vim.o.termguicolors = true"),
    "init.lua should set termguicolors"
  );
}

#[test]
fn init_sets_colors_name() {
  assert!(
    INIT.contains("vim.g.colors_name"),
    "init.lua should set vim.g.colors_name"
  );
}

#[test]
fn init_clears_highlights() {
  assert!(INIT.contains("hi clear"), "init.lua should clear existing highlights");
}

#[test]
fn init_loads_palette_module() {
  assert!(
    INIT.contains("require(\"warm-burnout.palette\")"),
    "init.lua should load palette module"
  );
}

#[test]
fn init_loads_highlights_module() {
  assert!(
    INIT.contains("require(\"warm-burnout.highlights\")"),
    "init.lua should load highlights module"
  );
}

#[test]
fn init_loads_terminal_module() {
  assert!(
    INIT.contains("require(\"warm-burnout.terminal\")"),
    "init.lua should load terminal module"
  );
}

#[test]
fn init_has_setup_function() {
  assert!(
    INIT.contains("function M.setup"),
    "init.lua should export setup function"
  );
}

#[test]
fn init_has_load_function() {
  assert!(INIT.contains("function M.load"), "init.lua should export load function");
}

// -- Color entry points --

#[test]
fn color_dark_loads_dark_variant() {
  assert!(
    COLOR_DARK.contains("load(\"dark\")"),
    "warm-burnout-dark.lua should load dark variant"
  );
}

#[test]
fn color_light_loads_light_variant() {
  assert!(
    COLOR_LIGHT.contains("load(\"light\")"),
    "warm-burnout-light.lua should load light variant"
  );
}

// -- Highlights uses palette references, not raw hex --

#[test]
fn highlights_uses_palette_references() {
  // highlights.lua should reference palette keys (p.fg, p.bg, etc.)
  // and should NOT contain raw hex values
  let hex_colors = extract_hex_colors(HIGHLIGHTS);
  assert!(
    hex_colors.is_empty(),
    "highlights.lua should not contain raw hex values, found {} occurrences: {:?}",
    hex_colors.len(),
    hex_colors.iter().take(5).collect::<Vec<_>>()
  );
}

// -- Selection color matches VS Code/Zed --

#[test]
fn dark_selection_matches_vscode() {
  let block = dark_block();
  let selection = nvim_palette_color(block, "bg_visual");
  assert_eq!(
    selection, "#8aa8b840",
    "dark selection should be steel patina with alpha"
  );
}

#[test]
fn light_selection_matches_vscode() {
  let block = light_block();
  let selection = nvim_palette_color(block, "bg_visual");
  assert_eq!(
    selection, "#8aa8b840",
    "light selection should be steel patina with alpha"
  );
}

// -- Init resolves alpha hex colors --

#[test]
fn init_has_alpha_blend_function() {
  assert!(
    INIT.contains("blend_alpha"),
    "init.lua should have alpha blending function"
  );
}

#[test]
fn init_resolves_palette_before_applying() {
  assert!(
    INIT.contains("resolve_palette"),
    "init.lua should resolve palette (blend alpha hex) before applying highlights"
  );
}

// -- Headless Neovim load tests --
// These tests require `nvim` to be installed. They run the colorscheme in
// headless mode and verify it loads without errors.

fn nvim_available() -> bool {
  Command::new("nvim").arg("--version").output().is_ok()
}

fn run_nvim_load_test(variant: &str) -> (bool, String) {
  let manifest_dir = env!("CARGO_MANIFEST_DIR");
  let rtp_path = format!("{manifest_dir}/nvim");
  let test_script = format!("{manifest_dir}/tests/nvim_load_test.lua");

  let output = Command::new("nvim")
    .args([
      "--headless",
      "-u",
      "NONE",
      "--cmd",
      &format!("set rtp+={rtp_path}"),
      "-l",
      &test_script,
      variant,
    ])
    .output()
    .expect("failed to run nvim");

  let stdout = String::from_utf8_lossy(&output.stdout).to_string();
  let stderr = String::from_utf8_lossy(&output.stderr).to_string();
  let combined = format!("{stdout}{stderr}");
  (output.status.success(), combined)
}

#[test]
fn headless_nvim_loads_dark_variant() {
  if !nvim_available() {
    eprintln!("skipping: nvim not found");
    return;
  }
  let (success, output) = run_nvim_load_test("dark");
  assert!(success, "dark variant failed to load in nvim:\n{output}");
  assert!(
    output.contains("OK:warm-burnout-dark"),
    "dark variant did not produce expected output:\n{output}"
  );
}

#[test]
fn headless_nvim_loads_light_variant() {
  if !nvim_available() {
    eprintln!("skipping: nvim not found");
    return;
  }
  let (success, output) = run_nvim_load_test("light");
  assert!(success, "light variant failed to load in nvim:\n{output}");
  assert!(
    output.contains("OK:warm-burnout-light"),
    "light variant did not produce expected output:\n{output}"
  );
}
