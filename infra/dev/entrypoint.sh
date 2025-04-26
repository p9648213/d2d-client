#!/bin/bash

# Run Tailwind CLI in background
npx @tailwindcss/cli -i ./tailwind.css -o ./assets/css/lib/tailwind.css --watch &

# Run your Rust app with cargo watch
cargo watch -c -w src -x run