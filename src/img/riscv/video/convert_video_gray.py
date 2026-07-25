import math

def convert_video_gray_raw_to_bin(raw_filename, bin_filename):
    # Кадр raw gray: 320 x 200 пикселей * 1 байт = 64 000 байт
    FRAME_SIZE = 320 * 200

    with open(raw_filename, "rb") as f_in, open(bin_filename, "wb") as f_out:
        total_pixels = 0
        
        while chunk := f_in.read(FRAME_SIZE):
            # Пропускаем неполный кадр, если файл закончился раньше времени
            if len(chunk) < FRAME_SIZE:
                break
                
            # Быстрое квантование байтов (0..255 -> 32..63)
            processed_frame = bytearray(
                32 + max(0, min(31, int(round(b / 8.2258)))) 
                for b in chunk
            )
            
            f_out.write(processed_frame)
            total_pixels += len(processed_frame)

    frames_count = total_pixels // FRAME_SIZE
    print(f"Готово! Обработано пикселей: {total_pixels} (Кадров: {frames_count}).")

convert_video_gray_raw_to_bin("video_320x200_gray.raw", "video_320x200_gray.bin")


# prepare video
# ffmpeg -ss 00:00:00 -i TomAndJerry.mp4 -t 15 -vf "scale=320:200,fps=15" -pix_fmt gray -f rawvideo video_320x200_gray.raw
 
