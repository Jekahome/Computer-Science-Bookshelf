#![no_std]
#![no_main]

use core::panic::PanicInfo;

const WIDTH: usize = 320;
const HEIGHT: usize = 200;

const FRAMEBUFFER: *mut u8 = 0x9000_0000 as *mut u8;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[inline(always)]
fn put_pixel(x: i32, y: i32, color: u8) {
    if x < 0 || y < 0 {
        return;
    }
    if x >= WIDTH as i32 || y >= HEIGHT as i32 {
        return;
    }
    unsafe {
        FRAMEBUFFER
            .add(y as usize * WIDTH + x as usize)
            .write_volatile(color);
    }
}

// Рисуем 8 симметричных точек окружности
#[inline(always)]
fn circle_points(
    xc: i32,
    yc: i32,
    x: i32,
    y: i32,
    color: u8
) {
    put_pixel(xc + x, yc + y, color);
    put_pixel(xc - x, yc + y, color);

    put_pixel(xc + x, yc - y, color);
    put_pixel(xc - x, yc - y, color);

    put_pixel(xc + y, yc + x, color);
    put_pixel(xc - y, yc + x, color);

    put_pixel(xc + y, yc - x, color);
    put_pixel(xc - y, yc - x, color);
}

fn draw_circle(
    xc: i32,
    yc: i32,
    radius: i32,
    color: u8
) {

    let mut x = 0;
    let mut y = radius;

    // Начальная ошибка
    let mut d = 3 - 2 * radius;
    while x <= y {
        circle_points(
            xc,
            yc,
            x,
            y,
            color
        );

        if d < 0 {
            d += 4 * x + 6;
        } else {
            d += 4 * (x - y) + 10;
            y -= 1;
        }
        x += 1;
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    // Большая окружность
    draw_circle(
        160,
        100,
        70,
        10
    );
    // маленькие окружности для теста
    draw_circle(
        80,
        50,
        30,
        20
    );
    draw_circle(
        240,
        150,
        20,
        30
    );
    loop {}
}
