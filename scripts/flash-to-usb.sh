#!/usr/bin/env bash
# Flash Cartridge OS disk image to USB drive
#
# WARNING: This will ERASE ALL DATA on the target USB drive!
#
# Usage: sudo ./scripts/flash-to-usb.sh [image_file] [device]

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
IMAGE_FILE="${1:-cartridge-os.img}"
TARGET_DEVICE="${2:-}"

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

show_usage() {
    echo "Usage: sudo $0 [image_file] [device]"
    echo
    echo "Examples:"
    echo "  sudo $0                                  # Interactive mode"
    echo "  sudo $0 cartridge-os.img                # Interactive device selection"
    echo "  sudo $0 cartridge-os.img /dev/sdb       # Direct flash to /dev/sdb"
    echo
}

# Check if running as root
if [[ $EUID -ne 0 ]]; then
    log_error "This script must be run as root (use sudo)"
    show_usage
    exit 1
fi

# Check if image exists
if [[ ! -f "$IMAGE_FILE" ]]; then
    log_error "Image file not found: $IMAGE_FILE"
    echo
    echo "Please build the image first:"
    echo "  ./scripts/create-disk-image.sh"
    exit 1
fi

echo
echo "========================================"
echo "  Cartridge OS USB Flasher"
echo "========================================"
echo
log_info "Image file: $IMAGE_FILE"
log_info "Size: $(du -h "$IMAGE_FILE" | cut -f1)"
echo

# Show available block devices
log_info "Available USB devices:"
echo
lsblk -o NAME,SIZE,TYPE,MOUNTPOINT,VENDOR,MODEL | grep -E "disk|NAME"
echo

# If device not specified, ask for it
if [[ -z "$TARGET_DEVICE" ]]; then
    echo -n "Enter target device (e.g., /dev/sdb): "
    read -r TARGET_DEVICE
fi

# Validate device
if [[ ! -b "$TARGET_DEVICE" ]]; then
    log_error "Device does not exist or is not a block device: $TARGET_DEVICE"
    exit 1
fi

# Check if it's a partition instead of whole device
if [[ "$TARGET_DEVICE" =~ [0-9]$ ]]; then
    log_warn "You specified a partition ($TARGET_DEVICE), not a whole device"
    log_warn "You probably want ${TARGET_DEVICE%[0-9]} instead"
    echo
    echo -n "Continue anyway? (y/N): "
    read -r response
    if [[ ! "$response" =~ ^[Yy]$ ]]; then
        log_info "Aborted"
        exit 0
    fi
fi

# Show device information
echo
log_warn "Target device: $TARGET_DEVICE"
echo
lsblk -o NAME,SIZE,TYPE,MOUNTPOINT,VENDOR,MODEL "$TARGET_DEVICE"
echo

# Check if device is mounted
if mount | grep -q "^$TARGET_DEVICE"; then
    log_warn "Device has mounted partitions:"
    mount | grep "^$TARGET_DEVICE"
    echo
    log_info "Unmounting..."
    umount "${TARGET_DEVICE}"* 2>/dev/null || true
fi

# Final confirmation
echo
echo "========================================"
echo "  WARNING: THIS WILL ERASE YOUR USB!"
echo "========================================"
echo
log_warn "ALL DATA on $TARGET_DEVICE will be PERMANENTLY LOST!"
echo
echo -n "Type YES (in capitals) to proceed: "
read -r CONFIRM

if [[ "$CONFIRM" != "YES" ]]; then
    log_info "Operation cancelled"
    exit 0
fi

echo
log_info "Flashing image to $TARGET_DEVICE..."
echo

# Flash the image using dd
if dd if="$IMAGE_FILE" of="$TARGET_DEVICE" bs=4M status=progress conv=fsync; then
    sync
    echo
    log_info "Flushing disk cache..."
    sync

    echo
    echo "========================================"
    echo "  USB Drive Ready!"
    echo "========================================"
    echo
    log_info "The USB drive is now bootable with Cartridge OS"
    echo
    echo "To boot from USB:"
    echo "  1. Insert the USB drive into target computer"
    echo "  2. Enter BIOS/UEFI settings (usually F2, F12, Del, or Esc)"
    echo "  3. Enable UEFI boot mode (disable Legacy/CSM)"
    echo "  4. Select the USB drive as boot device"
    echo "  5. Save and exit"
    echo
    echo "Expected boot sequence:"
    echo "  [Firmware] → [Bootloader] → [Kernel] → [Switcher] → [Cartridge]"
    echo
    echo "Output appears on:"
    echo "  - UEFI console: Bootloader messages"
    echo "  - Serial COM1: Kernel/switcher messages (115200 baud)"
    echo
    echo "To view serial output:"
    echo "  - Hardware: Connect USB-to-serial adapter to COM1"
    echo "  - QEMU: Run with -serial stdio"
    echo "  - screen /dev/ttyUSB0 115200"
    echo "  - minicom -D /dev/ttyUSB0 -b 115200"
    echo
else
    echo
    log_error "Failed to flash image"
    exit 1
fi
