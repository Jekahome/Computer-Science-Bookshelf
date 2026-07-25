from PIL import Image

def convert_didder_rgb222_to_hex(png_path, hex_filename):
    img = Image.open(png_path).convert("RGB")
    width, height = img.size
    pixels = img.load()
    
    HEX_HEADER = "v2.0 raw\n"
    
    with open(hex_filename, "w", encoding="utf-8") as f_out:
        f_out.write(HEX_HEADER)
        byte_count = 0
        
        for y in range(height):
            for x in range(width):
                r, g, b = pixels[x, y]
                
                # Переводим RGB-цвет из PNG в 2-битные каналы (0..3)
                r2 = max(0, min(3, int(round(r / 85.0))))
                g2 = max(0, min(3, int(round(g / 85.0))))
                b2 = max(0, min(3, int(round(b / 85.0))))
                
                # Формируем индекс RGB222 (64..127)
                color_index = 64 + (r2 << 4) + (g2 << 2) + b2
                
                f_out.write(f"{color_index:02x} ")
                byte_count += 1
                
                if byte_count % 16 == 0:
                    f_out.write("\n")
                    
    print(f"Готово! Сохранено {byte_count} байт в '{hex_filename}'.")

if __name__ == "__main__":
    convert_didder_rgb222_to_hex("result_didder_edm_recolor_FloydSteinberg.png", "didder.hex")

