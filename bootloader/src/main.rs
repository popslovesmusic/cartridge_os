//! UEFI Bootloader
//!
//! Verifies kernel signature/hash and hands off control.
//! This is the first link in the chain of trust.

#![no_std]
#![no_main]

use cartridge_common::{Artifact, ArtifactType};

/// Bootloader entry point (UEFI application)
#[no_mangle]
pub extern "C" fn efi_main() -> ! {
    // Step 1: Initialize UEFI services
    init_uefi();

    // Step 2: Load kernel artifact from boot partition
    let kernel_data = load_kernel_from_disk();

    // Step 3: Verify kernel artifact
    match Artifact::from_bytes(&kernel_data) {
        Ok(artifact) => {
            if artifact.header.artifact_type != ArtifactType::Kernel as u8 {
                panic_with_message("Invalid artifact type: expected kernel");
            }

            // TODO: Verify signature against trusted public key

            boot_log("Kernel verified successfully");

            // Step 4: Load kernel into memory and transfer control
            jump_to_kernel(&artifact);
        }
        Err(_) => {
            panic_with_message("Kernel verification failed");
        }
    }

    loop {}
}

fn init_uefi() {
    // TODO: Initialize UEFI boot services
}

fn load_kernel_from_disk() -> Vec<u8> {
    // TODO: Read kernel artifact from boot partition
    // For now, return dummy data
    Vec::new()
}

fn jump_to_kernel(artifact: &Artifact) -> ! {
    // TODO: Set up initial page tables
    // TODO: Load kernel at expected address
    // TODO: Transfer control to kernel entry point

    boot_log("Transferring control to kernel...");

    loop {}
}

fn boot_log(msg: &str) {
    // TODO: Output to UEFI console or serial
}

fn panic_with_message(msg: &str) -> ! {
    boot_log(&alloc::format!("BOOTLOADER PANIC: {}", msg));
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;
