# CPU Architecture 2

* [XOR](#xor)
* [Byte XOR](#byte-xor)
* [Byte Constant](#byte-constant)
* [Equality](#equality)
* [Unsigned Less](#unsigned-less)
* [Signed Less](#signed-less)
* [Wide Instructions](#wide-instructions)
* [Wire Spaghetti](#wire-spaghetti)
* [Opcodes](#opcodes)
* [Immediate Values](#immediate-values)


---

## XOR


> Задача: 
>
> Используя инструкции вашего компьютера, дважды считайте входные данные и выполните операцию [XOR](turingcomplete_basic_logic.html#xor--a-xor-b) над этими двумя значениями.


Воспользуемся одним из способов получения XOR: `A XOR B = (A NAND B) && (A || B)`

Assembly Editor:
```bash
0b10110001 # copy input to reg_1

0b10110010 # copy input to reg_2

# A XOR B = (A NAND B) && (A || B)

0b01000001 # calc (A NAND B) to reg_3
0b10011000 # copy reg_3 to reg_0

0b01000000 # calc (A || B) to reg_3
0b10011001 # copy reg_3 to reg_1
0b10000010 # copy reg_0 to reg_2

0b01000011 # calc (A NAND B) && (A || B) to reg_3
0b10011110 # copy reg_3 to output
```

---

## Byte XOR

> [!TIP]
> Разблокирует `8 Bit XNOR`, `8 Bit XOR`


> Задача: 
>
> Создайте схему которая выполняет операцию XOR для двух байт


![Byte XOR](/Computer-Science-Bookshelf/img/tc/8BitXOR.png)


---

## Byte Constant

> Задача: 
>
> Создайте схему которая всегда выводит значение `164`

Нужно использовать компонент Byte Combiner который "собирает" биты в байт.

---

## Equality

> [!TIP]
> Разблокирует `8 Bit Equality` 

> Задача: 
>
> Создайте схему которая на выходе дает 1 если два 8-ми битных входа одинаковые либо 0 и 0, либо 1 и 1 

Классическое логическое решение использовать XOR, так как его таблица истинности соответвует задаче.

```
# 1 bit
OUT = NOT (A XOR B)
```

Но другой вариант решения, обычно быстрее в контексте CPU, и может быть выгоднее по железу, если сумматор уже существует. Он основан на использовании цепочки переноса полного сумматора, которая «протекает» только если все биты равны.

```
X = A AND B      =  1010
Y = A NOR B      =  0101
X + Y            =  1111
   + carry_in 1  = 10000

carry_out = 1 → равно
```

```
X = A AND B     = 1000
Y = A NOR B     = 0101
X + Y           = 1101
  + carry_in 1  = 1110

carry_out = 0 → не равно
```


![Equality](/Computer-Science-Bookshelf/img/tc/Equality.png)

---

## Unsigned Less

> [!TIP]
> Разблокирует `8 Bit Less (Unsigned)` 


> Задача: 
>
> Создайте схему которая на выходе дает 1 если первое значение меньше второго. Интерпретируейте байти как беззнаковые.


Дано:
* два 8-битных числа A и B
* интерпретация без знака (0…255)

Нужно:
* `OUT = 1  ⇔  A < B`
* `OUT = 0  ⇔  A ≥ B`

> Чтобы узнать, меньше ли `A` чем `B` (беззнаковое сравнение), делают `A - B`.
> Для этого в процессорах используют **дополнение до двух**: инвертируют `B` и прибавляют 1.
> Суммирование с `A` происходит в полном сумматоре.
> Если `B > A`, возникает заём (borrow), который в архитектуре фиксируется как `carry_out = 0`.
> Если `A ≥ B`, carry_out = 1.
> Таким образом:
>
> ```
> A - B = A + (~B) + 1
> UnsignedLess = NOT(carry_out)
> ```


```
A < B ?
A = 00000011 # 3
B = 00000100 # 4

~B = NOT(B) = 11111011

  00000011  (A)
+ 11111011  (~B)
+ 00000001  (carry_in)
------------
0 11111111  (результат carry_out = 0)
 
A мешьше чем B
```

```
A < B ?
A = 00000100 # 4
B = 00000011 # 3

~B = NOT(B) = 11111100

  00000100  (A)
+ 11111100  (~B)
+ 00000001  (carry_in)
------------
1 11111101 (результат carry_out = 1)

A больше или равно B
```
 
![Unsigned Less](/Computer-Science-Bookshelf/img/tc/Unsigned_Less.png)

---

## Signed Less

> [!TIP]
> Разблокирует `8 Bit Less (Signed)` 

 
> Задача: 
>
> Создайте схему которая на выходе дает 1 если первое значение меньше второго. Интерпретируейте байти как знаковые. 


> Особенность: простой carry для вычитания работает только для беззнаковых чисел.
> Для signed надо учитывать **старший бит (MSB) = знак и overflow**.

*`A < B` (signed)* тогда и только тогда, когда:

1. Их **знаковые биты различаются** и
   * **`A`** отрицательное, а **`B`** положительное → **`A < B = true`**
   * **`A`** положительное, а **`B`** отрицательное → **`A < B = false`**
2. Если знаки одинаковы → сравнить их **высшие 7 бит как unsigned**
   * если `A[6:0] < B[6:0]` → результат зависит от знака
   * иначе наоборот

 
![Signed Less](/Computer-Science-Bookshelf/img/tc/Signed_Less.png)

[strategywiki.org/wiki/Turing_Complete/Signed_Less](https://strategywiki.org/wiki/Turing_Complete/Signed_Less)

---

## Wide Instructions
> [!TIP]
> Разблокирует компонент `Program` 

 
> Задача: 
>
> Создайте устройство которое сохраняет выход программы на четных тактах и выводит оба байта на нечетных тактах.

Мы делаем широкие инструкции, объединяя два байта в одну 16‑битную «команду»

Нам нужен регистровый буфер, который:
* На чётном такте (у четных чисел младший байт равен 0): сохраняет байт в REG_0
* На нечетном такте (1, 3, 5 …):
    * выводим REG_0 в один из OUTPUT
    * выводим текущую инстркцию в другой OUTPUT

![Wide Instructions](/Computer-Science-Bookshelf/img/tc/Wide_Instructions.png)

---

## Wire Spaghetti

> Задача: 
>
> Пришло время создавать архитектуру LEG (Logical Execution Grid / или просто твоя собственная CPU‑архитектура в Turing Complete).
>
> ISA (Instruction Set Architecture) — это архитектура набора команд, фундаментальный интерфейс между программным обеспечением и аппаратурой процессора.
>
> ```
> ISA архитектура: 
>
> LEG — это компьютер, который берет 4 байта за такт из программы
>  
> Первый байт описывает операцию (называемую OPCODE)
> * байт 1 → Opcode
>
> Поскольку многие операции принимают 2 аргумента (ADD, OR и т.д.), второй и третий байты используются для указания аргументов
> * байт 2 → Argument 1 (какой исчтоник направить в ALU (input 1) адрес/регистр/IO)
> * байт 3 → Argument 2 (какой исчтоник направить в ALU (input 2) адрес/регистр/IO)
> 
> Поскольку большинство операций возвращают одно значение результата, четвертый байт предназначен для указания результата
> * байт 4 → Result address (куда направить результат ALU)
> 
> [ Opcode 8bit][ Argument 1 8bit][ Argument 2 8bit][ Result address 8bit]
> ```
 

![LEG](/Computer-Science-Bookshelf/img/tc/LEG.png)

1. Создайте новую схему
2. Поместите на схему компонент PROGRAM с 4-мя выходами
3. Поместите на схему компонент Counter, установите инкремент на 4 и подключите его к PROGRAM
4. Поместите на схему шесть регистров
5. Отредактируйте связи PROGRAM (Link components) с другими компонентами:
    1. Подключите регистры с 0-го до 5-го pin
    1. Counter к 6-му pin
    1. Output к 7-му pin

На этом уровне OPCODE всегда равен 0 (т.е. нужно реализовать только операцию `ADD`). 
Это означает, что мы выполняем операцию `ADD` с `Argument 1` и `Argument 2` и сохраняем результат в место куда указывает `Result address` 4-й байт из вывода PROGRAM.

Значение выходов `Argument 1` и `Argument 2` или `Result address` из PROGRAM, относится к одному из следующих мест:
* 0: Register 0
* 1: Register 1
* 2: Register 2
* 3: Register 3
* 4: Register 4
* 5: Register 5
* 6: Counter (PC)
* 7: Input (для `Argument 1` и `Argument 2`) или Output (для `Result address`)

На следующем уровне будем реализовывать еще больше команд OPCODE для чего потребуется другой ALU

Для прохождения уровня мы можем использовать уже существующий компонент ALU для операции ADD. 

Решение основанно на MUX8Bit который аккумулирует все исчтоники ввода, что избавляет нас от множества tri-state buffers разделяя общую шину.

![Wire Spaghetti](/Computer-Science-Bookshelf/img/tc/Wire_Spaghetti.png)

Вспомогательный компонет MUX8Buf, нужен что бы не перегружать схему линиями питания:

![Wire Spaghetti](/Computer-Science-Bookshelf/img/tc/MUX8Buf.png)
 
[Turing Complete - CPU Architecture 2 (www.youtube.com) ](https://youtu.be/kBXghkY9dqw?si=lgvhaSRQbKwSrywn&t=6111)

---

## Opcodes

> Задача: 
>
> Реализовать opcodes:
> 
> ```
> 0 ADD
> 1 SUB
> 2 AND
> 3 OR
> 4 NOT
> 5 XOR
> ```
> 
> p.s. операция NOT игнорирует **второй** аргумент

У нас уже есть готовый компонет ALU, но с другими opcodes и без операций NOT и XOR. Создадим на его основе новый компонент.

Компонент ALU_CPU2:
```
0 ADD
1 SUB
2 AND
3 OR
4 NOT
5 XOR
6 NAND
7 NOR
```

![ALU_CPU2](/Computer-Science-Bookshelf/img/tc/ALU_CPU2.png)

---

## Immediate Values

Иногда бывает полезно загрузить значение непосредственно из программы, а не из регистров. Это называется загрузкой непосредственного значения (Immediate Values). В архитектуре LEG мы указываем, когда хотим это сделать, непосредственно в коде операции. 

Это можно сделать следующим образом:
* Когда 8-й бит (MSB) opcode `0xxxxxxx` равен HIGH, используйте `Argument 1` в качестве непосредственного значения, а не в качестве адреса регистра.
    * т.е. теперь в байте `Argument 1` содержатся данные
* Если 7-й бит opcode `x0xxxxxx` равен HIGH, используйте `Argument 2` в качестве непосредственного значения, а не в качестве адреса регистра.
    * т.е. теперь в байте `Argument 2` содержатся данные
 

![Immediate Values](/Computer-Science-Bookshelf/img/tc/Immediate_Values_CPU2.png)
 
![Immediate Values](/Computer-Science-Bookshelf/img/tc/Solution_Immediate_Values_CPU2.png)
 
Обновим ALU, 8-ми битный вход ему избыточен, достаточно 3 бита.

![ALU CPU2 upgrade](/Computer-Science-Bookshelf/img/tc/ALU_CPU2_upgrade.png)
 


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
 