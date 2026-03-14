mod common;

use common::{ghostty_color, hex_to_lower, nvim_palette_color, starship_palette_color, vscode_color, zed_editor_color};

fn zsh_foreground(src: &str) -> Option<String> {
  src.lines().find(|l| l.contains("[default]")).and_then(|l| {
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
