//===================================================================================================================================================================================//
//
//  /$$$$$$             /$$                                              /$$     /$$                           /$$$$$$$$                    /$$             
// |_  $$_/            | $$                                             | $$    |__/                          |__  $$__/                   | $$             
//   | $$   /$$$$$$$  /$$$$$$    /$$$$$$   /$$$$$$   /$$$$$$  /$$$$$$  /$$$$$$   /$$  /$$$$$$  /$$$$$$$          | $$  /$$$$$$   /$$$$$$$ /$$$$$$   /$$$$$$$
//   | $$  | $$__  $$|_  $$_/   /$$__  $$ /$$__  $$ /$$__  $$|____  $$|_  $$_/  | $$ /$$__  $$| $$__  $$         | $$ /$$__  $$ /$$_____/|_  $$_/  /$$_____/
//   | $$  | $$  \ $$  | $$    | $$$$$$$$| $$  \ $$| $$  \__/ /$$$$$$$  | $$    | $$| $$  \ $$| $$  \ $$         | $$| $$$$$$$$|  $$$$$$   | $$   |  $$$$$$ 
//   | $$  | $$  | $$  | $$ /$$| $$_____/| $$  | $$| $$      /$$__  $$  | $$ /$$| $$| $$  | $$| $$  | $$         | $$| $$_____/ \____  $$  | $$ /$$\____  $$
//  /$$$$$$| $$  | $$  |  $$$$/|  $$$$$$$|  $$$$$$$| $$     |  $$$$$$$  |  $$$$/| $$|  $$$$$$/| $$  | $$         | $$|  $$$$$$$ /$$$$$$$/  |  $$$$//$$$$$$$/
// |______/|__/  |__/   \___/   \_______/ \____  $$|__/      \_______/   \___/  |__/ \______/ |__/  |__/         |__/ \_______/|_______/    \___/ |_______/ 
//                                        /$$  \ $$                                                                                                         
//                                       |  $$$$$$/                                                                                                         
//                                        \______/                                                                                                          
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467
//?

//!
//! Integration Tests Root & Entry Point
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
 * Unit Tests
 *      Entry Point
 */


#[test_case]
fn test_println() {
    println!("test_println output");
}
