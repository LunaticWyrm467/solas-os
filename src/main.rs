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
#![test_runner(solas_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use solas_os::println;


/*
 * Entry
 *      Point
 */


/// Entry Point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    // Initialize the kernel.
    solas_os::init();

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

/// Panic Handler -- Unit Test Execution
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    solas_os::test_panic_handler(info)
}
