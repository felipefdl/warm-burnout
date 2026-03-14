# Xcode -- Agent Instructions

## Platform Reference

See the root [`AGENTS.md`](../AGENTS.md) for the canonical palette, design principles, and brand rules. Do not duplicate palette tables here.

## Xcode Color Theme Format

- Xcode uses `.xccolortheme` files -- XML plists conforming to Apple's PropertyList 1.0 DTD.
- Installed to `~/Library/Developer/Xcode/UserData/FontAndColorThemes/`.
- Top-level keys are flat strings like `DVTSourceTextBackground`, `DVTSourceTextInsertionPointColor`, etc.
- Nested dicts: `DVTSourceTextSyntaxColors` (color values) and `DVTSourceTextSyntaxFonts` (font values).

## Color Representation

- Colors are RGBA float strings: `"R G B A"` where each component is a `0.0`-`1.0` float.
- Convert from hex: divide each 8-bit component by 255, round to 6 decimal places.
- Example: `#ff8f40` -> `1.000000 0.560784 0.250980 1`
- Alpha is the 4th component. Use `1` for fully opaque, fractional for transparency (e.g., selection).

## Font Representation

- Fonts are PostScript name strings: `"FontName - Size"`.
- Three-tier system: `SFMono-Medium - 12.0` (normal), `SFMono-Bold - 12.0` (bold), `SFMono-MediumItalic - 12.0` (italic).
- Bold: `xcode.syntax.keyword`, `xcode.syntax.comment.doc.keyword`, `xcode.syntax.mark`.
- Italic: `xcode.syntax.comment`, `xcode.syntax.comment.doc`, `xcode.syntax.identifier.type`, `xcode.syntax.identifier.type.system`, `xcode.syntax.identifier.class`, `xcode.syntax.identifier.class.system`, `xcode.syntax.declaration.type`.
- Normal: everything else.

## Syntax Key Structure

- Keys follow the pattern `xcode.syntax.<category>[.<subcategory>]`.
- `.system` variants exist for types, classes, constants, variables, functions, and macros -- these distinguish standard library identifiers from user-defined ones.
- Both `.system` and user variants use the same color in Warm Burnout (we don't differentiate stdlib from user code visually).

## Known Quirks

- `DVTConsoleExectuableInputTextColor` and `DVTConsoleExectuableOutputTextColor` contain a typo in the key name (`Exectutable` instead of `Executable`). This is Apple's typo and must be preserved exactly -- Xcode will not recognize the corrected spelling.

## File Structure

```
xcode/
  Warm Burnout Dark.xccolortheme    # Dark variant
  Warm Burnout Light.xccolortheme   # Light variant
  README.md                         # Install instructions
  AGENTS.md                         # This file
```
