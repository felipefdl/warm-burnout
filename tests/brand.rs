const READMES: &[(&str, &str)] = &[
  ("root", include_str!("../README.md")),
  ("vscode", include_str!("../vscode/README.md")),
  ("ghostty", include_str!("../ghostty/README.md")),
  ("zsh", include_str!("../zsh/README.md")),
  ("starship", include_str!("../starship/README.md")),
  ("zed", include_str!("../zed/README.md")),
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
  ];
  for (name, content) in theme_files {
    for line in content.lines() {
      let lower = line.to_lowercase();
      if lower.contains("patina") && !lower.contains("steel_patina") && !lower.contains("steel patina") {
        panic!("{name}: line uses 'Patina' as brand name (should be 'Warm Burnout'): {line}");
      }
    }
  }
}
