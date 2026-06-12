#![no_std]
#![no_main]

use panic_halt as _;
use rmk::macros::rmk_keyboard;

#[defmt::global_logger]
struct Logger;

unsafe impl defmt::Logger for Logger {
    fn acquire() {}
    unsafe fn flush() {}
    unsafe fn release() {}
    unsafe fn write(_bytes: &[u8]) {}
}

#[rmk_keyboard]
mod keyboard {}
