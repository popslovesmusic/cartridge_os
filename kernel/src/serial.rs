//! Serial port driver (COM1)
//!
//! Provides basic serial output for kernel debugging.
//! This is our ONLY output mechanism until we have a screen driver.
//!
//! Hardware:
//! - COM1 base address: 0x3F8
//! - Baud rate: 115200
//! - Data bits: 8
//! - Parity: None
//! - Stop bits: 1
//!
//! Usage in QEMU: -serial stdio

use core::fmt;

/// COM1 serial port base address
const COM1: u16 = 0x3F8;

/// Serial port registers (offsets from base)
const DATA: u16 = 0;           // Data register (read/write)
const INT_ENABLE: u16 = 1;     // Interrupt enable
const FIFO_CTRL: u16 = 2;      // FIFO control
const LINE_CTRL: u16 = 3;      // Line control
const MODEM_CTRL: u16 = 4;     // Modem control
const LINE_STATUS: u16 = 5;    // Line status

/// Line status flags
const TX_READY: u8 = 0x20;     // Transmitter ready

/// Global serial port instance
static mut SERIAL_PORT: SerialPort = SerialPort::new(COM1);

/// Serial port controller
pub struct SerialPort {
    base: u16,
}

impl SerialPort {
    /// Create new serial port (not initialized)
    const fn new(base: u16) -> Self {
        Self { base }
    }

    /// Initialize the serial port
    pub fn init(&mut self) {
        unsafe {
            // Disable interrupts
            outb(self.base + INT_ENABLE, 0x00);

            // Set baud rate to 115200 (divisor = 1)
            // Enable DLAB (Divisor Latch Access Bit)
            outb(self.base + LINE_CTRL, 0x80);

            // Set divisor to 1 (lo byte)
            outb(self.base + DATA, 0x01);

            // Set divisor to 1 (hi byte)
            outb(self.base + INT_ENABLE, 0x00);

            // 8 bits, no parity, one stop bit (disable DLAB)
            outb(self.base + LINE_CTRL, 0x03);

            // Enable FIFO, clear TX/RX queues, 14-byte threshold
            outb(self.base + FIFO_CTRL, 0xC7);

            // Enable RTS/DSR (modem control)
            outb(self.base + MODEM_CTRL, 0x0B);

            // Enable interrupts (optional, not used yet)
            outb(self.base + INT_ENABLE, 0x01);
        }
    }

    /// Check if transmitter is ready
    fn is_transmit_ready(&self) -> bool {
        unsafe { inb(self.base + LINE_STATUS) & TX_READY != 0 }
    }

    /// Write a single byte to serial port
    pub fn write_byte(&mut self, byte: u8) {
        // Wait for transmitter to be ready
        while !self.is_transmit_ready() {
            core::hint::spin_loop();
        }

        unsafe {
            outb(self.base + DATA, byte);
        }
    }

    /// Write a string to serial port
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            // Convert \n to \r\n for proper terminal display
            if byte == b'\n' {
                self.write_byte(b'\r');
            }
            self.write_byte(byte);
        }
    }
}

/// Initialize global serial port
pub fn init() {
    unsafe {
        SERIAL_PORT.init();
    }
}

/// Write a byte to the serial port
pub fn write_byte(byte: u8) {
    unsafe {
        SERIAL_PORT.write_byte(byte);
    }
}

/// Write a string to the serial port
pub fn write_string(s: &str) {
    unsafe {
        SERIAL_PORT.write_string(s);
    }
}

/// Write formatted arguments to serial port
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        SERIAL_PORT.write_fmt(args).unwrap();
    }
}

/// Implement fmt::Write for SerialPort
impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

/// Serial print macro (like println! but for serial)
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => ($crate::serial::_print(format_args!($($arg)*)));
}

/// Serial println macro
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}

/// x86 port I/O functions

/// Read byte from I/O port
#[inline]
unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    core::arch::asm!(
        "in al, dx",
        out("al") value,
        in("dx") port,
        options(nomem, nostack, preserves_flags)
    );
    value
}

/// Write byte to I/O port
#[inline]
unsafe fn outb(port: u16, value: u8) {
    core::arch::asm!(
        "out dx, al",
        in("dx") port,
        in("al") value,
        options(nomem, nostack, preserves_flags)
    );
}
