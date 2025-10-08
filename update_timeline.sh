#!/bin/bash

# --- КОНСТАНТЫ И НАСТРОЙКИ ---
TOTAL_BOOKS=122
TOTAL_WEEKS=244
README_FILE="src/README.md"
PERC_DIR="img/perc"
PERC_SRC_DIR="src/img/perc"

# Установите дату начала (Год-Месяц-День)
START_DATE="2025-09-30"

# --- ФУНКЦИИ ---

# Функция для безопасного копирования SVG файла
copy_svg() {
    local percentage=$1
    local target_file=$2

    # Убеждаемся, что процент - это целое число.
    if ! [[ "$percentage" =~ ^[0-9]+$ ]]; then
        percentage=0
    fi
    
    # Ограничиваем процент от 0 до 100
    if [ "$percentage" -lt 0 ]; then
        percentage=0
    elif [ "$percentage" -gt 100 ]; then
        percentage=100
    fi

    local source_file="$PERC_DIR/${percentage}.svg"

    if [ -f "$source_file" ]; then
        cp "$source_file" "$target_file"
        echo "✅ Обновлен $target_file: скопирован $source_file"
    else
        echo "❌ Ошибка: Файл $source_file не найден. Прогресс не обновлен."
    fi
}

# 🛑 ИСПРАВЛЕННАЯ ФУНКЦИЯ для безопасного расчета процента с округлением
# Аргументы: $1 - числитель, $2 - знаменатель
calculate_percent() {
    local numerator=$1
    local denominator=$2

    # CRITICAL FIX: Ensure all multiplication/division happens inside bc.
    # We use scale=2 for intermediate precision, then apply the standard
    # rounding method (+ 0.5) and set scale=0 to get the final integer.
    local result=$(echo "scale=2; ( ($numerator * 100) / $denominator ) + 0.5" | bc | awk '{printf "%d", $1}')
    
    # bc может вернуть пустую строку или синтаксическую ошибку, если деление на ноль,
    # поэтому мы делаем финальную проверку.
    if ! [[ "$result" =~ ^[0-9]+$ ]]; then
        echo 0
    else
        echo "$result"
    fi
}

# ----------------------------------------------------
## 1. Обновление запланированного прогресса (schedule.svg)
# ----------------------------------------------------

echo "--- Начинаем обновление запланированного прогресса (schedule.svg) ---"

START_SECONDS=$(date -d "$START_DATE" +%s)
CURRENT_SECONDS=$(date +%s)

# Вычисляем прошедшие недели (округляем вниз)
ELAPSED_SECONDS=$((CURRENT_SECONDS - START_SECONDS))
ELAPSED_WEEKS=$((ELAPSED_SECONDS / 86400 / 7))

# Вычисляем процент
SCHEDULE_PERCENT=$(calculate_percent "$ELAPSED_WEEKS" "$TOTAL_WEEKS")

echo "📅 Прошло недель: $ELAPSED_WEEKS (Начало: $START_DATE)"
echo "🎯 Запланированный прогресс: ${SCHEDULE_PERCENT}%"

copy_svg "$SCHEDULE_PERCENT" "$PERC_DIR/schedule.svg"
copy_svg "$SCHEDULE_PERCENT" "$PERC_SRC_DIR/schedule.svg"

# ----------------------------------------------------
## 2. Обновление фактического прогресса (reality.svg)
# ----------------------------------------------------

echo ""
echo "--- Начинаем обновление фактического прогресса (reality.svg) ---"

# Подсчет книг (с учетом отступов)
READ_BOOKS=$(grep -ic '^\s*-\s*\[[xX]\]' "$README_FILE")

# Вычисляем процент с помощью исправленной функции
REALITY_PERCENT=$(calculate_percent "$READ_BOOKS" "$TOTAL_BOOKS")

echo "📖 Прочитано книг: $READ_BOOKS из $TOTAL_BOOKS"
echo "📈 Фактический прогресс: ${REALITY_PERCENT}%"

copy_svg "$REALITY_PERCENT" "$PERC_DIR/reality.svg"
copy_svg "$REALITY_PERCENT" "$PERC_SRC_DIR/reality.svg"

echo ""
echo "🎉 Обновление завершено!"
