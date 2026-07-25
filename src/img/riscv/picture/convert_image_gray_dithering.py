 
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

def convert_image_with_dithering(
    image_path, hex_filename, target_width=320, target_height=200
):
    # 1. Загружаем картинку и переводим в честный Grayscale (0.0 .. 255.0)
    img = Image.open(image_path).convert("L")
    img = img.resize((target_width, target_height), Image.Resampling.LANCZOS)

    # Преобразуем в массив float для расчета ошибок
    pixels = np.array(img, dtype=float)

    HEX_HEADER = "v2.0 raw\n"

    # 2. Алгоритм Флойда — Стейнберга
    for y in range(target_height):
        for x in range(target_width):
            old_val = pixels[y, x]

            # Ограничиваем значение яркости (0..255)
            old_val_clamped = max(0.0, min(255.0, old_val))

            # Квантуем до 32 уровней (0..31)
            level = int(round(old_val_clamped / 8.2258))
            level = max(0, min(31, level))

            # Вычисляем точное значение яркости, которое выдаст выбранный уровень
            new_val = level * 8.2258

            # Ошибка квантования
            error = old_val - new_val

            # Сохраняем итоговый уровень в массив
            pixels[y, x] = level

            # Рассеиваем ошибку по соседним пикселям
            if x + 1 < target_width:
                pixels[y, x + 1] += error * 7 / 16
            if y + 1 < target_height:
                if x > 0:
                    pixels[y + 1, x - 1] += error * 3 / 16
                pixels[y + 1, x] += error * 5 / 16
                if x + 1 < target_width:
                    pixels[y + 1, x + 1] += error * 1 / 16

    # 3. Запись в HEX-файл для Digital (смещение +32)
    with open(hex_filename, "w", encoding="utf-8") as f_out:
        f_out.write(HEX_HEADER)

        byte_count = 0
        for y in range(target_height):
            for x in range(target_width):
                level = int(pixels[y, x])
                gray_index = 32 + level

                f_out.write(f"{gray_index:02x} ")
                byte_count += 1

                if byte_count % 16 == 0:
                    f_out.write("\n")

    print(
        f"Готово! Сгенерирован HEX с дизерингом Флойда-Стейнберга ({byte_count} байт)."
    )


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

    convert_image_with_dithering("prepared_rgb222.png", "image_320x200_grayscale_dithered.hex")
