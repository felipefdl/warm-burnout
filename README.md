# Warm Burnout

The theme suite for developers who are already burned out but still have deadlines.

Built on one premise: your eyes have been bullied enough by radioactive blue themes. Warm Burnout is a mostly warm syntax palette with one cool steel-blue accent for types and WCAG-audited contrast. Two variants: **Dark** for the 3am sessions and **Light** for when someone forces you to open the blinds.

![Dark and Light side by side](screenshots/split-comparison.png)

## The Problem With Your Current Theme

Most themes look great in screenshots. Then you use them for 14 hours straight and your eyes feel like they've been sandpapered.

The blues, cyans, and purples that make themes look "cool" also turn code into a neon aquarium after twelve hours of work and pretending it is fine.

Warm Burnout pushes almost every syntax role into warm materials: amber, terra cotta, sage, brass, coral, and old stone. Types keep one cool accent because otherwise warm themes turn into mud.

## Contrast Audit

We ran a WCAG contrast audit. Not taste. Math.

### Dark: Every Token AAA

Every syntax color meets **WCAG AAA** (>= 7:1 contrast ratio) against the editor background. Most themes can't even get comments past 4:1.

| Role | Color | Ratio | Style |
|------|-------|-------|-------|
| Foreground | `#bfbdb6` | 9.6:1 | normal |
| Keywords | `#ff8f40` | 8.0:1 | **bold** |
| Functions | `#ffb454` | 10.3:1 | normal |
| Types/classes | `#90aec0` | 7.8:1 | *italic* |
| Strings | `#b4bc78` | 9.0:1 | normal |
| Constants | `#d4a8b8` | 8.7:1 | normal |
| Comments | `#b4a89c` | 7.8:1 | *italic* |
| Errors | `#f49090` | 7.9:1 | normal |

Your comments are readable at 3am. On purpose.

![Editor - Dark variant](screenshots/editor-dark.png)

### Light: AA Minimum, Sepia Cream Background

The light theme uses `#F5EDE0`, a warm sepia cream, not white. It is 14.6% less bright than pure white by relative luminance. Still bright enough to read code, less like staring into a dentist lamp.

| Role | Color | Ratio | Style |
|------|-------|-------|-------|
| Foreground | `#3a3630` | 10.3:1 | normal |
| Keywords | `#924800` | 5.7:1 | **bold** |
| Functions | `#855700` | 5.4:1 | normal |
| Types/classes | `#285464` | 7.2:1 AAA | *italic* |
| Strings | `#4d5c1a` | 6.3:1 | normal |
| Constants | `#7e4060` | 6.5:1 | normal |
| Comments | `#544c40` | 7.3:1 AAA | *italic* |
| Errors | `#b03434` | 5.3:1 | normal |

All 15 syntax tokens pass AA. No exceptions.

![Editor - Light variant](screenshots/editor-light.png)

## Design Checks

No fake scores, just checks.

| Check | Decision | Why |
|-------|----------|-----|
| Contrast | Dark syntax is AAA, light syntax is AA | Tired eyes still need legible comments |
| Backgrounds | No pure black, no pure white | The abyss and the dentist lamp can both stay away |
| Palette | Mostly warm, one cool type accent | Warm themes without one landmark turn into mud |
| Font styles | Bold structure, italic types and comments | Color alone gets worse when you are exhausted |
| Cursor | Gold, not red | A red cursor looks like an error having a panic attack |
| Search and diffs | Warm tints, visible in both variants | Finding bugs should not require finding your cursor first |

## How It Works

### Background: no pure black, no pure white

The dark background (`#1a1510`) is a warm brown-black. Pure black (#000000) can make text bloom or bleed for some users, especially with astigmatism. The warm brown-black steps back from pure black.

The light background (`#F5EDE0`) is warm sepia cream. Inspired by Solarized Light's cream approach but shifted toward brown-sepia. No pure white anywhere.

### Three-tier font style system

Color alone gets worse under fatigue and color vision differences. When warm tones blur together, shape still has to carry meaning.

Warm Burnout uses font styles as a second discrimination channel:

- **Bold**: structural keywords (`if`, `return`, `const`) and HTML tags. Your eye scans these for code flow.
- *Italic*: types and comments. The single cool accent (steel-blue/teal) plus italic makes types easier to pick out.
- Normal: everything else.

A developer running on no sleep can rely on more than color. Luxury.

### One cool accent: oxidized copper

The palette is mostly warm, not fully warm. Types/classes use `#90aec0` (dark) / `#285464` (light), the color of oxidized copper. One cool accent keeps the warm palette from turning into mud.

### Cursor: gold, not red

The accent color is copper rust (`#b8522e`) for buttons and badges. But the cursor uses warm gold: `#f5c56e` dark, `#8a6600` light. A copper cursor gets confused with error indicators. Gold is visible everywhere, including YAML where syntax colors are muted.

## The Palette

Inspired by materials that age well. Unlike your eyes.

| Material | Dark | Light | Used for |
|----------|------|-------|----------|
| Amber | `#ffb454` | `#855700` | Functions |
| Burnt orange | `#ff8f40` | `#924800` | Keywords |
| Terra cotta | `#dc9e92` | `#8e4632` | HTML tags |
| Dried sage | `#b4bc78` | `#4d5c1a` | Strings |
| Verdigris | `#96b898` | `#286a48` | Regex, escapes |
| Dusty mauve | `#d4a8b8` | `#7e4060` | Numbers, constants |
| Coral | `#ec9878` | `#883850` | Member variables |
| Warm stone | `#b4a89c` | `#544c40` | Comments |
| Aged brass | `#deb074` | `#74501c` | CSS properties |
| Steel patina | `#90aec0` | `#285464` | Types, classes |
| Gold | `#e6c08a` | `#7a5a1c` | Decorators |

## Platforms

![Terminal - Ghostty + tmux + Starship](screenshots/terminal-dark.png)

| Platform | Status | Install |
|----------|--------|---------|
| VS Code | Available | [Marketplace](https://marketplace.visualstudio.com/items?itemName=felip3fdl.warm-burnout) |
| Open VSX | Available | [Registry](https://open-vsx.org/extension/felip3fdl/warm-burnout) (Antigravity, Cursor, Windsurf, Kiro, VSCodium) |
| JetBrains | Available | [Marketplace](https://plugins.jetbrains.com/plugin/30694-warm-burnout) (IntelliJ, Android Studio, PyCharm, WebStorm, RustRover) |
| Ghostty | Available | [`ghostty/`](ghostty/) |
| Zed | Available | [`zed/`](zed/) |
| Neovim | Available | [`nvim/`](nvim/) |
| Vim | Available | [`vim/`](vim/) |
| Helix | Available | [`helix/`](helix/) |
| Bat | Available | [`bat/`](bat/) |
| Xcode | Available | [`xcode/`](xcode/) |
| iTerm2 | Available | [`iterm2/`](iterm2/) |
| Windows Terminal | Available | [`windows-terminal/`](windows-terminal/) |
| Warp | Available | [`warp/`](warp/) |
| WezTerm | Available | [`wezterm/`](wezterm/) |
| Alacritty | Available | [`alacritty/`](alacritty/) |
| tmux | Available | [`tmux/`](tmux/) |
| Zellij | Available | [`zellij/`](zellij/) |
| Starship | Available | [`starship/`](starship/) |
| Zsh | Available | [`zsh/`](zsh/) |
| Home Assistant | Available | [`home-assistant/`](home-assistant/) |
| eza | Available | [`eza/`](eza/) |
| Obsidian | Available | [`obsidian/`](obsidian/) |
| Emacs | Available | [`emacs/`](emacs/) |

Each platform lives in its own directory with its own README, build process, and release workflow.

## License

[MIT](LICENSE). Use it, fork it, burn it out.
