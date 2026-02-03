//! Interrupt Descriptor Table (IDT)
//!
//! Provides minimal exception handling to prevent triple-faults.
//! Without an IDT, any CPU exception (invalid opcode, page fault, etc.)
//! will cause an instant reboot.

use core::arch::asm;

/// IDT entry for x86_64
#[repr(C, packed)]
#[derive(Clone, Copy)]
struct IdtEntry {
    offset_low: u16,     // Handler address bits 0..15
    selector: u16,       // Code segment selector (0x08 for kernel)
    ist: u8,             // Interrupt Stack Table offset
    flags: u8,           // Type and attributes
    offset_mid: u16,     // Handler address bits 16..31
    offset_high: u32,    // Handler address bits 32..63
    reserved: u32,       // Must be zero
}

impl IdtEntry {
    /// Create a null (empty) IDT entry
    const fn null() -> Self {
        Self {
            offset_low: 0,
            selector: 0,
            ist: 0,
            flags: 0,
            offset_mid: 0,
            offset_high: 0,
            reserved: 0,
        }
    }

    /// Create an IDT entry for an interrupt handler
    ///
    /// # Arguments
    /// * `handler` - Function pointer to exception handler
    /// * `selector` - Code segment selector (0x08 for kernel CS)
    /// * `flags` - IDT entry flags (0x8E = present, DPL=0, interrupt gate)
    fn new(handler: unsafe extern "C" fn(), selector: u16, flags: u8) -> Self {
        let addr = handler as u64;
        Self {
            offset_low: (addr & 0xFFFF) as u16,
            selector,
            ist: 0,
            flags,
            offset_mid: ((addr >> 16) & 0xFFFF) as u16,
            offset_high: ((addr >> 32) & 0xFFFFFFFF) as u32,
            reserved: 0,
        }
    }
}

/// IDT pointer structure for LIDT instruction
#[repr(C, packed)]
struct IdtPointer {
    limit: u16,   // Size of IDT - 1
    base: u64,    // Base address of IDT
}

/// Global Interrupt Descriptor Table (256 entries)
static mut IDT: [IdtEntry; 256] = [IdtEntry::null(); 256];

/// Initialize the IDT and load it into the CPU
pub fn init() {
    unsafe {
        // Setup exception handlers (0-31 are CPU exceptions)
        IDT[0] = IdtEntry::new(exception_0, 0x08, 0x8E);   // Divide Error
        IDT[1] = IdtEntry::new(exception_1, 0x08, 0x8E);   // Debug
        IDT[2] = IdtEntry::new(exception_2, 0x08, 0x8E);   // NMI
        IDT[3] = IdtEntry::new(exception_3, 0x08, 0x8E);   // Breakpoint
        IDT[4] = IdtEntry::new(exception_4, 0x08, 0x8E);   // Overflow
        IDT[5] = IdtEntry::new(exception_5, 0x08, 0x8E);   // Bound Range Exceeded
        IDT[6] = IdtEntry::new(exception_6, 0x08, 0x8E);   // Invalid Opcode
        IDT[7] = IdtEntry::new(exception_7, 0x08, 0x8E);   // Device Not Available
        IDT[8] = IdtEntry::new(exception_8, 0x08, 0x8E);   // Double Fault
        IDT[10] = IdtEntry::new(exception_10, 0x08, 0x8E); // Invalid TSS
        IDT[11] = IdtEntry::new(exception_11, 0x08, 0x8E); // Segment Not Present
        IDT[12] = IdtEntry::new(exception_12, 0x08, 0x8E); // Stack-Segment Fault
        IDT[13] = IdtEntry::new(exception_13, 0x08, 0x8E); // General Protection Fault
        IDT[14] = IdtEntry::new(exception_14, 0x08, 0x8E); // Page Fault

        // Create IDT pointer
        let idtr = IdtPointer {
            limit: (core::mem::size_of::<[IdtEntry; 256]>() - 1) as u16,
            base: &IDT as *const _ as u64,
        };

        // Load IDT into CPU
        asm!("lidt [{}]", in(reg) &idtr, options(readonly, nostack, preserves_flags));
    }

    crate::serial_println!("[KERNEL] IDT initialized (256 entries)");
}

//
// Exception Handlers
//

unsafe extern "C" fn exception_0() {
    crate::serial_println!("\n[EXCEPTION #0] Divide Error");
    crate::serial_println!("  Division by zero or division overflow");
    halt_loop();
}

unsafe extern "C" fn exception_1() {
    crate::serial_println!("\n[EXCEPTION #1] Debug");
    halt_loop();
}

unsafe extern "C" fn exception_2() {
    crate::serial_println!("\n[EXCEPTION #2] Non-Maskable Interrupt");
    halt_loop();
}

unsafe extern "C" fn exception_3() {
    crate::serial_println!("\n[EXCEPTION #3] Breakpoint");
    halt_loop();
}

unsafe extern "C" fn exception_4() {
    crate::serial_println!("\n[EXCEPTION #4] Overflow");
    halt_loop();
}

unsafe extern "C" fn exception_5() {
    crate::serial_println!("\n[EXCEPTION #5] Bound Range Exceeded");
    halt_loop();
}

unsafe extern "C" fn exception_6() {
    crate::serial_println!("\n[EXCEPTION #6] Invalid Opcode");
    crate::serial_println!("  Attempted to execute invalid instruction");
    crate::serial_println!("  (This was the bug when entry point was 0!)");
    halt_loop();
}

unsafe extern "C" fn exception_7() {
    crate::serial_println!("\n[EXCEPTION #7] Device Not Available");
    halt_loop();
}

unsafe extern "C" fn exception_8() {
    crate::serial_println!("\n[EXCEPTION #8] Double Fault");
    crate::serial_println!("  An exception occurred while handling another exception");
    halt_loop();
}

unsafe extern "C" fn exception_10() {
    crate::serial_println!("\n[EXCEPTION #10] Invalid TSS");
    halt_loop();
}

unsafe extern "C" fn exception_11() {
    crate::serial_println!("\n[EXCEPTION #11] Segment Not Present");
    halt_loop();
}

unsafe extern "C" fn exception_12() {
    crate::serial_println!("\n[EXCEPTION #12] Stack-Segment Fault");
    halt_loop();
}

unsafe extern "C" fn exception_13() {
    crate::serial_println!("\n[EXCEPTION #13] General Protection Fault");
    crate::serial_println!("  Invalid memory access or privilege violation");
    halt_loop();
}

unsafe extern "C" fn exception_14() {
    crate::serial_println!("\n[EXCEPTION #14] Page Fault");
    crate::serial_println!("  Invalid memory address or page not present");
    halt_loop();
}

/// Halt the CPU in a loop (called after fatal exception)
fn halt_loop() -> ! {
    crate::serial_println!("[KERNEL] System halted due to exception");
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
