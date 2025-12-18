#!/usr/bin/env bash
set -euo pipefail

# Simple example script

# Function example
greet() {
    local name="$1"
    echo "Hello, $name!"
}

# Main
main() {
    if [[ $# -lt 1 ]]; then
        echo "Usage: $0 <name>"
        exit 1
    fi

    greet "$1"
}

main "$@"
