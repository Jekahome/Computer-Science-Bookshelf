.section .rodata
msg:
    .asciz "Hello"

.section .text
.global _start

_start:
    la t0, msg

loop:
    lbu t1, 0(t0)

    beqz t1, done

    li t2, 0x10000000
    sb t1, 0(t2)

    addi t0, t0, 1

    j loop

done:
    j done
    