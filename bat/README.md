# Warm Burnout for Bat

Syntax-highlighted cat, now with warmer damage. Your terminal deserved a better autopsy light.

## Install

Copy the `.tmTheme` files into Bat's theme directory:

```sh
mkdir -p "$(bat --config-dir)/themes"
cp "Warm Burnout Dark.tmTheme" "Warm Burnout Light.tmTheme" "$(bat --config-dir)/themes/"
bat cache --build
```

Then use either theme:

```sh
bat --theme="Warm Burnout Dark" README.md
bat --theme="Warm Burnout Light" README.md
```

To make it permanent, add one of these to your Bat config file:

```sh
--theme="Warm Burnout Dark"
```

Bat can also switch by terminal appearance:

```sh
--theme=auto:system
--theme-dark="Warm Burnout Dark"
--theme-light="Warm Burnout Light"
```

Find your config file path with:

```sh
bat --config-file
```

## Verify

After rebuilding the cache, check that Bat can see the themes:

```sh
bat --list-themes | grep 'Warm Burnout'
```

If the names do not appear, rebuild the cache again. Bat likes a ritual. It is still less dramatic than your deadlines.

## Palette

Both themes derive from the canonical Warm Burnout palette in the root [`AGENTS.md`](../AGENTS.md). The scope mapping mirrors the editor themes where Bat's TextMate theme format allows it.
