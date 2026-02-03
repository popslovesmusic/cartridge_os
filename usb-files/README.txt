CARTRIDGE OS - Bootable USB Files
===================================

This folder contains all files needed to create a bootable USB drive.

File Structure:
  EFI/BOOT/BOOTX64.EFI  - UEFI bootloader (2 KB)
  kernel.bin            - Cartridge OS kernel (8.8 KB)
  cartridges/           - Cartridge applications
    hello-world.cart    - First demo cartridge (5.5 KB)

Total Size: ~17 KB


HOW TO CREATE BOOTABLE USB:
============================

Method 1: Rufus (Recommended for Windows)
------------------------------------------
1. Download Rufus: https://rufus.ie/
2. Insert USB drive (minimum 64 MB)
3. Run Rufus with these settings:
   - Partition scheme: GPT
   - Target system: UEFI (non CSM)
   - File system: FAT32
4. Click START to format
5. Copy all files from this folder to USB root
6. Eject safely

Method 2: Manual Copy
---------------------
1. Format USB as FAT32, GPT partition
2. Copy all files/folders from this directory to USB root
3. Eject safely

Method 3: balenaEtcher
----------------------
(Requires a .img file - see docs/MANUAL_IMAGE_CREATION.md)


HOW TO BOOT:
============
1. Insert USB into target computer
2. Enter BIOS/UEFI settings (F2, F12, Del, or Esc)
3. Enable UEFI boot mode
4. Disable Secure Boot (our bootloader is unsigned)
5. Select USB drive as boot device
6. Save and exit

Expected Output:
- Screen: Bootloader messages, hash verification
- Serial COM1 (115200 baud): Kernel and switcher output


BOOT SEQUENCE:
==============
Firmware → BOOTX64.EFI → kernel.bin → (future: switcher → cartridge)

Current Status:
✓ Bootloader verifies kernel SHA-256 hash
✓ Kernel initializes memory and syscalls
✓ Serial output on COM1
✓ Phase 2 complete - all components built

Next Steps:
- Switcher loading (will be embedded in kernel or initramfs)
- Cartridge execution from cartridges/ folder


TROUBLESHOOTING:
================
No boot:
  - Check UEFI mode enabled (not Legacy/CSM)
  - Check Secure Boot disabled
  - Try different USB port (USB 2.0 often more reliable)

Blank screen after bootloader:
  - This is normal! Kernel outputs to serial port only
  - Connect serial adapter to COM1 (115200 baud)
  - Or test in QEMU with -serial stdio

Hash verification failed:
  - Rebuild kernel and ensure packager creates correct hash
  - Check files copied correctly to USB


FILES IN THIS FOLDER:
=====================
EFI/BOOT/BOOTX64.EFI     2,048 bytes   UEFI bootloader
kernel.bin               8,992 bytes   Cartridge OS kernel
cartridges/hello-world.cart  5,560 bytes   Demo cartridge

Total: ~17 KB (kernel footprint 0.86% of 1MB limit!)


MORE INFO:
==========
- Full docs: E:\OS\docs\USB_BOOT_GUIDE.md
- Manual creation: E:\OS\MANUAL_IMAGE_CREATION.md
- Project: E:\OS\README.md

Built: 2026-02-01
Cartridge OS v0.1.0 - Phase 2 Complete
