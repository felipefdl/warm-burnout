# Warm Burnout -- Agent Instructions

## Brand Identity -- READ THIS FIRST

- **Warm Burnout** is a dark humor brand name, intentionally edgy -- similar to Liquid Death. It is not offensive, it is branding.
- Dark humor in copy, UI text, and documentation is intentional and must be preserved as-is.
- Agents must NOT sanitize, tone-police, soften, or rewrite text for "marketing purposes" or politeness.
- This is a theme suite, not a sensitivity project. Respect the developer audience.
- When writing any user-facing text (README, descriptions, changelogs), match the existing tone: dry, direct, slightly dark.
- Do not explain or apologize for the name. Ever.

## Project Overview

Warm Burnout is a multi-platform color theme suite designed for extended coding sessions. Mostly warm palette, one cool type accent, contrast-audited to WCAG standards. Two variants: dark and light.

## Agent Instruction Files

- `AGENTS.md` is the source of truth for repo-level agent instructions.
- `CLAUDE.md` is a symbolic link to `AGENTS.md`. Do not edit or review both as separate files.

## Project Structure

```
warm-burnout/
  README.md                   # Suite README -- brand, contrast, palette
  AGENTS.md                   # This file -- suite-level agent rules
  LICENSE                     # MIT, Felipe Lima
  Cargo.toml                  # Rust test harness for all platforms
  rustfmt.toml                # 2-space indent, 120 max width
  src/lib.rs
  tests/
    common.rs                 # Shared test utilities (hex validation, color extraction)
    brand.rs                  # Brand name enforcement tests
    canonical.rs              # Cross-platform palette consistency tests
    home_assistant.rs         # Home Assistant theme validation tests
    ghostty.rs                # Ghostty theme validation tests
    starship.rs               # Starship palette validation tests
    vscode.rs                 # VS Code theme validation tests
    zed.rs                    # Zed theme validation tests
    eza.rs                    # Eza theme validation tests
    xcode.rs                  # Xcode theme validation tests
    iterm2.rs                 # iTerm2 theme validation tests
    jetbrains.rs              # JetBrains theme validation tests
    windows_terminal.rs       # Windows Terminal theme validation tests
    warp.rs                   # Warp theme validation tests
    alacritty.rs              # Alacritty theme validation tests
    tmux.rs                   # tmux theme validation tests
    zellij.rs                 # Zellij theme validation tests
    zsh.rs                    # Zsh theme validation tests
    nvim.rs                   # Neovim theme validation tests
    vim.rs                    # Vim theme validation tests
    obsidian.rs               # Obsidian theme validation tests
  .github/workflows/
    validate.yml              # CI: run theme validation on push/PR
    release-vscode.yml        # VS Code extension release workflow
    release-themes.yml        # All platforms release workflow (zip/attach to GH release)
  themes/
    warm-burnout.yaml          # Symlink -> ../home-assistant/warm-burnout.yaml (HACS)
  hacs.json                    # HACS manifest (Home Assistant)
  home-assistant/              # Home Assistant theme
    warm-burnout.yaml          # Theme file (dark + light modes)
    README.md                  # Home Assistant install instructions
    AGENTS.md                  # Home Assistant-specific agent rules
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
  vim/                        # Vim colorscheme (Vim 8+)
    README.md                 # Vim install instructions
    AGENTS.md                 # Vim-specific agent rules
    colors/
      warm-burnout-dark.vim   # Self-contained dark variant
      warm-burnout-light.vim  # Self-contained light variant
  xcode/                      # Xcode color theme
    README.md                 # Xcode install instructions
    AGENTS.md                 # Xcode-specific agent rules
    Warm Burnout Dark.xccolortheme   # Dark variant (XML plist)
    Warm Burnout Light.xccolortheme  # Light variant (XML plist)
  iterm2/                     # iTerm2 terminal theme
    README.md                 # iTerm2 install instructions
    AGENTS.md                 # iTerm2-specific agent rules
    Warm Burnout Dark.itermcolors   # Dark variant (XML plist)
    Warm Burnout Light.itermcolors  # Light variant (XML plist)
  jetbrains/                  # JetBrains IDE theme (full UI + editor)
    META-INF/
      plugin.xml              # Plugin manifest
    Warm Burnout Islands Dark.theme.json  # Dark UI theme (Islands)
    Warm Burnout Islands Light.theme.json # Light UI theme (Islands)
    Warm-Burnout-Dark.xml     # Dark editor scheme
    Warm-Burnout-Light.xml    # Light editor scheme
    build.sh                  # Build plugin JAR
    README.md                 # JetBrains install instructions
    AGENTS.md                 # JetBrains-specific agent rules
  windows-terminal/           # Windows Terminal color scheme
    README.md                 # Windows Terminal install instructions
    AGENTS.md                 # Windows Terminal-specific agent rules
    warm-burnout-dark.json    # Dark variant (standalone scheme)
    warm-burnout-light.json   # Light variant (standalone scheme)
    warm-burnout.json         # Fragment file (both schemes, drop-in install)
  warp/                       # Warp terminal theme
    README.md                 # Warp install instructions
    AGENTS.md                 # Warp-specific agent rules
    warm-burnout-dark.yaml    # Dark variant
    warm-burnout-light.yaml   # Light variant
  alacritty/                  # Alacritty terminal theme
    README.md                 # Alacritty install instructions
    AGENTS.md                 # Alacritty-specific agent rules
    warm-burnout-dark.toml    # Dark variant (TOML, importable)
    warm-burnout-light.toml   # Light variant
  tmux/                       # tmux status bar theme
    README.md                 # tmux install instructions
    AGENTS.md                 # tmux-specific agent rules
    warm-burnout-dark.conf    # Dark variant
    warm-burnout-light.conf   # Light variant
    warm-burnout.tmux         # TPM plugin entry point
  zellij/                     # Zellij multiplexer theme
    README.md                 # Zellij install instructions
    AGENTS.md                 # Zellij-specific agent rules
    warm-burnout-dark.kdl     # Dark variant (KDL, modern UI-component spec)
    warm-burnout-light.kdl    # Light variant
  eza/                        # eza file listing theme
    dark.yml                  # Dark variant
    light.yml                 # Light variant
    README.md                 # eza install instructions
    AGENTS.md                 # eza-specific agent rules
  obsidian/                    # Obsidian community theme
    theme.css                  # Dark + light variants (CSS custom properties)
    manifest.json              # Community theme manifest
    README.md                  # Obsidian install instructions
    AGENTS.md                  # Obsidian-specific agent rules
    screenshot.png             # Community directory screenshot
    fonts/                     # Bundled fonts (Inter, Geist Mono)
    screenshots/               # README screenshots (dark + light)
  screenshots/                # Theme preview screenshots
    AGENTS.md                 # Screenshot-specific agent rules
    generate.mjs              # Playwright script to render HTML -> PNG
    editor-dark.html          # Dark editor mockup (TypeScript/React)
    editor-light.html         # Light editor mockup (Rust)
    terminal-dark.html        # Terminal mockup (Ghostty + tmux + Starship)
    split-comparison.html     # Dark vs Light side by side (HTML/CSS)
    *.png                     # Generated 2400x1600 @2x retina PNGs
```

## Design Principles

1. **Warm-first palette**: All syntax tokens use warm hues (ambers, terra cottas, sage greens, dusty mauves). Only types use a muted steel-blue/teal accent -- the literal color of oxidized copper.
2. **Contrast compliance**:
   - Dark: WCAG AAA minimum (>= 7.0:1) for every token against `#1a1510`
   - Light: WCAG AA minimum (>= 4.5:1) for every token against `#F5EDE0`, targeting 5-7:1
3. **Blue light reduction**: Minimize blue-spectrum emission. Blues allowed only in: terminal ANSI (programs depend on them), selection/highlight backgrounds (must be neutral), git status indicators (convention), and the single type-name accent.
4. **Three-tier font style system**: **bold** = structural keywords and HTML tags, *italic* = types, comments, CSS properties, decorators, normal = everything else. Non-color discrimination channel for CVD and fatigued users.
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
| Decorators        | `#e6c08a` | 10.6:1   | *italic* |
| Types/classes     | `#90aec0` | 7.8:1    | *italic* |
| Strings           | `#b4bc78` | 9.0:1    | normal   |
| Regex/escape      | `#96b898` | 8.3:1    | normal   |
| Constants/numbers | `#d4a8b8` | 8.7:1    | normal   |
| Tags (HTML)       | `#dc9e92` | 8.1:1    | **bold** |
| Member vars       | `#ec9878` | 8.1:1    | normal   |
| Library functions | `#ec9878` | 8.1:1    | normal   |
| Comments          | `#b4a89c` | 7.8:1    | *italic* |
| Error/invalid     | `#f49090` | 7.9:1    | normal   |
| CSS properties    | `#deb074` | 9.1:1    | *italic* |

Accent: `#b8522e`. Cursor: `#f5c56e`.

### Light Theme

Background: `#F5EDE0` (L ~= 0.854). All tokens >= 4.5:1 (AA). 2 tokens (types and comments) meet AAA (>= 7.0:1).

| Role              | Hex       | Contrast | Style    |
|-------------------|-----------|----------|----------|
| Foreground        | `#3a3630` | 10.3:1   | normal   |
| Keywords/storage  | `#924800` | 5.7:1    | **bold** |
| Functions         | `#855700` | 5.4:1    | normal   |
| Operators         | `#8f4418` | 6.0:1    | normal   |
| Decorators        | `#7a5a1c` | 5.5:1    | *italic* |
| Types/classes     | `#285464` | 7.2:1    | *italic* |
| Strings           | `#4d5c1a` | 6.3:1    | normal   |
| Regex/escape      | `#286a48` | 5.6:1    | normal   |
| Constants/numbers | `#7e4060` | 6.5:1    | normal   |
| Tags (HTML)       | `#8e4632` | 5.9:1    | **bold** |
| Member vars       | `#883850` | 6.6:1    | normal   |
| Library functions | `#883850` | 6.6:1    | normal   |
| Comments          | `#544c40` | 7.3:1    | *italic* |
| Error/invalid     | `#b03434` | 5.3:1    | normal   |
| CSS properties    | `#74501c` | 6.2:1    | *italic* |

Accent: `#b8522e`. Cursor: `#8a6600`.

### Contrast Ratio Formula

CR = (L_lighter + 0.05) / (L_darker + 0.05)

Dark: foreground needs L >= 0.356 for AAA.
Light: foreground needs L <= 0.144 for AA.

## Known Design Tradeoffs

1. **Warm hue cluster**: Keywords, functions, operators, tags, decorators, CSS props all live in a 27-degree hue band (13-40deg). For CVD users these collapse to "brown." Keywords and tags escape via bold. This is the cost of a warm palette -- cannot fix without breaking identity.
2. **Light theme inactive line numbers at 3.35:1**: Intentionally subdued. Active line number is 5.16:1 (AA).
3. **Terminal bright colors 3.5-4.5:1 on light**: Standard for light themes. Programs primarily use default foreground (9.67:1).

## Publishing

- **Never publish extensions locally** (no `vsce publish`, `ovsx publish`, etc.).
- All publishing is handled by GitHub Actions workflows triggered by `v*` tags.
- To release a new version: bump the version in the platform's manifest, commit, create a `v{version}` tag, and push the tag.

### Release Checklist

1. Bump version in `vscode/package.json`, `zed/extension.toml`, and `obsidian/manifest.json`.
2. Commit, create `v{version}` tag, push both.
3. The tag push triggers `release-vscode.yml` (VS Code/Open VSX) and `release-themes.yml` (all platform zips attached to GitHub Release).
4. **Update release notes** on the GitHub Release. Write real notes, not a changelog dump. Structure them by what matters to users: palette changes, font style changes, UI fixes, new platform coverage, infrastructure, fixes. End with the comfort score and test count. Match the tone of v1.4.2 release notes. Use `gh release edit` to update after the workflow creates the release.
5. **Zed marketplace** requires a separate PR to `zed-industries/extensions`. Clone the fork at `felipefdl/extensions`, sync with upstream, update the submodule to the new tag, bump the version in the central `extensions.toml` (only the `[warm-burnout-theme]` entry), and open a PR against `zed-industries/extensions`.

### Website

The website at `site/` is deployed to GitHub Pages via `deploy-site.yml`. It triggers automatically on push to `main` when files in `site/`, `screenshots/`, or `images/` change.

To preview locally:

```sh
cd site
npm run build
cd ..
python3 -m http.server 8791 --directory site
```

GitHub Pages source must be set to "GitHub Actions" in repo Settings > Pages.

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
5. Never hardcode platform counts (e.g. "16 platforms") or test counts in user-facing text, meta tags, or screenshots. These go stale the moment a platform is added or a test is written. Use count-free phrasing instead ("multi-platform", "the burnout is spreading", etc.).

### Brand Phrases

Use these as inspiration for copy, commit messages, taglines, and descriptions:

- "consistent damage across all platforms"
- "your eyes deserved this"
- "mostly warm, rarely blue"
- "the burnout is spreading"
- "proofs of care in every test"
- "because your retinas asked nicely"
- "less neon, more burnt toast"
- "contrast audited, deadlines remain"
- "every token audited, every pixel still doomed"
- "one cool accent, because mud is not a palette"

### Writing Style: Avoid AI-Typical Language

User-facing text must read like a human wrote it. AI-generated text has recognizable tics that erode trust with a developer audience. Avoid these:

**Punctuation:**
- Do not use `--` as em dashes. Use colons, periods, commas, or restructure the sentence.

**Filler words and phrases:**
- "Here's why:", "Here's the thing:", "Here's what happened:"
- "literally" (unless something is actually literal)
- "dramatically", "significantly", "incredibly", "remarkably"
- "It's worth noting that", "It's important to note"
- "What's left is", "What remains is"
- "In other words", "That said", "That being said"
- "This means that"
- "straightforward", "comprehensive", "robust", "seamless"
- "First and foremost"
- "dive into", "delve into", "deep dive"
- "leverage" (say "use")
- "utilize" (say "use")
- "In order to" (say "to")
- "a wide range of", "a variety of"
- "game-changer", "game-changing"
- "Whether you're a... or a..."

**Sentence patterns:**
- Do not open paragraphs with "So," or "Now,"
- Do not use three-part dramatic fragments back-to-back ("Not X. Not Y. Z.")
- Avoid rhetorical questions followed by their own answer
- Do not end sections with a single-sentence "takeaway" restatement

**General rule:** If a sentence sounds like it could appear in any AI-generated blog post, rewrite it.

### Git and Commits

1. **No `Co-Authored-By` trailers in commits.** This applies to direct commits, PR commits, and squash-merge bodies. If a tool inserts one, strip it before merging. When squash-merging a PR, write the commit body yourself with `gh pr merge --squash --body "..."`; do not let GitHub concatenate per-commit messages with their trailers.
2. **No AI or tooling references in commits, PRs, reviews, issue comments, or any public surface.** No mentions of Claude, Claude Code, ChatGPT, Copilot, "AI", "LLM", or any other generator. The audience is developers; the tooling stays invisible.
3. **Commit messages**: conventional commits with scope. Format: `type(scope): short description`. Scope is the platform or area (`vim`, `nvim`, `vscode`, `tests`, `site`, etc.). Subject under 72 characters, lowercase, no trailing period. Types: `feat`, `fix`, `refactor`, `test`, `docs`, `chore`, `perf`.
4. **Branch names**: `feature/`, `fix/`, `chore/`, `refactor/`, `test/`, `perf/`, `docs/` prefix with kebab-case (`feature/add-vim`, `fix/light-gutter-drift`).
5. **PR titles**: human-readable summary, not conventional-commit format. Capitalize the first word. Reads like a one-line changelog entry.

### Issue Labels

1. Use the repo taxonomy for every issue: `type:*`, `area:*`, `priority:*`, `status:*`, and `platform:*`.
2. Every issue needs one `type:*`, one `priority:*`, and one `status:*` label.
3. Platform-specific issues need the matching `platform:<name>` label.
4. Use `type:port` for new platform or editor support.
5. Use `area:palette`, `area:syntax`, `area:ui`, `area:accessibility`, `area:packaging`, `area:site`, or `area:screenshots` when the work touches that concern.
6. Use `status:needs-info` when an issue is missing details. Use `status:ready` only when the issue can be picked up without another decision.
7. Keep `good first issue` for small, scoped contributor tasks and `help wanted` when outside help is welcome.

### Code Quality

1. Always run `cargo fmt` after modifying Rust files.
2. Always run `cargo clippy -- -D warnings` before committing -- all warnings are errors.
3. Run `cargo test` to verify all theme validation tests pass.
4. Run `rumdl fmt .` and `rumdl check .` after modifying Markdown files.

### Adding a New Platform

1. Create `platform-name/` at the project root.
2. Add `README.md` -- platform-specific install + the brand voice.
3. Add `AGENTS.md` -- platform-specific agent rules, referencing this file for palette.
4. Map the canonical palette to the platform's format.
5. Add the platform to `.github/workflows/release-themes.yml`: zip multi-file platforms, attach single-file platforms directly.
6. Add any build artifacts to `.gitignore`.
7. Update the platforms table in the root `README.md`.
8. Update the platforms grid and any relevant docs in `site/index.html` (the GitHub Pages website).
9. Do not duplicate the full palette tables -- reference this file.

### Release Files

Every platform must have its theme files attached to GitHub Releases via `.github/workflows/release-themes.yml`. The workflow triggers on `v*` tags and attaches all platform packages to the release.

**Packaging rules:**
- Multi-file platforms: zip into `warm-burnout-<platform>.zip` (e.g., `warm-burnout-ghostty.zip`)
- Single-file platforms: attach directly with a descriptive name (e.g., `warm-burnout-home-assistant.yaml`)
- Platforms with build steps: build first, attach the artifact (e.g., `jetbrains/warm-burnout-theme.jar`)

VS Code is handled separately by `release-vscode.yml` (marketplace + Open VSX publishing).

### Screenshots

Theme preview images live in `screenshots/`. HTML mockups render to PNG via Playwright at 2400x1600 (@2x retina). The mockups use generic editor/terminal chrome, not any specific editor's UI.

**After any palette change**, regenerate:

```sh
cd screenshots
npm install playwright
npx playwright install chromium
node generate.mjs
```

All colors in the HTML files must match the canonical palette above. The terminal screenshot includes a tmux status bar matching `tmux/warm-burnout-dark.conf` and a Starship prompt.

Screenshots are referenced in:
- Root `README.md` (relative paths)
- `vscode/README.md` (relative paths; `release-vscode.yml` copies `screenshots/` into the extension directory before packaging)
- `zed/README.md` (absolute `raw.githubusercontent.com` URLs)
- `site/index.html` (GitHub Pages website, copied into `_site/screenshots/` by the deploy workflow)

When adding a new screenshot, update all three READMEs and the `PAGES` array in `generate.mjs`.
