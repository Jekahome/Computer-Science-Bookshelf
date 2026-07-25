.section .text
.global main

main:
    
    # Печатаем 'A'
    li t0, 0x41 # 65 'A'
    li t1, 0x10000000
    sb t0, 0(t1)

    # Вызываем исключение
    ebreak

    # После возврата из обработчика
    li t0, 'B'
    sb t0, 0(t1)
     
loop:
    j main
 