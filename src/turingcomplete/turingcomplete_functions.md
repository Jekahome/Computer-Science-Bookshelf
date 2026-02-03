# Functions

* [Hex Racer](#hex-racer)
* [Shift](#shift)
* [RAM](#ram)
* [Delay](#delay)
* [The Product of Nibbles](#the-product-of-nibbles)
* [Divide](#divide)
* [Stack](#stack)
* [The Lab](#the-lab)

---

## Hex Racer
> [!TIP]
> Разблокирует компонент Hexadecimal `FF`

> Задача: 
>
> Преобразование шестнадцатеричной системы счисления в двоичную в условиях ограниченного времени.
>
> Переключите биты на панели уровней так, чтобы их сумма равнялась шестнадцатеричному числу, указанному в вопросе.

```
1 hex-цифра = 4 бита (4 бита могут закодировать 16 разных значений)

[1-я цифра HEX][2-я цифра HEX] = 8 bit
[128, 64,32,16][8, 4, 2, 1]    = 8 bit

HEX: 3F
BIN: 0011 1111
Decimal: 48+15=63

HEX: 1E
BIN: 0001 1110
Decimal: 16+14=30
```

| HEX | Десятичное | Бинарное |
| --: | ---------: | -------: |
|   0 |          0 |     0000 |
|   1 |          1 |     0001 |
|   2 |          2 |     0010 |
|   3 |          3 |     0011 |
|   4 |          4 |     0100 |
|   5 |          5 |     0101 |
|   6 |          6 |     0110 |
|   7 |          7 |     0111 |
|   8 |          8 |     1000 |
|   9 |          9 |     1001 |
|   A |         10 |     1010 |
|   B |         11 |     1011 |
|   C |         12 |     1100 |
|   D |         13 |     1101 |
|   E |         14 |     1110 |
|   F |         15 |     1111 |

При работе с большим количеством битов, двоичная система может стать сложной для чтения.

Пример 16 битного числа:
```
Binary:      1111111111111111
Decimal:     65535
Hexadecimal: FFFF
```

---

## Shift

> Задача: 
>
> Задача на этом уровне — сдвинуть первый входной сигнал **влево** на значение, заданное во втором входном сигнале. 
> Значение во втором входном сигнале никогда не превысит 7.


Сдвиг значения на 1 влево означает перемещение всех битов байта влево на 1 позицию,а справа биты обнуляются.

Сдвиг влево на 1 позицию эквивалентен умножению числа на 2.

```
0011 0101   (53)
<< 1
────────
0110 1010   (53*2=106)
```

Сдвиг влево на 2 позиции эквивалентен умножению на 4, а сдвиг на 3 позиции эквивалентен умножению на 8

```
0000 0110   (6)
<< 3
────────
0011 0000   (6*8=48)
```

Переполнение типа (overflow)
```
1100 0000   (192)
<< 1
────────
1000 0000   (128)

384 mod 256 = 128
```

```rust
fn main() {
    let x: u8 = 0b1100_0000; // 192

    let y = x << 1;

    println!("x  = {:08b} ({})", x, x);
    println!("x<<1 = {:08b} ({})", y, y);
}

```

> Сдвиг на несколько разрядов можно реализовать с помощью сдвигающих регистров как результат нескольких одноразрядных сдвигов, но это занимает много времени. 
> **Быстрые сдвигатели** перемещают слово сразу на несколько разрядов в соответствии с указанным значением сдвига.
>
> Схемотехника быстрых сдвигателей насчитывает ряд вариантов, из которых основными являются сдвигатели, управляемые кодом "1 из N" (Barrel Shifters), и сдвигатели, управляемые двоичным кодом (Logarithmic Shifters).

#### Circuit Simulation: Barrel Shifters "1 из N"

<div class="sim-wrapper" data-circuit-id="49">
  <button class="sim-fullscreen-btn" data-circuit-id="49">⛶</button>
  <iframe 
      id="49"
      data-circuit-id="49"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=true&startCircuit=/turingcomplete/49_barrel_shifters.txt"
      loading="lazy">
  </iframe>
</div> 

> Сдвигатель типа Barrel Shifter отличается высоким быстродействием, т. к. сигнал сдвига замыкает для каждой передачи входного бита на выходной буфер цепь только из одного ключа. Однако это справедливо, если управление ведется кодом "1 из N" и имеет малую разрядность.   
> Самый главный недостаток. Количество точек коммутации (переключателей) растёт как `(число_бит) * (диапазон_сдвига)` - экспоненциальный рост сложности (`O(N²)`)
> * Для 8 бит и сдвига 0-7: 8 * 8 = 64 точки коммутации
> * Для 32 бит: 32 * 32 = 1024 точек коммутации
>
>
> При больших величинах сдвигов, преимущество по технико-экономическим показателям обычно оказывается на стороне сдвигателя, непосредственно управляемого двоичными кодами и называемого логарифмическим.
> 

#### Circuit Simulation: Logarithmic Shifter/Каскадный сдвигатель

Каждая ступень каскада делает фиксированный сдвиг на степень двойки (1, 2, 4...).
* `000` данные выход как есть, без сдвига
* `xx1` 1-й каскад сдвиг на 1 позицию влево, эквиваленно умножению на 2
* `x1x` 2-й каскад сдвиг на 2 позиции влево, эквиваленно умножению на 4
* `1xx` 3-й каскад сдвиг на 4 позиции влево, эквиваленно умножению на 16

Пример: Чтобы сдвинуть на 5 позиций (`101` в двоичном), включаются ступени «сдвиг на 4» (S2=1) и «сдвиг на 1» (S0=1). Композиция 4+1 даёт итоговый сдвиг на 5.
 
<div class="sim-wrapper" data-circuit-id="50">
  <button class="sim-fullscreen-btn" data-circuit-id="50">⛶</button>
  <iframe 
      id="50"
      data-circuit-id="50"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=true&startCircuit=/turingcomplete/50_logarithmic_shifter.txt"
      loading="lazy">
  </iframe>
</div> 



![Shift](/Computer-Science-Bookshelf/img/tc/Shift.png)

> Помимо компонента, позволяющего сдвигать схему влево, мы также поручили стажёру создать компонент, позволяющий сдвигать схему вправо. Всё, что ему нужно было сделать, это создать зеркальную версию этой схемы.

---

## RAM

> Задача: 
>
> Добавить блок ОЗУ, таким образом компьютер может оперировать дополнительными 256 байтами памяти.
> 
> Вам нужно найти как выбирать какие из 256 байтов будут использоваться. Выберите регистр и соедините его таким образом, чтобы его значение всегда было адресом ОЗУ. 
> В будущем когда вы захотите вывести или сохранить что-либо в ОЗУ, вам нужно будет сначала скопировать адрес в этот регистр.
>
> На этом уровне, во-первых скопируйте 32 раза значения со входа и сохраните их. 
> После этого, выведите эти значения в таком же порядке в каком и сохранили. Вывод до сохранения всех входных значений приведёт к провалу уровня.


Обновим компонет MUX8Buf
* добавим возможность выбора 9-го источника данных (RAM)

![MUX8Buf upg](/Computer-Science-Bookshelf/img/tc/MUX8Buf_upg.png)

* Выберем `REG 0` для роли адреса RAM. Нам нужен регистр с постоянным режимом Load
* Для выбора RAM в роли source (`Argument 1` и `Argument 2`) или destination (`Result address`) выберем 4-й бит (8) 

![RAM](/Computer-Science-Bookshelf/img/tc/RAM.png)


<details>
<summary>Assembly Editor:</summary>

```bash
const start_save_to_ram 4
const start_output_ram 20
const start_continue_output 24
const iterations 32
 
# set index RAM[reg_0], reg_0=0  
0b11000000 #1 opcode ADD 0+0
0b00000000 #2 arg1 source ImVal
0b00000000 #3 arg2 source ImVal
0b00000000 #4 destination reg_0

# start_save_to_ram ROM[4] -------------------
# копировать с input в RAM
# (через ALU ADD с нулем)
0b01000000 #1 opcode ADD INPUT 0
0b00000111 #2 arg1 source INPUT
0b00000000 #3 arg2 source ImVal
0b00001000 #4 destination RAM

# increment index RAM[reg_0], reg_0+=1 
0b10000000 #1 opcode ADD reg_0+1
0b00000001 #2 arg1 source ImVal
0b00000000 #3 arg2 source reg_0 
0b00000000 #4 destination reg_0


# если reg_0 >= 32 то начать выводить
0b01100101 #1 cond reg_0 >= arg2
0b00000000 #2 arg1 source reg_0
iterations #3 arg2 source ImVal
start_output_ram #4

# всегда прыгать 
0b11100000 #1 cond arg1 == arg2
0b00000000 #2 arg1 source ImVal
0b00000000 #3 arg2 source ImVal
start_save_to_ram #4

# start_output_ram ROM[20] -------------------
# сброс index RAM, reg_0=0 
0b11000000 #1 opcode ADD 0+0
0b00000000 #2 arg1 source ImVal
0b00000000 #3 arg2 source ImVal
0b00000000 #4 destination reg_0

# start_continue_output ROM[24]
0b01000000 #1 opcode ADD RAM + 0
0b00001000 #2 arg1 source RAM
0b00000000 #3 arg2 source ImVal
0b00000111 #4 destination Output

# increment index RAM[reg_0], reg_0+=1 
0b10000000 #1 opcode ADD reg_0+1
0b00000001 #2 arg1 source ImVal
0b00000000 #3 arg2 source reg_0 
0b00000000 #4 destination reg_0

# всегда прыгать 
0b11100000 #1 cond arg1 == arg2
0b00000000 #2 arg1 source ImVal
0b00000000 #3 arg2 source ImVal
start_continue_output

```
</details>

---

## Delay
> [!TIP]
> Разблокирует компонент настраиваемой задержки `Configurable delay`
 

> Задача: 
>
> У каждого компонента есть задержка.
>
> Суммарная задержка схемы определяется **самым медленным путём**. Это значит что обычно лучше делать вещи параллельно.
>
> На этом уровне вы должны показать что понимаете это понятие.
>
> Задержка любого компонента выводится из задержки базовых элементов, задержка которых 2. Создайте цепь с задержкой 6 и стоимостью 5 базовых компонентов.


Для задержки в 6 тактов нужно все 3 компонента, а что бы количество элементов стало 5 мы просто парралельно добавим их в схему.

---

## The Product of Nibbles
> [!TIP]
> Разблокирует компоненты Multipliers `MUL 8/16/32/64 bit` 

> Задача: 
>
> Умножение двух 4-х битных чисел дает 8-и битное число. Создайте схему, которая это делает.

Два варианта параллельных умножителей за один такт:
* Горизонтальный умножитель состоит из 16 AND + 12 сумматоров. Все частичные произведения складываются одновременно по параллельной схеме. Схема физически представляет собой таблицу умножения.
* Вертикальный умножитель состоит из 16 AND + 4-х Logarithmic Shifter и 3-х 8-битных сумматоров. Есть возможность умножать 8-битные числа.

#### Circuit Simulation: Multiplier

**Горизонтальный умножитель**:

<div class="sim-wrapper" data-circuit-id="51">
  <button class="sim-fullscreen-btn" data-circuit-id="51">⛶</button>
  <iframe 
      id="51"
      data-circuit-id="51"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=true&startCircuit=/turingcomplete/51_multiplier.txt"
      loading="lazy">
  </iframe>
</div> 

**Вертикальный умножитель**:


*A* - Множимое, *Bi* - Множитель

|Shift| A dec 7|         |Bi  dec 2|Bi dec 3|Bi dec 4|Bi dec 7|
|-----|--------|---------|---------|--------|--------|--------|
|     |xxxx0111|         |xxxx0010 |xxxx0011|xxxx0100|xxxx0111|
|     |        |         |         |        |        |        |
| << 0|00000111|# 1x7=7  |         |xxxxxxx1|        |xxxxxxx1|
| << 1|00001110|# 2x7=14 |xxxxxx1x |xxxxxx1x|        |xxxxxx1x|
| << 2|00011100|# 4x7=28 |         |        |xxxxx1xx|xxxxx1xx|
| << 3|00111000|# 8x7=56 |         |        |        |        |
|     |        |         |ADD 0+14+0+0=14 |ADD 7+14+0+0=21|ADD 0+0+28+0=28|ADD 7+14+28+0=49|

<br>

<div class="sim-wrapper" data-circuit-id="52">
  <button class="sim-fullscreen-btn" data-circuit-id="52">⛶</button>
  <iframe 
      id="52"
      data-circuit-id="52"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=true&startCircuit=/turingcomplete/52_multiplier_v.txt"
      loading="lazy">
  </iframe>
</div> 

 
![The Product of Nibbles](/Computer-Science-Bookshelf/img/tc/The_Product_of_Nibbles.png)

[The Product of Nibbles (youtube)](https://youtu.be/T9gl-hHou-g?si=duwenzOUiuenijRb&t=3137)

---

## Divide
> [!TIP]
> Разблокирует компоненты Divide `DIV 8/16/32/64 bit` 


> Задача: 
>
> Целочисленно разделите два числа чтобы найти частное и остаток.
> 
> Рассмотрим дробь $7/3$.
> *Три* умещается в *семь* аж *два* раза, при этом, *один* остается в остатке.
> В этом случае *два* принято называть **ЧАСТНОЕ**, а *один* **ОСТАТКОМ ОТ ДЕЛЕНИЯ**.
>
> В этом задании вы получаете `Dividend` числитель (например семь) и `Divisor` знаменатель (например три), и мы ожидаем на выходе `Quotient` ЧАСТНОЕ (два) и `Remainder` ОСТАТОК (один).

Деление - это цикл вычитания знаменателя с числителя пока числитель больше знаменателя, в остатке числителя остается остаток, а количество итераций цикла это частное.

```bash
Dividend = 7
Divisor = 3
Quotient = 0
while Dividend >= Divisor:
    Dividend = Dividend - Divisor
    Quotient = Quotient + 1
Remainder = Dividend
```

 ```rust
 fn divide(mut a: u8, b: u8) -> (u8, u8) {
    let mut quotient: u8 = 0;

    // пока a >= b
    while a >= b {
        a = a - b;      // SUB
        quotient += 1;  // INC
    }

    let remainder = a;
    (quotient, remainder)
}

fn main() {
    let a = 1;
    let b = 2;

    let (q, r) = divide(a, b);

    println!("{} / {} = {}", a, b, q);
    println!("remainder = {}", r);
}
```

<details>
<summary>Assembly Editor:</summary>

```bash
const start_while 12  
const start_output 28
# 1. Dividend input to reg_1
0b01000000 #1 opcode ADD INPUT 0
0b00000111 #2 arg1 source INPUT
0b00000000 #3 arg2 source ImVal
0b00000001 #4 destination reg_1
 
# 2. Divisor input to reg_2
0b01000000 #1 opcode ADD INPUT 0
0b00000111 #2 arg1 source INPUT
0b00000000 #3 arg2 source ImVal
0b00000010 #4 destination reg_2

# 3. Quotient set 0 to reg_3
0b11000000 #1 opcode ADD 0 0
0b00000000 #2 arg1 source ImVal
0b00000000 #3 arg2 source ImVal
0b00000011 #4 destination reg_3

# start_while ROM[12]
# 4. IF_LESS  
0b00100010   #1 opcode Cond IF_LESS
0b00000001   #2 arg1 source reg_1
0b00000010   #3 arg2 source reg_2
start_output #4 destination

# 5. SUB reg_1 - reg_2 = reg_1
0b00000001 #1 opcode SUB
0b00000001 #2 arg1 source reg_1
0b00000010 #3 arg2 source reg_2
0b00000001 #4 destination reg_1

# 6. Quotient ADD 1
0b10000000 #1 opcode ADD 1 reg_3
0b00000001 #2 arg1 source ImVal
0b00000011 #3 arg2 source reg_3
0b00000011 #4 destination reg_3

# always jump
0b11100000  #1 cond arg1 == arg2
0b00000000  #2 arg1 source ImVal
0b00000000  #3 arg2 source ImVal
start_while #4 destination

# start_output ROM[28]
# 7. Output Quotient
0b10000000 #1 opcode ADD 0 Quotient
0b00000000 #2 arg1 source ImVal
0b00000011 #3 arg2 source reg_3
0b00000111 #4 destination Output

# 8. Output Remainder
0b10000000 #1 opcode ADD 0 reg_1
0b00000000 #2 arg1 source ImVal
0b00000001 #3 arg2 source reg_1
0b00000111 #4 destination Output
```
</details>

---

## Stack

> Задача: 
>
> В целях сокращения расходов было решено изменить систему очередей в государственных учреждениях, чтобы уменьшить поток посетителей. Вместо принципа «кто первый пришел, тот и обслуживается» мы вводим принцип «кто последний пришел, тот и обслуживается». 
> 
> Представьте это как стопку (stack) пронумерованных бланков, где граждане либо кладут бланк сверху (это называется **PUSH**), либо чиновники берут бланк сверху стопки (это называется **POP**). Мы хотим, чтобы вы внедрили эту новую систему.
>
> * Когда на входе POP сигнал HIGH вам следует извлечь данные из stack и вывести в output 
> * Когда на входе PUSH сигнал HIGH вам следует поместить в stack данные из input 

 
![Stack](/Computer-Science-Bookshelf/img/tc/Stack.png)

Варианты менее избыточных реализаций [Stack (youtube)](https://www.youtube.com/watch?v=Z6RWuAm9R0U) и [Stack (youtube)](https://youtu.be/T9gl-hHou-g?si=F0H2_SM8tjKWvl6h&t=3151)

---

## The Lab

Лаборатория — как и фабрика компонентов это не обычный уровень, а **инструмент отладки**. Лаборатория позволяет игроку сохранять программы для автоматической проверки их работоспособности.

Ключевое слово **expect** позволяет проверять внутреннее значение компонентов. Оно указывает адрес памяти для следующей инструкции. Принимает два аргумента: 
* первый — индекс связанного компонента (индекс связи это по порядковый номер подключения, который вы устанавливаете в ручную [при редактировании компонента PROGRAM](turingcomplete_cpu_architecture_2.html#wire-spaghetti) (Link components))
* второй — его корректное значение
Значение проверяется после каждой инструкции, поэтому, если значение снова изменится, необходимо указать новую строку в expect.

```
expect 2 4 # ожидаем что второй подключенный компонент будет иметь значение 4 на следующем такте
```

Ключевое слово **set_input** позволяет задавать пользовательские входные значения, что дает возможность тестировать инструкции, связанные с вводом данных.

```
set_input 123
```

Все подключенные компоненты проверяются на соответствие ожиданиям **каждый такт**. Ожидается что все подключенные компоненты будут иметь значение `0` на момент начала программы, ожидания для компонентов памяти меняются только если вы обновляете ожидания. Единственное исключение это счётчики, от которых ожидается что они будут увеличивать своё значение каждый такт.


> [!FAILURE]
> Инструкция `set_input` работает только в начале кода.

Assembly Editor:

```bash
set_input 45
expect 7 45 # expect Output 45
0b10000000 #1 opcode ADD 0 INPUT
0b00000000 #2 arg1 source ImVal
0b00000111 #3 arg2 source reg_1
0b00000111 #4 destination Output

expect 7 12 # expect Output 12
0b11000000 #1 opcode ADD 0 ImVal
0b00000110 #2 arg1 source ImVal
0b00000110 #3 arg2 source ImVal
0b00000111 #4 destination Output

expect 0 8 # expect reg_0 8
0b11000000 #1 opcode ADD 0 ImVal
0b00000100 #2 arg1 source ImVal
0b00000100 #3 arg2 source ImVal
0b00000000 #4 destination reg_0
```


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