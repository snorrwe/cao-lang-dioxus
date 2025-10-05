_default:
    @just --list

dev:
    #!/usr/bin/env bash
    set -euxo pipefail
    tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch &
    dx serve
