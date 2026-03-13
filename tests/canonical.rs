mod common;

use common::{ghostty_color, hex_to_lower, starship_palette_color, vscode_color, zed_editor_color};

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
