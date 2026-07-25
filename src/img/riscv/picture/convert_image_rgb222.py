from PIL import Image, ImageEnhance, ImageFilter
import numpy as np

def preprocess_image(image_path, target_width=320, target_height=200, 
                     contrast=1.25, saturation=1.35, brightness=1.05, sharpness=1.5):
    """
    Предварительная подготовка кадра перед квантованием в RGB222.
    """
    # 1. Загрузка и первичный ресайз с качественной фильтрацией
    img = Image.open(image_path).convert("RGB")
    img = img.resize((target_width, target_height), Image.Resampling.LANCZOS)
    
    # 2. Коррекция контрастности
    if contrast != 1.0:
        img = ImageEnhance.Contrast(img).enhance(contrast)
        
    # 3. Коррекция насыщенности
    if saturation != 1.0:
        img = ImageEnhance.Color(img).enhance(saturation)
        
    # 4. Коррекция яркости
    if brightness != 1.0:
        img = ImageEnhance.Brightness(img).enhance(brightness)
        
    # 5. Повышение резкости (чтобы дизеринг лучше хватался за детали)
    if sharpness > 1.0:
        img = ImageEnhance.Sharpness(img).enhance(sharpness)
        
    return img

def rgb888_to_rgb222_index(r, g, b):
    """
    Конвертирует 24-битный цвет (RGB888) в индекс палитры Digital Graphic RAM (64..127).
    """
    # Сжимаем 8-битные каналы (0..255) в 2-битные (0..3)
    r2 = r >> 6  # эквивалентно r // 64
    g2 = g >> 6  # эквивалентно g // 64
    b2 = b >> 6  # эквивалентно b // 64
    
    # Формируем индекс в диапазоне 64..127
    return 64 + (r2 << 4) + (g2 << 2) + b2

def convert_image_to_rgb222_hex(image_path, hex_filename, target_width=320, target_height=200):
    HEX_HEADER = "v2.0 raw\n"
    
    img = Image.open(image_path).convert("RGB")
    img = img.resize((target_width, target_height))
    pixels = img.load()
    
    with open(hex_filename, "w", encoding="utf-8") as f_out:
        f_out.write(HEX_HEADER)
        byte_count = 0
        
        for y in range(target_height):
            for x in range(target_width):
                r, g, b = pixels[x, y]
                color_index = rgb888_to_rgb222_index(r, g, b)
                
                f_out.write(f"{color_index:02x} ")
                byte_count += 1
                
                if byte_count % 16 == 0:
                    f_out.write("\n")
                    
    print(f"Готово! Записано {byte_count} пикселей в режиме RGB222.")

if __name__ == "__main__":
    # Готовим картинку
    prepared_img = preprocess_image(
        "image_320x200.png", 
        target_width=320, 
        target_height=200,
        contrast=1.25,     # +25% контраста
        saturation=1.30,   # +30% насыщенности
        brightness=1.03,   # +3% яркости
        sharpness=1.40     # +40% резкости
    )
    prepared_img.save("prepared_rgb222.png")

    convert_image_to_rgb222_hex("prepared_rgb222.png", "image_320x200_rgb222_new.hex")

 
