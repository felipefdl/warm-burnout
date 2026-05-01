codegen := "cargo run --quiet --release -p warm-burnout-codegen --"

# List recipes.
default:
    @just --list

# Generate JetBrains theme files from templates and palette, then build the JAR.
jetbrains-build:
    {{codegen}} --palette jetbrains/palette.yaml --variant dark \
      --template jetbrains/templates/theme.json.tera \
      --output 'jetbrains/Warm Burnout Islands Dark.theme.json'
    {{codegen}} --palette jetbrains/palette.yaml --variant light \
      --template jetbrains/templates/theme.json.tera \
      --output 'jetbrains/Warm Burnout Islands Light.theme.json'
    {{codegen}} --palette jetbrains/palette.yaml --variant dark \
      --template jetbrains/templates/editor.xml.tera \
      --output jetbrains/Warm-Burnout-Dark.xml
    {{codegen}} --palette jetbrains/palette.yaml --variant light \
      --template jetbrains/templates/editor.xml.tera \
      --output jetbrains/Warm-Burnout-Light.xml
    cd jetbrains && ./build.sh

# Diff JetBrains/rider-theme-pack against the pinned revision and print added attribute keys.
jetbrains-sync:
    bash scripts/jetbrains-sync.sh

# Remove all generated JetBrains theme files and the JAR.
jetbrains-clean:
    rm -f \
      'jetbrains/Warm Burnout Islands Dark.theme.json' \
      'jetbrains/Warm Burnout Islands Light.theme.json' \
      jetbrains/Warm-Burnout-Dark.xml \
      jetbrains/Warm-Burnout-Light.xml \
      jetbrains/warm-burnout-theme.jar
