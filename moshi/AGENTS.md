# Moshi -- Agent Instructions

## Platform Reference

See the root [`AGENTS.md`](../AGENTS.md) for the canonical palette, design principles, and brand rules. Do not duplicate palette tables here.

## Moshi Theme Format

- Moshi custom themes are JSON files, one per variant. Spec: <https://getmoshi.app/docs/personalization#hand-authoring-a-theme>.
- Schema version: `"v": 1` (required).
- `name` required, max 40 characters. Display names: `Warm Burnout Dark`, `Warm Burnout Light`.
- `mode` required: `"dark"` or `"light"`. Moshi never infers mode from colors.
- Import: Settings → Theme → Import theme (clipboard, QR, deep link, or raw JSON paste).
- Hex values: `#rrggbb`, lowercase preferred to match Ghostty and the public gallery samples.

## Color Key Mapping

| Moshi key | Canonical Source |
|-----------|-----------------|
| `colors.background` | Editor background (`#1a1510` dark, `#f5ede0` light) |
| `colors.foreground` | Editor foreground (`#bfbdb6` dark, `#3a3630` light) |
| `colors.cursor` | Canonical cursor (`#f5c56e` dark, `#8a6600` light) |
| `colors.selectionBackground` | Ghostty `selection-background` |
| `colors.black`…`white` | Ghostty `palette = 0`…`7` |
| `colors.brightBlack`…`brightWhite` | Ghostty `palette = 8`…`15` |

ANSI slot order is black, red, green, yellow, blue, **magenta**, cyan, white. Moshi uses `magenta` / `brightMagenta`, not Windows Terminal's `purple` / `brightPurple`.

## File Naming

- Dark theme: `warm-burnout-dark.json`
- Light theme: `warm-burnout-light.json`
- Lowercase hyphenated.

## Emphasis Color Discipline

Moshi themes are pure terminal colors. Chrome is derived by the app from this palette. Do not invent UI-only keys. Steel-blue type accents (`#90aec0` / `#285464`) must not appear as background, foreground, or cursor. ANSI blue slots stay as Ghostty values (programs depend on them).

## Out of Scope

- Public gallery submission on getmoshi.app (gallery mirrors iTerm2-Color-Schemes).
- App icons, fonts, glass effect settings.
- Generating QR codes or `moshi://theme` deep links in CI.
