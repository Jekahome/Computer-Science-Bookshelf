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
const start_save_to_ram 4
const start_output_ram 20
const start_continue_output 24
const iterations 32
# reg_0 SP (Stack Pointer)

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
const start_while 4  
const start_pop 28

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
# start_while
 
# take INPUT
0b10000000 #1 opcode ADD 0 INPUT
0b00000000 #2 arg1 source ImVal
0b00000111 #3 arg2 source INPUT
0b00000001 #4 destination reg_1

# cond IF_EQUAL 0 == reg_1
0b10100000 #1 opcode cond IF_EQUAL 
0b00000000 #2 arg1 source ImVal
0b00000001 #3 arg2 source reg_1
start_pop #4 destination jump

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
start_while #4 destination


# start_pop------------------
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
start_while #4 destination

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


> Задача: 
>
> На этом уровне вам поручено реализовать вызовы функций и возвраты с помощью инструкций **call** и **ret**.
>
> Обратите внимание, что при возврате из функции следует переходить к адресу, который был указан ПОСЛЕ инструкции call, иначе возникнет бесконечный цикл.
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

Текущая **Архитектура ISA процессора LEG:**

```
[первый байт][второй байт    ][трейтий байт   ][четвертый байт       ]
[что делать ][источник 1     ][источник 2     ][куда девать результат]
[Opcode 8bit][Argument 1 8bit][Argument 2 8bit][Result address 8bit  ]
```

Opcode 8bit:
```
биты xxxxx111 режима ALU, результат его работы в общую шину:
0: ALU xxxxxxx1 #1
1: ALU xxxxxx1x #2
2: ALU xxxxx1xx #4

бит режима CALL, данные из Result address (четвертый байт ISA инструкции) в общую шину для PC:
3: CALL xxxx1xxx #8

бит режима RET, данные из Result address (четвертый байт ISA инструкции) в общую шину для PC:
4: RET xxx1xxxx #16

биты xx1xx111 режима COND, данные из Result address (четвертый байт ISA инструкции) в общую шину для PC. 
0: COND xxxxxxx1 #1
1: COND xxxxxx1x #2
2: COND xxxxx1xx #4
5: COND xx1xxxxx #32

биты 11xxxxxx режима Immediate values:
6: x1xxxxxx #64 выбор в роли Immediate values Argument 2 (трейтий байт ISA инструкции)
7: 1xxxxxxx #128 выбор в роли Immediate values Argument 1 (второй байт ISA инструкции)
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


![Functions](/Computer-Science-Bookshelf/img/tc/Functions.png)

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
const start_fn_double 24
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
start_fn_double #4 destination PC

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
start_fn_double #4 destination PC

# output result
0b10000000 #1 opcode ADD 0+reg_2
0b00000000 #2 arg1 source ImVal
0b00000010 #3 arg2 source reg_2
0b00000111 #4 destination OUTPUT
# -------------------------------------
 
# HALT (!unimplemented opcode)

# start_fn_double ---------------------
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


*Логика разбора Opcode напрашивается в отдельную коробочку*
 * (чтобы код не сломал процессор) Убедиться, что RET и CALL не конфликтуют с COND. Например, что будет, если написать инструкцию, где включены RET и/или CALL в opcode для COND?

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



```
1. У нас еще не реализована возможность завершить работу процессора, остановив счетчик программ PC или CLOCK - `HALT` (можно реализовать через 5-1 бит или 0 в Opcode)

2. Поддержка относительных переходов (Relative Jumps)
    Сейчас твой Result address — это всегда абсолютный адрес (например, "прыгни на 100"). Это мешает делать перемещаемый код (когда ты можешь скопировать кусок программы в другое место памяти, и он продолжит работать).

    * Улучшение: Добавь бит режима Relative.

    * Логика: Если бит активен, то значение из Result address не просто заменяет PC, а складывается с текущим PC. Это позволит делать короткие циклы типа "прыгни на 5 байт назад".

3. Условный CALL (Conditional CALL)
    Сейчас нет возможности одной инструкцией выполнить проверку условия и выполнить вызов CALL.
    Это позволит писать код типа if (a == 10) run_function(). 
    Сейчас тебе для этого нужно две инструкции (сначала COND для прыжка на вызов, а потом сам CALL с запоминанием адреса возврата).

4. Косвенный переход (Indirect Jump)
    Сейчас твой Result address (4-й байт) — это всегда число (адрес в RAM). Но что, если ты захочешь прыгнуть по адресу, который лежит в регистре?
    Это нужно для реализации оператора switch (таблицы переходов)
    "Возьми адрес для прыжка не из 4-го байта Result address, а из reg_N, на который этот байт указывает".

5. Бит записи (Write Enable) для ALU
    У тебя сейчас ALU всегда пишет результат в Result address. Но иногда нам нужно выполнить операцию только ради того, чтобы увидеть результат на шине (например, вывести в OUTPUT), не затирая регистры. 
    Если ты добавишь возможность отключать запись в регистр/память, твоя архитектура станет еще гибче.



6. Расширение работы со Стеком (Push / Pop для данных)
    Это концепция «Программного стека». Сейчас у тебя есть «Железный стек» (только для адресов возврата), но программисту часто нужно спрятать в безопасное место не адрес, а данные (значения регистров).
    Представь ситуацию:
        В reg_1 лежит очень важное число (например, баланс игрока).
        Тебе нужно вызвать функцию double(), но она внутри себя тоже использует reg_1 для своих расчетов.
        Беда: Функция затрет твой баланс!
    Чтобы этого не случилось, в начале функции делают PUSH (кладут значение в RAM «на хранение»), а в конце — POP (забирают обратно).    

    У тебя есть аппаратный стек для CALL/RET (адреса возврата), но программисту часто нужно сохранить значения регистров (например, перед вызовом функции спасти reg_1).

    Улучшение: Добавь команды PUSH и POP для обычных регистров, используя твой REG_0 (SP) и RAM.
    Как это работает:
        PUSH reg_1: RAM[SP] = reg_1, затем SP = SP + 1.
        POP reg_1: SP = SP - 1, затем reg_1 = RAM[SP].
        Зачем: Это позволит делать функции с любым количеством аргументов, не боясь, что они перезапишут регистры основной программы.


    Если ты хочешь сделать жизнь программиста легче, не убирая аппаратный стек, добавь в свою "коробочку" (Декодер) автоматику для RAM:
        Бит AUTO_INC: Если он включен вместе с записью в RAM, то reg_0 сам прибавляет +1 после операции.
        Бит AUTO_DEC: Если он включен вместе с чтением из RAM, то reg_0 сам вычитает -1 перед операцией.
        Это позволит тебе превратить ADD reg_1, 0 -> RAM[reg_0] в полноценный PUSH, но при этом не смешивать его с адресами возврата.

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