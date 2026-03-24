#!/bin/bash
set -euo pipefail
programs=(charfreq)
cargo build --release
for bin in "${programs[@]}"; do
    sudo cp "target/release/$bin" /usr/bin/
    sudo cp "$bin/$bin.1" /usr/share/man/man1/
done
