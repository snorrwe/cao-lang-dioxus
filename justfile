_default:
    @just --list

dev:
    tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch &
    dx serve
