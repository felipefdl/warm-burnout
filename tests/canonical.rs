mod common;

use common::{
  alacritty_color, emacs_palette_color, ghostty_ansi_color, ghostty_color, helix_palette_color, hex_to_lower,
  home_assistant_color, iterm2_color, jetbrains_attribute, jetbrains_color, nvim_palette_color, obsidian_color,
  starship_palette_color, tmux_option_value, tmux_style_bg, tmux_style_fg, vscode_color, warp_ansi_color, warp_color,
  windows_terminal_color, xcode_color, xcode_syntax_color, zed_editor_color, zellij_color,
};

fn zsh_foreground(src: &str) -> Option<String> {
  zsh_style_fg(src, "default")
}

fn zsh_style_fg(src: &str, style_key: &str) -> Option<String> {
  let pattern = format!("[{style_key}]");
  src.lines().find(|l| l.contains(&pattern)).and_then(|l| {
    l.split("fg=").nth(1).map(|s| {
      let hex = s.split('\'').next().unwrap_or(s).split(',').next().unwrap_or(s);
      hex_to_lower(hex)
    })
  })
}

// -- Background matches across all platforms --

#[test]
fn dark_background_vscode_matches_ghostty() {
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-dark.json"),
    "editor.background",
  );
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "background");
  assert_eq!(vscode, ghostty, "dark background: vscode={vscode} ghostty={ghostty}");
}

#[test]
fn light_background_vscode_matches_ghostty() {
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-light.json"),
    "editor.background",
  );
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "background");
  assert_eq!(vscode, ghostty, "light background: vscode={vscode} ghostty={ghostty}");
}

#[test]
fn dark_background_ghostty_matches_starship() {
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "background");
  let starship = starship_palette_color(
    include_str!("../starship/warm-burnout-dark.toml"),
    "warm_burnout_dark",
    "background",
  );
  assert_eq!(
    ghostty, starship,
    "dark background: ghostty={ghostty} starship={starship}"
  );
}

#[test]
fn light_background_ghostty_matches_starship() {
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "background");
  let starship = starship_palette_color(
    include_str!("../starship/warm-burnout-light.toml"),
    "warm_burnout_light",
    "background",
  );
  assert_eq!(
    ghostty, starship,
    "light background: ghostty={ghostty} starship={starship}"
  );
}

// -- Foreground matches across all platforms --

#[test]
fn dark_foreground_vscode_matches_ghostty() {
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-dark.json"),
    "editor.foreground",
  );
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "foreground");
  assert_eq!(vscode, ghostty, "dark foreground: vscode={vscode} ghostty={ghostty}");
}

#[test]
fn light_foreground_vscode_matches_ghostty() {
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-light.json"),
    "editor.foreground",
  );
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "foreground");
  assert_eq!(vscode, ghostty, "light foreground: vscode={vscode} ghostty={ghostty}");
}

#[test]
fn dark_foreground_ghostty_matches_starship() {
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "foreground");
  let starship = starship_palette_color(
    include_str!("../starship/warm-burnout-dark.toml"),
    "warm_burnout_dark",
    "foreground",
  );
  assert_eq!(
    ghostty, starship,
    "dark foreground: ghostty={ghostty} starship={starship}"
  );
}

#[test]
fn light_foreground_ghostty_matches_starship() {
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "foreground");
  let starship = starship_palette_color(
    include_str!("../starship/warm-burnout-light.toml"),
    "warm_burnout_light",
    "foreground",
  );
  assert_eq!(
    ghostty, starship,
    "light foreground: ghostty={ghostty} starship={starship}"
  );
}

#[test]
fn dark_foreground_ghostty_matches_zsh() {
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "foreground");
  let zsh = zsh_foreground(include_str!("../zsh/warm-burnout-dark.zsh-theme")).expect("no foreground in zsh dark");
  assert_eq!(ghostty, zsh, "dark foreground: ghostty={ghostty} zsh={zsh}");
}

#[test]
fn light_foreground_ghostty_matches_zsh() {
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "foreground");
  let zsh = zsh_foreground(include_str!("../zsh/warm-burnout-light.zsh-theme")).expect("no foreground in zsh light");
  assert_eq!(ghostty, zsh, "light foreground: ghostty={ghostty} zsh={zsh}");
}

// -- Background/foreground matches: vscode <-> zed --

#[test]
fn dark_background_vscode_matches_zed() {
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-dark.json"),
    "editor.background",
  );
  let zed = zed_editor_color(
    include_str!("../zed/themes/warm-burnout.json"),
    "Warm Burnout Dark",
    "editor.background",
  );
  assert_eq!(vscode, zed, "dark background: vscode={vscode} zed={zed}");
}

#[test]
fn light_background_vscode_matches_zed() {
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-light.json"),
    "editor.background",
  );
  let zed = zed_editor_color(
    include_str!("../zed/themes/warm-burnout.json"),
    "Warm Burnout Light",
    "editor.background",
  );
  assert_eq!(vscode, zed, "light background: vscode={vscode} zed={zed}");
}

#[test]
fn dark_foreground_vscode_matches_zed() {
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-dark.json"),
    "editor.foreground",
  );
  let zed = zed_editor_color(
    include_str!("../zed/themes/warm-burnout.json"),
    "Warm Burnout Dark",
    "editor.foreground",
  );
  assert_eq!(vscode, zed, "dark foreground: vscode={vscode} zed={zed}");
}

#[test]
fn light_foreground_vscode_matches_zed() {
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-light.json"),
    "editor.foreground",
  );
  let zed = zed_editor_color(
    include_str!("../zed/themes/warm-burnout.json"),
    "Warm Burnout Light",
    "editor.foreground",
  );
  assert_eq!(vscode, zed, "light foreground: vscode={vscode} zed={zed}");
}

// -- Background/foreground matches: zed <-> ghostty --

#[test]
fn dark_background_zed_matches_ghostty() {
  let zed = zed_editor_color(
    include_str!("../zed/themes/warm-burnout.json"),
    "Warm Burnout Dark",
    "editor.background",
  );
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "background");
  assert_eq!(zed, ghostty, "dark background: zed={zed} ghostty={ghostty}");
}

#[test]
fn light_background_zed_matches_ghostty() {
  let zed = zed_editor_color(
    include_str!("../zed/themes/warm-burnout.json"),
    "Warm Burnout Light",
    "editor.background",
  );
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "background");
  assert_eq!(zed, ghostty, "light background: zed={zed} ghostty={ghostty}");
}

// -- Cursor color matches between ghostty and starship --

#[test]
fn dark_cursor_ghostty_matches_starship() {
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "cursor-color");
  let starship = starship_palette_color(
    include_str!("../starship/warm-burnout-dark.toml"),
    "warm_burnout_dark",
    "cursor",
  );
  assert_eq!(ghostty, starship, "dark cursor: ghostty={ghostty} starship={starship}");
}

#[test]
fn light_cursor_ghostty_matches_starship() {
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "cursor-color");
  let starship = starship_palette_color(
    include_str!("../starship/warm-burnout-light.toml"),
    "warm_burnout_light",
    "cursor",
  );
  assert_eq!(ghostty, starship, "light cursor: ghostty={ghostty} starship={starship}");
}

// -- Neovim cross-platform consistency --

fn nvim_dark_block() -> &'static str {
  let src = include_str!("../nvim/lua/warm-burnout/palette.lua");
  let start = src.find("M.dark = {").expect("missing M.dark");
  let block = &src[start..];
  let end = block.find("\n}\n").expect("missing closing brace for M.dark");
  // SAFETY: returning a slice of a &'static str
  unsafe { &*((&block[..end + 2]) as *const str) }
}

fn nvim_light_block() -> &'static str {
  let src = include_str!("../nvim/lua/warm-burnout/palette.lua");
  let start = src.find("M.light = {").expect("missing M.light");
  let block = &src[start..];
  let end = block.find("\n}\n").expect("missing closing brace for M.light");
  unsafe { &*((&block[..end + 2]) as *const str) }
}

#[test]
fn dark_background_vscode_matches_nvim() {
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-dark.json"),
    "editor.background",
  );
  let nvim = nvim_palette_color(nvim_dark_block(), "bg");
  assert_eq!(vscode, nvim, "dark background: vscode={vscode} nvim={nvim}");
}

#[test]
fn light_background_vscode_matches_nvim() {
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-light.json"),
    "editor.background",
  );
  let nvim = nvim_palette_color(nvim_light_block(), "bg");
  assert_eq!(vscode, nvim, "light background: vscode={vscode} nvim={nvim}");
}

#[test]
fn dark_foreground_vscode_matches_nvim() {
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-dark.json"),
    "editor.foreground",
  );
  let nvim = nvim_palette_color(nvim_dark_block(), "fg");
  assert_eq!(vscode, nvim, "dark foreground: vscode={vscode} nvim={nvim}");
}

#[test]
fn light_foreground_vscode_matches_nvim() {
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-light.json"),
    "editor.foreground",
  );
  let nvim = nvim_palette_color(nvim_light_block(), "fg");
  assert_eq!(vscode, nvim, "light foreground: vscode={vscode} nvim={nvim}");
}

#[test]
fn dark_background_nvim_matches_ghostty() {
  let nvim = nvim_palette_color(nvim_dark_block(), "bg");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "background");
  assert_eq!(nvim, ghostty, "dark background: nvim={nvim} ghostty={ghostty}");
}

#[test]
fn light_background_nvim_matches_ghostty() {
  let nvim = nvim_palette_color(nvim_light_block(), "bg");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "background");
  assert_eq!(nvim, ghostty, "light background: nvim={nvim} ghostty={ghostty}");
}

#[test]
fn dark_foreground_nvim_matches_ghostty() {
  let nvim = nvim_palette_color(nvim_dark_block(), "fg");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "foreground");
  assert_eq!(nvim, ghostty, "dark foreground: nvim={nvim} ghostty={ghostty}");
}

#[test]
fn light_foreground_nvim_matches_ghostty() {
  let nvim = nvim_palette_color(nvim_light_block(), "fg");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "foreground");
  assert_eq!(nvim, ghostty, "light foreground: nvim={nvim} ghostty={ghostty}");
}

// -- Vim cross-platform consistency --
//
// Vim mirrors the nvim platform but is self-contained vimscript. Pin its
// background, foreground, and cursor to the nvim palette so the warm core
// can never drift between the two editors.

const VIM_DARK: &str = include_str!("../vim/colors/warm-burnout-dark.vim");
const VIM_LIGHT: &str = include_str!("../vim/colors/warm-burnout-light.vim");

fn vim_highlight_attr(src: &str, group: &str, key: &str) -> String {
  for line in src.lines() {
    let t = line.trim_start();
    let Some(rest) = t.strip_prefix("highlight ").or_else(|| t.strip_prefix("hi ")) else {
      continue;
    };
    let mut tokens = rest.split_whitespace();
    if tokens.next() != Some(group) {
      continue;
    }
    for token in tokens {
      if let Some(val) = token.strip_prefix(&format!("{key}=")) {
        return hex_to_lower(val);
      }
    }
  }
  panic!("no {key} for {group} in vim source");
}

#[test]
fn dark_background_vim_matches_nvim() {
  let vim = vim_highlight_attr(VIM_DARK, "Normal", "guibg");
  let nvim = nvim_palette_color(nvim_dark_block(), "bg");
  assert_eq!(vim, nvim, "dark background: vim={vim} nvim={nvim}");
}

#[test]
fn light_background_vim_matches_nvim() {
  let vim = vim_highlight_attr(VIM_LIGHT, "Normal", "guibg");
  let nvim = nvim_palette_color(nvim_light_block(), "bg");
  assert_eq!(vim, nvim, "light background: vim={vim} nvim={nvim}");
}

#[test]
fn dark_foreground_vim_matches_nvim() {
  let vim = vim_highlight_attr(VIM_DARK, "Normal", "guifg");
  let nvim = nvim_palette_color(nvim_dark_block(), "fg");
  assert_eq!(vim, nvim, "dark foreground: vim={vim} nvim={nvim}");
}

#[test]
fn light_foreground_vim_matches_nvim() {
  let vim = vim_highlight_attr(VIM_LIGHT, "Normal", "guifg");
  let nvim = nvim_palette_color(nvim_light_block(), "fg");
  assert_eq!(vim, nvim, "light foreground: vim={vim} nvim={nvim}");
}

#[test]
fn dark_cursor_vim_matches_nvim() {
  let vim = vim_highlight_attr(VIM_DARK, "Cursor", "guibg");
  let nvim = nvim_palette_color(nvim_dark_block(), "cursor");
  assert_eq!(vim, nvim, "dark cursor: vim={vim} nvim={nvim}");
}

#[test]
fn light_cursor_vim_matches_nvim() {
  let vim = vim_highlight_attr(VIM_LIGHT, "Cursor", "guibg");
  let nvim = nvim_palette_color(nvim_light_block(), "cursor");
  assert_eq!(vim, nvim, "light cursor: vim={vim} nvim={nvim}");
}

#[test]
fn dark_background_vim_matches_ghostty() {
  let vim = vim_highlight_attr(VIM_DARK, "Normal", "guibg");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "background");
  assert_eq!(vim, ghostty, "dark background: vim={vim} ghostty={ghostty}");
}

#[test]
fn light_background_vim_matches_ghostty() {
  let vim = vim_highlight_attr(VIM_LIGHT, "Normal", "guibg");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "background");
  assert_eq!(vim, ghostty, "light background: vim={vim} ghostty={ghostty}");
}

// -- Background/foreground matches: vscode <-> xcode --

#[test]
fn dark_background_vscode_matches_xcode() {
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-dark.json"),
    "editor.background",
  );
  let xcode = xcode_color(
    include_str!("../xcode/Warm Burnout Dark.xccolortheme"),
    "DVTSourceTextBackground",
  );
  assert_eq!(vscode, xcode, "dark background: vscode={vscode} xcode={xcode}");
}

#[test]
fn light_background_vscode_matches_xcode() {
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-light.json"),
    "editor.background",
  );
  let xcode = xcode_color(
    include_str!("../xcode/Warm Burnout Light.xccolortheme"),
    "DVTSourceTextBackground",
  );
  assert_eq!(vscode, xcode, "light background: vscode={vscode} xcode={xcode}");
}

#[test]
fn dark_foreground_vscode_matches_xcode() {
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-dark.json"),
    "editor.foreground",
  );
  let xcode = xcode_syntax_color(
    include_str!("../xcode/Warm Burnout Dark.xccolortheme"),
    "xcode.syntax.plain",
  );
  assert_eq!(vscode, xcode, "dark foreground: vscode={vscode} xcode={xcode}");
}

#[test]
fn light_foreground_vscode_matches_xcode() {
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-light.json"),
    "editor.foreground",
  );
  let xcode = xcode_syntax_color(
    include_str!("../xcode/Warm Burnout Light.xccolortheme"),
    "xcode.syntax.plain",
  );
  assert_eq!(vscode, xcode, "light foreground: vscode={vscode} xcode={xcode}");
}

#[test]
fn dark_cursor_nvim_matches_ghostty() {
  let nvim = nvim_palette_color(nvim_dark_block(), "cursor");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "cursor-color");
  assert_eq!(nvim, ghostty, "dark cursor: nvim={nvim} ghostty={ghostty}");
}

#[test]
fn light_cursor_nvim_matches_ghostty() {
  let nvim = nvim_palette_color(nvim_light_block(), "cursor");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "cursor-color");
  assert_eq!(nvim, ghostty, "light cursor: nvim={nvim} ghostty={ghostty}");
}

#[test]
fn dark_background_nvim_matches_zed() {
  let nvim = nvim_palette_color(nvim_dark_block(), "bg");
  let zed = zed_editor_color(
    include_str!("../zed/themes/warm-burnout.json"),
    "Warm Burnout Dark",
    "editor.background",
  );
  assert_eq!(nvim, zed, "dark background: nvim={nvim} zed={zed}");
}

#[test]
fn light_background_nvim_matches_zed() {
  let nvim = nvim_palette_color(nvim_light_block(), "bg");
  let zed = zed_editor_color(
    include_str!("../zed/themes/warm-burnout.json"),
    "Warm Burnout Light",
    "editor.background",
  );
  assert_eq!(nvim, zed, "light background: nvim={nvim} zed={zed}");
}

// -- Alacritty cross-platform consistency --
//
// Alacritty mirrors the Ghostty palette verbatim. Pin background, foreground,
// and cursor to the nvim palette and Ghostty so the warm core can never drift.

const ALACRITTY_DARK: &str = include_str!("../alacritty/warm-burnout-dark.toml");
const ALACRITTY_LIGHT: &str = include_str!("../alacritty/warm-burnout-light.toml");

#[test]
fn dark_background_alacritty_matches_nvim() {
  let alac = alacritty_color(ALACRITTY_DARK, "colors.primary.background");
  let nvim = nvim_palette_color(nvim_dark_block(), "bg");
  assert_eq!(alac, nvim, "dark background: alacritty={alac} nvim={nvim}");
}

#[test]
fn light_background_alacritty_matches_nvim() {
  let alac = alacritty_color(ALACRITTY_LIGHT, "colors.primary.background");
  let nvim = nvim_palette_color(nvim_light_block(), "bg");
  assert_eq!(alac, nvim, "light background: alacritty={alac} nvim={nvim}");
}

#[test]
fn dark_foreground_alacritty_matches_nvim() {
  let alac = alacritty_color(ALACRITTY_DARK, "colors.primary.foreground");
  let nvim = nvim_palette_color(nvim_dark_block(), "fg");
  assert_eq!(alac, nvim, "dark foreground: alacritty={alac} nvim={nvim}");
}

#[test]
fn light_foreground_alacritty_matches_nvim() {
  let alac = alacritty_color(ALACRITTY_LIGHT, "colors.primary.foreground");
  let nvim = nvim_palette_color(nvim_light_block(), "fg");
  assert_eq!(alac, nvim, "light foreground: alacritty={alac} nvim={nvim}");
}

#[test]
fn dark_cursor_alacritty_matches_nvim() {
  let alac = alacritty_color(ALACRITTY_DARK, "colors.cursor.cursor");
  let nvim = nvim_palette_color(nvim_dark_block(), "cursor");
  assert_eq!(alac, nvim, "dark cursor: alacritty={alac} nvim={nvim}");
}

#[test]
fn light_cursor_alacritty_matches_nvim() {
  let alac = alacritty_color(ALACRITTY_LIGHT, "colors.cursor.cursor");
  let nvim = nvim_palette_color(nvim_light_block(), "cursor");
  assert_eq!(alac, nvim, "light cursor: alacritty={alac} nvim={nvim}");
}

#[test]
fn dark_background_alacritty_matches_ghostty() {
  let alac = alacritty_color(ALACRITTY_DARK, "colors.primary.background");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "background");
  assert_eq!(alac, ghostty, "dark background: alacritty={alac} ghostty={ghostty}");
}

#[test]
fn light_background_alacritty_matches_ghostty() {
  let alac = alacritty_color(ALACRITTY_LIGHT, "colors.primary.background");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "background");
  assert_eq!(alac, ghostty, "light background: alacritty={alac} ghostty={ghostty}");
}

#[test]
fn dark_selection_alacritty_matches_ghostty() {
  let alac = alacritty_color(ALACRITTY_DARK, "colors.selection.background");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "selection-background");
  assert_eq!(alac, ghostty, "dark selection: alacritty={alac} ghostty={ghostty}");
}

#[test]
fn light_selection_alacritty_matches_ghostty() {
  let alac = alacritty_color(ALACRITTY_LIGHT, "colors.selection.background");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "selection-background");
  assert_eq!(alac, ghostty, "light selection: alacritty={alac} ghostty={ghostty}");
}

// -- tmux cross-platform consistency --

#[test]
fn dark_tmux_message_fg_matches_ghostty_foreground() {
  let tmux_msg = tmux_option_value(include_str!("../tmux/warm-burnout-dark.conf"), "message-style");
  let tmux_fg = tmux_style_fg(&tmux_msg);
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "foreground");
  assert_eq!(
    tmux_fg, ghostty,
    "dark tmux message fg={tmux_fg} should match ghostty foreground={ghostty}"
  );
}

#[test]
fn light_tmux_message_fg_matches_ghostty_foreground() {
  let tmux_msg = tmux_option_value(include_str!("../tmux/warm-burnout-light.conf"), "message-style");
  let tmux_fg = tmux_style_fg(&tmux_msg);
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "foreground");
  assert_eq!(
    tmux_fg, ghostty,
    "light tmux message fg={tmux_fg} should match ghostty foreground={ghostty}"
  );
}

#[test]
fn dark_tmux_accent_matches_canonical() {
  let tmux_border = tmux_option_value(
    include_str!("../tmux/warm-burnout-dark.conf"),
    "pane-active-border-style",
  );
  let accent = tmux_style_fg(&tmux_border);
  assert_eq!(accent, "#b8522e", "dark tmux accent should be canonical copper rust");
}

#[test]
fn light_tmux_accent_matches_canonical() {
  let tmux_border = tmux_option_value(
    include_str!("../tmux/warm-burnout-light.conf"),
    "pane-active-border-style",
  );
  let accent = tmux_style_fg(&tmux_border);
  assert_eq!(accent, "#b8522e", "light tmux accent should be canonical copper rust");
}

// -- Zellij cross-platform consistency --

#[test]
fn dark_zellij_active_ribbon_matches_tmux_active_window() {
  let zellij_bg = zellij_color(
    include_str!("../zellij/warm-burnout-dark.kdl"),
    "warm-burnout-dark",
    "ribbon_selected",
    "background",
  );
  let tmux_active = tmux_option_value(
    include_str!("../tmux/warm-burnout-dark.conf"),
    "window-status-current-style",
  );
  let tmux_bg = tmux_style_bg(&tmux_active);
  assert_eq!(
    zellij_bg, tmux_bg,
    "dark zellij ribbon_selected.background should match tmux active window bg"
  );
}

#[test]
fn light_zellij_active_ribbon_matches_tmux_active_window() {
  let zellij_bg = zellij_color(
    include_str!("../zellij/warm-burnout-light.kdl"),
    "warm-burnout-light",
    "ribbon_selected",
    "background",
  );
  let tmux_active = tmux_option_value(
    include_str!("../tmux/warm-burnout-light.conf"),
    "window-status-current-style",
  );
  let tmux_bg = tmux_style_bg(&tmux_active);
  assert_eq!(
    zellij_bg, tmux_bg,
    "light zellij ribbon_selected.background should match tmux active window bg"
  );
}

#[test]
fn dark_zellij_bar_bg_matches_ghostty_terminal_bg() {
  let zellij_bar = zellij_color(
    include_str!("../zellij/warm-burnout-dark.kdl"),
    "warm-burnout-dark",
    "text_unselected",
    "background",
  );
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "background");
  assert_eq!(
    zellij_bar, ghostty,
    "dark zellij status bar bg must match ghostty terminal bg so the bar feels integrated"
  );
}

#[test]
fn light_zellij_bar_bg_matches_ghostty_terminal_bg() {
  let zellij_bar = zellij_color(
    include_str!("../zellij/warm-burnout-light.kdl"),
    "warm-burnout-light",
    "text_unselected",
    "background",
  );
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "background");
  assert_eq!(
    zellij_bar, ghostty,
    "light zellij status bar bg must match ghostty terminal bg so the bar feels integrated"
  );
}

#[test]
fn dark_zellij_focused_frame_matches_tmux_active_pane_border() {
  let zellij_frame = zellij_color(
    include_str!("../zellij/warm-burnout-dark.kdl"),
    "warm-burnout-dark",
    "frame_selected",
    "base",
  );
  let tmux_border = tmux_option_value(
    include_str!("../tmux/warm-burnout-dark.conf"),
    "pane-active-border-style",
  );
  let tmux_fg = tmux_style_fg(&tmux_border);
  assert_eq!(
    zellij_frame, tmux_fg,
    "dark zellij frame_selected should match tmux active pane border"
  );
}

#[test]
fn light_zellij_focused_frame_matches_tmux_active_pane_border() {
  let zellij_frame = zellij_color(
    include_str!("../zellij/warm-burnout-light.kdl"),
    "warm-burnout-light",
    "frame_selected",
    "base",
  );
  let tmux_border = tmux_option_value(
    include_str!("../tmux/warm-burnout-light.conf"),
    "pane-active-border-style",
  );
  let tmux_fg = tmux_style_fg(&tmux_border);
  assert_eq!(
    zellij_frame, tmux_fg,
    "light zellij frame_selected should match tmux active pane border"
  );
}

#[test]
fn dark_zellij_accent_matches_canonical() {
  let zellij_frame = zellij_color(
    include_str!("../zellij/warm-burnout-dark.kdl"),
    "warm-burnout-dark",
    "frame_selected",
    "base",
  );
  assert_eq!(
    zellij_frame, "#b8522e",
    "dark zellij accent should be canonical copper rust"
  );
}

#[test]
fn light_zellij_accent_matches_canonical() {
  let zellij_frame = zellij_color(
    include_str!("../zellij/warm-burnout-light.kdl"),
    "warm-burnout-light",
    "frame_selected",
    "base",
  );
  assert_eq!(
    zellij_frame, "#b8522e",
    "light zellij accent should be canonical copper rust"
  );
}

// -- iTerm2 cross-platform consistency --

#[test]
fn dark_background_iterm2_matches_ghostty() {
  let iterm2 = iterm2_color(
    include_str!("../iterm2/Warm Burnout Dark.itermcolors"),
    "Background Color",
  );
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "background");
  assert_eq!(iterm2, ghostty, "dark background: iterm2={iterm2} ghostty={ghostty}");
}

#[test]
fn light_background_iterm2_matches_ghostty() {
  let iterm2 = iterm2_color(
    include_str!("../iterm2/Warm Burnout Light.itermcolors"),
    "Background Color",
  );
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "background");
  assert_eq!(iterm2, ghostty, "light background: iterm2={iterm2} ghostty={ghostty}");
}

#[test]
fn dark_foreground_iterm2_matches_ghostty() {
  let iterm2 = iterm2_color(
    include_str!("../iterm2/Warm Burnout Dark.itermcolors"),
    "Foreground Color",
  );
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "foreground");
  assert_eq!(iterm2, ghostty, "dark foreground: iterm2={iterm2} ghostty={ghostty}");
}

#[test]
fn light_foreground_iterm2_matches_ghostty() {
  let iterm2 = iterm2_color(
    include_str!("../iterm2/Warm Burnout Light.itermcolors"),
    "Foreground Color",
  );
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "foreground");
  assert_eq!(iterm2, ghostty, "light foreground: iterm2={iterm2} ghostty={ghostty}");
}

#[test]
fn dark_cursor_iterm2_matches_ghostty() {
  let iterm2 = iterm2_color(include_str!("../iterm2/Warm Burnout Dark.itermcolors"), "Cursor Color");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "cursor-color");
  assert_eq!(iterm2, ghostty, "dark cursor: iterm2={iterm2} ghostty={ghostty}");
}

#[test]
fn light_cursor_iterm2_matches_ghostty() {
  let iterm2 = iterm2_color(include_str!("../iterm2/Warm Burnout Light.itermcolors"), "Cursor Color");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "cursor-color");
  assert_eq!(iterm2, ghostty, "light cursor: iterm2={iterm2} ghostty={ghostty}");
}

// -- JetBrains cross-platform consistency --

#[test]
fn dark_background_jetbrains_matches_vscode() {
  let jetbrains = jetbrains_attribute(include_str!("../jetbrains/Warm-Burnout-Dark.xml"), "TEXT", "BACKGROUND");
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-dark.json"),
    "editor.background",
  );
  assert_eq!(
    jetbrains, vscode,
    "dark background: jetbrains={jetbrains} vscode={vscode}"
  );
}

#[test]
fn light_background_jetbrains_matches_vscode() {
  let jetbrains = jetbrains_attribute(
    include_str!("../jetbrains/Warm-Burnout-Light.xml"),
    "TEXT",
    "BACKGROUND",
  );
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-light.json"),
    "editor.background",
  );
  assert_eq!(
    jetbrains, vscode,
    "light background: jetbrains={jetbrains} vscode={vscode}"
  );
}

#[test]
fn dark_foreground_jetbrains_matches_vscode() {
  let jetbrains = jetbrains_attribute(include_str!("../jetbrains/Warm-Burnout-Dark.xml"), "TEXT", "FOREGROUND");
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-dark.json"),
    "editor.foreground",
  );
  assert_eq!(
    jetbrains, vscode,
    "dark foreground: jetbrains={jetbrains} vscode={vscode}"
  );
}

#[test]
fn light_foreground_jetbrains_matches_vscode() {
  let jetbrains = jetbrains_attribute(
    include_str!("../jetbrains/Warm-Burnout-Light.xml"),
    "TEXT",
    "FOREGROUND",
  );
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-light.json"),
    "editor.foreground",
  );
  assert_eq!(
    jetbrains, vscode,
    "light foreground: jetbrains={jetbrains} vscode={vscode}"
  );
}

#[test]
fn dark_cursor_jetbrains_matches_ghostty() {
  let jetbrains = jetbrains_color(include_str!("../jetbrains/Warm-Burnout-Dark.xml"), "CARET_COLOR");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "cursor-color");
  assert_eq!(
    jetbrains, ghostty,
    "dark cursor: jetbrains={jetbrains} ghostty={ghostty}"
  );
}

#[test]
fn light_cursor_jetbrains_matches_ghostty() {
  let jetbrains = jetbrains_color(include_str!("../jetbrains/Warm-Burnout-Light.xml"), "CARET_COLOR");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "cursor-color");
  assert_eq!(
    jetbrains, ghostty,
    "light cursor: jetbrains={jetbrains} ghostty={ghostty}"
  );
}

// -- Windows Terminal cross-platform consistency --

#[test]
fn dark_background_windows_terminal_matches_ghostty() {
  let wt = windows_terminal_color(include_str!("../windows-terminal/warm-burnout-dark.json"), "background");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "background");
  assert_eq!(wt, ghostty, "dark background: wt={wt} ghostty={ghostty}");
}

#[test]
fn light_background_windows_terminal_matches_ghostty() {
  let wt = windows_terminal_color(
    include_str!("../windows-terminal/warm-burnout-light.json"),
    "background",
  );
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "background");
  assert_eq!(wt, ghostty, "light background: wt={wt} ghostty={ghostty}");
}

#[test]
fn dark_foreground_windows_terminal_matches_ghostty() {
  let wt = windows_terminal_color(include_str!("../windows-terminal/warm-burnout-dark.json"), "foreground");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "foreground");
  assert_eq!(wt, ghostty, "dark foreground: wt={wt} ghostty={ghostty}");
}

#[test]
fn light_foreground_windows_terminal_matches_ghostty() {
  let wt = windows_terminal_color(
    include_str!("../windows-terminal/warm-burnout-light.json"),
    "foreground",
  );
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "foreground");
  assert_eq!(wt, ghostty, "light foreground: wt={wt} ghostty={ghostty}");
}

#[test]
fn dark_cursor_windows_terminal_matches_ghostty() {
  let wt = windows_terminal_color(
    include_str!("../windows-terminal/warm-burnout-dark.json"),
    "cursorColor",
  );
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "cursor-color");
  assert_eq!(wt, ghostty, "dark cursor: wt={wt} ghostty={ghostty}");
}

#[test]
fn light_cursor_windows_terminal_matches_ghostty() {
  let wt = windows_terminal_color(
    include_str!("../windows-terminal/warm-burnout-light.json"),
    "cursorColor",
  );
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "cursor-color");
  assert_eq!(wt, ghostty, "light cursor: wt={wt} ghostty={ghostty}");
}

// -- Warp cross-platform consistency --

const WARP_DARK: &str = include_str!("../warp/warm-burnout-dark.yaml");
const WARP_LIGHT: &str = include_str!("../warp/warm-burnout-light.yaml");

#[test]
fn dark_background_warp_matches_ghostty() {
  let warp = warp_color(WARP_DARK, "background");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "background");
  assert_eq!(warp, ghostty, "dark background: warp={warp} ghostty={ghostty}");
}

#[test]
fn light_background_warp_matches_ghostty() {
  let warp = warp_color(WARP_LIGHT, "background");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "background");
  assert_eq!(warp, ghostty, "light background: warp={warp} ghostty={ghostty}");
}

#[test]
fn dark_foreground_warp_matches_ghostty() {
  let warp = warp_color(WARP_DARK, "foreground");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "foreground");
  assert_eq!(warp, ghostty, "dark foreground: warp={warp} ghostty={ghostty}");
}

#[test]
fn light_foreground_warp_matches_ghostty() {
  let warp = warp_color(WARP_LIGHT, "foreground");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "foreground");
  assert_eq!(warp, ghostty, "light foreground: warp={warp} ghostty={ghostty}");
}

#[test]
fn dark_cursor_warp_matches_ghostty() {
  let warp = warp_color(WARP_DARK, "cursor");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "cursor-color");
  assert_eq!(warp, ghostty, "dark cursor: warp={warp} ghostty={ghostty}");
}

#[test]
fn light_cursor_warp_matches_ghostty() {
  let warp = warp_color(WARP_LIGHT, "cursor");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "cursor-color");
  assert_eq!(warp, ghostty, "light cursor: warp={warp} ghostty={ghostty}");
}

#[test]
fn dark_accent_warp_matches_canonical() {
  assert_eq!(
    warp_color(WARP_DARK, "accent"),
    "#b8522e",
    "dark warp accent must be canonical copper rust"
  );
}

#[test]
fn light_accent_warp_matches_canonical() {
  assert_eq!(
    warp_color(WARP_LIGHT, "accent"),
    "#b8522e",
    "light warp accent must be canonical copper rust"
  );
}

const WARP_ANSI_KEYS: &[&str] = &["black", "red", "green", "yellow", "blue", "magenta", "cyan", "white"];

fn assert_warp_bank_matches_ghostty(warp_src: &str, ghostty_src: &str, bank: &str, base: u8, variant: &str) {
  for (offset, name) in WARP_ANSI_KEYS.iter().enumerate() {
    let warp = warp_ansi_color(warp_src, bank, name);
    let ghostty = ghostty_ansi_color(ghostty_src, base + offset as u8);
    assert_eq!(
      warp,
      ghostty,
      "{variant} {bank}.{name} (palette {}): warp={warp} ghostty={ghostty}",
      base + offset as u8
    );
  }
}

#[test]
fn dark_normal_ansi_warp_matches_ghostty() {
  assert_warp_bank_matches_ghostty(
    WARP_DARK,
    include_str!("../ghostty/warm-burnout-dark"),
    "normal",
    0,
    "dark",
  );
}

#[test]
fn dark_bright_ansi_warp_matches_ghostty() {
  assert_warp_bank_matches_ghostty(
    WARP_DARK,
    include_str!("../ghostty/warm-burnout-dark"),
    "bright",
    8,
    "dark",
  );
}

#[test]
fn light_normal_ansi_warp_matches_ghostty() {
  assert_warp_bank_matches_ghostty(
    WARP_LIGHT,
    include_str!("../ghostty/warm-burnout-light"),
    "normal",
    0,
    "light",
  );
}

#[test]
fn light_bright_ansi_warp_matches_ghostty() {
  assert_warp_bank_matches_ghostty(
    WARP_LIGHT,
    include_str!("../ghostty/warm-burnout-light"),
    "bright",
    8,
    "light",
  );
}

// -- Home Assistant cross-platform consistency --

const HA_THEME: &str = include_str!("../home-assistant/warm-burnout.yaml");

#[test]
fn dark_background_home_assistant_matches_ghostty() {
  let ha = home_assistant_color(HA_THEME, "Warm Burnout", "dark", "primary-background-color");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "background");
  assert_eq!(ha, ghostty, "dark background: ha={ha} ghostty={ghostty}");
}

#[test]
fn light_background_home_assistant_matches_ghostty() {
  let ha = home_assistant_color(HA_THEME, "Warm Burnout", "light", "primary-background-color");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "background");
  assert_eq!(ha, ghostty, "light background: ha={ha} ghostty={ghostty}");
}

#[test]
fn dark_foreground_home_assistant_matches_ghostty() {
  let ha = home_assistant_color(HA_THEME, "Warm Burnout", "dark", "primary-text-color");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "foreground");
  assert_eq!(ha, ghostty, "dark foreground: ha={ha} ghostty={ghostty}");
}

#[test]
fn light_foreground_home_assistant_matches_ghostty() {
  let ha = home_assistant_color(HA_THEME, "Warm Burnout", "light", "primary-text-color");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "foreground");
  assert_eq!(ha, ghostty, "light foreground: ha={ha} ghostty={ghostty}");
}

#[test]
fn dark_accent_home_assistant_matches_canonical() {
  let ha = home_assistant_color(HA_THEME, "Warm Burnout", "dark", "accent-color");
  assert_eq!(ha, "#b8522e", "dark accent should be canonical copper rust");
}

#[test]
fn light_accent_home_assistant_matches_canonical() {
  let ha = home_assistant_color(HA_THEME, "Warm Burnout", "light", "accent-color");
  assert_eq!(ha, "#b8522e", "light accent should be canonical copper rust");
}

// -- Starship ANSI palette colors match Ghostty ANSI palette --

#[test]
fn dark_ansi_red_starship_matches_ghostty() {
  let starship = starship_palette_color(
    include_str!("../starship/warm-burnout-dark.toml"),
    "warm_burnout_dark",
    "red",
  );
  let ghostty = ghostty_ansi_color(include_str!("../ghostty/warm-burnout-dark"), 1);
  assert_eq!(
    starship, ghostty,
    "dark ansi red: starship={starship} ghostty={ghostty}"
  );
}

#[test]
fn light_ansi_red_starship_matches_ghostty() {
  let starship = starship_palette_color(
    include_str!("../starship/warm-burnout-light.toml"),
    "warm_burnout_light",
    "red",
  );
  let ghostty = ghostty_ansi_color(include_str!("../ghostty/warm-burnout-light"), 1);
  assert_eq!(
    starship, ghostty,
    "light ansi red: starship={starship} ghostty={ghostty}"
  );
}

#[test]
fn dark_ansi_green_starship_matches_ghostty() {
  let starship = starship_palette_color(
    include_str!("../starship/warm-burnout-dark.toml"),
    "warm_burnout_dark",
    "green",
  );
  let ghostty = ghostty_ansi_color(include_str!("../ghostty/warm-burnout-dark"), 2);
  assert_eq!(
    starship, ghostty,
    "dark ansi green: starship={starship} ghostty={ghostty}"
  );
}

#[test]
fn light_ansi_green_starship_matches_ghostty() {
  let starship = starship_palette_color(
    include_str!("../starship/warm-burnout-light.toml"),
    "warm_burnout_light",
    "green",
  );
  let ghostty = ghostty_ansi_color(include_str!("../ghostty/warm-burnout-light"), 2);
  assert_eq!(
    starship, ghostty,
    "light ansi green: starship={starship} ghostty={ghostty}"
  );
}

#[test]
fn dark_ansi_yellow_starship_matches_ghostty() {
  let starship = starship_palette_color(
    include_str!("../starship/warm-burnout-dark.toml"),
    "warm_burnout_dark",
    "yellow",
  );
  let ghostty = ghostty_ansi_color(include_str!("../ghostty/warm-burnout-dark"), 3);
  assert_eq!(
    starship, ghostty,
    "dark ansi yellow: starship={starship} ghostty={ghostty}"
  );
}

#[test]
fn light_ansi_yellow_starship_matches_ghostty() {
  let starship = starship_palette_color(
    include_str!("../starship/warm-burnout-light.toml"),
    "warm_burnout_light",
    "yellow",
  );
  let ghostty = ghostty_ansi_color(include_str!("../ghostty/warm-burnout-light"), 3);
  assert_eq!(
    starship, ghostty,
    "light ansi yellow: starship={starship} ghostty={ghostty}"
  );
}

#[test]
fn dark_ansi_blue_starship_matches_ghostty() {
  let starship = starship_palette_color(
    include_str!("../starship/warm-burnout-dark.toml"),
    "warm_burnout_dark",
    "blue",
  );
  let ghostty = ghostty_ansi_color(include_str!("../ghostty/warm-burnout-dark"), 4);
  assert_eq!(
    starship, ghostty,
    "dark ansi blue: starship={starship} ghostty={ghostty}"
  );
}

#[test]
fn light_ansi_blue_starship_matches_ghostty() {
  let starship = starship_palette_color(
    include_str!("../starship/warm-burnout-light.toml"),
    "warm_burnout_light",
    "blue",
  );
  let ghostty = ghostty_ansi_color(include_str!("../ghostty/warm-burnout-light"), 4);
  assert_eq!(
    starship, ghostty,
    "light ansi blue: starship={starship} ghostty={ghostty}"
  );
}

#[test]
fn dark_ansi_magenta_starship_matches_ghostty() {
  let starship = starship_palette_color(
    include_str!("../starship/warm-burnout-dark.toml"),
    "warm_burnout_dark",
    "magenta",
  );
  let ghostty = ghostty_ansi_color(include_str!("../ghostty/warm-burnout-dark"), 5);
  assert_eq!(
    starship, ghostty,
    "dark ansi magenta: starship={starship} ghostty={ghostty}"
  );
}

#[test]
fn light_ansi_magenta_starship_matches_ghostty() {
  let starship = starship_palette_color(
    include_str!("../starship/warm-burnout-light.toml"),
    "warm_burnout_light",
    "magenta",
  );
  let ghostty = ghostty_ansi_color(include_str!("../ghostty/warm-burnout-light"), 5);
  assert_eq!(
    starship, ghostty,
    "light ansi magenta: starship={starship} ghostty={ghostty}"
  );
}

#[test]
fn dark_ansi_cyan_starship_matches_ghostty() {
  let starship = starship_palette_color(
    include_str!("../starship/warm-burnout-dark.toml"),
    "warm_burnout_dark",
    "cyan",
  );
  let ghostty = ghostty_ansi_color(include_str!("../ghostty/warm-burnout-dark"), 6);
  assert_eq!(
    starship, ghostty,
    "dark ansi cyan: starship={starship} ghostty={ghostty}"
  );
}

#[test]
fn light_ansi_cyan_starship_matches_ghostty() {
  let starship = starship_palette_color(
    include_str!("../starship/warm-burnout-light.toml"),
    "warm_burnout_light",
    "cyan",
  );
  let ghostty = ghostty_ansi_color(include_str!("../ghostty/warm-burnout-light"), 6);
  assert_eq!(
    starship, ghostty,
    "light ansi cyan: starship={starship} ghostty={ghostty}"
  );
}

// -- Zsh comment color matches Ghostty/Starship palette --

#[test]
fn dark_zsh_comment_matches_starship() {
  let zsh =
    zsh_style_fg(include_str!("../zsh/warm-burnout-dark.zsh-theme"), "comment").expect("no comment fg in zsh dark");
  let starship = starship_palette_color(
    include_str!("../starship/warm-burnout-dark.toml"),
    "warm_burnout_dark",
    "comment",
  );
  assert_eq!(zsh, starship, "dark comment: zsh={zsh} starship={starship}");
}

#[test]
fn light_zsh_comment_matches_starship() {
  let zsh =
    zsh_style_fg(include_str!("../zsh/warm-burnout-light.zsh-theme"), "comment").expect("no comment fg in zsh light");
  let starship = starship_palette_color(
    include_str!("../starship/warm-burnout-light.toml"),
    "warm_burnout_light",
    "comment",
  );
  assert_eq!(zsh, starship, "light comment: zsh={zsh} starship={starship}");
}

#[test]
fn dark_zsh_error_matches_starship() {
  let zsh = zsh_style_fg(include_str!("../zsh/warm-burnout-dark.zsh-theme"), "unknown-token")
    .expect("no unknown-token fg in zsh dark");
  let starship = starship_palette_color(
    include_str!("../starship/warm-burnout-dark.toml"),
    "warm_burnout_dark",
    "error",
  );
  assert_eq!(zsh, starship, "dark error: zsh={zsh} starship={starship}");
}

#[test]
fn light_zsh_error_matches_starship() {
  let zsh = zsh_style_fg(include_str!("../zsh/warm-burnout-light.zsh-theme"), "unknown-token")
    .expect("no unknown-token fg in zsh light");
  let starship = starship_palette_color(
    include_str!("../starship/warm-burnout-light.toml"),
    "warm_burnout_light",
    "error",
  );
  assert_eq!(zsh, starship, "light error: zsh={zsh} starship={starship}");
}

// -- Obsidian cross-platform consistency --

const OBSIDIAN_THEME: &str = include_str!("../obsidian/theme.css");

#[test]
fn dark_background_obsidian_matches_vscode() {
  let obsidian = obsidian_color(OBSIDIAN_THEME, "dark", "bg");
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-dark.json"),
    "editor.background",
  );
  assert_eq!(obsidian, vscode, "dark background: obsidian={obsidian} vscode={vscode}");
}

#[test]
fn light_background_obsidian_matches_vscode() {
  let obsidian = obsidian_color(OBSIDIAN_THEME, "light", "bg");
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-light.json"),
    "editor.background",
  );
  assert_eq!(
    obsidian, vscode,
    "light background: obsidian={obsidian} vscode={vscode}"
  );
}

#[test]
fn dark_foreground_obsidian_matches_vscode() {
  let obsidian = obsidian_color(OBSIDIAN_THEME, "dark", "fg");
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-dark.json"),
    "editor.foreground",
  );
  assert_eq!(obsidian, vscode, "dark foreground: obsidian={obsidian} vscode={vscode}");
}

#[test]
fn light_foreground_obsidian_matches_vscode() {
  let obsidian = obsidian_color(OBSIDIAN_THEME, "light", "fg");
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-light.json"),
    "editor.foreground",
  );
  assert_eq!(
    obsidian, vscode,
    "light foreground: obsidian={obsidian} vscode={vscode}"
  );
}

#[test]
fn dark_accent_obsidian_matches_canonical() {
  let obsidian = obsidian_color(OBSIDIAN_THEME, "dark", "accent");
  assert_eq!(obsidian, "#b8522e", "dark accent should be canonical copper rust");
}

#[test]
fn light_accent_obsidian_matches_canonical() {
  let obsidian = obsidian_color(OBSIDIAN_THEME, "light", "accent");
  assert_eq!(obsidian, "#b8522e", "light accent should be canonical copper rust");
}

#[test]
fn dark_background_obsidian_matches_ghostty() {
  let obsidian = obsidian_color(OBSIDIAN_THEME, "dark", "bg");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "background");
  assert_eq!(
    obsidian, ghostty,
    "dark background: obsidian={obsidian} ghostty={ghostty}"
  );
}

#[test]
fn light_background_obsidian_matches_ghostty() {
  let obsidian = obsidian_color(OBSIDIAN_THEME, "light", "bg");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "background");
  assert_eq!(
    obsidian, ghostty,
    "light background: obsidian={obsidian} ghostty={ghostty}"
  );
}

#[test]
fn dark_foreground_obsidian_matches_ghostty() {
  let obsidian = obsidian_color(OBSIDIAN_THEME, "dark", "fg");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "foreground");
  assert_eq!(
    obsidian, ghostty,
    "dark foreground: obsidian={obsidian} ghostty={ghostty}"
  );
}

#[test]
fn light_foreground_obsidian_matches_ghostty() {
  let obsidian = obsidian_color(OBSIDIAN_THEME, "light", "fg");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "foreground");
  assert_eq!(
    obsidian, ghostty,
    "light foreground: obsidian={obsidian} ghostty={ghostty}"
  );
}

#[test]
fn dark_cursor_obsidian_matches_ghostty() {
  let obsidian = obsidian_color(OBSIDIAN_THEME, "dark", "cursor");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "cursor-color");
  assert_eq!(obsidian, ghostty, "dark cursor: obsidian={obsidian} ghostty={ghostty}");
}

#[test]
fn light_cursor_obsidian_matches_ghostty() {
  let obsidian = obsidian_color(OBSIDIAN_THEME, "light", "cursor");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "cursor-color");
  assert_eq!(obsidian, ghostty, "light cursor: obsidian={obsidian} ghostty={ghostty}");
}

// -- Emacs cross-platform consistency --
//
// Pin background, foreground, cursor, accent, and the syntax tokens that have
// a 1:1 palette key to ghostty/nvim so the emacs port can never silently drift
// from the rest of the suite.

const EMACS_SHARED: &str = include_str!("../emacs/warm-burnout.el");

#[test]
fn dark_background_emacs_matches_ghostty() {
  let emacs = emacs_palette_color(EMACS_SHARED, "dark", "bg");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "background");
  assert_eq!(emacs, ghostty, "dark background: emacs={emacs} ghostty={ghostty}");
}

#[test]
fn light_background_emacs_matches_ghostty() {
  let emacs = emacs_palette_color(EMACS_SHARED, "light", "bg");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "background");
  assert_eq!(emacs, ghostty, "light background: emacs={emacs} ghostty={ghostty}");
}

#[test]
fn dark_foreground_emacs_matches_ghostty() {
  let emacs = emacs_palette_color(EMACS_SHARED, "dark", "fg");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "foreground");
  assert_eq!(emacs, ghostty, "dark foreground: emacs={emacs} ghostty={ghostty}");
}

#[test]
fn light_foreground_emacs_matches_ghostty() {
  let emacs = emacs_palette_color(EMACS_SHARED, "light", "fg");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "foreground");
  assert_eq!(emacs, ghostty, "light foreground: emacs={emacs} ghostty={ghostty}");
}

#[test]
fn dark_cursor_emacs_matches_ghostty() {
  let emacs = emacs_palette_color(EMACS_SHARED, "dark", "cursor");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "cursor-color");
  assert_eq!(emacs, ghostty, "dark cursor: emacs={emacs} ghostty={ghostty}");
}

#[test]
fn light_cursor_emacs_matches_ghostty() {
  let emacs = emacs_palette_color(EMACS_SHARED, "light", "cursor");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "cursor-color");
  assert_eq!(emacs, ghostty, "light cursor: emacs={emacs} ghostty={ghostty}");
}

#[test]
fn dark_accent_emacs_matches_canonical() {
  let emacs = emacs_palette_color(EMACS_SHARED, "dark", "warn");
  assert_eq!(emacs, "#b8522e", "dark emacs accent should be canonical copper rust");
}

#[test]
fn light_accent_emacs_matches_canonical() {
  let emacs = emacs_palette_color(EMACS_SHARED, "light", "warn");
  assert_eq!(emacs, "#b8522e", "light emacs accent should be canonical copper rust");
}

#[test]
fn dark_type_emacs_matches_nvim() {
  let emacs = emacs_palette_color(EMACS_SHARED, "dark", "type");
  let nvim = nvim_palette_color(nvim_dark_block(), "type");
  assert_eq!(emacs, nvim, "dark type: emacs={emacs} nvim={nvim}");
}

#[test]
fn light_type_emacs_matches_nvim() {
  let emacs = emacs_palette_color(EMACS_SHARED, "light", "type");
  let nvim = nvim_palette_color(nvim_light_block(), "type");
  assert_eq!(emacs, nvim, "light type: emacs={emacs} nvim={nvim}");
}

#[test]
fn dark_keyword_emacs_matches_nvim() {
  let emacs = emacs_palette_color(EMACS_SHARED, "dark", "keyword");
  let nvim = nvim_palette_color(nvim_dark_block(), "keyword");
  assert_eq!(emacs, nvim, "dark keyword: emacs={emacs} nvim={nvim}");
}

#[test]
fn light_keyword_emacs_matches_nvim() {
  let emacs = emacs_palette_color(EMACS_SHARED, "light", "keyword");
  let nvim = nvim_palette_color(nvim_light_block(), "keyword");
  assert_eq!(emacs, nvim, "light keyword: emacs={emacs} nvim={nvim}");
}

#[test]
fn dark_string_emacs_matches_nvim() {
  let emacs = emacs_palette_color(EMACS_SHARED, "dark", "string");
  let nvim = nvim_palette_color(nvim_dark_block(), "string");
  assert_eq!(emacs, nvim, "dark string: emacs={emacs} nvim={nvim}");
}

#[test]
fn light_string_emacs_matches_nvim() {
  let emacs = emacs_palette_color(EMACS_SHARED, "light", "string");
  let nvim = nvim_palette_color(nvim_light_block(), "string");
  assert_eq!(emacs, nvim, "light string: emacs={emacs} nvim={nvim}");
}

#[test]
fn dark_comment_emacs_matches_nvim() {
  let emacs = emacs_palette_color(EMACS_SHARED, "dark", "comment");
  let nvim = nvim_palette_color(nvim_dark_block(), "comment");
  assert_eq!(emacs, nvim, "dark comment: emacs={emacs} nvim={nvim}");
}

#[test]
fn light_comment_emacs_matches_nvim() {
  let emacs = emacs_palette_color(EMACS_SHARED, "light", "comment");
  let nvim = nvim_palette_color(nvim_light_block(), "comment");
  assert_eq!(emacs, nvim, "light comment: emacs={emacs} nvim={nvim}");
}

// -- Helix cross-platform consistency --

const HELIX_DARK: &str = include_str!("../helix/warm_burnout_dark.toml");
const HELIX_LIGHT: &str = include_str!("../helix/warm_burnout_light.toml");

#[test]
fn dark_background_helix_matches_vscode() {
  let helix = helix_palette_color(HELIX_DARK, "bg");
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-dark.json"),
    "editor.background",
  );
  assert_eq!(helix, vscode, "dark background: helix={helix} vscode={vscode}");
}

#[test]
fn light_background_helix_matches_vscode() {
  let helix = helix_palette_color(HELIX_LIGHT, "bg");
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-light.json"),
    "editor.background",
  );
  assert_eq!(helix, vscode, "light background: helix={helix} vscode={vscode}");
}

#[test]
fn dark_foreground_helix_matches_vscode() {
  let helix = helix_palette_color(HELIX_DARK, "fg");
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-dark.json"),
    "editor.foreground",
  );
  assert_eq!(helix, vscode, "dark foreground: helix={helix} vscode={vscode}");
}

#[test]
fn light_foreground_helix_matches_vscode() {
  let helix = helix_palette_color(HELIX_LIGHT, "fg");
  let vscode = vscode_color(
    include_str!("../vscode/themes/warm-burnout-light.json"),
    "editor.foreground",
  );
  assert_eq!(helix, vscode, "light foreground: helix={helix} vscode={vscode}");
}

#[test]
fn dark_cursor_helix_matches_ghostty() {
  let helix = helix_palette_color(HELIX_DARK, "cursor");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "cursor-color");
  assert_eq!(helix, ghostty, "dark cursor: helix={helix} ghostty={ghostty}");
}

#[test]
fn light_cursor_helix_matches_ghostty() {
  let helix = helix_palette_color(HELIX_LIGHT, "cursor");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "cursor-color");
  assert_eq!(helix, ghostty, "light cursor: helix={helix} ghostty={ghostty}");
}

#[test]
fn dark_accent_helix_matches_canonical() {
  let helix = helix_palette_color(HELIX_DARK, "accent");
  assert_eq!(helix, "#b8522e", "dark helix accent must be canonical copper rust");
}

#[test]
fn light_accent_helix_matches_canonical() {
  let helix = helix_palette_color(HELIX_LIGHT, "accent");
  assert_eq!(helix, "#b8522e", "light helix accent must be canonical copper rust");
}

#[test]
fn dark_background_helix_matches_ghostty() {
  let helix = helix_palette_color(HELIX_DARK, "bg");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "background");
  assert_eq!(helix, ghostty, "dark background: helix={helix} ghostty={ghostty}");
}

#[test]
fn light_background_helix_matches_ghostty() {
  let helix = helix_palette_color(HELIX_LIGHT, "bg");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-light"), "background");
  assert_eq!(helix, ghostty, "light background: helix={helix} ghostty={ghostty}");
}

#[test]
fn dark_foreground_helix_matches_ghostty() {
  let helix = helix_palette_color(HELIX_DARK, "fg");
  let ghostty = ghostty_color(include_str!("../ghostty/warm-burnout-dark"), "foreground");
  assert_eq!(helix, ghostty, "dark foreground: helix={helix} ghostty={ghostty}");
}
