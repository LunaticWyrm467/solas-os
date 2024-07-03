//===================================================================================================================================================================================//
//
//  /$$       /$$ /$$                                             
// | $$      |__/| $$                                             
// | $$       /$$| $$$$$$$   /$$$$$$  /$$$$$$   /$$$$$$  /$$   /$$
// | $$      | $$| $$__  $$ /$$__  $$|____  $$ /$$__  $$| $$  | $$
// | $$      | $$| $$  \ $$| $$  \__/ /$$$$$$$| $$  \__/| $$  | $$
// | $$      | $$| $$  | $$| $$      /$$__  $$| $$      | $$  | $$
// | $$$$$$$$| $$| $$$$$$$/| $$     |  $$$$$$$| $$      |  $$$$$$$
// |________/|__/|_______/ |__/      \_______/|__/       \____  $$
//                                                       /$$  | $$
//                                                      |  $$$$$$/
//                                                       \______/ 
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467
//?

//!
//! Allows core features of the OS to be referenced and used in external libraries or dependencies.
//!

#![no_main]
#![no_std]

#![feature(custom_test_frameworks, abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod instructions;
pub mod drivers;

use core::{ panic::PanicInfo, any::type_name };

use x86_64::instructions::port::Port;

use instructions::{ interrupts, gdt };


/*
 * Initialization
 *      Routines
 */


/// A global kernel initialization function.
pub fn init() -> () {
    interrupts::init_idt();
    gdt::init();
}


/*
 * Test
 *      Runner
 */


/// The exit code that is written to Qemu's exit port.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed  = 0x11,
}

/// A trait that is implemented for every function (with no args or return type) that allows it to
/// be used as a unit test.
pub trait UnitTest {
    fn run_as_test(&self) -> ();
}

impl <T: Fn()> UnitTest for T {
    fn run_as_test(&self) -> () {
        serial_print!("{}...\t", type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

/// Entry Point for Unit Tests
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {}
}

/// Panic Handler for Unit Tests
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

/// Panic Handler for Unit Test Execution
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]");
    serial_println!("Error: {}", info);
    test_terminate(QemuExitCode::Failed);
    loop {}
}

/// Test Runner
pub fn test_runner(tests: &[&dyn UnitTest]) -> () {
    if tests.len() == 0 {
        test_terminate(QemuExitCode::Success);
    }
    serial_println!("Running {} test{}", tests.len(), if tests.len() != 1 { "s" } else { "" });

    for test in tests {
        test.run_as_test();
    }
    test_terminate(QemuExitCode::Success);
}

/// Runs on test completion or failure. Handles the communication between the OS and Qemu so that
/// the OS may exit accordingly.
pub fn test_terminate(exit_code: QemuExitCode) -> () {
    const IO_BASE_ISA_DEBUG_EXIT: u16 = 0xf4;

    let mut port: Port<u32> = Port::new(IO_BASE_ISA_DEBUG_EXIT);
    unsafe {
        port.write(exit_code as u32);
    }
}

