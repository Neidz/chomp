#!/usr/bin/env sh

DESKTOP_FILE="Chomp.desktop"
DEST_DIR="$HOME/.local/share/applications"

cargo install --path .
mkdir -p "$DEST_DIR"
cp "$DESKTOP_FILE" "$DEST_DIR/"
chmod +x "$DEST_DIR/$DESKTOP_FILE"
echo ".desktop file installed to $DEST_DIR"
