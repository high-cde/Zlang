#!/bin/bash
echo "[ZLANG] Avvio toolchain ZDOS..."
TARGET="$1"
if [ -z "$TARGET" ]; then
    echo "Uso: ./one-shot-zlang.sh <file.zl>"
    exit 1
fi
cargo build --release
./target/release/zlang "$TARGET"
