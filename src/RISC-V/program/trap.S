.section .text
.global trap_handler

##################################################
# Общий обработчик trap (Direct Mode)
##################################################

trap_handler:

    # mcause
    csrr t0, mcause

    ##############################################
    # Interrupt
    ##############################################

    # External Interrupt
    li t1, 0x8000000B
    beq t0, t1, irq_handler

    # Software Interrupt
    li t1, 0x80000003
    beq t0, t1, software_handler


    # Timer Interrupt
    li t1, 0x80000007
    beq t0, t1, timer_handler

    ##############################################
    # Exception
    ##############################################

    li t1, 11
    beq t0, t1, ecall_handler

    li t1, 3
    beq t0, t1, ebreak_handler

    li t1, 2
    beq t0, t1, illegal_handler

    li t1, 1    # Instruction Access Fault
    beq t0, t1, memory_error

    li t1, 5    # Load Access Fault
    beq t0, t1, memory_error

    li t1, 7    # Store Access Fault
    beq t0, t1, memory_error

hang:
    j hang

##################################################
# Instruction Access Fault
##################################################

memory_error:

    li t0, 0x4D        # 77 'M'
    li t1, 0x10000000   
    sb t0, 0(t1)        

    # Просто зависаем здесь, чтобы симуляция не улетела в космос
    memory_hang:
        j memory_hang

##################################################
# ECALL
##################################################

ecall_handler:

    li t0,0x45 # 69 'E'
    li t1,0x10000000
    sb t0,0(t1)

    csrr t2,mepc
    addi t2,t2,4
    csrw mepc,t2

    mret

##################################################
# EBREAK
##################################################

ebreak_handler:

    li t0,0x4B # 75 'K'
    li t1,0x10000000
    sb t0,0(t1)

    csrr t2,mepc
    addi t2,t2,4
    csrw mepc,t2

    mret

##################################################
# Illegal Instruction
##################################################

illegal_handler:

    li t0,73 # 'I'
    li t1,0x10000000
    sb t0,0(t1)

    csrr t2,mepc
    addi t2,t2,4
    csrw mepc,t2

    mret

##################################################
# External Interrupt
##################################################

irq_handler:

    li t0,81 # 'Q'
    li t1,0x10000000
    sb t0,0(t1)

    # Снять MEIP
    li t0,0
    li t1,0x10000010
    sw t0,0(t1)

    mret

##################################################
# Timer Interrupt
##################################################

timer_handler:

    li t0,'T'
    li t1,0x10000000
    sb t0,0(t1)

    # Снятие MTIP аппаратно

    # Перенести следующее событие таймера
    # mtimecmp = mtime + период

    # читаем mtime_lo (mtime_lo)
    csrr t2, 0xC01

    # Следующее срабатывание через 1000 тактов
    li t3,1000
    add t2,t2,t3

    # Записать новое значение mtimecmp_lo
    li t1,0x10000004
    sw t2,0(t1)

    mret    

##################################################
# Software Interrupt
##################################################

software_handler:

    # Вывести 'S'
    li t0, 'S'
    li t1, 0x10000000
    sb t0, 0(t1)

    # Снять MSIP
    li t0,0
    li t1,0x10000018
    sw t0,0(t1)

    mret
    