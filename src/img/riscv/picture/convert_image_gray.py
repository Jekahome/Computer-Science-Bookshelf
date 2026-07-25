from PIL import Image

def rgb_to_digital_grayscale_index(r, g, b):
    """
    Преобразует RGB в яркость (Luma), а затем маппит на диапазон 32..63 Digital Graphic RAM.
    """
    # Вычисляем взвешенную яркость (0..255)
    gray = int(0.299 * r + 0.587 * g + 0.114 * b)
    
    # Приводим диапазон 0..255 к 32 уровням (0..31)
    level = int(round(gray / 8.2258))
    level = max(0, min(31, level))
    
    # Смещаем в диапазон Graphic RAM (32..63)
    return 32 + level

def convert_image_to_grayscale_hex(image_path, hex_filename, target_width=320, target_height=200):
    HEX_HEADER = "v2.0 raw\n"
    
    # 1. Загружаем и масштабируем изображение под 320x200
    img = Image.open(image_path).convert("RGB")
    img = img.resize((target_width, target_height))
    pixels = img.load()
    
    # 2. Формируем HEX-файл
    with open(hex_filename, "w", encoding="utf-8") as f_out:
        f_out.write(HEX_HEADER)
        
        byte_count = 0
        for y in range(target_height):
            for x in range(target_width):
                r, g, b = pixels[x, y]
                gray_index = rgb_to_digital_grayscale_index(r, g, b)
                
                # Записываем байт индекса в HEX
                f_out.write(f"{gray_index:02x} ")
                byte_count += 1
                
                if byte_count % 16 == 0:
                    f_out.write("\n")
                    
    print(f"Готово! Записано {byte_count} пикселей в режиме градаций серого (32..63).")

if __name__ == "__main__":
    # Укажите путь к изображению
    convert_image_to_grayscale_hex("image_320x200.png", "image_320x200_grayscale.hex")
