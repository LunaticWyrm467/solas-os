//===================================================================================================================================================================================//
//
//  /$$$$$$$                      /$$                 /$$$$$$$                        /$$    
// | $$__  $$                    |__/                | $$__  $$                      | $$    
// | $$  \ $$  /$$$$$$   /$$$$$$$ /$$  /$$$$$$$      | $$  \ $$  /$$$$$$   /$$$$$$  /$$$$$$  
// | $$$$$$$  |____  $$ /$$_____/| $$ /$$_____/      | $$$$$$$  /$$__  $$ /$$__  $$|_  $$_/  
// | $$__  $$  /$$$$$$$|  $$$$$$ | $$| $$            | $$__  $$| $$  \ $$| $$  \ $$  | $$    
// | $$  \ $$ /$$__  $$ \____  $$| $$| $$            | $$  \ $$| $$  | $$| $$  | $$  | $$ /$$
// | $$$$$$$/|  $$$$$$$ /$$$$$$$/| $$|  $$$$$$$      | $$$$$$$/|  $$$$$$/|  $$$$$$/  |  $$$$/
// |_______/  \_______/|_______/ |__/ \_______/      |_______/  \______/  \______/    \___/  
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467
//?

//!
//! This holds tests that model features which should be able to be run in an environment
//! immediately after booting, but before any initialization routines are called.
//!

#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(solas_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use solas_os::*;


/*
 * Unit Tests
 *      Entry Point
 */


/// The entry point for the unit tests library.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
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


#[test_case]
fn test_println() {
    println!("test_println output");
}
