const READMES: &[(&str, &str)] = &[
  ("root", include_str!("../README.md")),
  ("vscode", include_str!("../vscode/README.md")),
  ("ghostty", include_str!("../ghostty/README.md")),
  ("zsh", include_str!("../zsh/README.md")),
  ("starship", include_str!("../starship/README.md")),
  ("zed", include_str!("../zed/README.md")),
  ("nvim", include_str!("../nvim/README.md")),
  ("xcode", include_str!("../xcode/README.md")),
  ("tmux", include_str!("../tmux/README.md")),
  ("zellij", include_str!("../zellij/README.md")),
  ("warp", include_str!("../warp/README.md")),
  ("iterm2", include_str!("../iterm2/README.md")),
  ("jetbrains", include_str!("../jetbrains/README.md")),
  ("obsidian", include_str!("../obsidian/README.md")),
  ("emacs", include_str!("../emacs/README.md")),
  ("helix", include_str!("../helix/README.md")),
];

#[test]
fn all_readmes_mention_warm_burnout() {
  for (name, content) in READMES {
    assert!(
      content.contains("Warm Burnout"),
      "{name}/README.md does not mention 'Warm Burnout'"
    );
  }
}

#[test]
fn no_theme_file_uses_patina_as_label() {
  let theme_files: &[(&str, &str)] = &[
    ("vscode/dark", include_str!("../vscode/themes/warm-burnout-dark.json")),
    ("vscode/light", include_str!("../vscode/themes/warm-burnout-light.json")),
    ("vscode/package.json", include_str!("../vscode/package.json")),
    ("ghostty/dark", include_str!("../ghostty/warm-burnout-dark")),
    ("ghostty/light", include_str!("../ghostty/warm-burnout-light")),
    ("starship/dark", include_str!("../starship/warm-burnout-dark.toml")),
    ("starship/light", include_str!("../starship/warm-burnout-light.toml")),
    ("starship/example", include_str!("../starship/starship.toml")),
    ("zed/theme", include_str!("../zed/themes/warm-burnout.json")),
    ("tmux/dark", include_str!("../tmux/warm-burnout-dark.conf")),
    ("tmux/light", include_str!("../tmux/warm-burnout-light.conf")),
    ("zellij/dark", include_str!("../zellij/warm-burnout-dark.kdl")),
    ("zellij/light", include_str!("../zellij/warm-burnout-light.kdl")),
    ("warp/dark", include_str!("../warp/warm-burnout-dark.yaml")),
    ("warp/light", include_str!("../warp/warm-burnout-light.yaml")),
    ("iterm2/dark", include_str!("../iterm2/Warm Burnout Dark.itermcolors")),
    ("iterm2/light", include_str!("../iterm2/Warm Burnout Light.itermcolors")),
    ("jetbrains/dark", include_str!("../jetbrains/Warm-Burnout-Dark.xml")),
    ("jetbrains/light", include_str!("../jetbrains/Warm-Burnout-Light.xml")),
    (
      "jetbrains/dark-theme",
      include_str!("../jetbrains/Warm Burnout Islands Dark.theme.json"),
    ),
    (
      "jetbrains/light-theme",
      include_str!("../jetbrains/Warm Burnout Islands Light.theme.json"),
    ),
    ("obsidian/theme", include_str!("../obsidian/theme.css")),
    ("emacs/shared", include_str!("../emacs/warm-burnout.el")),
    ("emacs/dark", include_str!("../emacs/warm-burnout-dark-theme.el")),
    ("emacs/light", include_str!("../emacs/warm-burnout-light-theme.el")),
    ("helix/dark", include_str!("../helix/warm-burnout-dark.toml")),
    ("helix/light", include_str!("../helix/warm-burnout-light.toml")),
  ];
  for (name, content) in theme_files {
    for line in content.lines() {
      let lower = line.to_lowercase();
      if lower.contains("patina")
        && !lower.contains("steel_patina")
        && !lower.contains("steel patina")
        && !lower.contains("steel-patina")
      {
        panic!("{name}: line uses 'Patina' as brand name (should be 'Warm Burnout'): {line}");
      }
    }
  }
}
