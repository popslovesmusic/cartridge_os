@echo off
REM Create bootable UEFI disk image for Cartridge OS (Windows version)
REM
REM Usage: scripts\create-disk-image.cmd [image_name]
REM
REM Prerequisites:
REM   - WSL2 installed (runs the Linux script internally)
REM   - Build artifacts compiled

setlocal enabledelayedexpansion

set IMAGE_NAME=%1
if "%IMAGE_NAME%"=="" set IMAGE_NAME=cartridge-os.img

echo.
echo Cartridge OS Disk Image Creator (Windows)
echo ==========================================
echo.

REM Check if WSL is available
wsl --version >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] WSL2 not found. Please install WSL2:
    echo   https://docs.microsoft.com/en-us/windows/wsl/install
    exit /b 1
)

echo [INFO] Detected WSL2
echo.

REM Check if artifacts exist
if not exist "target\x86_64-unknown-uefi\release\cartridge-bootloader.efi" (
    echo [ERROR] Bootloader not found. Build it first:
    echo   cargo build --release --package cartridge-bootloader --target x86_64-unknown-uefi
    exit /b 1
)

if not exist "target\x86_64-unknown-none\release\cartridge-kernel" (
    echo [ERROR] Kernel not found. Build it first:
    echo   cargo build --release --package cartridge-kernel --target x86_64-unknown-none
    exit /b 1
)

echo [INFO] All build artifacts found
echo.

REM Convert Windows path to WSL path
set CURRENT_DIR=%CD%
set WSL_PATH=!CURRENT_DIR:\=/!
set WSL_PATH=/mnt/!WSL_PATH:~0,1!!WSL_PATH:~2!
set WSL_PATH=!WSL_PATH:E=/e!

echo [INFO] Running disk image creation in WSL2...
echo.

REM Execute the Linux script in WSL
wsl bash -c "cd '!WSL_PATH!' && bash scripts/create-disk-image.sh %IMAGE_NAME%"

if %errorlevel% neq 0 (
    echo.
    echo [ERROR] Image creation failed
    exit /b 1
)

echo.
echo [SUCCESS] Disk image created: %IMAGE_NAME%
echo.
echo To boot in QEMU on Windows:
echo   1. Install QEMU for Windows: https://qemu.weidai.com.cn/
echo   2. Download OVMF firmware: https://www.kraxel.org/repos/jenkins/edk2/
echo   3. Run:
echo      qemu-system-x86_64 -drive format=raw,file=%IMAGE_NAME% ^
echo        -bios OVMF.fd -serial stdio -m 256M
echo.
echo Or use WSL2:
echo   wsl qemu-system-x86_64 -drive format=raw,file=%IMAGE_NAME% ^
echo     -bios /usr/share/ovmf/OVMF.fd -serial stdio -m 256M
echo.

endlocal
