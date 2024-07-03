//===================================================================================================================================================================================//
//
//  /$$$$$$$$             /$$                               /$$$$$$$           /$$             /$$    
// | $$_____/            | $$                              | $$__  $$         |__/            | $$    
// | $$       /$$$$$$$  /$$$$$$    /$$$$$$  /$$   /$$      | $$  \ $$ /$$$$$$  /$$ /$$$$$$$  /$$$$$$  
// | $$$$$   | $$__  $$|_  $$_/   /$$__  $$| $$  | $$      | $$$$$$$//$$__  $$| $$| $$__  $$|_  $$_/  
// | $$__/   | $$  \ $$  | $$    | $$  \__/| $$  | $$      | $$____/| $$  \ $$| $$| $$  \ $$  | $$    
// | $$      | $$  | $$  | $$ /$$| $$      | $$  | $$      | $$     | $$  | $$| $$| $$  | $$  | $$ /$$
// | $$$$$$$$| $$  | $$  |  $$$$/| $$      |  $$$$$$$      | $$     |  $$$$$$/| $$| $$  | $$  |  $$$$/
// |________/|__/  |__/   \___/  |__/       \____  $$      |__/      \______/ |__/|__/  |__/   \___/  
//                                          /$$  | $$                                                 
//                                         |  $$$$$$/                                                 
//                                          \______/
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467
//?

//!
//! Hello World!
//!

#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod drivers;

use core::{ panic::PanicInfo, any::type_name };

use x86_64::instructions::port::Port;


/*
 * Entry
 *      Point
 */


/// Entry Point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    // Handle unit tests if we have any.
    #[cfg(test)]
    test_main();

    // Do other stuff...
    loop {}
}


/*
 * Panic
 *      Handler
 */


/// Panic Handler -- On Normal Run
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    loop {}
}

/// Panic Handler -- During Unit Test Execution
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]");
    serial_println!("Error: {}", info);
    test_terminate(QemuExitCode::Failed);
    loop {}
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

/// Test Runner
#[cfg(test)]
pub fn test_runner(tests: &[&dyn UnitTest]) -> () {
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
