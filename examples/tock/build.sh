#!/bin/bash
set -e

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <binary-name> <target-triple>"
    echo "Example: $0 blinky thumbv6m-none-eabi"
    exit 1
fi

BIN_NAME="$1"
TARGET="$2"
PROFILE="release"
ELF_PATH="target/$TARGET/$PROFILE/$BIN_NAME"
TAB_PATH="target/$BIN_NAME.tab"
TBF_PATH="target/$TARGET/$PROFILE/$BIN_NAME.tbf"

# Build the binary
cargo build --release --bin "$BIN_NAME" --target "$TARGET"

# Check if the ELF file exists
if [ ! -f "$ELF_PATH" ]; then
    echo "Error: ELF file not found at $ELF_PATH"
    exit 1
fi

# Run elf2tab
elf2tab \
    --package-name $BIN_NAME \
    --kernel-major 2 \
    --kernel-minor 1 \
    --stack 1024 \
    --minimum-footer-size 256 \
    --output-file $TAB_PATH \
    "$ELF_PATH"

echo "Built \`$TAB_PATH\` and \`$TBF_PATH\`"