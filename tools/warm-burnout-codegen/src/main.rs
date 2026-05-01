use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use clap::Parser;
use serde::{Deserialize, Serialize};
use tera::{Context as TeraContext, Tera};

#[derive(Parser, Debug)]
#[command(about = "Render Warm Burnout theme templates against the canonical palette.")]
struct Cli {
  /// Path to the palette YAML file (jetbrains/palette.yaml).
  #[arg(long)]
  palette: PathBuf,

  /// Path to the Tera template to render.
  #[arg(long)]
  template: PathBuf,

  /// Variant to render: "dark" or "light".
  #[arg(long)]
  variant: String,

  /// Output file path.
  #[arg(long)]
  output: PathBuf,
}

#[derive(Debug, Deserialize)]
struct Palette {
  flavors: BTreeMap<String, Flavor>,
}

#[derive(Debug, Deserialize)]
struct Flavor {
  dark: bool,
  colors: BTreeMap<String, String>,
}

#[derive(Debug, Serialize)]
struct Color {
  hex: String,
}

fn main() -> Result<()> {
  let cli = Cli::parse();

  let palette_text =
    fs::read_to_string(&cli.palette).with_context(|| format!("reading palette {}", cli.palette.display()))?;
  let palette: Palette =
    serde_yml::from_str(&palette_text).with_context(|| format!("parsing palette {}", cli.palette.display()))?;

  let flavor = palette
    .flavors
    .get(&cli.variant)
    .with_context(|| format!("variant '{}' not present in palette", cli.variant))?;

  let mut ctx = TeraContext::new();
  ctx.insert("variant", &cli.variant);
  ctx.insert("dark", &flavor.dark);
  ctx.insert("light", &!flavor.dark);
  for (token, hex) in &flavor.colors {
    let normalized = normalize_hex(hex)
      .with_context(|| format!("flavor '{}' token '{}' has invalid hex '{}'", cli.variant, token, hex))?;
    ctx.insert(token, &Color { hex: normalized });
  }

  let template_text =
    fs::read_to_string(&cli.template).with_context(|| format!("reading template {}", cli.template.display()))?;

  let mut tera = Tera::default();
  tera
    .add_raw_template("template", &template_text)
    .with_context(|| format!("compiling template {}", cli.template.display()))?;

  let rendered = tera
    .render("template", &ctx)
    .with_context(|| format!("rendering template {}", cli.template.display()))?;

  if let Some(parent) = cli.output.parent() {
    fs::create_dir_all(parent).with_context(|| format!("creating output dir {}", parent.display()))?;
  }
  fs::write(&cli.output, rendered).with_context(|| format!("writing output {}", cli.output.display()))?;

  Ok(())
}

fn normalize_hex(input: &str) -> Result<String> {
  let trimmed = input.trim().trim_start_matches('#').to_ascii_lowercase();
  let valid_len = matches!(trimmed.len(), 6 | 8);
  let all_hex = trimmed.chars().all(|c| c.is_ascii_hexdigit());
  if !valid_len || !all_hex {
    bail!("hex must be 6 or 8 hex digits, optionally prefixed with #");
  }
  Ok(trimmed)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn normalize_strips_hash_and_lowercases() {
    assert_eq!(normalize_hex("#FF8F40").unwrap(), "ff8f40");
    assert_eq!(normalize_hex("ff8f40").unwrap(), "ff8f40");
  }

  #[test]
  fn normalize_accepts_8_digit_alpha() {
    assert_eq!(normalize_hex("#b8522e18").unwrap(), "b8522e18");
  }

  #[test]
  fn normalize_rejects_short_hex() {
    assert!(normalize_hex("#fff").is_err());
  }

  #[test]
  fn normalize_rejects_non_hex_chars() {
    assert!(normalize_hex("#zzzzzz").is_err());
  }
}
