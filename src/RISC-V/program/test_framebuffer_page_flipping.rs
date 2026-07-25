#![no_std]
#![no_main]
#![allow(dead_code)]

use core::panic::PanicInfo;

const WIDTH: usize = 320;
const HEIGHT: usize = 200;
const FRAME_SIZE: usize = WIDTH * HEIGHT; // 64 000 байт (0xFA00)

// Базовый адрес видеопамяти
const BASE_FRAMEBUFFER: *mut u8 = 0x9000_0000 as *mut u8;

// Регистр управления входом B (Display Control)
const DISP_CTRL: *mut u32 = 0x1000_000C as *mut u32;

// Глобальная переменная для отслеживания текущего ПОКАЗЫВАЕМОГО буфера (0 или 1)
static mut ACTIVE_BUFFER: u8 = 0;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Возвращает указатель на СКУКРЫТЫЙ (фоновый) буфер, в который сейчас можно рисовать
fn get_back_buffer() -> *mut u8 {
    unsafe {
        if ACTIVE_BUFFER == 0 {
            // Если экран показывает Буфер 0, рисуем в Буфер 1
            BASE_FRAMEBUFFER.add(FRAME_SIZE)
        } else {
            // Если экран показывает Буфер 1, рисуем в Буфер 0
            BASE_FRAMEBUFFER
        }
    }
}

/// Переключает кадры: обновляет регистр MMIO и меняет активный буфер
fn swap_buffers() {
    unsafe {
        // Инвертируем активный буфер (0 -> 1 или 1 -> 0)
        ACTIVE_BUFFER ^= 1;

        // Записываем новое значение в регистр 0x1000_000C, переключая вход B
        DISP_CTRL.write_volatile(ACTIVE_BUFFER as u32);
    }
}

fn put_pixel(x: usize, y: usize, color: u8) {
    if x >= WIDTH || y >= HEIGHT {
        return;
    }

    unsafe {
        get_back_buffer()
            .add(y * WIDTH + x)
            .write_volatile(color);
    }
}

fn fill(color: u8) {
    let back_buffer = get_back_buffer();
    unsafe {
        for i in 0..FRAME_SIZE {
            back_buffer.add(i).write_volatile(color);
        }
    }
    // Переключаем экран ТОЛЬКО ПОСЛЕ того, как кадр полностью закрашен
    swap_buffers();
}

fn fill_fast(color: u8) {
   let mut ptr = get_back_buffer(); // *mut u8
    
    // 64 000 / 16 = 4 000 итераций вместо 64 000
    let total_chunks = (WIDTH * HEIGHT) / 16; 

    unsafe {
        for _ in 0..total_chunks {
            // Пишем 16 байт подряд за одну итерацию
            ptr.offset(0).write_volatile(color);
            ptr.offset(1).write_volatile(color);
            ptr.offset(2).write_volatile(color);
            ptr.offset(3).write_volatile(color);
            ptr.offset(4).write_volatile(color);
            ptr.offset(5).write_volatile(color);
            ptr.offset(6).write_volatile(color);
            ptr.offset(7).write_volatile(color);
            ptr.offset(8).write_volatile(color);
            ptr.offset(9).write_volatile(color);
            ptr.offset(10).write_volatile(color);
            ptr.offset(11).write_volatile(color);
            ptr.offset(12).write_volatile(color);
            ptr.offset(13).write_volatile(color);
            ptr.offset(14).write_volatile(color);
            ptr.offset(15).write_volatile(color);

            ptr = ptr.add(16);
        }
    }

    swap_buffers();
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    // Инициализация: сбрасываем регистр управления в 0
    unsafe { DISP_CTRL.write_volatile(0);}

    // Теперь каждый вызов fill() незаметно заполняет скрытый буфер 
    // и мгновенно выводит его на экран 
    fill_fast(1);
    fill_fast(2);
    fill_fast(3);
    fill_fast(4);
    fill_fast(5);
    fill_fast(6);
    fill_fast(7);
    fill_fast(8);
    fill_fast(9);
   
    loop {}
}