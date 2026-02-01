#!/bin/bash
# Verify kernel size meets <1MB requirement

set -e

KERNEL_PATH="target/x86_64-unknown-none/release/cartridge-kernel"
MAX_SIZE=$((1024 * 1024))  # 1MB in bytes

# Build kernel if not exists
if [ ! -f "$KERNEL_PATH" ]; then
    echo "Building kernel..."
    cargo build --release --package cartridge-kernel
fi

# Get size
SIZE=$(stat -c%s "$KERNEL_PATH" 2>/dev/null || stat -f%z "$KERNEL_PATH" 2>/dev/null || wc -c < "$KERNEL_PATH")
SIZE_KB=$((SIZE / 1024))
SIZE_MB=$((SIZE / 1024 / 1024))

echo "==================================="
echo "Kernel Size Verification"
echo "==================================="
echo "Binary: $KERNEL_PATH"
echo "Size: $SIZE bytes ($SIZE_KB KB)"
echo "Limit: $MAX_SIZE bytes (1024 KB)"
echo ""

if [ "$SIZE" -lt "$MAX_SIZE" ]; then
    PERCENT=$((SIZE * 100 / MAX_SIZE))
    echo "✓ PASS: Kernel is ${PERCENT}% of 1MB limit"
    echo "  Remaining budget: $((MAX_SIZE - SIZE)) bytes"
    exit 0
else
    OVERAGE=$((SIZE - MAX_SIZE))
    echo "✗ FAIL: Kernel exceeds 1MB limit by $OVERAGE bytes"
    exit 1
fi
