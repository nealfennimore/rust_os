#![no_std]

// Add in core panic
use core::panic::PanicInfo;

// This function is called on panic
// If the main thread panics it will terminate all your threads and end your program with code 101.
// Otherwise panic is handled by a single thread if the panic! method is used
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { // The PanicInfo parameter contains the file and line where the panic happened and the optional panic message.
    // The function should never return, so it is marked as a diverging function by returning the “never” type !.

    // We can't do anything with it yet, so just loop
    loop {}
}

fn main() {
}
