# Warm Burnout for eza

Your `ls` replacement was still painting directories in searing blue. Not anymore.

## Requirements

- eza v0.19.2 or later (theme.yml support)
- A truecolor terminal

## Variants

| Variant | File | Background |
|---------|------|------------|
| Dark | `dark.yml` | `#1a1510` (warm charcoal) |
| Light | `light.yml` | `#F5EDE0` (warm cream) |

## Install

Copy the variant you want to your eza config directory:

**Dark:**

```sh
mkdir -p ~/.config/eza
cp dark.yml ~/.config/eza/theme.yml
```

**Light:**

```sh
mkdir -p ~/.config/eza
cp light.yml ~/.config/eza/theme.yml
```

Then add to your `.zshrc` (or `.bashrc`):

```sh
export EZA_CONFIG_DIR="$HOME/.config/eza"
```

Restart your shell. eza only reads one `theme.yml` at a time, so pick one.

## What it covers

The theme maps the full Warm Burnout palette to every eza element:

### Dark

| Element | Color | Hex |
|---------|-------|-----|
| Directories | Aged brass | `#deb074` |
| Symlinks | Verdigris | `#96b898` |
| Executables | Dried sage | `#b4bc78` |
| Mount points | Burnt orange | `#ff8f40` |
| Dates | Warm stone | `#b4a89c` |
| User (you) | Gold | `#e6c08a` |
| Permissions read | Gold | `#e6c08a` |
| Permissions write | Coral | `#ec9878` |
| Permissions execute | Dried sage | `#b4bc78` |
| Size (small) | Foreground | `#bfbdb6` |
| Size (large) | Amber | `#ffb454` |
| Git new | Dried sage | `#b4bc78` |
| Git modified | Amber | `#ffb454` |
| Git deleted | Coral | `#ec9878` |
| Source files | Steel patina | `#90aec0` |
| Errors | Error | `#f49090` |
| Punctuation | Warm stone | `#b4a89c` |

### Light

| Element | Color | Hex |
|---------|-------|-----|
| Directories | Aged brass | `#74501c` |
| Symlinks | Verdigris | `#286a48` |
| Executables | Dried sage | `#4d5c1a` |
| Mount points | Burnt orange | `#924800` |
| Dates | Warm stone | `#544c40` |
| User (you) | Gold | `#7a5a1c` |
| Permissions read | Gold | `#7a5a1c` |
| Permissions write | Coral | `#883850` |
| Permissions execute | Dried sage | `#4d5c1a` |
| Size (small) | Foreground | `#3a3630` |
| Size (large) | Amber | `#855700` |
| Git new | Dried sage | `#4d5c1a` |
| Git modified | Amber | `#855700` |
| Git deleted | Coral | `#883850` |
| Source files | Steel patina | `#285464` |
| Errors | Error | `#b03434` |
| Punctuation | Warm stone | `#544c40` |

## Verify

Run `ll` or `eza -la`. Warm tones everywhere, no blue.

## Palette

Derives from the canonical Warm Burnout palette defined in the root [`AGENTS.md`](../AGENTS.md).
