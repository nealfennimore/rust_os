// Remove standard library linking
#![no_std]
 // Remove having to have the main function defined, as we will not have
 // access to the Rust runtime or C runtime library (crt0)
#![no_main]

//
// ─── IMPORTS ────────────────────────────────────────────────────────────────────
//

// Add in core panic
use core::panic::PanicInfo;

//
// ────────────────────────────────────────────────── I ──────────
//   :::::: E N T R Y : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────────
//

#[no_mangle] // Make sure we output the function name as "_start"
// This is our system entry function. "_start" is the default entry point name for most systems
// extern "C" specifics to use C calling convention for this function (https://en.wikipedia.org/wiki/Calling_convention)
pub extern "C" fn _start() -> ! {
    // The function should never return, so it is marked as a diverging function by returning the “never” type !.
    loop {}
}

//
// ────────────────────────────────────────────────── II ──────────
//   :::::: P A N I C : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────────
//

// This function is called on panic
// If the main thread panics it will terminate all your threads and end your program with code 101.
// Otherwise panic is handled by a single thread if the panic! method is used
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { // The PanicInfo parameter contains the file and line where the panic happened and the optional panic message.
    // The function should never return, so it is marked as a diverging function by returning the “never” type !.

    // We can't do anything with it yet, so just loop
    loop {}
}