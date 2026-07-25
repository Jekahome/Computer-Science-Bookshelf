.section .text
.global _start

.extern main

.extern __stack_top
.extern __data_load
.extern __data_start
.extern __data_end

.extern __bss_start
.extern __bss_end

_start:

    # 1. Инициализация стека
    la sp, __stack_top

    # 2. Установить адрес общего обработчика trap
    la t0, trap_handler
    csrw mtvec, t0

    # 3. Копирование .data из ROM в RAM
    # цикл: ROM to RAM, копирует все слова секции .data
    la t0, __data_load
    la t1, __data_start
    la t2, __data_end

copy_data:
    beq t1, t2, copy_done

    lw t3, 0(t0)
    sw t3, 0(t1)

    addi t0, t0, 4
    addi t1, t1, 4

    j copy_data

copy_done:

    # 3. Обнулить .bss
    la t1, __bss_start
    la t2, __bss_end

clear_bss:
    beq t1, t2, bss_done
    sw zero, 0(t1)
    addi t1, t1, 4
    j clear_bss

bss_done:
    # Разрешаем внешние прерывания в регистре mie (11-й бит)
    li t0, 0x800
    csrs mie, t0

    # 4. Разрешаем глобальные прерывания в mstatus (MIE=1)
    li t0, 0x8
    csrs mstatus, t0

    # 5. Вызвать main
    call main
    
    # Если main вернулся — зависнуть

hang:
    j hang
