//===================================================================================================================================================================================//
//
//   /$$$$$$  /$$$$$$$  /$$   /$$       /$$$$$$             /$$                                                         /$$             
//  /$$__  $$| $$__  $$| $$  | $$      |_  $$_/            | $$                                                        | $$             
// | $$  \__/| $$  \ $$| $$  | $$        | $$   /$$$$$$$  /$$$$$$    /$$$$$$   /$$$$$$   /$$$$$$  /$$   /$$  /$$$$$$  /$$$$$$   /$$$$$$$
// | $$      | $$$$$$$/| $$  | $$        | $$  | $$__  $$|_  $$_/   /$$__  $$ /$$__  $$ /$$__  $$| $$  | $$ /$$__  $$|_  $$_/  /$$_____/
// | $$      | $$____/ | $$  | $$        | $$  | $$  \ $$  | $$    | $$$$$$$$| $$  \__/| $$  \__/| $$  | $$| $$  \ $$  | $$   |  $$$$$$ 
// | $$    $$| $$      | $$  | $$        | $$  | $$  | $$  | $$ /$$| $$_____/| $$      | $$      | $$  | $$| $$  | $$  | $$ /$$\____  $$
// |  $$$$$$/| $$      |  $$$$$$/       /$$$$$$| $$  | $$  |  $$$$/|  $$$$$$$| $$      | $$      |  $$$$$$/| $$$$$$$/  |  $$$$//$$$$$$$/
//  \______/ |__/       \______/       |______/|__/  |__/   \___/   \_______/|__/      |__/       \______/ | $$____/    \___/ |_______/ 
//                                                                                                         | $$                         
//                                                                                                         | $$                         
//                                                                                                         |__/                         
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467
//?

//!
//! Handles CPU exceptions (interrupts).
//!

use x86_64::structures::idt::{ InterruptDescriptorTable as InterruptDescTable, InterruptStackFrame };
use lazy_static::lazy_static;

use crate::println;
use super::gdt;


/*
 * Interrupt Descriptor Table
 *      Declaration
 */


lazy_static! {

    /// A static InterruptDescriptorTable that has the same lifetime as the kernel.
    static ref IDT: InterruptDescTable = {
        let mut idt: InterruptDescTable = InterruptDescTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

/// Initializes a new Interrupt Descriptor Table.
pub fn init_idt() {
    IDT.load();
}


/*
 * Exception
 *      Handlers
 */


/// Handles breakpoints in CPU execution.
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{stack_frame:#?}");
}

/// Handles double faults.
extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{stack_frame:#?}");
}


/*
 * Unit
 *      Tests
 */


#[test_case]
fn test_breakpoint_exception() -> () {
    x86_64::instructions::interrupts::int3();
}
