use core::arch::global_asm;
use core::fmt::Write;

use ignore_result::Ignore;

use crate::arch::x86_64::idt::*;
use super::serial::{SerialPort, ComPort::COM1};

pub fn load_exceptions(idt: &mut Idt) {
        idt.set_gate(0, isr_divide_by_zero, 1 << 3, true, true);
        idt.set_gate(8, isr_double_fault, 1 << 3, true, true);
        idt.set_gate(13, isr_general_protection_fault, 1 << 3, true, true);
        idt.set_gate(14, isr_page_fault, 1 << 3, true, true);
}

global_asm!(include_str!("exceptions.asm"));

extern "C" {
        fn isr_divide_by_zero();
        fn isr_double_fault();
        fn isr_general_protection_fault();
        fn isr_page_fault();
}

#[no_mangle]
extern "C" fn ih_double_fault(error_code: u64) {
        let mut logger = SerialPort::try_new(COM1).unwrap();

        writeln!(&mut logger, "A double fault has occurred! Panicking!").ignore();
}

#[no_mangle]
extern "C" fn ih_divide_by_zero() {
        let mut logger = SerialPort::try_new(COM1).unwrap();

        writeln!(&mut logger, "A divide by zero has occurred in kernelspace! Panicking!").ignore();
}

#[no_mangle]
extern "C" fn ih_general_protection_fault(error_code: u64) {
        let mut logger = SerialPort::try_new(COM1).unwrap();

        writeln!(&mut logger, "A general protection fault has occurred in kernelspace! Panicking!").ignore();
}

#[no_mangle]
extern "C" fn ih_page_fault(error_code: u64) {
        let mut logger = SerialPort::try_new(COM1).unwrap();

        writeln!(&mut logger, "A page fault has occurred with error code {:x}", error_code).ignore();
}