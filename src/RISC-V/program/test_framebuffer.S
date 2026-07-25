.section .text
.global main

##################################################
# main
# Fill 320x200 framebuffer with green
##################################################

main:

    # t0 = 0x90000000
    lui  t0, 0x90000


    # t1 = 64000 pixels (0xFA00)
    lui  t1, 0x10
    addi t1, t1, -1536


    # t2 = green color index 0x02
    addi t2, zero, 2


fill_loop:

    sb   t2, 0(t0)

    addi t0, t0, 1
    addi t1, t1, -1

    bne  t1, zero, fill_loop


hang:

    jal  zero, hang
    

    