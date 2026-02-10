# Assembly Challenges

* [Robot Racing](#robot-racing)

---

## Robot Racing


> Задача:
> 
> Гонки роботов — любимый вид спорта на космическом корабле. Роботы с разным программированием преодолевают полосу препятствий. Среди роботов, прошедших трассу, победителем становится тот, у кого была самая короткая программа.
> 
> 
> На этот раз вы управляете Fastbot, он не видит, что находится перед ним, но может развернуться в новом направлении и двигаться за тот же тик. А ещё он носит модные красные кроссовки.
> 

Fastbot controls:
* 0 Go right
* 1 Go down
* 2 go left
* 3 go up

Лабиринт  отличается от лабиринта на уровне [The Maze](turingcomplete_programming.html#the-maze) тем, что нет поворота робота и обратной свзяи.
Лабиринт разбит на четыре секции но последовательности действий для прохождения лабиринта всего две. 
* Вторая с третьей имеют общий алгоритм,  с одним отличием - выход из секции.
* Стартовая секция и завершающая это второй алгоритм последовательности шагов, оличаются выходом из секции и зеркальным отражением. 
    * Если в первой группе мы можем просто передать последовательность шагов то при зеркальном раположении лабиринтов нам нужно переопределить направления
    * Если повернуть первую секцию так как мы смотрим на вторую и третью то чтобы двигать влево нам нужно передать сигнал идти вверх и т.д. А для четвертой секции, чтобы идти влево нужно передать сигнал вниз

    |sec.|right|down|left|up |finish|
    |----|-----|----|----|---|------|
    |1|1|2|3|0|2|
    |2|0|1|2|3|2|
    |3|0|1|2|3|1|
    |4|3|0|1|2|2|


![Robot Racing](/Computer-Science-Bookshelf/img/tc/Robot_Racing.png)


Для реализации ассемблера нам потребуется наличие команд `CALL` и `RET`, `MOV`. Количество тиков - 86.



<details>
<summary>Assembly Editor:</summary>

```bash
# Fastbot controls:
# 0 Go right
# 1 Go down
# 2 go left
# 3 go up
 
const CALL 0b00001000
const RET 0b00010000
const jump_fn 84

# prepare 1
# 12302
# rewrite right
0b10001100 #1 opcode MOV reg_0=1
0b00000001 #2 arg1 source ImVal
0b00000000 #3 arg2 source unused
0b00000000 #4 destination reg_0
# rewrite down
0b10001100 #1 opcode MOV reg_1=2
0b00000010 #2 arg1 source ImVal
0b00000000 #3 arg2 source unused
0b00000001 #4 destination reg_1
# rewrite left
0b10001100 #1 opcode MOV reg_2=3
0b00000011 #2 arg1 source ImVal
0b00000000 #3 arg2 source unused
0b00000010 #4 destination reg_2
# rewrite up
0b10001100 #1 opcode MOV reg_3=0
0b00000000 #2 arg1 source ImVal
0b00000000 #3 arg2 source unused
0b00000011 #4 destination reg_3
# finish
0b10001100 #1 opcode MOV reg_4=2
0b00000010 #2 arg1 source ImVal
0b00000000 #3 arg2 source unused
0b00000100 #4 destination reg_4
 

# CALL
CALL        #1 opcode CALL
0b00000000  #2 arg1 source unused
0b00000000  #3 arg2 source unused
jump_fn     #4 destination PC

# prepare 2 
# 01232

0b10001100 #1 opcode MOV reg_0=0
0b00000000 #2 arg1 source ImVal
0b00000000 #3 arg2 source unused
0b00000000 #4 destination reg_0

0b10001100 #1 opcode MOV reg_1=1
0b00000001 #2 arg1 source ImVal
0b00000000 #3 arg2 source unused
0b00000001 #4 destination reg_1

0b10001100 #1 opcode MOV reg_2=2
0b00000010 #2 arg1 source ImVal
0b00000000 #3 arg2 source unused
0b00000010 #4 destination reg_2

0b10001100 #1 opcode MOV reg_3=3
0b00000011 #2 arg1 source ImVal
0b00000000 #3 arg2 source unused
0b00000011 #4 destination reg_3
# finish
0b10001100 #1 opcode MOV reg_4=2
0b00000010 #2 arg1 source ImVal
0b00000000 #3 arg2 source unused
0b00000100 #4 destination reg_4

 
# CALL
CALL        #1 opcode CALL
0b00000000  #2 arg1 source unused
0b00000000  #3 arg2 source unused
jump_fn     #4 destination PC

# prepare 3 
# 01231

# finish
0b10001100 #1 opcode MOV reg_4=1
0b00000001 #2 arg1 source ImVal
0b00000000 #3 arg2 source unused
0b00000100 #4 destination reg_4
 
# CALL
CALL        #1 opcode CALL
0b00000000  #2 arg1 source unused
0b00000000  #3 arg2 source unused
jump_fn     #4 destination PC

# prepare 4 
# 30122

0b10001100 #1 opcode MOV reg_0=3
0b00000011 #2 arg1 source ImVal
0b00000000 #3 arg2 source unused
0b00000000 #4 destination reg_0

0b10001100 #1 opcode MOV reg_1=0
0b00000000 #2 arg1 source ImVal
0b00000000 #3 arg2 source unused
0b00000001 #4 destination reg_1

0b10001100 #1 opcode MOV reg_2=1
0b00000001 #2 arg1 source ImVal
0b00000000 #3 arg2 source unused
0b00000010 #4 destination reg_2

0b10001100 #1 opcode MOV reg_3=2
0b00000010 #2 arg1 source ImVal
0b00000000 #3 arg2 source unused
0b00000011 #4 destination reg_3
# finish
0b10001100 #1 opcode MOV reg_4=2
0b00000010 #2 arg1 source ImVal
0b00000000 #3 arg2 source unused
0b00000100 #4 destination reg_4

# CALL
CALL        #1 opcode CALL
0b00000000  #2 arg1 source unused
0b00000000  #3 arg2 source unused
jump_fn     #4 destination PC


# JMP Exit 
0b10001100 #1 opcode MOV source destination
0b00000000 #2 arg1 source ImVal
0b00000000 #3 arg2 Unused
146       #4 destination PC

# FUNCTION ----------------------

# MOV reg_N to OUTPUT
# 0
0b10001100 #1 opcode MOV
0b00000000 #2 arg1 source reg_N
0b00000000 #3 arg2 source unused
0b00000111 #4 destination OUTPUT
# 3
0b10001100 #1 opcode MOV
0b00000011 #2 arg1 source reg_N
0b00000000 #3 arg2 source unused
0b00000111 #4 destination OUTPUT

# 2
0b10001100 #1 opcode MOV
0b00000010 #2 arg1 source reg_N
0b00000000 #3 arg2 source unused
0b00000111 #4 destination OUTPUT

# 3
0b10001100 #1 opcode MOV
0b00000011 #2 arg1 source reg_N
0b00000000 #3 arg2 source unused
0b00000111 #4 destination OUTPUT

# 3
0b10001100 #1 opcode MOV
0b00000011 #2 arg1 source reg_N
0b00000000 #3 arg2 source unused
0b00000111 #4 destination OUTPUT

# 0
0b10001100 #1 opcode MOV
0b00000000 #2 arg1 source reg_N
0b00000000 #3 arg2 source unused
0b00000111 #4 destination OUTPUT

# 1
0b10001100 #1 opcode MOV
0b00000001 #2 arg1 source reg_N
0b00000000 #3 arg2 source unused
0b00000111 #4 destination OUTPUT

# 0
0b10001100 #1 opcode MOV
0b00000000 #2 arg1 source reg_N
0b00000000 #3 arg2 source unused
0b00000111 #4 destination OUTPUT

# 3
0b10001100 #1 opcode MOV
0b00000011 #2 arg1 source reg_N
0b00000000 #3 arg2 source unused
0b00000111 #4 destination OUTPUT

# 0
0b10001100 #1 opcode MOV
0b00000000 #2 arg1 source reg_N
0b00000000 #3 arg2 source unused
0b00000111 #4 destination OUTPUT

# 1
0b10001100 #1 opcode MOV
0b00000001 #2 arg1 source reg_N
0b00000000 #3 arg2 source unused
0b00000111 #4 destination OUTPUT

# 1
0b10001100 #1 opcode MOV
0b00000001 #2 arg1 source reg_N
0b00000000 #3 arg2 source unused
0b00000111 #4 destination OUTPUT

# 2
0b10001100 #1 opcode MOV
0b00000010 #2 arg1 source reg_N
0b00000000 #3 arg2 source unused
0b00000111 #4 destination OUTPUT

# 1
0b10001100 #1 opcode MOV
0b00000001 #2 arg1 source reg_N
0b00000000 #3 arg2 source unused
0b00000111 #4 destination OUTPUT
 
# 0
0b10001100 #1 opcode MOV
0b00000000 #2 arg1 source reg_N
0b00000000 #3 arg2 source unused
0b00000111 #4 destination OUTPUT

# 4
0b10001100 #1 opcode MOV
0b00000100 #2 arg1 source reg_N
0b00000000 #3 arg2 source unused
0b00000111 #4 destination OUTPUT


RET         #1 opcode RET
0b00000000  #2 arg1 source unused
0b00000000  #3 arg2 source unused
0b00000000  #4 destination unused

```
</details>
 


![Robot Racing](/Computer-Science-Bookshelf/img/tc/Robot_Racing.gif)

Остановки робота происходят когда загружаются пять регистров.

В этом видео [Robot Racing (zoickx youtube)](https://youtu.be/FojkEeoPfcs?si=dAXdFiMMQkS24dL2&t=5833) замысловатое решение на 803 тика.

---

<!-- Feedback -->
<!-- Read the Formbutton docs at formspree.io/formbutton/docs. See more examples at codepen.io/formspree -->
<!-- <script src="https://formspree.io/js/formbutton-v1.min.js" defer></script> -->
<script>
  window.formbutton = window.formbutton || function() {
    (formbutton.q = formbutton.q || []).push(arguments)
  };
  formbutton("create", {
    action: "https://formspree.io/f/xkogdkjd",
    title: "Feedback",
    fields: [
      { 
        type: "text", 
        label: "Name:", 
        name: "name",
        required: true,
        placeholder: "Your name"
      },
      {
        type: "textarea",
        label: "Message:",
        name: "message",
        required: true,
        placeholder: "Please share your thoughts...",
        rows: 5
      },
      {
        type: "file",
        label: "Attach file (optional, max 10MB):",
        name: "file",
        required: false,
        multiple: false,
        accept: "image/*,.pdf,.doc,.docx,.txt"
      },
     { 
        type: "email", 
        label: "Email (optional, for reply):", 
        name: "email",
        required: false,
        placeholder: "your@email.com"
      },
      { type: "submit" }      
    ],
    styles: {
      title: {
        backgroundColor: "#333",
        color: "#fff"
      },
      input: {
        borderBottom: "1px solid #CCC",
        borderRight: "1px solid #CCC",
        padding: "5px 0"
      },
      button: {
        backgroundColor: "#4a5568",
        color: "#fff"
      },
      form: {
        backgroundColor: "#f7fafc",
        maxWidth: "400px"
      },
      submitInput: {padding: "0.75em 1em"}
    },
  });
</script>
 
<style>
table {
  margin: 0px !important;  
  border-collapse: collapse;
}
</style> 

