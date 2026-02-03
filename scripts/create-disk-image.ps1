# Create bootable UEFI disk image for Cartridge OS (PowerShell version)
#
# Usage: .\scripts\create-disk-image.ps1 [-ImageName "cartridge-os.img"]

param(
    [string]$ImageName = "cartridge-os.img",
    [int]$ImageSizeMB = 32
)

$ErrorActionPreference = "Stop"

function Write-Info {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor Green
}

function Write-Error {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
}

function Write-Warn {
    param([string]$Message)
    Write-Host "[WARN] $Message" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Cartridge OS Disk Image Creator (PowerShell)"
Write-Host "============================================="
Write-Host ""

# Check prerequisites
Write-Info "Checking build artifacts..."

$bootloader = "target\x86_64-unknown-uefi\release\cartridge-bootloader.efi"
$kernel = "target\x86_64-unknown-none\release\cartridge-kernel"

if (-not (Test-Path $bootloader)) {
    Write-Error "Bootloader not found: $bootloader"
    Write-Host ""
    Write-Host "Build it first:"
    Write-Host "  cargo build --release --package cartridge-bootloader --target x86_64-unknown-uefi"
    exit 1
}

if (-not (Test-Path $kernel)) {
    Write-Error "Kernel not found: $kernel"
    Write-Host ""
    Write-Host "Build it first:"
    Write-Host "  cargo build --release --package cartridge-kernel --target x86_64-unknown-none"
    exit 1
}

Write-Info "All build artifacts found"
Write-Host ""

# Remove old image
if (Test-Path $ImageName) {
    Write-Warn "Removing existing $ImageName"
    Remove-Item $ImageName -Force
}

# Create empty image file
Write-Info "Creating disk image: $ImageName ($ImageSizeMB MB)"
$bytes = [byte[]]::new($ImageSizeMB * 1024 * 1024)
[System.IO.File]::WriteAllBytes((Resolve-Path ".").Path + "\$ImageName", $bytes)

Write-Info "Image file created"
Write-Host ""

# For now, create a note about manual setup
Write-Warn "Automatic disk partitioning requires WSL2 or Linux"
Write-Host ""
Write-Host "To complete the image, you have two options:"
Write-Host ""
Write-Host "=== Option 1: Use WSL2 (Recommended) ==="
Write-Host "  1. Install WSL2 if not already installed:"
Write-Host "     wsl --install"
Write-Host "  2. Install required tools in WSL:"
Write-Host "     wsl sudo apt install dosfstools util-linux mtools"
Write-Host "  3. Run the Linux script:"
Write-Host "     wsl bash scripts/create-disk-image.sh"
Write-Host ""
Write-Host "=== Option 2: Manual Setup with Rufus ==="
Write-Host "  1. Download Rufus: https://rufus.ie/"
Write-Host "  2. In Rufus:"
Write-Host "     - Device: Select a USB drive (or create image)"
Write-Host "     - Partition: GPT"
Write-Host "     - Target: UEFI (non-CSM)"
Write-Host "     - File system: FAT32"
Write-Host "  3. After creating partition, manually copy:"
Write-Host "     - Bootloader → \EFI\BOOT\BOOTX64.EFI"
Write-Host "     - Kernel → \kernel.bin"
Write-Host ""
Write-Host "=== Option 3: Use Pre-made Image ==="
Write-Host "  If you have access to a Linux system or WSL2,"
Write-Host "  run scripts/create-disk-image.sh there and transfer"
Write-Host "  the resulting .img file back to Windows"
Write-Host ""

Write-Info "Partial image file created: $ImageName"
Write-Info "See options above to complete the bootable image"
Write-Host ""
