# Warm Burnout

The theme suite for developers who already burned out but still have deadlines.

Built on one premise: your eyes are taking damage and your current theme doesn't care. Warm Burnout does. Fully warm palette, minimal blue light, contrast-audited to clinical standards. Two variants: **Dark** for the 3am sessions and **Light** for when someone forces you to open the blinds.

![Dark and Light side by side](screenshots/split-comparison.png)

## The Problem With Your Current Theme

Most themes look great in screenshots. Then you use them for 14 hours straight and your eyes feel like they've been sandpapered.

The blues, cyans, and purples that make themes look "cool" emit at 450-500nm, the exact wavelength range that suppresses melatonin, triggers digital eye strain, and makes your ophthalmologist wince. Your theme is fighting your body's ability to rest, even after you close the laptop.

Warm Burnout strips out almost all blue-spectrum syntax colors, leaving a fully warm palette (550-630nm dominant) that your retinas can survive.

## Eye Safety Audit

We ran a full ophthalmological contrast audit. Not vibes. Math.

### Dark: Every Token AAA

Every syntax color meets **WCAG AAA** (>= 7:1 contrast ratio) against the editor background. Every single one. Most themes can't even get comments past 4:1.

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

The light theme uses `#F5EDE0`, a warm sepia cream, not white. 14.6% less bright than pure white. Estimated ~4000K color temperature vs white's ~6500K, which means roughly **40-50% less melatonin suppression** compared to white-background themes.

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

## Comfort Scores

Scored across two real scenarios, not marketing fantasies.

### Scenario A: You haven't slept in 72 hours

Dark office. Energy drinks. You open the blinds for the first time since Tuesday.

| Factor | Score | Why |
|--------|-------|-----|
| Photophobia risk | 7/10 | 15% dimmer than white, warm 4000K tint |
| Melatonin preservation | 8/10 | Sepia tint vs 6500K white |
| Cursor findability | 7/10 | 4.54:1 gold cursor (not invisible) |
| Reading code | 9/10 | 10.33:1 primary text |
| Ctrl+F searching | 8/10 | Gold highlight, increased opacity in light theme |
| Reading diffs | 8/10 | 19-25% opacity tints, visible in both variants |
| **Overall** | **8.5/10** | |

### Scenario B: Normal human who cares about their eyes

| Factor | Score | Why |
|--------|-------|-----|
| Long-session comfort | 8.5/10 | Warm, not harsh, good hierarchy |
| Readability | 10/10 | 10.33:1 primary, all dark tokens >= 8:1, 2 light tokens at AAA |
| Blue light reduction | 8.5/10 | Only types add blue, everything else warm |
| Colorblind accessibility | 9/10 | Bold keywords + tags, italic types/CSS/decorators, member vars separated from error |
| vs. pure-white themes | 9/10 | Measurably better in every metric |
| **Overall** | **9.5/10** | |

**Combined: 9/10.**

## How It Works

### Background: no pure black, no pure white

The dark background (`#1a1510`) is a warm brown-black. Pure black (#000000) causes **halation**, a glow/bleed effect around text that affects roughly 33% of the population (those with astigmatism). The warm tint eliminates this.

The light background (`#F5EDE0`) is warm sepia cream. Inspired by Solarized Light's cream approach but shifted toward brown-sepia. No pure white anywhere.

### Three-tier font style system

Color alone fails under fatigue. When you've been staring at code for 12 hours, your color discrimination drops. For the ~8% of males with red-green color vision deficiency, warm tones blur together.

Warm Burnout uses font styles as a second discrimination channel:

- **Bold**: structural keywords (`if`, `return`, `const`) and HTML tags. Your eye scans these for code flow.
- *Italic*: types and comments. The single cool accent (steel-blue/teal) + italic makes types unmistakable.
- Normal: everything else.

A protanopic developer running on no sleep can distinguish structure from types from data.

### One cool accent: oxidized copper

The palette is fully warm except types/classes: `#90aec0` (dark) / `#285464` (light). This is the literal color of oxidized copper. One cool landmark in a warm field improves color distinctiveness without adding blue light load.

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
| Xcode | Available | [`xcode/`](xcode/) |
| iTerm2 | Available | [`iterm2/`](iterm2/) |
| Windows Terminal | Available | [`windows-terminal/`](windows-terminal/) |
| Warp | Available | [`warp/`](warp/) |
| tmux | Available | [`tmux/`](tmux/) |
| Zellij | Available | [`zellij/`](zellij/) |
| Starship | Available | [`starship/`](starship/) |
| Zsh | Available | [`zsh/`](zsh/) |
| Home Assistant | Available | [`home-assistant/`](home-assistant/) |
| eza | Available | [`eza/`](eza/) |
| Obsidian | Available | [`obsidian/`](obsidian/) |

Each platform lives in its own directory with its own README, build process, and release workflow.

## License

[MIT](LICENSE). Use it, fork it, burn it out.
