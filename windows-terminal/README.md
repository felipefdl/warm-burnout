# Warm Burnout for Windows Terminal

Even your terminal on Windows deserved better. The burnout is spreading.

## Install

### Option A: JSON fragment (no settings.json editing)

Drop the scheme files into the Windows Terminal fragments directory and they'll be picked up automatically.

**All users:**

```
C:\ProgramData\Microsoft\Windows Terminal\Fragments\WarmBurnout\
```

**Current user only:**

```
C:\Users\<your-username>\AppData\Local\Microsoft\Windows Terminal\Fragments\WarmBurnout\
```

Create the directory if it doesn't exist, then place a single `warm-burnout.json` file inside it:

```json
{
  "schemes": [
    {
      "name": "Warm Burnout Dark",
      "background": "#1A1510",
      "foreground": "#BFBDB6",
      "cursorColor": "#F5C56E",
      "selectionBackground": "#33393A",
      "black": "#23211B",
      "red": "#F06B73",
      "green": "#70BF56",
      "yellow": "#FDB04C",
      "blue": "#4FBFFF",
      "purple": "#D0A1FF",
      "cyan": "#93E2C8",
      "white": "#C7C7C7",
      "brightBlack": "#686868",
      "brightRed": "#F07178",
      "brightGreen": "#AAD94C",
      "brightYellow": "#FFB454",
      "brightBlue": "#59C2FF",
      "brightPurple": "#D2A6FF",
      "brightCyan": "#95E6CB",
      "brightWhite": "#FFFFFF"
    },
    {
      "name": "Warm Burnout Light",
      "background": "#F5EDE0",
      "foreground": "#3A3630",
      "cursorColor": "#8A6600",
      "selectionBackground": "#E5E8E2",
      "black": "#3A3630",
      "red": "#B82820",
      "green": "#2D6A14",
      "yellow": "#8A6000",
      "blue": "#2060A0",
      "purple": "#8A3090",
      "cyan": "#146858",
      "white": "#C0B8AA",
      "brightBlack": "#686868",
      "brightRed": "#C83028",
      "brightGreen": "#3A7A20",
      "brightYellow": "#9A7008",
      "brightBlue": "#2870B0",
      "brightPurple": "#9A38A0",
      "brightCyan": "#208870",
      "brightWhite": "#FAF6F0"
    }
  ]
}
```

Restart Windows Terminal. The schemes will appear in the color scheme dropdown.

### Option B: Edit settings.json directly

1. Open Windows Terminal
2. Press **Ctrl+Shift+,** to open `settings.json` directly
3. Find the **`"schemes"`** array (not `"themes"`, that's for UI chrome, not terminal colors)
4. Paste the contents of `warm-burnout-dark.json` and `warm-burnout-light.json` as new objects inside the `"schemes"` array
5. Save the file

Each scheme object goes directly inside `"schemes"`:

```json
"schemes": [
    { "name": "Warm Burnout Dark", "background": "#1A1510", ... },
    { "name": "Warm Burnout Light", "background": "#F5EDE0", ... }
]
```

## Configure

After adding the schemes, set one as your profile's color scheme:

1. Open **Settings** (Ctrl+,)
2. Select your profile under **Profiles**
3. Go to **Appearance** > **Color scheme**
4. Select:
   - **Warm Burnout Dark**: for the 3am sessions
   - **Warm Burnout Light**: for when someone forces you to open the blinds

Or set it directly in `settings.json`:

```json
{
  "profiles": {
    "defaults": {
      "colorScheme": "Warm Burnout Dark"
    }
  }
}
```

## Verify

Run `ls --color` or any ANSI color test script. Warm browns instead of searing blues.

## Palette

Both themes derive from the canonical Warm Burnout palette defined in the root [`AGENTS.md`](../AGENTS.md). The ANSI colors match the Ghostty terminal theme exactly.
