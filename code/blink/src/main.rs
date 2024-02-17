#![no_std]
#![no_main]

use core::ffi::c_int;
use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(_panic_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main() -> c_int {
    circle_rs::startup::EXIT_REBOOT
}
