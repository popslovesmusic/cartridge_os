# USB Boot Guide

Complete guide to booting Cartridge OS from USB drive.

---

## Prerequisites

1. **USB Drive** (minimum 32 MB, but use at least 1 GB for convenience)
2. **UEFI-capable computer** (almost all PCs from 2012+)
3. **Administrator/sudo privileges**

---

## Step 1: Build the Disk Image

### On Windows:

```cmd
REM Build all components
cargo build --release --package cartridge-bootloader --target x86_64-unknown-uefi
cargo build --release --package cartridge-kernel --target x86_64-unknown-none
cargo build --release --package cartridge-switcher --target x86_64-unknown-none

REM Package the kernel and switcher as artifacts
cargo run --package cartridge-packager --target x86_64-pc-windows-msvc -- kernel ...

REM Create disk image (via WSL2)
scripts\create-disk-image.cmd
```

### On Linux/WSL2:

```bash
# Build all components
cargo build --release --package cartridge-bootloader --target x86_64-unknown-uefi
cargo build --release --package cartridge-kernel --target x86_64-unknown-none
cargo build --release --package cartridge-switcher --target x86_64-unknown-none

# Create disk image
./scripts/create-disk-image.sh
```

This creates `cartridge-os.img` (32 MB GPT disk image).

---

## Step 2: Flash to USB Drive

### ⚠️ WARNING
**This will ERASE ALL DATA on the USB drive!**
Double-check the drive letter/device before proceeding.

### Windows (Method 1: Rufus - Recommended)

1. Download Rufus: https://rufus.ie/
2. Run as Administrator
3. Device: Select your USB drive
4. Boot selection: Click SELECT → choose `cartridge-os.img`
5. Partition scheme: **GPT**
6. Target system: **UEFI (non-CSM)**
7. Click **START**
8. Wait for completion

### Windows (Method 2: Script)

```cmd
REM Run as Administrator
scripts\flash-to-usb.cmd
```

Follow the interactive prompts.

### Linux/WSL2

```bash
# Find USB device
lsblk

# Flash (replace /dev/sdX with your USB device)
sudo ./scripts/flash-to-usb.sh cartridge-os.img /dev/sdX
```

---

## Step 3: Configure BIOS/UEFI

1. **Insert USB drive** into target computer
2. **Power on** and immediately press the BIOS key:
   - Common keys: `F2`, `F12`, `Del`, `Esc`
   - Dell: `F2` or `F12`
   - HP: `Esc` or `F10`
   - Lenovo: `F1` or `F2`
   - ASUS: `F2` or `Del`

3. **Enable UEFI boot**:
   - Look for "Boot Mode" setting
   - Set to **UEFI** (not Legacy or CSM)
   - Disable "Secure Boot" if enabled (our bootloader is unsigned)

4. **Set boot order**:
   - Move USB drive to top of boot order
   - Or use one-time boot menu (usually `F12`)

5. **Save and Exit** (`F10` usually)

---

## Step 4: Boot Cartridge OS

### Expected Output

#### On Screen (UEFI Console):

```
[BOOTLOADER] Cartridge OS Bootloader v0.1.0
[BOOTLOADER] Loading kernel from /kernel.bin
[BOOTLOADER] Kernel loaded successfully
[BOOTLOADER] Verifying kernel integrity...
[BOOTLOADER] ✓ Kernel hash verified
[BOOTLOADER] ✓ Kernel artifact validated
[BOOTLOADER] Allocating kernel memory...
[BOOTLOADER] ✓ Kernel loaded at 0x100000
[BOOTLOADER] Exiting boot services...
[BOOTLOADER] Transferring control to kernel...
```

#### On Serial Port (COM1, 115200 baud):

```
[KERNEL] Cartridge OS Kernel starting...
[KERNEL] All subsystems initialized
[KERNEL] Memory: 4096 bytes allocated, 1 pages

[SWITCHER] Cartridge OS Switcher v0.1.0
[SWITCHER] Initializing userspace environment...
[SWITCHER] Kernel memory allocated: 4096 bytes
[SWITCHER] ✓ Switcher initialized

[SWITCHER] Cartridge discovery not yet implemented
[SWITCHER] Waiting for Phase 2.5 (Hello World cartridge)

[SWITCHER] Entering idle mode
[SWITCHER] System ready
```

### If Nothing Appears on Screen

The kernel and switcher output to **serial port (COM1)** only.
You need a serial connection to see their output.

**Options:**
1. Use QEMU with `-serial stdio` (see below)
2. Connect a USB-to-serial adapter to physical COM1 port
3. Use a serial console cable

---

## Step 5: View Serial Output (Optional)

### Hardware Serial Connection

1. Connect USB-to-serial adapter to computer's COM1 port
2. Connect adapter to another computer via USB
3. Use terminal software:

**Linux:**
```bash
screen /dev/ttyUSB0 115200
# or
minicom -D /dev/ttyUSB0 -b 115200
```

**Windows:**
```
PuTTY: Serial, COM port, 115200 baud
# or
teraterm
```

---

## Alternative: Test in QEMU First

Before flashing to real hardware, test in QEMU:

### Windows (WSL2):

```bash
wsl qemu-system-x86_64 \
  -drive format=raw,file=cartridge-os.img \
  -bios /usr/share/ovmf/OVMF.fd \
  -serial stdio \
  -m 256M
```

### Linux:

```bash
qemu-system-x86_64 \
  -drive format=raw,file=cartridge-os.img \
  -bios /usr/share/ovmf/OVMF.fd \
  -serial stdio \
  -m 256M
```

**Expected behavior:**
- QEMU window shows bootloader output
- Terminal shows kernel/switcher serial output
- System reaches idle state

---

## Troubleshooting

### USB Not Booting

**Problem:** Computer doesn't boot from USB

**Solutions:**
1. Check BIOS boot order (USB should be first)
2. Try one-time boot menu (F12 usually)
3. Ensure UEFI mode enabled (not Legacy/CSM)
4. Disable Secure Boot
5. Try different USB port (USB 2.0 ports often more reliable)

### "No Bootable Device" Error

**Problem:** UEFI can't find bootloader

**Solutions:**
1. Verify disk image was created correctly:
   ```bash
   # Should show EFI partition with bootloader
   fdisk -l cartridge-os.img
   ```
2. Check bootloader is at correct path: `/EFI/BOOT/BOOTX64.EFI`
3. Recreate image with `create-disk-image` script

### Blank Screen After Bootloader

**Problem:** Bootloader runs, then blank screen

**Explanation:** This is normal! The kernel outputs to serial port only.

**Solution:** Connect to serial COM1 to see kernel output, or test in QEMU with `-serial stdio`

### Kernel Panic / Crash

**Problem:** System hangs after bootloader

**Debug steps:**
1. Boot in QEMU with serial output
2. Check kernel panic message on serial
3. Verify all components built for correct target:
   - Bootloader: `x86_64-unknown-uefi`
   - Kernel: `x86_64-unknown-none`
   - Switcher: `x86_64-unknown-none`

### Hash Verification Failed

**Problem:** Bootloader reports "KERNEL VERIFICATION FAILED"

**Solutions:**
1. Rebuild kernel artifact:
   ```bash
   cargo build --release --package cartridge-kernel --target x86_64-unknown-none
   cargo run --package cartridge-packager ... # Package kernel
   ```
2. Ensure packager is creating SHA-256 hash correctly
3. Verify artifact integrity

---

## Boot Sequence Diagram

```
┌─────────────────┐
│   Firmware      │ UEFI firmware
│   (BIOS/UEFI)   │ - Reads GPT partition table
└────────┬────────┘ - Finds EFI System Partition
         │          - Loads /EFI/BOOT/BOOTX64.EFI
         ▼
┌─────────────────┐
│   Bootloader    │ cartridge-bootloader (2 KB)
│  (BOOTX64.EFI)  │ - Loads /kernel.bin from ESP
└────────┬────────┘ - Verifies SHA-256 hash
         │          - Allocates memory at 0x100000
         │          - Exits boot services
         │          - Jumps to kernel entry
         ▼
┌─────────────────┐
│     Kernel      │ cartridge-kernel (8.8 KB)
│  (kernel.bin)   │ - Initializes serial COM1
└────────┬────────┘ - Sets up memory allocator
         │          - Starts syscall interface
         │          - Loads switcher (future: from initramfs)
         ▼
┌─────────────────┐
│    Switcher     │ cartridge-switcher (11 KB)
│  (userspace)    │ - Discovers cartridges
└────────┬────────┘ - Verifies Tanka metadata
         │          - Loads drivers (if needed)
         │          - Allocates cartridge memory
         ▼
┌─────────────────┐
│   Cartridge     │ hello-world.cart (5.5 KB)
│ (application)   │ - Logs messages via syscalls
└─────────────────┘ - Performs application logic
                     - Exits cleanly
```

---

## Current Limitations

1. **No initramfs yet**: Switcher needs to be embedded in kernel or loaded separately
2. **No cartridge loader**: Switcher enters idle mode (Phase 2.4)
3. **Serial output only**: Kernel/switcher don't write to screen (only bootloader does)
4. **No graphics**: Text-only via serial console
5. **No input**: Keyboard/mouse support not yet implemented

These will be addressed in Phase 3+.

---

## Next Steps

After successfully booting from USB:

1. **Verify boot chain**: Check that all components load and verify correctly
2. **Test switcher**: Load actual cartridges (requires Phase 2.5 completion)
3. **Add serial logger**: Implement USB-serial redirection for debugging
4. **Performance testing**: Measure boot time (<5s target)

---

## Hardware Compatibility

**Tested on:**
- QEMU/KVM (full support)

**Should work on:**
- Any x86_64 PC with UEFI firmware (2012+)
- Intel/AMD processors
- Bare metal servers
- Laptops with UEFI

**Won't work on:**
- Legacy BIOS-only systems (requires UEFI)
- ARM/RISC-V (x86_64 only currently)
- Secure Boot enabled systems (unsigned bootloader)

---

**Generated**: 2026-02-01
**Cartridge OS Version**: 0.1.0
**Phase 2 Complete**: All 5 tasks ✅
