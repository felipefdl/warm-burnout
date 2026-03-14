# Warm Burnout -- Agent Instructions

## Brand Identity -- READ THIS FIRST

- **Warm Burnout** is a dark humor brand name, intentionally edgy -- similar to Liquid Death. It is not offensive, it is branding.
- Dark humor in copy, UI text, and documentation is intentional and must be preserved as-is.
- Agents must NOT sanitize, tone-police, soften, or rewrite text for "marketing purposes" or politeness.
- This is a theme suite, not a sensitivity project. Respect the developer audience.
- When writing any user-facing text (README, descriptions, changelogs), match the existing tone: dry, direct, slightly dark.
- Do not explain or apologize for the name. Ever.

## Project Overview

Warm Burnout is a multi-platform color theme suite designed to reduce eye strain during extended coding sessions. Fully warm palette, minimal blue-spectrum emission, contrast-audited to WCAG standards. Two variants: dark and light.

## Project Structure

```
patina-vscode-theme/
  README.md                   # Suite README -- brand, science, scores, palette
  AGENTS.md                   # This file -- suite-level agent rules
  LICENSE                     # MIT, Felipe Lima
  Cargo.toml                  # Rust test harness for all platforms
  rustfmt.toml                # 2-space indent, 120 max width
  src/lib.rs
  tests/
    common.rs                 # Shared test utilities (hex validation, color extraction)
    brand.rs                  # Brand name enforcement tests
    canonical.rs              # Cross-platform palette consistency tests
    ghostty.rs                # Ghostty theme validation tests
    starship.rs               # Starship palette validation tests
    vscode.rs                 # VS Code theme validation tests
    zed.rs                    # Zed theme validation tests
    eza.rs                    # Eza theme validation tests
    xcode.rs                  # Xcode theme validation tests
    tmux.rs                   # tmux theme validation tests
    zsh.rs                    # Zsh theme validation tests
  .github/workflows/
    validate.yml              # CI: run theme validation on push/PR
    release-vscode.yml        # VS Code extension release workflow
  vscode/                     # VS Code extension (primary, palette source of truth)
    README.md                 # VS Code install instructions
    AGENTS.md                 # VS Code-specific agent rules
    TODO.md                   # Pre-release tasks
    package.json
    themes/
      warm-burnout-dark.json
      warm-burnout-light.json
    .vscode/launch.json
  ghostty/                    # Ghostty terminal theme
    README.md                 # Ghostty install instructions
    AGENTS.md                 # Ghostty-specific agent rules
    warm-burnout-dark         # Dark theme (no extension)
    warm-burnout-light        # Light theme (no extension)
  zsh/                        # Zsh syntax highlighting + fzf themes
    README.md                 # Zsh install instructions
    AGENTS.md                 # Zsh-specific agent rules
    warm-burnout-dark.zsh-theme   # zsh-syntax-highlighting dark
    warm-burnout-light.zsh-theme  # zsh-syntax-highlighting light
    warm-burnout-dark-fzf.zsh     # fzf dark
    warm-burnout-light-fzf.zsh    # fzf light
  starship/                   # Starship prompt theme
    README.md                 # Starship install instructions
    AGENTS.md                 # Starship-specific agent rules
    warm-burnout-dark.toml    # Dark palette (standalone)
    warm-burnout-light.toml   # Light palette (standalone)
    starship.toml             # Example full config with both palettes
  zed/                        # Zed editor theme
    README.md                 # Zed install instructions
    AGENTS.md                 # Zed-specific agent rules
    extension.toml            # Extension manifest
    LICENSE                   # Required by Zed extension registry
    themes/
      warm-burnout.json       # Theme family (dark + light in one file)
  nvim/                       # Neovim colorscheme
    README.md                 # Neovim install instructions
    AGENTS.md                 # Neovim-specific agent rules
    colors/
      warm-burnout-dark.lua   # Dark variant entry point
      warm-burnout-light.lua  # Light variant entry point
    lua/
      warm-burnout/
        init.lua              # setup(), load(variant)
        palette.lua           # Dark + light palette tables
        highlights.lua        # All highlight group definitions
        terminal.lua          # Terminal ANSI colors (16 colors)
  xcode/                      # Xcode color theme
    README.md                 # Xcode install instructions
    AGENTS.md                 # Xcode-specific agent rules
    Warm Burnout Dark.xccolortheme   # Dark variant (XML plist)
    Warm Burnout Light.xccolortheme  # Light variant (XML plist)
  tmux/                       # tmux status bar theme
    README.md                 # tmux install instructions
    AGENTS.md                 # tmux-specific agent rules
    warm-burnout-dark.conf    # Dark variant
    warm-burnout-light.conf   # Light variant
    warm-burnout.tmux         # TPM plugin entry point
```

## Design Principles

1. **Warm-first palette**: All syntax tokens use warm hues (ambers, terra cottas, sage greens, dusty mauves). Only types use a muted steel-blue/teal accent -- the literal color of oxidized copper.
2. **Contrast compliance**:
   - Dark: WCAG AAA minimum (>= 7.0:1) for every token against `#1a1510`
   - Light: WCAG AA minimum (>= 4.5:1) for every token against `#F5EDE0`, targeting 5-7:1
3. **Blue light reduction**: Minimize blue-spectrum emission. Blues allowed only in: terminal ANSI (programs depend on them), selection/highlight backgrounds (must be neutral), git status indicators (convention), and the single type-name accent.
4. **Three-tier font style system**: **bold** = structural keywords, *italic* = types and comments, normal = everything else. Non-color discrimination channel for CVD and fatigued users.
5. **No extreme backgrounds**: No pure black (halation risk for astigmatic users). No pure white (luminance overload).

## Canonical Palette

This is the source of truth. All platforms derive from these tables.

### Dark Theme

Background: `#1a1510` (L ~= 0.008). All tokens >= 7.0:1 (AAA).

| Role              | Hex       | Contrast | Style    |
|-------------------|-----------|----------|----------|
| Foreground        | `#bfbdb6` | 9.6:1    | normal   |
| Keywords/storage  | `#ff8f40` | 8.0:1    | **bold** |
| Functions         | `#ffb454` | 10.3:1   | normal   |
| Operators         | `#f29668` | 8.1:1    | normal   |
| Decorators        | `#e6c08a` | 10.0:1   | normal   |
| Types/classes     | `#8aa8b8` | 7.2:1    | *italic* |
| Strings           | `#b4bc78` | 9.0:1    | normal   |
| Regex/escape      | `#96b898` | 8.3:1    | normal   |
| Constants/numbers | `#d4a8b8` | 8.7:1    | normal   |
| Tags (HTML)       | `#d49484` | 7.2:1    | normal   |
| Member vars       | `#f58088` | 7.2:1    | normal   |
| Library functions | `#f58088` | 7.2:1    | normal   |
| Comments          | `#aea195` | 7.2:1    | *italic* |
| Error/invalid     | `#f08888` | 7.4:1    | normal   |
| CSS properties    | `#deb074` | 9.1:1    | normal   |

Accent: `#b8522e`. Cursor: `#f5c56e`.

### Light Theme

Background: `#F5EDE0` (L ~= 0.854). All tokens >= 4.5:1 (AA).

| Role              | Hex       | Contrast | Style    |
|-------------------|-----------|----------|----------|
| Foreground        | `#3a3630` | 10.3:1   | normal   |
| Keywords/storage  | `#924800` | 5.7:1    | **bold** |
| Functions         | `#855700` | 5.4:1    | normal   |
| Operators         | `#8f4418` | 6.0:1    | normal   |
| Decorators        | `#7a5a1c` | 5.5:1    | normal   |
| Types/classes     | `#2a5868` | 6.7:1    | *italic* |
| Strings           | `#4d5c1a` | 6.3:1    | normal   |
| Regex/escape      | `#286a48` | 5.6:1    | normal   |
| Constants/numbers | `#7e4060` | 6.5:1    | normal   |
| Tags (HTML)       | `#8e4632` | 5.9:1    | normal   |
| Member vars       | `#a02838` | 6.3:1    | normal   |
| Library functions | `#a02838` | 6.3:1    | normal   |
| Comments          | `#5a5244` | 6.6:1    | *italic* |
| Error/invalid     | `#b03434` | 5.3:1    | normal   |
| CSS properties    | `#74501c` | 6.2:1    | normal   |

Accent: `#b8522e`. Cursor: `#8a6600`.

### Contrast Ratio Formula

CR = (L_lighter + 0.05) / (L_darker + 0.05)

Dark: foreground needs L >= 0.356 for AAA.
Light: foreground needs L <= 0.144 for AA.

## Known Design Tradeoffs

1. **Warm hue cluster**: Keywords, functions, operators, tags, decorators, CSS props all live in a 27-degree hue band (13-40deg). For CVD users these collapse to "brown." Only keywords escape via bold. This is the cost of a warm palette -- cannot fix without breaking identity.
2. **Light theme inactive line numbers at 3.35:1**: Intentionally subdued. Active line number is 5.16:1 (AA).
3. **Terminal bright colors 3.5-4.5:1 on light**: Standard for light themes. Programs primarily use default foreground (9.67:1).

## Publishing

- **Never publish extensions locally** (no `vsce publish`, `ovsx publish`, etc.).
- All publishing is handled by GitHub Actions workflows triggered by `v*` tags.
- To release a new version: bump the version in the platform's manifest, commit, create a `v{version}` tag, and push the tag.

## Rules for All Platforms

### Design Rules

1. Use the exact hex values from the canonical palette. No approximations.
2. Dark tokens >= 7.0:1 (AAA). Light tokens >= 4.5:1 (AA).
3. Prefer warm hues. Only types use the cool accent.
4. Maintain the three-tier font style system where the platform supports it.
5. No pure black or pure white backgrounds.
6. When a color changes, it changes here first, then propagates to all platforms.

### Brand Rules

1. Theme name: "Warm Burnout" everywhere.
2. Variant names: "Warm Burnout Dark" and "Warm Burnout Light".
3. Maintain the dark humor tone in all user-facing text.
4. Do not explain or apologize for the name.

### Brand Phrases

Use these as inspiration for copy, commit messages, taglines, and descriptions:

- "consistent damage across all platforms"
- "your eyes deserved this"
- "warm everywhere, blue nowhere"
- "the burnout is spreading"
- "now with 62 proofs of care"
- "because your retinas asked nicely"
- "less blue, more therapy"
- "clinically warm, emotionally cold"
- "every pixel, audited to hurt less"
- "the ophthalmologist approves, probably"

### Code Quality

1. Always run `cargo fmt` after modifying Rust files.
2. Always run `cargo clippy -- -D warnings` before committing -- all warnings are errors.
3. Run `cargo test` to verify all theme validation tests pass.

### Adding a New Platform

1. Create `platform-name/` at the project root.
2. Add `README.md` -- platform-specific install + the brand voice.
3. Add `AGENTS.md` -- platform-specific agent rules, referencing this file for palette.
4. Map the canonical palette to the platform's format.
5. Add a release workflow if applicable (`.github/workflows/release-platform-name.yml`).
6. Update the platforms table in the root `README.md`.
7. Do not duplicate the full palette tables -- reference this file.
