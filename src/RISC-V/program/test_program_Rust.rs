#![no_std]
#![no_main]

use core::panic::PanicInfo;

static MESSAGE: &[u8] = b"Hello Rust!\n";

fn putchar(c: u8) {
    unsafe {
        *(0x10000000 as *mut u8) = c;
    }
}

fn print_array(data: &[u8]) {
    for &b in data {
        putchar(b);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    print_array(MESSAGE);

    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}