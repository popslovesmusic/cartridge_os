@echo off
REM Flash Cartridge OS disk image to USB drive
REM
REM WARNING: This will ERASE ALL DATA on the target USB drive!
REM
REM Prerequisites:
REM   - Rufus (portable) or dd for Windows
REM   - Administrator privileges

setlocal enabledelayedexpansion

set IMAGE_FILE=%1
if "%IMAGE_FILE%"=="" set IMAGE_FILE=cartridge-os.img

echo.
echo ========================================
echo  Cartridge OS USB Flasher
echo ========================================
echo.

REM Check if image exists
if not exist "%IMAGE_FILE%" (
    echo [ERROR] Image file not found: %IMAGE_FILE%
    echo.
    echo Please build the image first:
    echo   scripts\create-disk-image.cmd
    exit /b 1
)

echo [INFO] Image file: %IMAGE_FILE%
echo [INFO] Size:
for %%A in ("%IMAGE_FILE%") do echo   %%~zA bytes
echo.

echo ========================================
echo  WARNING: THIS WILL ERASE YOUR USB!
echo ========================================
echo.
echo This script will write the disk image to a USB drive.
echo ALL DATA on the target drive will be PERMANENTLY LOST.
echo.

REM List available drives
echo Available drives:
echo.
wmic logicaldisk get name,volumename,size,description
echo.

set /p DRIVE_LETTER="Enter USB drive letter (e.g., E): "

REM Validate drive letter
if "%DRIVE_LETTER%"=="" (
    echo [ERROR] No drive letter entered
    exit /b 1
)

REM Remove colon if present
set DRIVE_LETTER=%DRIVE_LETTER::=%

REM Check if drive exists
if not exist "%DRIVE_LETTER%:\" (
    echo [ERROR] Drive %DRIVE_LETTER%: does not exist
    exit /b 1
)

echo.
echo [WARNING] You selected drive %DRIVE_LETTER%:
echo.
wmic logicaldisk where "DeviceID='%DRIVE_LETTER%:'" get DeviceID,VolumeName,Size,FileSystem
echo.

set /p CONFIRM="Type YES (in capitals) to proceed: "

if not "%CONFIRM%"=="YES" (
    echo [INFO] Operation cancelled
    exit /b 0
)

echo.
echo [INFO] Flashing image to USB drive...
echo.

REM Check for Rufus
where rufus >nul 2>&1
if %errorlevel% equ 0 (
    echo [INFO] Using Rufus to flash image
    echo.
    echo Please use Rufus GUI to:
    echo   1. Select device: %DRIVE_LETTER%:
    echo   2. Select image: %IMAGE_FILE%
    echo   3. Partition scheme: GPT
    echo   4. Target system: UEFI
    echo   5. Click START
    echo.
    start rufus
    goto :end
)

REM Check for dd (Git Bash, WSL, or standalone)
where dd >nul 2>&1
if %errorlevel% equ 0 (
    echo [INFO] Using dd to flash image
    echo.
    echo [WARNING] Make sure you have Administrator privileges
    echo.

    REM Convert to dd-compatible path
    set DD_INPUT=%IMAGE_FILE:\=/%
    set DD_OUTPUT=\\.\PhysicalDrive%DRIVE_LETTER%

    echo Command: dd if=%DD_INPUT% of=%DD_OUTPUT% bs=4M status=progress
    echo.
    echo [INFO] This may take several minutes...
    echo.

    dd if=%DD_INPUT% of=%DD_OUTPUT% bs=4M status=progress

    if %errorlevel% equ 0 (
        echo.
        echo [SUCCESS] Image flashed successfully!
        goto :success
    ) else (
        echo.
        echo [ERROR] Failed to flash image
        echo.
        echo Try running as Administrator:
        echo   Right-click Command Prompt ^> Run as administrator
        echo   Then run this script again
        exit /b 1
    )
)

REM Neither tool found - show manual instructions
echo [INFO] No flashing tool found (Rufus or dd)
echo.
echo Please use one of these methods:
echo.
echo === Method 1: Rufus (Recommended for Windows) ===
echo   1. Download Rufus: https://rufus.ie/
echo   2. Run Rufus as Administrator
echo   3. Select device: %DRIVE_LETTER%:
echo   4. Click SELECT and choose: %IMAGE_FILE%
echo   5. Partition scheme: GPT
echo   6. Target system: UEFI (non-CSM)
echo   7. Click START
echo.
echo === Method 2: balenaEtcher ===
echo   1. Download: https://www.balena.io/etcher/
echo   2. Select image: %IMAGE_FILE%
echo   3. Select target: %DRIVE_LETTER%:
echo   4. Click Flash
echo.
echo === Method 3: dd (WSL/Git Bash) ===
echo   1. Open WSL or Git Bash as Administrator
echo   2. Run: dd if=%IMAGE_FILE% of=/dev/sd? bs=4M status=progress
echo      (replace ? with correct drive letter)
echo.
goto :end

:success
echo.
echo ========================================
echo  USB Drive Ready!
echo ========================================
echo.
echo The USB drive is now bootable with Cartridge OS.
echo.
echo To boot from USB:
echo   1. Insert the USB drive into target computer
echo   2. Enter BIOS/UEFI settings (usually F2, F12, Del, or Esc at boot)
echo   3. Enable UEFI boot mode (disable Legacy/CSM if present)
echo   4. Select the USB drive as boot device
echo   5. Save and exit
echo.
echo Expected boot sequence:
echo   [Firmware] -^> [Bootloader] -^> [Kernel] -^> [Switcher] -^> [Cartridge]
echo.
echo Output will appear on:
echo   - UEFI console (bootloader messages)
echo   - Serial port COM1 (kernel/switcher messages)
echo.
echo To view serial output:
echo   - Hardware: Connect USB-to-serial adapter to COM1
echo   - Virtual: Use QEMU with -serial stdio
echo.

:end
echo.
pause
endlocal
