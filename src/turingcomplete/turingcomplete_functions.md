# Functions

* [Hex Racer](#hex-racer)
* [Shift](#shift)
* [RAM](#ram)
* [Delay](#delay)
* [The Product of Nibbles](#the-product-of-nibbles)
* [Divide](#divide)
* [Stack](#stack)
* [The Lab](#the-lab)
* [Push and Pop](#push-and-pop)
* [Functions](#functions-1)
    * [Текущая архитектура ISA процессора LEG](#Текущая-архитектура-isa-процессора-leg)
* [Расширение opcode инструкций](#Расширение-opcode-инструкции)    

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

* SP (Stack Pointer) выберем `REG 0` для роли адреса RAM. Нам нужен регистр с постоянным режимом Load
* Для выбора RAM в роли source (`Argument 1` и `Argument 2`) или destination (`Result address`) выберем 4-й бит `xxxx1xxx` (8) 

![RAM](/Computer-Science-Bookshelf/img/tc/RAM.png)


<details>
<summary>Assembly Editor:</summary>

```bash
const jump_save_to_ram 4
const jump_output_ram 20
const jump_continue_output 24
const iterations 32
# reg_0 SP (Stack Pointer)

# set index RAM[reg_0], reg_0=0  
0b11000000 #1 opcode ADD 0+0
0b00000000 #2 arg1 source ImVal
0b00000000 #3 arg2 source ImVal
0b00000000 #4 destination reg_0

# jump_save_to_ram ROM[4] -------------------
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
jump_output_ram #4

# всегда прыгать 
0b11100000 #1 cond arg1 == arg2
0b00000000 #2 arg1 source ImVal
0b00000000 #3 arg2 source ImVal
jump_save_to_ram #4

# jump_output_ram ROM[20] -------------------
# сброс index RAM, reg_0=0 
0b11000000 #1 opcode ADD 0+0
0b00000000 #2 arg1 source ImVal
0b00000000 #3 arg2 source ImVal
0b00000000 #4 destination reg_0

# jump_continue_output ROM[24]
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
jump_continue_output

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
const jump_while 12  
const jump_output 28
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

# jump_while ROM[12]
# 4. IF_LESS  
0b00100010   #1 opcode Cond IF_LESS
0b00000001   #2 arg1 source reg_1
0b00000010   #3 arg2 source reg_2
jump_output #4 destination

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
jump_while #4 destination

# jump_output ROM[28]
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

Варианты менее избыточных реализаций [Stack Registers (youtube)](https://www.youtube.com/watch?v=Z6RWuAm9R0U) и [Stack (youtube)](https://youtu.be/T9gl-hHou-g?si=F0H2_SM8tjKWvl6h&t=3151)

![Stack](/Computer-Science-Bookshelf/img/tc/Stack2.png)

---

## The Lab

Лаборатория — как и фабрика компонентов (но попасть в него можно с карты уровней) для экспериментов, а **инструмент отладки**. Лаборатория позволяет игроку сохранять программы для автоматической проверки их работоспособности.

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
0b00000111 #3 arg2 source INPUT
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

## Push and Pop

> Задача: 
>
> На этом уровне вы должны добавить Stack в свою архитектуру компьютера и написать программу.
>
> * Когда ввод равен 0, вы должны (`pop`) извлечь значение из стека и вывести его. 
> * Когда вход не равен 0, вы должны (`push`) поместить это значение в стек.


Три варианта:
* Аппаратный стек, стек как внешний компонент, который физически отделен от основной шины данных.
    * Это архитектура, похожая на Гарвардскую архитектуру (где память команд и память данных разделены)
    * Достоинство: надежность - вы не сможете "залезть" случайно на адреса данных RAM и затереть их данными для стека
    * Недостаток: 
        * Фиксированный лимит памяти для данных стека, иначе `Stack Overflow`
        * Невозможность "произвольного доступа", видно только вершину стека, нельзя применять арифметику смещения по адресам
        * Отсутствует возможность адресации через указатель, так как ячейки стека не имеют индексов в общем адресном пространстве памяти. Доступно только верхнее значение (вершина стека)
* Разместить стек в RAM
    * Чтобы RAM превратилась в стек, процессору нужен специальный регистр — `Stack Pointer` (SP) (Указатель стека).
    * Начало адресов стека это последний индекс (255) в RAM и расти он будет к середине адресов RAM, т.е. уменьшаться `Descending Stack` (убывающий стек)
        * **Heap в начале адресов RAM, в Stack в конце**
* **Гибрид**, аппаратный стек для стека возвратов (Return Stack), и стек в RAM для стека данных (Data Stack)
    * Достоинство: надежность - программа всё равно сможет корректно выполнить `RET` и вернуться в основное меню, потому что адрес возврата лежит отдельно от данных
* и другие гибриды:
    * Стек-кэш (Register Stack)
    * "Теневой" стек (Shadow Stack)  


Разместить стек в RAM. Descending Stack (убывающий стек)

<details>
<summary>Assembly Editor:</summary>

Схема не изменилась с уровня [RAM](#ram).

```bash
const jump_while 4  
const jump_pop 28

# reg_4 - Stack Pointer (SP)
# reg_1 - INPUT
# reg_0 - RAM Pointer

# ------------------------------
# Stack Pointer (SP) reg_4
0b11000000 #1 opcode ADD 0 255
0b00000000 #2 arg1 source ImVal
0b11111111 #3 arg2 source 255
0b00000100 #4 destination reg_4

 
#-------------------------------
# jump_while
 
# take INPUT
0b10000000 #1 opcode ADD 0 INPUT
0b00000000 #2 arg1 source ImVal
0b00000111 #3 arg2 source INPUT
0b00000001 #4 destination reg_1

# cond IF_EQUAL 0 == reg_1
0b10100000 #1 opcode cond IF_EQUAL 
0b00000000 #2 arg1 source ImVal
0b00000001 #3 arg2 source reg_1
jump_pop #4 destination jump

# start push--------------------
# reg_0=reg_4
0b10000000 #1 opcode ADD 0+reg_4 
0b00000000 #2 arg1 source ImVal
0b00000100 #3 arg2 source reg_4
0b00000000 #4 destination reg_0

# SP-1 push
0b01000001 #1 opcode SUB reg_4-1
0b00000100 #2 arg1 source reg_4
0b00000001 #3 arg2 source ImVal
0b00000100 #4 destination reg_4

# push INPUT значение в стек
0b10000000 #1 opcode ADD 0 reg_1
0b00000000 #2 arg1 source ImVal
0b00000001 #3 arg2 source reg_1
0b00001000 #4 destination RAM

# always jump
0b11100000  #1 cond IF_EQUAL arg1 == arg2
0b00000000  #2 arg1 source ImVal
0b00000000  #3 arg2 source ImVal
jump_while #4 destination


# jump_pop------------------
# pop значение из стека и вывести его 
# SP+1 pop
0b10000000 #1 opcode ADD 1+reg_4
0b00000001 #2 arg1 source ImVal
0b00000100 #3 arg2 source reg_4
0b00000100 #4 destination reg_4

# reg_0=reg_4
0b10000000 #1 opcode ADD 0+reg_4 
0b00000000 #2 arg1 source ImVal
0b00000100 #3 arg2 source reg_4
0b00000000 #4 destination reg_0

# stack output
0b10000000 #1 opcode ADD 0+reg_4
0b00000000 #2 arg1 source ImVal
0b00001000 #3 arg2 source RAM
0b00000111 #4 destination OUTPUT
 
# ------------------------
  
# always jump
0b11100000  #1 cond IF_EQUAL arg1 == arg2
0b00000000  #2 arg1 source ImVal
0b00000000  #3 arg2 source ImVal
jump_while #4 destination

```

</details>



---

## Functions

Иногда полезно повторно использовать фрагмент кода.

Обычных команд JUMP (прыжков) достаточно для любой сложной логики (циклы while, ветвления if/else). Но это работает только с кодом который не предполагает повторное использование фрагментов кода, так как мы не знаем куда вернуться (вернее мы можем это сделать, но придется запоминать все адреса возврата в регистрах и потом проверять какой из них выбрать)


Мы называем эти фрагменты "функциями". Для реализации механизма повторного использования кода, можно прыгнуть из места вызова к началу нужной функции и прыгнуть обратно в место вызова в конце функции. Мы называем прыжок в начало функции - "calling" (CALL), а прыжок обратно из функции - "returning" (RET).

Но для того, чтобы этот фрагмент кода действительно можно было использовать **повторно**, оператор return должен позволять возвращаться в разные места в зависимости от того, откуда мы вызвали функцию.

Мы могли бы запомнить адрес, сохранив значение счётчика в регистре прежде чем прыгнуть в функцию и использовать это значение при возврате. Однако, если сделать это таким образом, функция не сможет вызывать другую функцию, поскольку это перезапишет обратный адрес.

Допустим функция **А** вызывает функцию **В**, которая вызывает функцию **С**. Когда мы хотим вернуться из **С**, нам нужен только адрес для **В**, а в **В** нам нужен только адрес для **А**. В целом, независимо от того, на какую функцию мы посмотрим и как они вложены, последний обратный адрес, который мы добавили всегда понадобится нам первым. Это в точности поведение стека.

Так же рекурсия физически невозможна без стека, так как адрес возврата будет постоянно перезаписываться в регистре.

Например функция возведения в квадрат `fn square(a){return mul(a, a)}` использует вложенный вызов функцию умножения `fn mul(a, b){return a*b}`:
``` 
  address:         asm code:	 # comment:
0b00000000	reg_1=7	         # arg_1=7
0b00000001	reg_4=0b00000011 # return address 
0b00000010	call 0b00000100  # square(7)
0b00000011	...	         #
0b00000100	reg_5=reg_4	 # fn square(a){return mul(a, a)} 
                                    # сохраним теущий адрес возврата в reg_5 так как мы знаем, что по нему будет jump в ф-ции mul 
                                    # и мы не хотим что бы с mul сразу переход на 0b00000011 
                                    # Но проблема в том, что регистров ограниченное количество, что бы сохранять каждый вложенные вызов в 
                                    # новый регистр.
                                    # Невозможность рекурсии: Если функция должна вызвать саму себя, никакой reg_5 не поможет, так как 
                                    # при втором вызове он перезапишется тем же значением, что и в первый раз.
0b00000101	reg_4=0b00001000 # return address
0b00000110	reg_2=reg_1      # arg_2=arg_1
0b00000111	call 0b00001010  # mul(7, 7)
0b00001000	jump reg_5       # return address, вернемся в 0b00000011
0b00001001	...	         #
0b00001010	reg_3=reg_1*reg_2# fn mul(a, b){return a*b}
0b00001011	jump reg_4       # return address
```

> [!IMPORTANT]
>
> Команда CALL увеличивает стек адресом возврата, поэтому следует помнить о переполнении стека (Stack Overflow) если не вызывать после команду RET, которая уменьшает стек.
> Для простых переходов (прыжков) JUMP следует использовать прямое указание адреса, что неувеличивает стек адресов возврата. 

> Задача: 
>
> На этом уровне вам поручено реализовать вызовы функций и возвраты с помощью инструкций **call** и **ret**.
>
> Обратите внимание, что при возврате из функции следует переходить к адресу, который был указан ПОСЛЕ инструкции call, иначе возникнет бесконечный цикл.
> (нужно что бы после счетчика PC стоял инкремент и это уже значение сохранять в стек возвратов)
>
> Инструкция по вызову должна содержать следующее:
> * Добавьте ширину инструкции к значению счетчика и поместите его в стек.
> * Перейти (jump) к адресу функции
> 
> Инструкция **ret** должна выполнять следующие действия:
> * Извлеките (pop) адрес возврата из стека и перейдите (jump) к нему.
>
> Вы можете передавать информацию в функцию и из функции, сохраняя данные в регистры обычным способом. Также имейте в виду, какие регистры функция будет перезаписывать, прежде чем вызывать её.

> [!INFO]
> Раз мы уже реализовали стек в RAM, мы могли бы хранить адреса возврата там. Но тогда команды `PUSH` (для чисел) и `CALL` (для адресов) будут "драться" за одну и ту же память.
>
> Гибрид (аппаратный стек для адресов) нужен просто для того, чтобы вы не думали об адресах возврата вообще. Они на уровне железа сохраняются в отдельной памяти, и вам не нужно тратить такты на вычисление адреса в RAM при каждой команде CALL или RET. Весь ваш стек в RAM остается только для ваших непосредственных данных - аргументов функций, локальных переменных, массивов и расчетов...



Поставим компонент Stack специально для инструкций CALL / RET. А для вычислений и деления использовать SP (Stack Pointer) и RAM.

Нам нужно добавить в схему компонент Stack, но подключить его не к общей шине данных, а к шине адреса и управляющим сигналам CALL / RET


* Вход данных аппаратного стека: Подключим к выходу Program Counter (PC) (*обычно через инкрементатор разрядности инструкции, чтобы сохранить адрес следующей инструкции, а не текущей*).
* Выход данных аппаратного стека: Подключим ко входу Jump Address нашего PC
* Команда CALL (сигнал): Должна одновременно активировать Push на аппаратном стеке и Jump на PC.
    * Схватить текущий номер строки из `Program Counter` (например, 8).
    * Прибавить к нему 4 (адрес следующей команды, т.е. 12).
    * Сохранить это число внутри себя (нажать `Push`).
    * *После этого процессор прыгает на код деления.*
* Команда RET (сигнал): Должна активировать Pop на аппаратном стеке и подать это значение на вход PC для прыжка.
    * Выдать последнее сохраненное число (нажать `Pop`).
    * Отправить это число прямо в `Program Counter`.
    * *Процессор мгновенно оказывается на строке 12.*

Нам доступен `xxxx1xxx` (8) и `xxx1xxxx` бит (16) в opcode (1-й байт инструкции)
* команда CALL `xxxx1xxx` (8)
    * `CALL <address> — Процессор сам делает Stack.push(PC + 4) и PC = <address>`
* команда RET `xxx1xxxx` (16)
    * `RET — Процессор делает PC = Stack.pop()`

С 4-байтовыми инструкциями адрес возврата меняется. Если наша инструкция занимает 4 ячейки памяти, то следующая команда начинается через 4 адреса.
Инкрементатор перед стеком должен делать не `+1`, а `+4`. Поскольку каждая наша инструкция занимает 4 байта, адрес "возврата" всегда равен **текущий адрес + 4**. Нужно: `PC -> Сумматор (+4) -> Stack Input`

### *Текущая архитектура ISA процессора LEG:*

```
[первый байт][второй байт    ][трейтий байт   ][четвертый байт       ]
[что делать ][источник 1     ][источник 2     ][куда девать результат]
[Opcode 8bit][Argument 1 8bit][Argument 2 8bit][Result address 8bit  ]
```

Opcode 8bit:
```
биты xxxxx111 режима ALU, результат его работы в общую шину:
(По умолчанию все команды для ALU. Source данные всегда идут через ALU)
0: ALU xxxxxxx1 #1
1: ALU xxxxxx1x #2
2: ALU xxxxx1xx #4

бит режима CALL, данные из Result address (четвертый байт ISA инс.) в общую шину для PC:
3: CALL 00001000 #8 идентификатор режима

бит режима RET, данные из Result address (четвертый байт ISA инс.) в общую шину для PC:
4: RET 00010000 #16 идентификатор режима

биты xx1xx111 режима COND, данные из Result address (четвертый байт ISA инс.) в общую шину для PC. 
0: COND xxxxxxx1 #1
1: COND xxxxxx1x #2
2: COND xxxxx1xx #4
5: COND xx1xxxxx #32 идентификатор режима, если бит установлен

биты 11xxxxxx режима Immediate values:
6: x1xxxxxx #64 выбор в роли Immediate values Argument 2 (трейтий байт ISA инс.) идентификатор режима, если бит установлен
7: 1xxxxxxx #128 выбор в роли Immediate values Argument 1 (второй байт ISA инс.) идентификатор режима, если бит установлен
```

Argument 1/Argument 2/Result address 8bit:
```
биты xxxx1111 кодируют адрес исчтоника/пунтка назначения:
0: #1 участвует в кодировании восьми адресов (Register 0-5, PC, Input/Output)
1: #2 участвует в кодировании восьми адресов (Register 0-5, PC, Input/Output)
2: #4 участвует в кодировании восьми адресов (Register 0-5, PC, Input/Output)
4: #8 xxxx1xxx только для RAM

Результат декодирования трех младших бит xxxxx111:
0: Register 0
1: Register 1
2: Register 2
3: Register 3
4: Register 4
5: Register 5
6: Counter (PC)
7: Input (для Argument 1 и Argument 2) или Output (для Result address)
```

И мы выбрали `REG_0` для роли `Stack Pointer` (SP) (Указатель стека) для RAM (ОЗУ).

Схема уровня Functions:

![Functions](/Computer-Science-Bookshelf/img/tc/Functions.png)

(p.s. есть ошибка в схеме, в режиме условий COND мы должны перекрыть общую шину на пути к счетчику и открыть шину только для данных Result address)

<details>
<summary>Assembly Editor:</summary>

Проверка механизма CALL и RET. Реализуем функцию удвоения числа и вызовем ее два раза.

```bash
# reg_4 - Stack Pointer (SP)
# reg_0 - RAM Pointer
# reg_1 - arg_1
# reg_2 - result
# CALL opcode `xxxx1xxx` (8)
# RET opcode `xxx1xxxx` (16)
const jump_fn_double 24
const CALL 0b00001000
const RET 0b00010000

# first call fn ----------------------
# set arg_1=5 
0b11000000 #1 opcode ADD 5+0
0b00000101 #2 arg1 source ImVal
0b00000000 #3 arg2 source ImVal
0b00000001 #4 destination reg_1

# CALL fn double
CALL        #1 opcode CALL
0b00000000  #2 arg1 source unused
0b00000000  #3 arg2 source unused
jump_fn_double #4 destination PC

# output result
0b10000000 #1 opcode ADD 0+reg_2
0b00000000 #2 arg1 source ImVal
0b00000010 #3 arg2 source reg_2
0b00000111 #4 destination OUTPUT
# ------------------------------------

# second call fn----------------------
# set arg_1=8 
0b11000000 #1 opcode ADD 8+0
0b00001000 #2 arg1 source ImVal
0b00000000 #3 arg2 source ImVal
0b00000001 #4 destination reg_1

# CALL fn double
CALL        #1 opcode CALL
0b00000000  #2 arg1 source unused
0b00000000  #3 arg2 source unused
jump_fn_double #4 destination PC

# output result
0b10000000 #1 opcode ADD 0+reg_2
0b00000000 #2 arg1 source ImVal
0b00000010 #3 arg2 source reg_2
0b00000111 #4 destination OUTPUT
# -------------------------------------
 
# HALT (!unimplemented opcode)

# jump_fn_double ---------------------
# function implementation 
# fn double(arg_1){reg_2=arg_1+arg_1; return reg_2}
0b00000000 #1 opcode ADD arg_1+arg_1
0b00000001 #2 arg1 source reg_1
0b00000001 #3 arg2 source reg_1
0b00000010 #4 destination reg_2

# RET
RET         #1 opcode RET
0b00000000  #2 arg1 source unused
0b00000000  #3 arg2 source unused
0b00000000  #4 destination unused
# ------------------------------------

```

</details>

---


*Логика разбора Opcode напрашивается в отдельную коробочку Control Unit (CU) — Блок управления*
 * (чтобы код не сломал процессор) Убедиться, что RET и CALL не конфликтуют с COND. Например, что будет, если написать инструкцию, где включены RET и/или CALL в opcode для COND?

Control Unit (CU) — Блок управления:

![Control Unit (CU) — Блок управления](/Computer-Science-Bookshelf/img/tc/Control_Unit.png)
 

Что есть:
* режим COND 
    * данные берет от исчточников (Argument 1 и Argument 2)
    * Result address пишет в общую шину **только** для PC
    * запрещает писать в RAM
* режим CALL
    * не использует источники (Argument 1 и Argument 2)
    * Result address пишет в общую шину **только** для PC 
* режим RET
    * не использует источники (Argument 1 и Argument 2) и Result address
* режим Immediate values
* режим ALU
    * всегда работа через него, что бы скопировать один исчочник в другой нужно выполнять обработку + ADD что занимает 104 тика задержки


Обновленная версия (более понятная)

![Functions Review](/Computer-Science-Bookshelf/img/tc/FunctionsReview.png)


## Расширение opcode инструкции

У нас есть четыре бита `xx1111xx` для кодирования дополнительных инструкций, за вычетом пересечений получим +9 свободных комбинаций. 
Не трогаем два старших бита, чтобы иметь возможность напрямую их использовать (Immediate values) с новыми режимами, более того `xxxxx1xx` 3-й бит конфликтует с режимом COND в новых комбинациях.
Поэтому из двух комбинаций `xxxxx1xx` и `xxxxx0xx` мы получаем одну полноценную для совместного использования с режимом COND. 
 
```
xx0011xx MOV
xx0101xx mix (PUSH,POP,DIV,IRJ)
xx0110xx CRJ (+ изменить 6-й (32) бит режима COND на 1)
xx0111xx CRJ (+ изменить 6-й (32) бит режима COND на 1)
xx1010xx CC для разных условий где 3-й бит 0
xx1011xx CC для разных условий где 3-й бит 1
xx1100xx ICC для разных условий где 3-й бит 0
xx1101xx ICRC для разных условий где 3-й бит 1
xx1110xx ICRC для разных условий где 3-й бит 0
xx1111xx ICC для разных условий где 3-й бит 1
```
  
И зафиксируем режим ALU явно: 
```
xx000xxx
```
(именно эти три бита LOW идентифицируют режим ALU, так как эти биты в HIGH включают режимы COND,RET,CALL которые не нуждаются в вычислениях) 

Для увеличения количества команд, можно взять одну команду `xx0101xx` и закодить ее в четыре (PUSH,POP,RJ,DIV) которые не используют режим COND.

```
128 64|xxxx|2 1
------------
x    x|0101|x x

Биты 128 и 64 это старшие биты для Immediate values, для покрытия всех их пересечений map'им четыре комбинации на одну: 

128 64 1 2|command|
----------|-------|
1 1 1 1   | PUSH (for example 11_0101_11 or 10_0101_11 or 00_0101_11 or 01_0101_11 )
0 1 1 1   |
1 0 1 1   |
0 0 1 1   |
----------|-------|
1 1 0 1   | POP (for example 11_0101_01 or 10_0101_01 or 00_0101_01 or 01_0101_01 )
0 1 0 1   |
1 0 0 1   |
0 0 0 1   |
----------|-------|
1 1 1 0   | IRJ (for example 11_0101_10 ... )
0 1 1 0   |
1 0 1 0   |
0 0 1 0   |
----------|-------|
1 1 0 0   | DIV (for example 11_0101_00 ... )
0 1 0 0   |
1 0 0 0   |
0 0 0 0   |
----------|-------|

```


Для декодирования 4 бит на 10 вариантов, реализуем неполный декодер 4 на 10 `DEC4_10`

![DEC4_10](/Computer-Science-Bookshelf/img/tc/DEC4_10.png)


#### 1. Передача данных

<details>
<summary>Команды MOV/LOAD/STORE:</summary>

* **MOV**: регистр → регистр
* **LOAD**: память → регистр
* **STORE**: регистр → память

```bash
Команда MOV
Нам нужна операция MOV, что бы мы могли записать данные, минуя ALU

Opcode: xx0011xx
Arg 1: Source
Arg 2: Unused
Result addr: Destination

Как работают регистры при одновременном чтении и записи? - Они могут одновременно выдавать своё старое значение и принимать новое.
 
-----------------------------------------
# Сценарий проверки режима MOV/LOAD/STORE
# reg_4 - Stack Pointer (SP)
# reg_0 - RAM Pointer
# CALL opcode 00001000 (8)
# RET opcode 00010000 (16)
# MOV opcode xx0011xx
 
const CALL 0b00001000
const RET 0b00010000
 
# MOV ImVal to reg_1
0b10001100 #1 opcode MOV
0b00000101 #2 arg1 source ImVal
0b00000000 #3 arg2 source unused
0b00000001 #4 destination reg_1

# MOV reg_1 to reg_2
0b00001100 #1 opcode MOV
0b00000001 #2 arg1 source reg_1
0b00000000 #3 arg2 source unused
0b00000010 #4 destination reg_2

# MOV reg_2 to OUTPUT
0b00001100 #1 opcode MOV
0b00000010 #2 arg1 source reg_2
0b00000000 #3 arg2 source unused
0b00000111 #4 destination OUTPUT

# MOV (STORE) 
# reg_2 to RAM 
0b00001100 #1 opcode MOV
0b00000010 #2 arg1 source ImVal
0b00000001 #3 arg2 source unused
0b00001000 #4 destination RAM

# MOV (LOAD)
# RAM to reg_3
0b00001100 #1 opcode MOV
0b00001000 #2 arg1 source RAM
0b00000001 #3 arg2 source unused
0b00000011 #4 destination reg_3

# JMP (MOV)  
0b10001100 #1 opcode MOV source destination
0b00000000 #2 arg1 source ImVal
0b00000001 #3 arg2 Unused
0b00000110 #4 destination PC

```

</details>

---

#### 2. Сравнение и переходы (ветвления)

Без них нет циклов, условий (if, while, for, match), функций.

<details>
<summary>Команда Conditional Jump (CJMP)</summary>

**CJMP** - Условный переход реализован через команду COND с Result Address в PC:

```bash
# CJMP always 
0b11100000  #1 cond IF_EQUAL arg1 == arg2
0b00000000  #2 arg1 source ImVal
0b00000000  #3 arg2 source ImVal
0           #4 destination
```

</details>

<br>

<details>
<summary>Команда JMP</summary>

**JMP** безусловный переход реализован через MOV с Result Address в PC:

```bash
# JMP  
0b10001100 #1 opcode MOV source destination
0b00000000 #2 arg1 source ImVal
0b00000000 #3 arg2 Unused
0b00000110 #4 destination PC
```
 
</details>

---

#### 3. Работа со стеком или вызовами
 
<details>
<summary>CALL и RET</summary>

Проверка механизма **CALL** и **RET**. Реализуем функцию удвоения числа и вызовем ее два раза.

```bash 
# reg_4 - Stack Pointer (SP)
# reg_0 - RAM Pointer
# reg_1 - arg_1
# reg_2 - result
# CALL opcode xxxx1xxx (8)
# RET opcode xxx1xxxx (16)
const jump_fn_double 24
const CALL 0b00001000
const RET 0b00010000

# first call fn ----------------------
# set arg_1=5 
0b11000000 #1 opcode ADD 5+0
0b00000101 #2 arg1 source ImVal
0b00000000 #3 arg2 source ImVal
0b00000001 #4 destination reg_1

# CALL fn double
CALL        #1 opcode CALL
0b00000000  #2 arg1 source unused
0b00000000  #3 arg2 source unused
jump_fn_double #4 destination PC

# output result
0b10000000 #1 opcode ADD 0+reg_2
0b00000000 #2 arg1 source ImVal
0b00000010 #3 arg2 source reg_2
0b00000111 #4 destination OUTPUT
# ------------------------------------

# second call fn----------------------
# set arg_1=8 
0b11000000 #1 opcode ADD 8+0
0b00001000 #2 arg1 source ImVal
0b00000000 #3 arg2 source ImVal
0b00000001 #4 destination reg_1

# CALL fn double
CALL        #1 opcode CALL
0b00000000  #2 arg1 source unused
0b00000000  #3 arg2 source unused
jump_fn_double #4 destination PC

# output result
0b10000000 #1 opcode ADD 0+reg_2
0b00000000 #2 arg1 source ImVal
0b00000010 #3 arg2 source reg_2
0b00000111 #4 destination OUTPUT
# -------------------------------------
 
# HALT (!unimplemented opcode)

# jump_fn_double ---------------------
# function implementation 
# fn double(arg_1){reg_2=arg_1+arg_1; return reg_2}
0b00000000 #1 opcode ADD arg_1+arg_1
0b00000001 #2 arg1 source reg_1
0b00000001 #3 arg2 source reg_1
0b00000010 #4 destination reg_2

# RET
RET         #1 opcode RET
0b00000000  #2 arg1 source unused
0b00000000  #3 arg2 source unused
0b00000000  #4 destination unused
# ------------------------------------

```

</details>


<br>

<details>
<summary>Режим PUSH/POP</summary>

* **PUSH** - добавить в стек
* **POP** - забрать из стека

Мы можем использовать RAM как стек с последних адресов и использовать регистр REG_0 в роли Stack Pointer (SP) (Указатель стека).

На данный момент для реализации поведения стека мы используем две инструкции
* изменение индекса регистра Stack Pointer (SP)
* запись/чтение из стека RAM[SP]

Для команд PUSH/POP необходимо ограничится одной инструкцией т.е. изменение SP должно происходить в железе. А возможность реализовать стек используя REG_0 в роли SP осталась, просто область адресов нужно выбирать выше адресов стека. 

Чтобы не потерять/затереть значение в регистре, в начале вызова функции делают PUSH (кладут значение в RAM «на хранение»), а в конце — POP (забирают обратно).    
Это позволит делать функции с любым количеством аргументов, не боясь, что они перезапишут регистры основной программы.


**Push**

```bash
Opcode: xx010111
Arg 1: Source  
Arg 2: Unused
Result addr: Unused (target RAM) 
```

p.s. свободные два байта можно было бы использовать:
* `Arg 2` при наличии его, по прямому назначению для ALU
* `Result addr` в роли инструкций ALU/Cond

**Pop**

```bash
Opcode: xx010101
Arg 1: 0b00001000 зафиксирован источник RAM  
Arg 2: Unused
Result addr: Destination
```


Реализация (auto-increment/decrement) 
```bash
# test PUSH 

# 2. PUSH
# 1. POP
# -------------
# PUSH
0b10010111 #1 opcode PUSH
0b00000001 #2 arg1 source ImVal
0b00000000 #3 arg2 Unused
0b00000100 #4 destination stack RAM
# -------------
 
# PUSH
0b10010111 #1 opcode PUSH
0b00000010 #2 arg1 source ImVal
0b00000000 #3 arg2 Unused
0b00000100 #4 destination stack RAM
# -------------
 
# PUSH
0b10010111 #1 opcode PUSH
0b00000011 #2 arg1 source ImVal
0b00000000 #3 arg2 Unused
0b00000100 #4 destination stack RAM
# -------------
# POP
0b00010101 #1 opcode POP
0b00001000 #2 arg1 stack RAM
0b00000000 #3 arg2 Unused
0b00000111 #4 destination OUTPUT

# -------------
# POP
0b00010101 #1 opcode POP
0b00001000 #2 arg1 stack RAM
0b00000000 #3 arg2 Unused
0b00000111 #4 destination OUTPUT
 
# POP
0b00010101 #1 opcode POP
0b00001000 #2 arg1 stack RAM
0b00000000 #3 arg2 Unused
0b00000111 #4 destination OUTPUT
 
# JMP  
0b10001100 #1 opcode MOV source destination
0b00000000 #2 arg1 source ImVal
0b00000000 #3 arg2 Unused
0b00000110 #4 destination PC

```


</details>

<br>

<details>
<summary>Режим Conditional CALL (CC):</summary>

**CC** - Условный CALL (Conditional CALL)

Сейчас нет возможности одной инструкцией выполнить проверку условия и выполнить вызов CALL.

Это позволит писать код типа if (a == 10) run_function(). 
* Если arg1 == arg2, мы не просто меняем PC, мы сначала запоминаем, где мы стояли, чтобы потом вернуться (командой RET)
* Нужно в «железе» объединить логику записи в стек и логику проверки условия.

Сейчас для этого нужно две инструкции (сначала COND для прыжка на вызов, а потом сам CALL с запоминанием адреса возврата).


```
Режим CC

Opcode: xx1010xx и xx1011xx
Arg 1: Source  
Arg 2: Source 
Result addr: Destination 

xx1011xx - для условий COND: IF_GREATER_OR_EQUAL,IF_GREATER
xx1010xx - для остальных условий COND
```

```
# Поведение сейчас:
    # always jump
    0b11100000  #1 cond IF_EQUAL arg1 == arg2
    0b00000000  #2 arg1 source ImVal
    0b00000000  #3 arg2 source ImVal
    jump_while #4 destination

    # и тут мы не можем выполнить RET так как адрес возврата никто не запоминал. Адрес возврата запоминается при команде CALL но не COND
    # ...

# Поведение которое нам нужно:
    # always jump
    0b11010100  #1 COND_CALL IF_EQUAL arg1 == arg2
    0b00000000  #2 arg1 source ImVal
    0b00000000  #3 arg2 source ImVal
    jump_while #4 destination

    # метка jump_while и нас сюда перебросил PC
    # мы выполняем RET и нас перебрасывает на следующую инструкцию после места с которого мы пришли по условию COND_CALL
    RET         #1 opcode RET
    0b00000000  #2 arg1 source unused
    0b00000000  #3 arg2 source unused
    0b00000000  #4 destination unused
```

```bash
# Сценарий проверки режима CC
# MOV opcode xx0011xx
# CC opcode xx1010xx для остальных условий COND
# CC opcode xx1011xx для условий COND: IF_GREATER_OR_EQUAL,IF_GREATER
const CALL 0b00001000
const RET 0b00010000
#--------------------
# expected behavior:
# OUTPUT: 6, 7, 8, 6, 7, 8...
#--------------------
label jump_start
 
# test CC IF 0==0
0b11101000   #1 CC
0            #2 arg1 source ImVal
0            #3 arg2 source ImVal
jump_success #4 destination
 
# print 7
0b11000000 #1 ALU ADD
4          #2 arg1 source ImVal
3          #3 arg2 source ImVal
0b00000111 #4 destination OUTPUT

# test CC IF 1==0
0b11101000   #1 CC
1            #2 arg1 source ImVal
0            #3 arg2 source ImVal
jump_success #4 destination

# print 8
0b11000000 #1 ALU ADD
4          #2 arg1 source ImVal
4          #3 arg2 source ImVal
0b00000111 #4 destination OUTPUT

# always jump 
0b11100000  #1 COND IF_EQUAL arg1 == arg2
0b00000000  #2 arg1 source ImVal
0b00000000  #3 arg2 source ImVal
jump_start  #4 destination

label jump_success
# print 6
0b11000000 #1 ALU ADD
4          #2 arg1 source ImVal
2          #3 arg2 source ImVal
0b00000111 #4 destination OUTPUT
 
RET        #1 RET
0b00000000 #2 arg1 source
0b00000000 #3 arg2 source 
0b00000000 #4 destination
```

</details>


<br>


<details>
<summary>Режим Indirect Jump (ICC):</summary>

Про Indirect Jump:
* Косвенный переход (Indirect Jump)
* Сейчас Result address (4-й байт) — это всегда число (адрес в RAM). Но что, если мы захотим прыгнуть по адресу, который лежит в регистре?
* Это нужно для реализации оператора switch (таблицы переходов)
* "Возьми адрес для прыжка не из 4-го байта Result address (Immediate values), а из reg_N, на который этот байт указывает".
    * И эта возможность у нас уже реализованна в режиме ALU (или MOV), мы просто для Result address выбираем PC и результат операции ADD данных из reg_N + 0 перезапишет PC
 

Про Indirect Conditional CALL (ICC):
* Реализуя поведение Indirect Jump вместе с Conditional и CALL, получим Indirect Conditional CALL (**ICC**), что позволит выполнять переход на указанный адрес из регистра/RAM по условию с сохранением адреса возврата.

```bash
Opcode: xx1111xx и xx1100xx 
Arg 1: Source  
Arg 2: Source
Result addr: используется в роли Source для Result addr 

xx1111xx - для условий COND: IF_GREATER_OR_EQUAL,IF_GREATER
xx1100xx - для остальных условий COND
```

```bash
# Сценарий проверки режима ICC

# MOV opcode xx0011xx
# CC opcode xx1011xx для условий COND: IF_GREATER_OR_EQUAL,IF_GREATER
# CC opcode xx1010xx для остальных условий COND
# ICC opcode xx1111xx для условий COND: IF_GREATER_OR_EQUAL,IF_GREATER
# ICC opcode xx1100xx для остальных условий COND
const RET 0b00010000
#--------------------
# expected behavior:
# OUTPUT: 6,7,8,6,7,8...  
#--------------------
 
label jump_start
# MOV 12 to reg_4
0b10001100   #1 MOV
jump_success #2 arg1 source ImVal
0b00000000   #3 arg2 source unused
0b00000100   #4 destination reg_4

# ICC cond IF_EQUAL 44==44
0b11110000 #1 ICC
44         #2 arg1 source ImVal
44         #3 arg2 source ImVal
0b00000100 #4 destination reg_4

# print 7
0b11000000 #1 ALU ADD
5          #2 arg1 source ImVal
2          #3 arg2 source ImVal
0b00000111 #4 destination OUTPUT

0b10001100 jump_second_step 0 6 #jump

label jump_success
# print 6
0b11000000 #1 ALU ADD
4          #2 arg1 source ImVal
2          #3 arg2 source ImVal
0b00000111 #4 destination OUTPUT

RET        #1 RET
0b00000000 #2 arg1 source
0b00000000 #3 arg2 source 
0b00000000 #4 destination

label jump_second_step
# ICC cond IF_EQUAL reg_4==0
0b01110000 #1 ICC
0b00000100 #2 arg1 source reg_4
0b00000000 #3 arg2 source ImVal
0b00000100 #4 destination reg_4
 
# print 8
0b11000000 #1 ALU ADD
4          #2 arg1 source ImVal
4          #3 arg2 source ImVal
0b00000111 #4 destination OUTPUT

# JMP  
0b10001100 #1 MOV source destination
jump_start #2 arg1 source ImVal
0b00000000 #3 arg2 Unused
0b00000110 #4 destination PC
```

Плюс третий декодер источников специально для этого режима, и регистры заменились на прозрачные т.е. всегда LOAD, тогда может и декодеры для выбора источника не нужны?


</details>

<br>

<details>
<summary>Команды Relative Jumps: IRJ, CRJ, ICRC</summary>

Поддержка относительных переходов (Relative Jumps)
* Сейчас в режиме COND Result address — это всегда абсолютный адрес (например, "прыгни на 100"). 
* Это мешает делать перемещаемый код (когда мы можем скопировать кусок программы в другое место памяти, и он продолжит работать).
* Логика: Значение из Result address не просто заменяет PC, а складывается с текущим PC. Это позволит делать короткие циклы типа "прыгни на 5 байт назад".

Про Relative Jumps (**RJ**): (unimplemented)
* RJ относительный переход

p.s. команда RJ не использует старшие биты, что дает возможность закодировать еще +3 команды без условных операций.

Про Indirect Relative Jumps (**IRJ**): 
* IRJ относительный переход


```bash
Opcode: xx010110 
Arg 1: Source  
Arg 2: Source
Result addr: Unused (target ADD Source+Source=PC) 

 
------------------------
# test IRJ:
#--------------------
# expected behavior:
# jump to jump_success
# OUTPUT: 3,3...  
#--------------------
label jump_start
# MOV 6 to reg_3
0b10001100 #1 MOV
6          #2 arg1 source ImVal
0          #3 arg2 unused
0b00000011 #4 destination reg_3
 
# IRJ ADD source+source=PC
0b10010110 #1 IRJ 2+6=8 result addr
2          #2 arg1 source ImVal
0b00000011 #3 arg2 source reg_3
0          #4 unused

# print 5
0b10001100 #1 MOV
5          #2 arg1 source ImVal
0          #3 arg2 Unused
0b00000111 #4 destination OUTPUT
 
label jump_success
# print 3
0b10001100 #1 MOV
3          #2 arg1 source ImVal
0          #3 arg2 Unused
0b00000111 #4 destination OUTPUT

# JMP  
0b10001100 #1 MOV source destination
jump_start #2 arg1 source ImVal
0          #3 arg2 Unused
0b00000110 #4 destination PC
```






Про Conditional Relative Jumps (**CRJ**):
* CRJ условный относительный переход, т.е. мы по условию выполняем прыжок по результату сложения текущего адреса PC с значением из Result addr
* Применяется когда мы хотим просто прыгнуть по условию внутри if или цикла (без возврата) но без сохранения адреса возврата в стеке.

```bash
Opcode: xx0110xx и xx0111xx 
Arg 1: Source  
Arg 2: Source
Result addr: ImVal 

xx0111xx CRJ - для условий COND: IF_GREATER_OR_EQUAL,IF_GREATER
xx0110xx CRJ - для остальных условий COND

 
--------------------------
# test CRJ:
#--------------------
# expected behavior:
# jump to jump_success
# OUTPUT: 6,7,6,7...  
#--------------------
label jump_start
# print 7
0b11000000 #1 ADD
4          #2 arg1 source ImVal
3          #3 arg2 source ImVal
0b00000111 #4 destination OUTPUT
 
# CRJ cond IF_EQUAL 0==0
0b11011000 #1 CRJ +16
0          #2 arg1 source ImVal
0          #3 arg2 source ImVal
16         #4 destination ImVal

# print 5
0b11000000 #1 ADD
0b00000100 #2 arg1 source ImVal
0b00000001 #3 arg2 source ImVal
0b00000111 #4 destination OUTPUT

label jump_second_step
# CRJ cond IF_EQUAL 1==0
0b11011000 #1 CRJ +8
1          #2 arg1 source reg_1
0          #3 arg2 source reg_2
8          #4 destination ImVal

# JMP  
0b10001100 #1 MOV source destination
jump_start #2 arg1 source ImVal
0          #3 arg2 Unused
0b00000110 #4 destination PC

label jump_success
# print 6
0b11000000 #1 ADD
4          #2 arg1 source ImVal
2          #3 arg2 source ImVal
0b00000111 #4 destination OUTPUT

# JMP  
0b10001100 #1 MOV source destination
jump_second_step #2 arg1 source ImVal
0          #3 arg2 Unused
0b00000110 #4 destination PC

```


Про Indirect Conditional Relative CALL (**ICRC**):  
* ICRC Косвенный условный относительный переход с сохранением адреса возврата (т.е нам нужно выполнять RET)

```bash
Opcode: xx1101xx и xx1110xx 
Arg 1: Source  
Arg 2: Source
Result addr: используется в роли Source для Result addr 

xx1101xx ICRC - для условий COND: IF_GREATER_OR_EQUAL,IF_GREATER
xx1110xx ICRC - для остальных условий COND


--------------------------------
 
# test ICRC
#--------------------
# expected behavior:
# jump to jump_success
# OUTPUT: 5,6,7,5,6,7 ...  
#-------------------- 
# RET opcode 00010000 (16)
const RET 0b00010000

label jump_start
# MOV reg_4 20
0b10001100 #1 MOV
20         #2 arg1 source ImVal
0b00000000 #3 arg2 Unused
0b00000100 #4 destination reg_4

# ICRC cond IF_EQUAL 1==1
0b11111000 #1 ICRC +reg_4
1          #2 arg1 source ImVal
1          #3 arg2 source ImVal
0b00000100 #4 destination reg_4

# print 6
0b11000000 #1 ADD
3          #2 arg1 source ImVal
3          #3 arg2 source ImVal
0b00000111 #4 destination OUTPUT

# ICRC cond IF_EQUAL 1==0
0b11111000 #1 ICRC +reg_4
1          #2 arg1 source ImVal
0          #3 arg2 source ImVal
0b00000100 #4 destination reg_4

# print 7
0b11000000 #1 ADD
4          #2 arg1 source ImVal
3          #3 arg2 source ImVal
0b00000111 #4 destination OUTPUT

# JMP  
0b10001100 #1 MOV source destination
jump_start #2 arg1 source ImVal
0b00000000 #3 arg2 Unused
0b00000110 #4 destination PC

label jump_success
# print 5
0b11000000 #1 ADD
4          #2 arg1 source ImVal
1          #3 arg2 source ImVal
0b00000111 #4 destination OUTPUT

# RET
RET #1 RET
0 #2 arg1 source
0 #3 arg2 source 
0 #4 destination

# JMP  
0b10001100 #1 opcode MOV source destination
jump_start #2 arg1 source ImVal
0          #3 arg2 Unused
0b00000110 #4 destination PC

```
  
</details>

<br>

<details>
<summary>Команды ALU MUL,SIN,COS,: (unimplemented)</summary>

Арифметичиеские операции - это основа вычислений, поэтому они должны быть максимально реализованы.

* INC/DEC (увеличение/уменьшение на 1)
* MUL/IMUL (умножение без/со знаком) 
* DIV/IDIV (деление)

Архитектура ALU через флаги.

Флаги — это **специальные биты** (Zero, Carry, Sign и др.), которые автоматически обновляются процессором после арифметических или логических операций.
* Z (Zero) Результат = 0
* N (Negative) Результат < 0 (старший бит)
* C (Carry) Была перенос/заём при сложении/вычитании
* V (Overflow) Переполнение знакового числа

**Зачем они нужны:**

1. **Для условных переходов:** Позволяют выполнять `if/else`, `while` и `for` на основе результата предыдущей операции (например, «если результат равен нулю», «если число отрицательное»).
2. **Для математики:** Позволяют обрабатывать переносы (carry) при вычислениях с числами больше разрядности процессора (например, 16-битное число на 8-битном процессоре).
3. **Экономия ресурсов:** Позволяют принимать логические решения, не сохраняя промежуточные результаты вычислений в регистры.

</details>

<br>

<details>
<summary>Команда HALT: (unimplemented)</summary>

У нас еще не реализована возможность завершить работу процессора, остановив счетчик программ PC или CLOCK - `HALT`  

</details>

<br>

<details>
<summary>Двухуровневая архитектура: (unimplemented)</summary>

Двухуровневая архитектура, с 8-ми битными инструкциями для скорости и 32-х битными для мощности.

</details>


<details>
<summary>Поддержка прерываний (Interrupts): (unimplemented)</summary>

Возможность процессора мгновенно бросить выполнение текущей программы, если нажата клавиша или сработал таймер.

Для многозадачности (переключение программ по таймеру) и мгновенной реакции на ввод-вывод.

</details>


* Компонент stack с автоматическим счетчиком SP:
![StackSP](/Computer-Science-Bookshelf/img/tc/StackSP.png)

* Компонент ALU с DIV:
![ALU_CP2](/Computer-Science-Bookshelf/img/tc/ALU_CP2_DIV.png)

* Компонент DIV:
![DIV](/Computer-Science-Bookshelf/img/tc/DIV.png)

* Компонент [COND_CPU2](turingcomplete_cpu_architecture_2.html#conditionals)

* Компонент Control Unit (CU) — Блок управления
![Control Unit2](/Computer-Science-Bookshelf/img/tc/Control_Unit2.png)
 
* Компонент Decoder 4 to 4 
![DEC4_4](/Computer-Science-Bookshelf/img/tc/DEC4_4.png)

* Компонент Decoder 4 to 10 
![DEC4_10](/Computer-Science-Bookshelf/img/tc/DEC4_10.png)

* Компонент Stack адресов возврата
![Stack](/Computer-Science-Bookshelf/img/tc/Stack2v.png)

* Компонент Bus Master
![Bus_Master](/Computer-Science-Bookshelf/img/tc/Bus_Master.png)

* Компонент MUX8Buf
![MUX8Buf](/Computer-Science-Bookshelf/img/tc/MUX8Buf_upg.png)

LEG CPU2:

![Functions_CPU2](/Computer-Science-Bookshelf/img/tc/Functions_CPU2.png)
 

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