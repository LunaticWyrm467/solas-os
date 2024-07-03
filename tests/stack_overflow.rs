//===================================================================================================================================================================================//
//
//   /$$$$$$   /$$                         /$$              /$$$$$$                                 /$$$$$$  /$$                        
//  /$$__  $$ | $$                        | $$             /$$__  $$                               /$$__  $$| $$                        
// | $$  \__//$$$$$$    /$$$$$$   /$$$$$$$| $$   /$$      | $$  \ $$ /$$    /$$ /$$$$$$   /$$$$$$ | $$  \__/| $$  /$$$$$$  /$$  /$$  /$$
// |  $$$$$$|_  $$_/   |____  $$ /$$_____/| $$  /$$/      | $$  | $$|  $$  /$$//$$__  $$ /$$__  $$| $$$$    | $$ /$$__  $$| $$ | $$ | $$
//  \____  $$ | $$      /$$$$$$$| $$      | $$$$$$/       | $$  | $$ \  $$/$$/| $$$$$$$$| $$  \__/| $$_/    | $$| $$  \ $$| $$ | $$ | $$
//  /$$  \ $$ | $$ /$$ /$$__  $$| $$      | $$_  $$       | $$  | $$  \  $$$/ | $$_____/| $$      | $$      | $$| $$  | $$| $$ | $$ | $$
// |  $$$$$$/ |  $$$$/|  $$$$$$$|  $$$$$$$| $$ \  $$      |  $$$$$$/   \  $/  |  $$$$$$$| $$      | $$      | $$|  $$$$$$/|  $$$$$/$$$$/
//  \______/   \___/   \_______/ \_______/|__/  \__/       \______/     \_/    \_______/|__/      |__/      |__/ \______/  \_____/\___/ 
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467
//?

//!
//! This holds tests that model events which may result in stack overflows and tests againsts the
//! OS's safeguards..
//!

#![no_std]
#![no_main]

#![feature(custom_test_frameworks, abi_x86_interrupt)]
#![test_runner(solas_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use volatile::Volatile;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use solas_os::{ instructions::gdt, serial_print, serial_println, test_terminate, QemuExitCode };


/*
 * Unit Tests
 *      Entry Point
 */


/// The entry point for the unit tests library.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    gdt::init();
    init_test_idt();
    
    serial_println!("Running 1 test");
    serial_print!("stack_overflow::stack_overflow...\t");
    test_stack_overflow();

    loop {}
}

/// The tests panic handler.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    solas_os::test_panic_handler(info)
}



/*
 * Unit Test
 *      Cases
 */


fn test_stack_overflow() {
    
    // trigger a stack overflow
    #[allow(unconditional_recursion)]
    fn stack_overflow() {
        stack_overflow();         // For each recursion, the return address is pushed.
        Volatile::new(0).read();  // Prevent tail recursion optimizations.
    }
    stack_overflow();

    panic!("Execution continued after stack overflow!");
}


/*
 * Test IDT
 *      Initialization
 *
 *  # Note:
 *  We initialize a custom IDT as to allow Qemu to recieve a Success status when it handles a
 *  double fault (e.g. stack overflow).
 */


lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt: InterruptDescriptorTable = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

fn init_test_idt() -> () {
    TEST_IDT.load();
}

extern "x86-interrupt" fn test_double_fault_handler(_: InterruptStackFrame, _: u64) -> ! {
    serial_println!("[ok]");
    test_terminate(QemuExitCode::Success);
    loop {}
}
