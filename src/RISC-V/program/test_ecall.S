.section .text
.global _start

_start:

    # Адрес обработчика исключений
    la t0, trap_handler
    csrw mtvec, t0

    # Печатаем 'A'
    li t0, 'A'
    li t1, 0x10000000
    sb t0, 0(t1)

    # Вызываем исключение
    ecall

    # После возврата из обработчика
    li t0, 'B'
    sb t0, 0(t1)

loop:
    j loop


##################################################
# Обработчик исключений
##################################################

trap_handler:

    # Печатаем 'E'
    li t0, 'E'
    li t1, 0x10000000
    sb t0, 0(t1)

    # mepc = mepc + 4
    csrr t2, mepc
    addi t2, t2, 4
    csrw mepc, t2

    # Вернуться
    mret
    