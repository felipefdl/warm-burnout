# Bat -- Agent Instructions

Follow the root [`AGENTS.md`](../AGENTS.md). The canonical palette lives there; do not duplicate the palette tables here.

## Bat Theme Format

- Bat custom themes must be Sublime Text `.tmTheme` XML plist files.
- Newer `.sublime-color-scheme` files are not supported by Bat.
- Users install themes into `$(bat --config-dir)/themes` and run `bat cache --build`.
- Bat uses the `.tmTheme` filename as the theme name, so filenames must stay title-cased:
  - `Warm Burnout Dark.tmTheme`
  - `Warm Burnout Light.tmTheme`

## Scope Mapping

- TextMate scopes mirror the VS Code and Helix mappings where possible.
- **bold** = keywords, storage scopes, and tags.
- *italic* = comments, types/classes, decorators, CSS properties, and language built-ins where the platform supports it.
- Normal = functions, strings, constants, operators, variables, and punctuation.

## Color Rules

1. Use exact hex values from the canonical palette. No approximations.
2. Background and foreground must match Ghostty terminal base colors.
3. Selection uses pre-blended opaque values: `#33393a` dark and `#e5e8e2` light.
4. Line highlight uses pre-blended opaque values: `#222018` dark and `#e2dace` light.
5. Do not use terminal ANSI colors here; Bat themes are truecolor syntax themes.
6. Keep the steel-blue type accent limited to type/class scopes.

## Testing

Run:

```sh
cargo test --test bat
cargo test --test canonical -- bat
```

If `bat` is installed, also validate with a temporary config dir:

```sh
tmp=$(mktemp -d)
mkdir -p "$tmp/themes"
cp bat/*.tmTheme "$tmp/themes/"
BAT_CONFIG_DIR="$tmp" bat cache --build
BAT_CONFIG_DIR="$tmp" bat --list-themes | grep 'Warm Burnout'
```
