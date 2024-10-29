//! The panic handler

use crate::sbi::shutdown;
use core::panic::PanicInfo;
use log::*;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        error!(
            "[kernel] Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().as_str().unwrap()
        );
    } else {
        error!("[kernel] Panicked: {}", info.message().as_str().unwrap());
    }
    shutdown(true)
}
