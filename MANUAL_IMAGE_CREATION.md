# Manual Disk Image Creation

Since automated tools (WSL2/Linux) aren't available, here's how to manually create a bootable USB.

---

## Option 1: Direct USB Creation with Rufus (Easiest)

### Step 1: Prepare Files

Create a folder structure:
```
E:\OS\usb-files\
├── EFI\
│   └── BOOT\
│       └── BOOTX64.EFI    (copy from target\x86_64-unknown-uefi\release\cartridge-bootloader.efi)
└── kernel.bin              (copy from target\x86_64-unknown-none\release\cartridge-kernel)
```

Commands:
```cmd
mkdir E:\OS\usb-files\EFI\BOOT
copy target\x86_64-unknown-uefi\release\cartridge-bootloader.efi E:\OS\usb-files\EFI\BOOT\BOOTX64.EFI
copy target\x86_64-unknown-none\release\cartridge-kernel E:\OS\usb-files\kernel.bin
```

### Step 2: Create Bootable USB with Rufus

1. **Download Rufus**: https://rufus.ie/ (portable version, no install)

2. **Insert USB drive** (minimum 64 MB, will be erased!)

3. **Run Rufus** with these settings:
   - Device: Your USB drive
   - Boot selection: **Non bootable**
   - Partition scheme: **GPT**
   - Target system: **UEFI (non CSM)**
   - File system: **FAT32**
   - Cluster size: Default

4. **Click START**

5. **After format completes**, manually copy files:
   ```
   Copy E:\OS\usb-files\* to USB drive
   ```

6. **Eject USB safely**

### Step 3: Boot

1. Insert USB into target computer
2. Enter BIOS (F2/F12/Del)
3. Enable UEFI mode, disable Secure Boot
4. Boot from USB

---

## Option 2: VirtualBox Image (For Testing)

### Create VirtualBox VM:

1. **Create New VM**:
   - Name: Cartridge OS
   - Type: Other
   - Version: Other/Unknown (64-bit)
   - Memory: 256 MB
   - Hard disk: Create virtual hard disk now

2. **Hard Disk Settings**:
   - Type: VDI
   - Storage: Dynamically allocated
   - Size: 64 MB

3. **VM Settings**:
   - System → Motherboard:
     - Enable EFI (required!)
   - System → Processor: 1 CPU
   - Display → Screen: 16 MB VRAM

4. **Mount ISO/Image**:
   - Create ISO from usb-files folder using ImgBurn or similar
   - OR attach the folder as a shared folder

5. **Manual Setup** (one-time):
   - Boot VM from live USB/CD
   - Mount VM disk
   - Copy files to VM disk in proper structure

---

## Option 3: QEMU (Testing Without USB)

If you have QEMU installed:

### Create Raw Image:

```powershell
# Create 32MB image
$size = 32 * 1024 * 1024
$bytes = [byte[]]::new($size)
[System.IO.File]::WriteAllBytes("E:\OS\cartridge-os.img", $bytes)
```

### Then use Linux/WSL to partition:

```bash
# If you can access WSL later
wsl sudo apt install dosfstools mtools
wsl bash scripts/create-disk-image.sh
```

### Boot in QEMU:

```powershell
qemu-system-x86_64.exe `
  -drive format=raw,file=E:\OS\cartridge-os.img `
  -bios OVMF.fd `
  -serial stdio `
  -m 256M
```

(Requires OVMF firmware: https://www.kraxel.org/repos/jenkins/edk2/)

---

## Option 4: Quick Test - Just Bootloader

For quick verification without full disk image:

### Test Bootloader Only in QEMU:

```powershell
# Create minimal FAT image with just bootloader
qemu-system-x86_64.exe `
  -drive format=raw,file=fat:rw:E:\OS\usb-files `
  -bios OVMF.fd `
  -serial stdio
```

This creates a temporary FAT filesystem from the usb-files folder.

---

## Current File Summary

Your built files:
- **Bootloader**: `target\x86_64-unknown-uefi\release\cartridge-bootloader.efi` (2 KB)
- **Kernel**: `target\x86_64-unknown-none\release\cartridge-kernel` (8.8 KB)
- **Switcher**: `target\x86_64-unknown-none\release\cartridge-switcher` (11 KB)
- **Cartridge**: `cartridges\hello-world.cart` (5.5 KB)

Where they need to go:
- **USB:\EFI\BOOT\BOOTX64.EFI** ← bootloader
- **USB:\kernel.bin** ← kernel
- **USB:\cartridges\hello-world.cart** ← cartridge (optional for now)

---

## Recommended: Install WSL2 for Future Builds

For easier automation in future:

```powershell
# Enable WSL
wsl --install

# After reboot, install Ubuntu
wsl --install -d Ubuntu

# In WSL, install tools
wsl sudo apt update
wsl sudo apt install dosfstools util-linux mtools

# Then use our script
wsl bash scripts/create-disk-image.sh
```

---

## Next Steps After USB Creation

Once you have a bootable USB:

1. **Test in VirtualBox/QEMU first** (safer)
2. **Then test on real hardware**
3. **Connect serial adapter** to see kernel output (COM1, 115200 baud)

Expected boot sequence:
```
[BOOTLOADER] Cartridge OS Bootloader v0.1.0
[BOOTLOADER] Loading kernel...
[BOOTLOADER] ✓ Kernel hash verified
[BOOTLOADER] Transferring control to kernel...

(Screen goes blank - output now on serial COM1)

[KERNEL] Cartridge OS Kernel starting...
[KERNEL] All subsystems initialized
[SWITCHER] Cartridge OS Switcher v0.1.0
[SWITCHER] System ready
```

---

**Note**: The automated scripts require Linux tools. For Windows-only workflow, use Rufus (Option 1) for now.
