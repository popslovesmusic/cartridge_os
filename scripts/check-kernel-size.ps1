# Verify kernel size meets <1MB requirement (PowerShell version)

$ErrorActionPreference = "Stop"

$KernelPath = "target\x86_64-unknown-none\release\cartridge-kernel"
$MaxSize = 1024 * 1024  # 1MB

# Build kernel if not exists
if (-not (Test-Path $KernelPath)) {
    Write-Host "Building kernel..."
    cargo build --release --package cartridge-kernel
}

# Get size
$Size = (Get-Item $KernelPath).Length
$SizeKB = [math]::Round($Size / 1024, 2)
$SizeMB = [math]::Round($Size / 1024 / 1024, 3)

Write-Host "==================================="
Write-Host "Kernel Size Verification"
Write-Host "==================================="
Write-Host "Binary: $KernelPath"
Write-Host "Size: $Size bytes ($SizeKB KB)"
Write-Host "Limit: $MaxSize bytes (1024 KB)"
Write-Host ""

if ($Size -lt $MaxSize) {
    $Percent = [math]::Round(($Size * 100) / $MaxSize, 2)
    $Remaining = $MaxSize - $Size
    Write-Host "[PASS] Kernel is $Percent% of 1MB limit" -ForegroundColor Green
    Write-Host "  Remaining budget: $Remaining bytes"
    exit 0
} else {
    $Overage = $Size - $MaxSize
    Write-Host "[FAIL] Kernel exceeds 1MB limit by $Overage bytes" -ForegroundColor Red
    exit 1
}
