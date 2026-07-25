.section .text
.global main

main:
 
    # START устанавливаем время 30 до следующего прерывания Timer Interrupt
    lui  x5, 0x10000      # x5 = 0x10000000
    addi x5, x5, 4        # x5 = 0x10000004
    addi x6, x0, 30
    sw x6, 0(x5)

sub_loop:
    # Печатаем 'A'
    li t0, 'A'
    li t1, 0x10000000
    sb t0, 0(t1)
    j sub_loop

loop:
    j main
 