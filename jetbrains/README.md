# Warm Burnout for JetBrains

Your IDE was also incinerating your retinas. The burnout is spreading to every JetBrains product.

Full UI theme, not just syntax colors. Sidebar, tabs, toolbar, popups, everything. Works in IntelliJ IDEA, Android Studio, WebStorm, PyCharm, GoLand, RustRover, and every other JetBrains IDE.

![Dark and Light side by side](https://raw.githubusercontent.com/felipefdl/warm-burnout/main/screenshots/split-comparison.png)

## Install

```sh
just jetbrains-build
```

Then in your IDE: **Settings** > **Plugins** > gear icon > **Install Plugin from Disk...** > select `jetbrains/warm-burnout-theme.jar`.

The recipe runs the local codegen binary, generates the theme files from `jetbrains/palette.yaml` and the Tera templates, and zips the result into a plugin JAR.

## Configure

After installing the plugin, go to **Settings** > **Appearance & Behavior** > **Appearance**, then select **Warm Burnout Dark** or **Warm Burnout Light** from the Theme dropdown.

## Verify

Open any source file. Keywords should be bold burnt orange, types should be italic steel patina, and the entire IDE (sidebar, tabs, popups) should be warm.

## Palette

Both themes derive from the canonical Warm Burnout palette defined in the root [`AGENTS.md`](../AGENTS.md). Every syntax token and UI element maps directly from the canonical hex values.
