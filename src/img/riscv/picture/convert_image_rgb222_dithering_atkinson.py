from PIL import Image
import numpy as np

def convert_color_atkinson_dithering(image_path, hex_filename, target_width=320, target_height=200):
    img = Image.open(image_path).convert("RGB")
    img = img.resize((target_width, target_height), Image.Resampling.LANCZOS)
    
    pixels = np.array(img, dtype=float)
    HEX_HEADER = "v2.0 raw\n"
    
    for y in range(target_height):
        for x in range(target_width):
            old_rgb = pixels[y, x].copy()
            clamped_rgb = np.clip(old_rgb, 0.0, 255.0)
            
            r_level = max(0, min(3, int(round(clamped_rgb[0] / 85.0))))
            g_level = max(0, min(3, int(round(clamped_rgb[1] / 85.0))))
            b_level = max(0, min(3, int(round(clamped_rgb[2] / 85.0))))
            
            new_rgb = np.array([r_level * 85.0, g_level * 85.0, b_level * 85.0])
            
            # В Аткинсоне от ошибки берется ровно 1/8 (6 из 8 частей распределяются, 2/8 теряются)
            error = (old_rgb - new_rgb) / 8.0
            pixels[y, x] = [r_level, g_level, b_level]
            
            # Соседи по схеме Аткинсона: (x+1, y), (x+2, y), (x-1, y+1), (x, y+1), (x+1, y+1), (x, y+2)
            if x + 1 < target_width:  pixels[y, x + 1] += error
            if x + 2 < target_width:  pixels[y, x + 2] += error
            if y + 1 < target_height:
                if x > 0:             pixels[y + 1, x - 1] += error
                pixels[y + 1, x]     += error
                if x + 1 < target_width: pixels[y + 1, x + 1] += error
            if y + 2 < target_height: pixels[y + 2, x]     += error

    # Запись в HEX
    with open(hex_filename, "w", encoding="utf-8") as f_out:
        f_out.write(HEX_HEADER)
        byte_count = 0
        for y in range(target_height):
            for x in range(target_width):
                r2, g2, b2 = pixels[y, x].astype(int)
                color_index = 64 + (r2 << 4) + (g2 << 2) + b2
                f_out.write(f"{color_index:02x} ")
                byte_count += 1
                if byte_count % 16 == 0:
                    f_out.write("\n")

if __name__ == "__main__":
    convert_color_atkinson_dithering("image_320x200.png", "image_320x200_rgb222_dithered_atkinson.hex")
