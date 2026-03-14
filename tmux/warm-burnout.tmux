#!/usr/bin/env bash
# Warm Burnout -- TPM plugin entry point
# https://github.com/felipefdl/warm-burnout

CURRENT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

variant="$(tmux show-option -gqv @warm-burnout-variant)"
variant="${variant:-dark}"

tmux source-file "$CURRENT_DIR/warm-burnout-${variant}.conf"
