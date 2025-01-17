#!/bin/bash

# Source and destination directories
SOURCE_DIR="$HOME"
TEMP_DIR="/tmp/source_copy"
DEST_DIR="/dapp"

# Array of filenames to exclude
EXCLUDE_FILES=("Cargo.toml" "Cargo.lock" "rust-toolchain.toml", "example_sns_init.yaml")

# Create a temporary copy of the source directory
rm -rf "$TEMP_DIR" # Clean up any existing temp directory
cp -r "$SOURCE_DIR" "$TEMP_DIR"

# Delete specified files from the temporary copy
for file in "${EXCLUDE_FILES[@]}"; do
    if [ -f "$TEMP_DIR/$file" ]; then
        rm "$TEMP_DIR/$file"
    fi
done

# Create the destination directory if it doesn't exist
mkdir -p "$DEST_DIR"

# Copy the remaining files to the destination directory
cp -r "$TEMP_DIR"/* "$DEST_DIR"

# Clean up the temporary directory
rm -rf "$TEMP_DIR"

echo "Files copied successfully, excluding specified files."

# now copy some of own configuration files from /dapp/scripts/local-sns-testing

# example_sns_init.yaml
cp /dapp/scripts/local-sns-testing/example_sns_init.yaml /dapp

