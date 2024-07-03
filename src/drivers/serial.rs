//===================================================================================================================================================================================//
//
//   /$$$$$$                      /$$           /$$       /$$$$$$             /$$                          /$$$$$$                             
//  /$$__  $$                    |__/          | $$      |_  $$_/            | $$                         /$$__  $$                            
// | $$  \__/  /$$$$$$   /$$$$$$  /$$  /$$$$$$ | $$        | $$   /$$$$$$$  /$$$$$$    /$$$$$$   /$$$$$$ | $$  \__//$$$$$$   /$$$$$$$  /$$$$$$ 
// |  $$$$$$  /$$__  $$ /$$__  $$| $$ |____  $$| $$        | $$  | $$__  $$|_  $$_/   /$$__  $$ /$$__  $$| $$$$   |____  $$ /$$_____/ /$$__  $$
//  \____  $$| $$$$$$$$| $$  \__/| $$  /$$$$$$$| $$        | $$  | $$  \ $$  | $$    | $$$$$$$$| $$  \__/| $$_/    /$$$$$$$| $$      | $$$$$$$$
//  /$$  \ $$| $$_____/| $$      | $$ /$$__  $$| $$        | $$  | $$  | $$  | $$ /$$| $$_____/| $$      | $$     /$$__  $$| $$      | $$_____/
// |  $$$$$$/|  $$$$$$$| $$      | $$|  $$$$$$$| $$       /$$$$$$| $$  | $$  |  $$$$/|  $$$$$$$| $$      | $$    |  $$$$$$$|  $$$$$$$|  $$$$$$$
//  \______/  \_______/|__/      |__/ \_______/|__/      |______/|__/  |__/   \___/   \_______/|__/      |__/     \_______/ \_______/ \_______/
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467
//?

//!
//! A driver used to send information through a Serial port, such as through Qemu to write to an
//! external terminal.
//!

use core::fmt::{ self, Write };

use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;


/*
 * Constant & Static
 *      Declarations
 */


/// The standard port number for the first serial interface.
const SERIAL_1_PORT: u16 = 0x3F8;

lazy_static! {

    /// A publicly accessible serial port for the first serial interface.
    pub static ref SERIAL_1: Mutex<SerialPort> = {
        let mut serial_port: SerialPort = unsafe { SerialPort::new(SERIAL_1_PORT) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}


/*
 * Print Macro
 *      Support
 */


/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::drivers::serial::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    ()                       => ($crate::serial_print!("\n"));
    ($fmt:expr)              => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}

/// A global print function to stream information through the serial port.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    SERIAL_1.lock().write_fmt(args).expect("Printing to serial failed");
}
