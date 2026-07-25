.section .rodata
value:
    .word 0x12345678

.section .text
.global _start

_start:
    la t0, value      # t0 = адрес value
    lw t1, 0(t0)      # читаем из ROM

loop:
    j loop
    