from PIL import Image
import numpy as np

def convert_color_image_with_dithering(image_path, hex_filename, target_width=320, target_height=200):
    # 1. Загружаем цветное изображение и масштабируем
    img = Image.open(image_path).convert("RGB")
    img = img.resize((target_width, target_height), Image.Resampling.LANCZOS)
    
    # Массив float32 для подсчета ошибок квантования по каналам (R, G, B)
    pixels = np.array(img, dtype=float)
    
    HEX_HEADER = "v2.0 raw\n"
    
    # 2. Алгоритм Флойда — Стейнберга по трем каналам
    for y in range(target_height):
        for x in range(target_width):
            old_rgb = pixels[y, x].copy()
            
            # Ограничиваем яркости (0..255)
            clamped_rgb = np.clip(old_rgb, 0.0, 255.0)
            
            # Квантуем каждый канал к 2 битам (4 уровня: 0, 1, 2, 3)
            # Шаг между уровнями: 255 / 3 = 85.0
            r_level = int(round(clamped_rgb[0] / 85.0))
            g_level = int(round(clamped_rgb[1] / 85.0))
            b_level = int(round(clamped_rgb[2] / 85.0))
            
            r_level = max(0, min(3, r_level))
            g_level = max(0, min(3, g_level))
            b_level = max(0, min(3, b_level))
            
            # Эквивалентный RGB, который отобразит выбранный уровень
            new_rgb = np.array([r_level * 85.0, g_level * 85.0, b_level * 85.0])
            
            # Ошибка квантования
            error = old_rgb - new_rgb
            
            # Сохраняем вычисленные 2-битные уровни
            pixels[y, x] = [r_level, g_level, b_level]
            
            # Рассеиваем ошибку на соседние пиксели
            if x + 1 < target_width:
                pixels[y, x + 1] += error * 7 / 16
            if y + 1 < target_height:
                if x > 0:
                    pixels[y + 1, x - 1] += error * 3 / 16
                pixels[y + 1, x] += error * 5 / 16
                if x + 1 < target_width:
                    pixels[y + 1, x + 1] += error * 1 / 16

    # 3. Упаковка в форматированный HEX (индексы 64..127)
    with open(hex_filename, "w", encoding="utf-8") as f_out:
        f_out.write(HEX_HEADER)
        byte_count = 0
        
        for y in range(target_height):
            for x in range(target_width):
                r2, g2, b2 = pixels[y, x].astype(int)
                
                # Рассчитываем индекс RGB222 в Graphic RAM
                color_index = 64 + (r2 << 4) + (g2 << 2) + b2
                
                f_out.write(f"{color_index:02x} ")
                byte_count += 1
                
                if byte_count % 16 == 0:
                    f_out.write("\n")
                    
    print(f"Готово! Сгенерирован цветной HEX с дизерингом ({byte_count} байт).")

if __name__ == "__main__":
    convert_color_image_with_dithering("image_320x200.png", "image_320x200_rgb222_dithered.hex")
