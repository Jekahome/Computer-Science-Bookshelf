import math

def convert_video_raw_to_rgb222_bin(raw_filename, bin_filename):
    # Кадр raw24: 320 x 200 пикселей * 3 байта (R, G, B) = 192 000 байт
    RAW_FRAME_SIZE = 320 * 200 * 3 

    with open(raw_filename, "rb") as f_in, open(bin_filename, "wb") as f_out:
        total_pixels = 0
        
        while frame_bytes := f_in.read(RAW_FRAME_SIZE):
            # Пропускаем неполный кадр, если файл закончился раньше времени
            if len(frame_bytes) < RAW_FRAME_SIZE:
                break
            # Создаем буфер под сжатый кадр (64 000 байт)
            out_frame = bytearray(len(frame_bytes) // 3)
            
            # Быстрый шаг по 3 байта (R, G, B)
            for i in range(0, len(frame_bytes), 3):
                r = frame_bytes[i]
                g = frame_bytes[i + 1]
                b = frame_bytes[i + 2]
                
                # RGB222 с офсетом +64
                color_index = 64 + ((r >> 6) << 4) + ((g >> 6) << 2) + (b >> 6)
                out_frame[i // 3] = color_index
            
            f_out.write(out_frame)
            total_pixels += len(out_frame)

    frames_count = total_pixels // (320 * 200)
    print(f"Готово! Обработано пикселей: {total_pixels} (Кадров: {frames_count}).")



convert_video_raw_to_rgb222_bin("video_320x200_rgb24.raw", "video_320x200_rgb222.bin")


# ---------------------------------------------------------------------------------------
# prepare video
# ffmpeg -ss 00:00:00 -i TomAndJerry.mp4 -t 15 -vf "scale=320:200,fps=15" -pix_fmt rgb24 -f rawvideo video_320x200_rgb24.raw
 
# Если адрес 24 бита то максимальный размер видео файла формата bin 16 Мб это 262 кадра	~17.4 секунд (при 15 fps)
