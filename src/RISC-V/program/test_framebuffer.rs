#![no_std]
#![no_main]
#![allow(dead_code)]

use core::panic::PanicInfo;

const WIDTH: usize = 320;
const HEIGHT: usize = 200;

const FRAMEBUFFER: *mut u8 = 0x9000_0000 as *mut u8;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn put_pixel(x: usize, y: usize, color: u8) {
    if x >= WIDTH || y >= HEIGHT {
        return;
    }

    unsafe {
        FRAMEBUFFER
            .add(y * WIDTH + x)
            .write_volatile(color);
    }
}

fn clear(color: u8) {
    unsafe {
        for i in 0..(WIDTH * HEIGHT) {
            FRAMEBUFFER.add(i).write_volatile(color);
        }
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    clear(0);
    clear(1);
    clear(2);
    clear(3);
    clear(4);
    clear(5);
    clear(6);
    clear(7);
    clear(8);
    clear(9);

    loop {}
}