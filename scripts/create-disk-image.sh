#!/usr/bin/env bash
# Create bootable UEFI disk image for Cartridge OS
#
# Usage: ./scripts/create-disk-image.sh [image_name]
#
# Creates a GPT-partitioned disk image with:
# - EFI System Partition (FAT32)
# - Bootloader at /EFI/BOOT/BOOTX64.EFI
# - Kernel at /kernel.bin

set -euo pipefail

# Configuration
IMAGE_NAME="${1:-cartridge-os.img}"
IMAGE_SIZE="32M"  # Small image for testing
MOUNT_POINT="/tmp/cartridge-os-mount"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."

    local missing=()

    command -v dd >/dev/null 2>&1 || missing+=("dd")
    command -v mkfs.fat >/dev/null 2>&1 || missing+=("mkfs.fat (dosfstools)")
    command -v sfdisk >/dev/null 2>&1 || missing+=("sfdisk (util-linux)")
    command -v mcopy >/dev/null 2>&1 || missing+=("mcopy (mtools)")
    command -v mmd >/dev/null 2>&1 || missing+=("mmd (mtools)")

    if [ ${#missing[@]} -gt 0 ]; then
        log_error "Missing required tools:"
        printf '  - %s\n' "${missing[@]}"
        echo
        echo "Install with:"
        echo "  sudo apt install dosfstools util-linux mtools"
        exit 1
    fi

    log_info "All prerequisites satisfied"
}

# Check that artifacts exist
check_artifacts() {
    log_info "Checking build artifacts..."

    local missing=()

    [ -f "target/x86_64-unknown-uefi/release/cartridge-bootloader.efi" ] || \
        missing+=("bootloader (run: cargo build --release --package cartridge-bootloader --target x86_64-unknown-uefi)")

    [ -f "target/x86_64-unknown-none/release/cartridge-kernel" ] || \
        missing+=("kernel (run: cargo build --release --package cartridge-kernel --target x86_64-unknown-none)")

    if [ ${#missing[@]} -gt 0 ]; then
        log_error "Missing build artifacts:"
        printf '  - %s\n' "${missing[@]}"
        exit 1
    fi

    log_info "All build artifacts found"
}

# Create empty disk image
create_image() {
    log_info "Creating disk image: $IMAGE_NAME ($IMAGE_SIZE)"

    dd if=/dev/zero of="$IMAGE_NAME" bs=1M count=32 status=progress

    log_info "Created $IMAGE_NAME"
}

# Partition disk with GPT
partition_disk() {
    log_info "Creating GPT partition table..."

    # Create GPT with single EFI System Partition
    sfdisk "$IMAGE_NAME" <<EOF
label: gpt
type=C12A7328-F81F-11D2-BA4B-00A0C93EC93B, name="EFI System"
EOF

    log_info "Partitioned disk"
}

# Format ESP as FAT32
format_esp() {
    log_info "Formatting EFI System Partition as FAT32..."

    # Extract partition offset (in 512-byte sectors)
    local offset=$(sfdisk -l "$IMAGE_NAME" | grep "EFI System" | awk '{print $2}')
    local offset_bytes=$((offset * 512))

    # Format the partition using mformat (no loop device needed)
    mformat -i "$IMAGE_NAME@@$offset_bytes" -F ::

    log_info "Formatted ESP"
}

# Copy bootloader and kernel to ESP
populate_esp() {
    log_info "Copying bootloader and kernel to ESP..."

    # Extract partition offset
    local offset=$(sfdisk -l "$IMAGE_NAME" | grep "EFI System" | awk '{print $2}')
    local offset_bytes=$((offset * 512))
    local img_offset="$IMAGE_NAME@@$offset_bytes"

    # Create /EFI/BOOT directory
    mmd -i "$img_offset" ::/EFI
    mmd -i "$img_offset" ::/EFI/BOOT

    # Copy bootloader as BOOTX64.EFI (standard UEFI boot path)
    mcopy -i "$img_offset" \
        target/x86_64-unknown-uefi/release/cartridge-bootloader.efi \
        ::/EFI/BOOT/BOOTX64.EFI

    log_info "Copied bootloader → /EFI/BOOT/BOOTX64.EFI"

    # Copy kernel as kernel.bin
    mcopy -i "$img_offset" \
        target/x86_64-unknown-none/release/cartridge-kernel \
        ::/kernel.bin

    log_info "Copied kernel → /kernel.bin"
}

# Verify image contents
verify_image() {
    log_info "Verifying image contents..."

    local offset=$(sfdisk -l "$IMAGE_NAME" | grep "EFI System" | awk '{print $2}')
    local offset_bytes=$((offset * 512))
    local img_offset="$IMAGE_NAME@@$offset_bytes"

    echo
    echo "ESP contents:"
    mdir -i "$img_offset" -/ ::/

    log_info "Disk image ready"
}

# Print usage instructions
print_usage() {
    echo
    echo -e "${GREEN}================================================================${NC}"
    echo -e "${GREEN}  Cartridge OS Bootable Disk Image Created${NC}"
    echo -e "${GREEN}================================================================${NC}"
    echo
    echo "Image: $IMAGE_NAME"
    echo "Size:  $(du -h "$IMAGE_NAME" | cut -f1)"
    echo
    echo "To boot in QEMU:"
    echo
    echo "  qemu-system-x86_64 \\"
    echo "    -drive format=raw,file=$IMAGE_NAME \\"
    echo "    -bios /usr/share/ovmf/OVMF.fd \\"
    echo "    -serial stdio \\"
    echo "    -m 256M"
    echo
    echo "Note: Install OVMF firmware if missing:"
    echo "  sudo apt install ovmf"
    echo
    echo -e "${GREEN}================================================================${NC}"
    echo
}

# Main execution
main() {
    echo
    echo "Cartridge OS Disk Image Creator"
    echo "================================"
    echo

    check_prerequisites
    check_artifacts

    # Remove old image if exists
    [ -f "$IMAGE_NAME" ] && {
        log_warn "Removing existing $IMAGE_NAME"
        rm -f "$IMAGE_NAME"
    }

    create_image
    partition_disk
    format_esp
    populate_esp
    verify_image
    print_usage
}

main "$@"
