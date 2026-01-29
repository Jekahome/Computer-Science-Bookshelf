# CPU Architecture

* [Arithmetic Engine](#arithmetic-engine)
* [Registers](#registers)
* [Instruction Decoder](#instruction-decoder)
* [Calculations](#calculations)
* [Program](#program)
* [Conditions](#conditions)
* [Immediate values](#immediate-values)
* [Turing Complete](#turing-complete)

## Arithmetic Engine

> [!TIP]
> Разблокирует фабрику компонентов, что позволит создавать собственные компоненты для уровней CPU Architecture.
 
Arithmetic Logic Unit (**ALU**) Арифметико-логическое устройство

Коротко что это значит:
* Arithmetic — сложение, вычитание, инкремент, декремент
* Logic — AND, OR, XOR, NOT, сравнения
* Unit — отдельный функциональный блок процессора

> Задача: 
>
> Добавить еще два варианта в схему [Logic Engine](turingcomplete_arithmetic.html#logic-engine):
> * ADD для сложения двух восьмибитных чисел
> * SUB для вычитания двух восьмибитных чисел
>
> Для 8 комбинаций, нам хватит первых трех младших битов
> ```
> V| OPCODE :
> -|---------------
> 0| xxxxx000   OR
> 1| xxxxx001   NAND
> 2| xxxxx010   NOR
> 3| xxxxx011   AND
> 4| xxxxx100   ADD
> 5| xxxxx101   SUB
> ```
>
> [x x x x x D2 D1 D0 ]
>

Реализация ALU через NAND.

![Arithmetic Engine](/Computer-Science-Bookshelf/img/tc/Arithmetic_Engine.png)

Схема созданного вами компонента определяет его функционал, а планировка определяет форму. Имеено по этой причине, нельзя было двигать красные компоненты на схеме, так как они участвуют в форме нового компонента. 

Реализация ALU через OR.

![New_Component_ALU](/Computer-Science-Bookshelf/img/tc/New_Component_ALU.png)
 
#### Circuit Simulation: ALU

Необходимые компоненты схемы ALU:
* [3 Bit Decoder](turingcomplete_arithmetic.html#3-bit-decoder)
* [8 Bit Switch (SWC)](turingcomplete_memory#bit-switch-tri-state-buffer)
* [Adding Bytes](turingcomplete_arithmetic.html#adding-bytes)
* [8 bit NEG](turingcomplete_arithmetic.html#signed-negator)
* [8 bit NOT](turingcomplete_arithmetic.html#byte-not)
* [8 bit NAND](turingcomplete_arithmetic.html#logic-engine) или OR (но расход mosfet транзисторов больше)


Проверка:
```
Input 1:     00000100 # 4

Input 2:     00000011 # 3
---------------------------
Instructions xxxx0000 # OR
Output       00000111 # 7

Instructions xxxx0001 # NAND
Output       11111111 # 255 или -1 

Instructions xxxx0010 # NOR
Output       11111000 # 248 или -8

Instructions xxxx0011 # AND
Output       00000000 # 0

Instructions xxxx0100 # ADD
Output       00000111 # 7  


Instructions xxxx0101 # SUB
Output       00000001 # 1      
  
```

<div class="sim-wrapper" data-circuit-id="25">
  <button class="sim-fullscreen-btn" data-circuit-id="25">⛶</button>
  <iframe 
      id="25"
      data-circuit-id="25"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=true&startCircuit=/turingcomplete/25_arithmetic_engine_alu.txt"
      loading="lazy">
  </iframe>
</div> 

---

## Registers

Пришло время создать свой главный проект, реализующий компьютерную архитектуру OVERTURE. Это будет настоящая машина, полная по Тьюрингу, истинный компьютер во всех смыслах!

Этот уровень — реализация инструкции MOV между регистрами, где адреса источника и назначения закодированы в 8-ми битной инструкции.

> Задача: Создайте схему распределения/получения данных соответствующую спецификации 
>
> На схеме вам дается:
> * 8-ми битный компонент для получения инструкции 
> * 5 ячеек памяти в виде 8-ми битных регистров
> * внешний вход 8-ми битных данных
> * внешний выход 8-ми битных данных
>
> На этом уровне вам нужно создать схему которая может копировать из источника в место назначения.
> 
> Байт инструкции на этом уровне определяет источник Source и место назначения Destination.
> 
> Инструкция — это 8 бит, но используются 6 из них:
>
> ```
> [ x  x | S2  S1  S0 | D2  D1  D0  ]
> [ x  x | Source     | Destination ]
> ```
>

![REG](/Computer-Science-Bookshelf/img/tc/REG.png)

> В качестве источника и получателя может выступать один из 6 регистров, которые мы обозначим соответственно REG 0,... REG 5.
>
> Кроме того, этот уровень имеет отдельный входной компонент, который может быть источником, и выходной компонент, который может быть пунктом назначения.

Адреса для источника Source:
```
OPCODE Source:

S2 S1 S0
--------------
0  0  0  REG 0
0  0  1  REG 1
0  1  0  REG 2
0  1  1  REG 3
1  0  0  REG 4
1  0  1  REG 5
1  1  0  INPUT # использовать внешний вход
1  1  1  UNUSED
```

Адреса для назначения Destination:
```
OPCODE Destination:

D2 D1 D0 
--------------
0  0  0  REG 0
0  0  1  REG 1
0  1  0  REG 2
0  1  1  REG 3
1  0  0  REG 4
1  0  1  REG 5
1  1  0  OUTPUT # использовать внешний выход
1  1  1  UNUSED
```

Например такие инструкции: 
* `xx000110 => Source 000 (REG 0) и Destination 110 (OUTPUT)` что означает взять данные из `REG 0` и переслать их во внешний `OUTPUT`
* `xx011001 => Source 011 (REG 3) и Destination 001 (REG 1)` что означает взять данные из `REG 3` и переслать их в `REG 1`
* `xx110110 => Source 110 (INPUT) и Destination 110 (OUTPUT)` что означает взять данные из внешнего `INPUT` и переслать их во внешний `OUTPUT`

![Registers](/Computer-Science-Bookshelf/img/tc/Registers.png)


#### Circuit Simulation: Registers

Необходимые компоненты:
* [Register 8 bit](turingcomplete_memory.html#saving-bytes)
* [3 Bit Decoder](turingcomplete_arithmetic.html#3-bit-decoder)
* [8 Bit Switch (SWC)](turingcomplete_memory#bit-switch-tri-state-buffer)

 

Проверка:
```
INPUT: 00000111 # 7

# tick 1
Instruction: 00110000
    Source 110 (INPUT)
    Destination 000 (REG 0)

# tick 2
Instruction: 00000110
    Source 000 (REG 0)
    Destination 110 (OUTPUT)

```

<div class="sim-wrapper" data-circuit-id="30">
  <button class="sim-fullscreen-btn" data-circuit-id="30">⛶</button>
  <iframe 
      id="30"
      data-circuit-id="30"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=true&startCircuit=/turingcomplete/30_registers.txt"
      loading="lazy">
  </iframe>
</div> 


---

## Instruction Decoder

Декодер 2 на 4

Схема, которую вы построили на уровне Registers, может копировать значения между регистрами, в то время как "Арифметический Блок" (ALU) может выполнять различные операции над 2-мя входами. 

Но вам нужно делать и то и другое в одной и той же схеме. 

> Задача: Постройте "декодер", который будет определять в каком режиме наш компьютер находится, основываясь на 2-х битах которые вы до сих пор не использовали.

Что бы отличать 4 инструкции MODE, они будут кодировать первыми (старшими) двумя битами:

```
[ M1 M0 | S2 S1 S0 | D2  D1  D0  ]
[ MODE  | Source   | Destination ]

OPCODE MODE:
---------------------------------
00xxxxxx Immediate values
01xxxxxx вычислить (ALU) CALC
10xxxxxx копировать COPY
11xxxxxx условия Conditions
```

Определите текущий режим работы по входным данным, затем отправьте 1 на соответствующий выход.

![Instruction Decoder](/Computer-Science-Bookshelf/img/tc/Decoder_2_to_4.png)

Или используйте Byte Splitter для получения доступа к исходным битам, а затем 3-битный декодер для декодирования старших бит на четыре выхода.

![Instruction Decoder](/Computer-Science-Bookshelf/img/tc/Turing_Complete_Instruction_Decoder_2_to_4.png)


<div class="sim-wrapper" data-circuit-id="31">
  <button class="sim-fullscreen-btn" data-circuit-id="31">⛶</button>
  <iframe 
      id="31"
      data-circuit-id="31"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=true&startCircuit=/turingcomplete/31_decoder_2_to_4.txt"
      loading="lazy">
  </iframe>
</div> 


---

## Calculations

Пришло время объединить "Арифметический Блок" (ALU) который вы сделали ранее [Arithmetic Engine](#arithmetic-engine) и схему регистров [Registers](#registers). 

Вычислительная схема была сохранена (ALU) и декодер 2 на 4 (DEC) в заводе компонентов и теперь могут быть добавлены как компоненты в схему. 

Если вы забыли какое соединение что делает, посмотрите на схему в заводе компонентов.
 
> Задача: 
> 
> Используйте декодер (Декодер 2 на 4) который вы построили ранее для OPCODE MODE, чтобы понять, что делать с регистрами `REG1, REG2, REG3`: 
> * копировать COPY (OPCODE = 10) 
> * или вычислять (ALU) CALC (OPCODE = 01). 
>
> (для упрощения уровня, в режиме CALC регистры заданы жестко т.е. конкретные, их пока не адресуем) 
>
> Вот 4 режима (напоминание):
> ```
> [ M1 M0 | S2 S1 S0 | D2  D1  D0  ]
> [ MODE  | Source   | Destination ]
>
> OPCODE MODE:
> ---------------------------------------
> 0 0 Immediate values (не нужен сейчас)
> 0 1 вычислить (ALU) CALC
> 1 0 копировать COPY
> 1 1 условия Conditions (не нужен сейчас)
> ```
>  
> Когда вы находитесь в режиме вычислений (ALU) CALC, используйте `REG 1` и `REG 2` в качестве Source (входов), а Destination (результат) сохраните в `REG 3`.
> 
> Не забудьте что у декодера 3 на 8 есть выключающий бит при HIGH сигнале, так как нам в режиме CALC не нужны из OPCODE инструкции Source и Destination, ведь мы жестко фиксировали работу с регистрами `REG1, REG2, REG3`.
> Но в режиме COPY, декодеры 3 на 8 должны использоваться для адресации регистров либо внешнего входа/выхода.
>
> Еще, в этом уровне, модифицированные регистры, у них есть дополнительно ножка "Always output", т.е. всегда можно его прочитать без необходимости выставлять сигнал HIGH на pin Load.

> Так как режим MODE CALC использует младшие 6 бит не по назначению, тогда нужно убедится, что они не влияют на шину и регистры, для этого нужно что бы декодеры Source и Destination не декодировали инструкцию, а выдавали такие сигналы которые отключают регистры от входа и выхода. 
>

Для двух старших бит можно использовать декодер 2 на 4 который мы построили ранее на уровне [Instruction Decoder](#instruction-decoder) он тоже принимает 8 бит но реагирует только на первые два старших, в принципе его можно переделать, для этого нам есть завод компонентов, заменим 8-ми битный вход на 2-х битный. Или можно взять еще один декодер 3 на 8 (избыточно). 

Наш компонет ALU принимает 8 бит инструкцию, входом для нее будет служить младшие биты которые мы используем для адресации выхода Destination: `D2  D1  D0`

(COPY тут не реализовано)
![Calculations](/Computer-Science-Bookshelf/img/tc/Calculations1.png)

Либо можно вообще не заниматься двойным передокированием младших битов инструкции `D2  D1  D0`, и напрямую пустить в ALU все 8 бит инструкции. 

![Calculations](/Computer-Science-Bookshelf/img/tc/Calculations2.png)

А вот, переделанный декодер с двумя входными пинами вместо 8

![2bit_input_decoder_2_to_4](/Computer-Science-Bookshelf/img/tc/2bit_input_decoder_2_to_4.png)

#### Circuit Simulation: Calculations
 
Необходимые компоненты:
* [ALU](turingcomplete_cpu_architecture.html#arithmetic-engine)
* [8 bit Multuplexers (MUX) Tri-state buffer](turingcomplete_memory.html#bit-switch-tri-state-buffer)

В режиме CALC (ALU), Conditions и Immediate Values декодеры для Source/Destination не нужны
 

---

## Program

Компонент ввода инструкций был удален. Он заменен компонентом программирования Program Memory. 

Раньше мы использовали ручной ввод инструкции (Instruction Input) прямой передачей 8 бит. 
Теперь нам выдают Program Memory (ПЗУ **ROM** (Read-Only Memory)) который содержит в своей постоянной памяти 8-ми битные инструкции.
Что бы их достать нам нужно их поочередно брать по индексу, для этого нам нужно использовать [8-ми битный счетчик](turingcomplete_memory.html#counter) который будет с каждым тактом увеличиваться и мы будем получать следующую инструкцию из блока Program Memory.

В блоке Program Memory инструкции хранятся в определенной последовательности.
Это и есть последовательное выполнение программы, код языка программирования выполняется последовательно.

![Program](/Computer-Science-Bookshelf/img/tc/Program.png)

#### Circuit Simulation: 8-bin decimal display

Для отображения 8-ми битного десятичного числа потребуется 8 бит что-бы кодировать 256 значений от 0-255. 
Воспользуемся методом BCD (Double Dabble), закодируем каждую комбинацию из 8-ми бит в 12 бит, по 4 бита на дисплей.

Например значение `00001010=000000010000` означает число 10 в двоичном виде `00001010` это адрес на входе, а значение `000000010000` это 12 бит которые будут распределены на три 4-х битных дисплея `0000 0001 0000` т.е. для дисплея для десятков мы передаем `1` и на трех дисплея мы видим `010`

Формат данных для пользовательского компонента имитирующего RAM:
<details>
<summary>address 8 bit:BCD 12 bit</summary> 

```
00000000=000000000000
00000001=000000000001
00000010=000000000010
00000011=000000000011
00000100=000000000100
00000101=000000000101
00000110=000000000110
00000111=000000000111
00001000=000000001000
00001001=000000001001
00001010=000000010000
00001011=000000010001
00001100=000000010010
00001101=000000010011
00001110=000000010100
00001111=000000010101
00010000=000000010110
00010001=000000010111
00010010=000000011000
00010011=000000011001
00010100=000000100000
00010101=000000100001
00010110=000000100010
00010111=000000100011
00011000=000000100100
00011001=000000100101
00011010=000000100110
00011011=000000100111
00011100=000000101000
00011101=000000101001
00011110=000000110000
00011111=000000110001
00100000=000000110010
00100001=000000110011
00100010=000000110100
00100011=000000110101
00100100=000000110110
00100101=000000110111
00100110=000000111000
00100111=000000111001
00101000=000001000000
00101001=000001000001
00101010=000001000010
00101011=000001000011
00101100=000001000100
00101101=000001000101
00101110=000001000110
00101111=000001000111
00110000=000001001000
00110001=000001001001
00110010=000001010000
00110011=000001010001
00110100=000001010010
00110101=000001010011
00110110=000001010100
00110111=000001010101
00111000=000001010110
00111001=000001010111
00111010=000001011000
00111011=000001011001
00111100=000001100000
00111101=000001100001
00111110=000001100010
00111111=000001100011
01000000=000001100100
01000001=000001100101
01000010=000001100110
01000011=000001100111
01000100=000001101000
01000101=000001101001
01000110=000001110000
01000111=000001110001
01001000=000001110010
01001001=000001110011
01001010=000001110100
01001011=000001110101
01001100=000001110110
01001101=000001110111
01001110=000001111000
01001111=000001111001
01010000=000010000000
01010001=000010000001
01010010=000010000010
01010011=000010000011
01010100=000010000100
01010101=000010000101
01010110=000010000110
01010111=000010000111
01011000=000010001000
01011001=000010001001
01011010=000010010000
01011011=000010010001
01011100=000010010010
01011101=000010010011
01011110=000010010100
01011111=000010010101
01100000=000010010110
01100001=000010010111
01100010=000010011000
01100011=000010011001
01100100=000100000000
01100101=000100000001
01100110=000100000010
01100111=000100000011
01101000=000100000100
01101001=000100000101
01101010=000100000110
01101011=000100000111
01101100=000100001000
01101101=000100001001
01101110=000100010000
01101111=000100010001
01110000=000100010010
01110001=000100010011
01110010=000100010100
01110011=000100010101
01110100=000100010110
01110101=000100010111
01110110=000100011000
01110111=000100011001
01111000=000100100000
01111001=000100100001
01111010=000100100010
01111011=000100100011
01111100=000100100100
01111101=000100100101
01111110=000100100110
01111111=000100100111
10000000=000100101000
10000001=000100101001
10000010=000100110000
10000011=000100110001
10000100=000100110010
10000101=000100110011
10000110=000100110100
10000111=000100110101
10001000=000100110110
10001001=000100110111
10001010=000100111000
10001011=000100111001
10001100=000101000000
10001101=000101000001
10001110=000101000010
10001111=000101000011
10010000=000101000100
10010001=000101000101
10010010=000101000110
10010011=000101000111
10010100=000101001000
10010101=000101001001
10010110=000101010000
10010111=000101010001
10011000=000101010010
10011001=000101010011
10011010=000101010100
10011011=000101010101
10011100=000101010110
10011101=000101010111
10011110=000101011000
10011111=000101011001
10100000=000101100000
10100001=000101100001
10100010=000101100010
10100011=000101100011
10100100=000101100100
10100101=000101100101
10100110=000101100110
10100111=000101100111
10101000=000101101000
10101001=000101101001
10101010=000101110000
10101011=000101110001
10101100=000101110010
10101101=000101110011
10101110=000101110100
10101111=000101110101
10110000=000101110110
10110001=000101110111
10110010=000101111000
10110011=000101111001
10110100=000110000000
10110101=000110000001
10110110=000110000010
10110111=000110000011
10111000=000110000100
10111001=000110000101
10111010=000110000110
10111011=000110000111
10111100=000110001000
10111101=000110001001
10111110=000110010000
10111111=000110010001
11000000=000110010010
11000001=000110010011
11000010=000110010100
11000011=000110010101
11000100=000110010110
11000101=000110010111
11000110=000110011000
11000111=000110011001
11001000=001000000000
11001001=001000000001
11001010=001000000010
11001011=001000000011
11001100=001000000100
11001101=001000000101
11001110=001000000110
11001111=001000000111
11010000=001000001000
11010001=001000001001
11010010=001000010000
11010011=001000010001
11010100=001000010010
11010101=001000010011
11010110=001000010100
11010111=001000010101
11011000=001000010110
11011001=001000010111
11011010=001000011000
11011011=001000011001
11011100=001000100000
11011101=001000100001
11011110=001000100010
11011111=001000100011
11100000=001000100100
11100001=001000100101
11100010=001000100110
11100011=001000100111
11100100=001000101000
11100101=001000101001
11100110=001000110000
11100111=001000110001
11101000=001000110010
11101001=001000110011
11101010=001000110100
11101011=001000110101
11101100=001000110110
11101101=001000110111
11101110=001000111000
11101111=001000111001
11110000=001001000000
11110001=001001000001
11110010=001001000010
11110011=001001000011
11110100=001001000100
11110101=001001000101
11110110=001001000110
11110111=001001000111
11111000=001001001000
11111001=001001001001
11111010=001001010000
11111011=001001010001
11111100=001001010010
11111101=001001010011
11111110=001001010100
11111111=001001010101
```

</details>

**Особенность реализации RAM (именно в симуляторе circuitjs)**

RAM:
* A0, A1, A2 — это адресные входы (куда подаётся адрес ячейки)
* D0, D1, D2... D7 — это выходы данных (откуда читается содержимое ячейки)
 

| Сигнал | Значение | Режим работы |
|--------|----------|--------------|
| **WE = 1** | HIGH | **ЧТЕНИЕ** — данные из памяти выводятся на выходы D0-D7 | 
| **WE = 0** | LOW | **ЗАПИСЬ** — данные с входов D0-D7 записываются в память |
| **OE = 1** | HIGH | **Разрешение выхода** — данные появляются на выходах |
| **OE = 0** | LOW | **Высокий импеданс** — выходы отключены (Z-состояние) |


RAM в симуляторе не понимает, что в поле данных находится формат BCD, RAM просто видит 12 бит `000000010000` как одно целое число и автоматически преобразует в десятичное представление, получается `10000` что соответвует значению `16` в десятичной системе. 

Поэтому если вручную вводить в поле данных RAM, нужно преобразовывать требуемый код BCD в его представление числа в десятичной форме. 


```
# ввод данных в RAM 
0:0
1:1
2:2
3:3
4:4
5:5
6:6
7:7
8:8
9:9
10:16

# симулятор перекодирует данные RAM в свой формат. 
# Данные распределены по строкам из 8-ми значений. И позиция числа соответвует его номеру адреса.
0: 0 1 2 3 4 5 6 7
8: 8 9 16
```


<details>
<summary>Код для создания данных RAM</summary> 


```rust
use std::fs::File;
use std::io::Write;
/*
Данные для RAM.
Индекс массива это адрес для RAM.
Данные представлены в десятичном формате, будут преобразованы в 12-ти битные значения для логики BCD.
*/
fn main() -> std::io::Result<()> {
    // --- Наглядный массив чисел, который можно редактировать вручную ---
    let numbers: [u8; 256] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
        10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
        30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
        40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
        50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
        60, 61, 62, 63, 64, 65, 66, 67, 68, 69,
        70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
        80, 81, 82, 83, 84, 85, 86, 87, 88, 89,
        90, 91, 92, 93, 94, 95, 96, 97, 98, 99,
        100, 101, 102, 103, 104, 105, 106, 107, 108, 109,
        110, 111, 112, 113, 114, 115, 116, 117, 118, 119,
        120, 121, 122, 123, 124, 125, 126, 127, 128, 129,
        130, 131, 132, 133, 134, 135, 136, 137, 138, 139,
        140, 141, 142, 143, 144, 145, 146, 147, 148, 149,
        150, 151, 152, 153, 154, 155, 156, 157, 158, 159,
        160, 161, 162, 163, 164, 165, 166, 167, 168, 169,
        170, 171, 172, 173, 174, 175, 176, 177, 178, 179,
        180, 181, 182, 183, 184, 185, 186, 187, 188, 189,
        190, 191, 192, 193, 194, 195, 196, 197, 198, 199,
        200, 201, 202, 203, 204, 205, 206, 207, 208, 209,
        210, 211, 212, 213, 214, 215, 216, 217, 218, 219,
        220, 221, 222, 223, 224, 225, 226, 227, 228, 229,
        230, 231, 232, 233, 234, 235, 236, 237, 238, 239,
        240, 241, 242, 243, 244, 245, 246, 247, 248, 249,
        250, 251, 252, 253, 254, 255
    ];

    // --- преобразования числа в 12-битный BCD (десятичное число для RAM)) ---
    fn to_bcd_decimal(n: u8) -> u16 {
        let hundreds = n / 100;
        let tens = (n / 10) % 10;
        let ones = n % 10;
        ((hundreds as u16) << 8) | ((tens as u16) << 4) | (ones as u16)
    }

    // --- Генерация текста в формате RAM симулятора circuitjs ---
    for (addr, &num) in numbers.iter().enumerate() {
        let bcd_value = to_bcd_decimal(num);
        println!("{}:{}", addr, bcd_value);
    }
/*
    // --- Создание бинарного файла ---
    let mut file = File::create("number.bin")?;
    for &num in numbers.iter() {
        let bcd_value = to_bcd_decimal(num);
        // Записываем как 16-битное слово (little-endian)
        file.write_all(&bcd_value.to_le_bytes())?;
    }
*/
    Ok(())
}
```
 
</details>


Формат данных для RAM:
<details>
<summary>address (decimal form):BCD (12-bit number in decimal form)</summary> 

number:address
```
0:0
1:1
2:2
3:3
4:4
5:5
6:6
7:7
8:8
9:9
10:16
11:17
12:18
13:19
14:20
15:21
16:22
17:23
18:24
19:25
20:32
21:33
22:34
23:35
24:36
25:37
26:38
27:39
28:40
29:41
30:48
31:49
32:50
33:51
34:52
35:53
36:54
37:55
38:56
39:57
40:64
41:65
42:66
43:67
44:68
45:69
46:70
47:71
48:72
49:73
50:80
51:81
52:82
53:83
54:84
55:85
56:86
57:87
58:88
59:89
60:96
61:97
62:98
63:99
64:100
65:101
66:102
67:103
68:104
69:105
70:112
71:113
72:114
73:115
74:116
75:117
76:118
77:119
78:120
79:121
80:128
81:129
82:130
83:131
84:132
85:133
86:134
87:135
88:136
89:137
90:144
91:145
92:146
93:147
94:148
95:149
96:150
97:151
98:152
99:153
100:256
101:257
102:258
103:259
104:260
105:261
106:262
107:263
108:264
109:265
110:272
111:273
112:274
113:275
114:276
115:277
116:278
117:279
118:280
119:281
120:288
121:289
122:290
123:291
124:292
125:293
126:294
127:295
128:296
129:297
130:304
131:305
132:306
133:307
134:308
135:309
136:310
137:311
138:312
139:313
140:320
141:321
142:322
143:323
144:324
145:325
146:326
147:327
148:328
149:329
150:336
151:337
152:338
153:339
154:340
155:341
156:342
157:343
158:344
159:345
160:352
161:353
162:354
163:355
164:356
165:357
166:358
167:359
168:360
169:361
170:368
171:369
172:370
173:371
174:372
175:373
176:374
177:375
178:376
179:377
180:384
181:385
182:386
183:387
184:388
185:389
186:390
187:391
188:392
189:393
190:400
191:401
192:402
193:403
194:404
195:405
196:406
197:407
198:408
199:409
200:512
201:513
202:514
203:515
204:516
205:517
206:518
207:519
208:520
209:521
210:528
211:529
212:530
213:531
214:532
215:533
216:534
217:535
218:536
219:537
220:544
221:545
222:546
223:547
224:548
225:549
226:550
227:551
228:552
229:553
230:560
231:561
232:562
233:563
234:564
235:565
236:566
237:567
238:568
239:569
240:576
241:577
242:578
243:579
244:580
245:581
246:582
247:583
248:584
249:585
250:592
251:593
252:594
253:595
254:596
255:597
```

</details>

<br>

Применяется 8-ми битный счетчик для последовательного перебора адресов с 0-255, а данные по этим адресам просто совпадают со значением адреса.
<div class="sim-wrapper" data-circuit-id="34">
  <button class="sim-fullscreen-btn" data-circuit-id="34">⛶</button>
  <iframe 
      id="34"
      data-circuit-id="34"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=true&startCircuit=/turingcomplete/34_8-bin_decimal_display.txt"
      loading="lazy">
  </iframe>
</div> 


[8-ми битный десятичный дисплей](https://www.falstad.com/circuit/circuitjs.html?ctz=CQAgjCAMB0l5AWAnC1b0DZwGYCssNslsAmZSEkyAdhBNxAd0kYFMBaMMAKAHcR21DAA4BYEsJbskCBGIktIfAZKlhs4gTLmcNJKMumyx2Y+2GkTxpf3NIsuoVrPqnNlfbHCEU4Z8dY7tKanD6iRjpgYQa2SBJislJx4WBIiDHOkYla8aE+GREJ1PqFnMSBhly4JeouYNXyCKJB4pa6dQ3tci1gIiYhXH2cJJYtIzV6Yr0pkz3e8pJT88O9Ba01CkuRm2NwjSlUaodr4-vye8NNJ6srDuI3XPNjbfd3JA-vFbbrW-Iv0wVqNg7uUBCIUqCgsU1GkdOCxLDAelOMJimDhClUfoep16ssuMi8d1KvUahgHuI1OSvlNgWIMFdOKT6VcgrgMGpJOF2Wpmc8asz2Dypg02XEvJBueKmX1+VMhrhpYNmoYMGRfrdfkE1ZEuWD1UynqqMecjhdKYpDNRCZ8Na81gDhm1oX9sSTcWRwkC3qzDBYanrzC9TMTbKp+iVwyjJYChmBA0IDosgkgBlitANoincqlkX4QekWsC1NgnJwaBM3JViwkY+W0aEY0WOQl8vWatlm2oEA2wBWim7voLUnSmbiRJah6Pc5F1HcGSqh1R6RTlyi0S1EGpcLjhzvB2I4ClqrzcSe1keJSlL5xzziOybpGWoiaizmJwsqcnq+-zaQUh+RYDAun4souEZiOyMzAb63w1recaTAh4HDP49TTvB6E0qh84fGhuB0mMiycNS8hoaRRHbvimy3oW1YtrR2zEfUdFwQxvR-r+loIOoCTsvSLwIAavSjCAoiaIo4B0CA2AgHIDBYLQohIOAWBgLQ8bgCpy7vHQ+gjHQcj0DJ+jYLJpgyQwdKljJohEHJLA8XJojIIwLC4BADQEYw8lMFguC0AyIAYApWAYIFogYCp1ogNQEBokCYksMIEDCPoFhiXIwgMMIWComJoh+CAnhILQSDKSpfYsH2EB9vofayX2ch9gwXD1bxXDNfU4CrFwGmaVwlXLvc4DxOIlXYNVGg4PVZk4M1eA4GpNlRM1CCtQgak9uAVxRJVzDgB5h31d59BYPQtD0KI9DaS2JDknQaqPbJ91GSFdBoiQiVfUZ1AMF953UJd1DXdQ2mLBIEASNdRUkGkdCpgj+nEAjRlIP9nhw7Qk1yJNVmQFgk3YzGk0qRoLAaBAkwaOZy6kFTlAmbTuPGaQhNfSZdlwzJ1k2WZdn2aYFNOaYpkIOZximFZDR4OZuC4zulmEwFll2YqMktsCVN9MCZMxaWVNoqW5nULjf0yU4pbY-M2DZbZhP5bbdlFUQFOI0Qpkoz4jmQBAPj6D4sk+HIPgMD4WA+LQ0Q+CpHn6B5skefJ3Uef56mMJpHmx8u1SefE1Sx5NjDTQR8dzQR8mLQR-k2fL8nrT5-nbfLojy7HB07p5XnecK7KeU97KJ0F7J+f54WMH07Kx04AW0AFrdg4wiy4ClS-xxlK-yXbirxyjiryejjCeIqtCcAIJDcKkp9ziWLj-lYxJX6BJHMacL+Wk-7YDoebZRBul8q3YPCMoo4GQwkIrFKABRgEWnRJifIUIbQ3GtLqf+tgXROhKBgmcgJpygkENOT0Bh6in3zAiUc3gIQQNoJOTI5CHBkJAdhRh8YGypgDGgqYhJN5cN1AgyoW5IK4kEdGFCpU7hNHCOIq8BgAAekEnKcGtEcFA9JSpyVUiAAALgAQwAEYABtWAAB0ADOAB7AAZmY8Q7A9EAEstFmIAMbmIACasFMWYyxAAncxABbMxkAzFaPMWY+guASSEnTEyNcrCDxMkJIiJkeDWIihqPggkkRIQAOvk1ehh5+JMKgTQuRCRFokXHroNK9IgpGTkAAJQAIIAFluAABk+LzkEo+ES2IpKWJ0QY0xrBGAGE6bodihAJhTKCtVEAgzhmjIYEoAAhCADxgyACuBitFQIACLUAADT7IwCc3AJyEAnOwCckgJywAnJYE0m5TS7lNIeU0yARyABCNzvl3O+Q875XyADCNyQV3JBQ8kFLBnFbNMaEwJpiDHmIAOb2OcVAeA2LjEAEdsUEvgMYgAdoS32eKyXYrACSwlfYKWUt9sE0lBKuD0spay5l8AqpstpXAGlVLyX4vZeSzlcACQ8pZXS0VvtWVCrJVwaloqqrBLlSyolSqxUSq5ZqjVdLVXavVQK2V8quX8rFdy-VYqrVmplYKk1MqbUKpVSa7lSqFVasZTVM1yrnW8vFZy5V1LLWeo5QavV8rlXevNR6gkIrtUWt5TigNnqtU+sVfG8NarrXJvdZawN6bzVErzVaplXKqpBsTV6gNNVfUCttd6mqFas3uurQmtVkbW12vbTqst4q80+obbm2lgb+U+trWW7NJbNX9ujcywN47C2uqncayVKa50ltTY20tjKU0zpbSWvtw6O0HpXVS2NBaa18v1WO7dl6m0GtNeumtEq023pDS+s9o7bULvnW+gk97l1xpDZmh9-qrUKoAzumVF6B3XqnaOkdcGd0wetXBgdc7y2QcvUB-9V7u2MoQ7u-DobbWHqNTuwjQ7yNVvNZGtD9aMPlo-Y2mDz60PHugw69jPboNtonUm2jXrr1poQ06l987RNUf4w60lN7xMbtkyy+Ta6oMEeE4WmlaasOYfTb++Tx753aeQ5pid4n0NQak7Rx9yGi2SvLaW+zXbe2zpsyBqzS77Onvc3GzDeGz1bpM6Rsz0HAtieE7GhzgmzNMcU7ButBKTOIfi9Zu946x1Ss9dG1V6XdOZbS3WxLRGH0qZrWRqdDHMuWa07liLPLf2RfPXVsNiW2N1vM1urzjmfOcfyzZlrQm2suZY25zzfLYsQaa5VzTY7JsBdkyJ7L4HQ0RZG4Wsb36uNtZi9+vjN71vnqc3t3Tvm2U3oy+e09Z3jtQdO1m6bxm5XpZ86V2tOWHOlfvTl67sbbsPvu-+37B7-tlfq+93jgOR3zbi4BgjUPTWPbDcd+zEOKNw824BmjTrdsZve6VlH+6scgfzYawnn3+NI7o2GhLUOmMI9UxTtTVPJ0QfRzZobLPXsHv2yzsnrnpss9+ydmnUnHMtr24Lm7xKzsS5i09wX6Gcvy8fa+znO2mWvt5xdklWnVe4fV-5pXVb6uG6lb+zXLHtfxt17VqXGb5ey-47duTtvF1O6Uy7zjbvtWW6iwjrTPvod5dR11snhmA+oatxpl3mGvchejxRv38H48DfA0D5PROg-6+A27yHvm-OVezXn0PyHFW+Yzy9rP-7i94+jz9xPc2dOHcqxynT5eNsB4m-Xh3M2-czdtwtnHyuLOc5W-r-NTvx-943b3hTqXQ+ectydmfDG72XZ2y3oPE-JcV-n8ZqfrWlsK+bxP3PIb88scL2f+fMf98g8466j7a+sciq3bZw-K+NvX7j6-8vTqxs-+v270vVO3F37zx0e2lzAPrQgLVUXwtxgM-TAMpzW1nx60+w1zgIPw2w-0JxAINyQN22NwIMOzN0wJBy6zHwg3QOXW1zkwQORyl3SzwJL1oIFWYOAMYM-XoLXVH1e081NzD24LF2J3YI7RW0uyL1YJuyFT23-1F3pVkNL1GwUJNSkMbRUOHTULbTOzkNdxkL9S0MzSu3V1bw0PwL1z8ye10KS1Uz3z1yMOS1YJj30NAycIT3KwYNwxTxVxbw+19Q12sJCxcNTyUPb2CLPycNpw8O-w638LPXVSr3Rx1xfwvziPKxMPvzMJE04IF2CIkxyM7xhxkz-zfzNzkIH2Hx4NHyyKjxKJXTN18McxqLEIOybUMx1T-zI2JwSIuzaOkyUOwyyM8PsKGNnWfwcMXXKPfTyJr0J0sInX9TmL6MXUaM9xmJaOGyGNlxKLSOdw5y7QoJSIOw0NfVoK1xcP90YL1xOINTOPAK53AzuOgIeJX1JxuNozuOQNFx6M6PeO2OfxuIVx52WOwwGL-0BMLyoIONT2KKoImIr0+Ko1GyrW4CoHCA0HHEZD0CpFZA2VYG2V2SgUkmJKJLGWJNyXPijFSGWC+lxHKm6EgVoXYAkEZGpJ0AUBhCNFsA5IRE6AkBeHRgSR5LKDXEhhBGXCUHkXYFIBsiUUZDhkPjlLDk0V0UMRMQsWsVMVsQcScVMVcQ8S8VMV8QCSCRCTCVMQiUMHul7GrnPnCgDH4iCC+mnA5CkGdLuGymwmFL6UpPNAxBQhugyVFI5AhAlKtLAQRBpJ3kjMfkAUDJjPPjrgTJYFKSdPFgyRpPTLIi9J4jeGIjIAeEFIKDIHNFenPm9gRDDO5Ntm2HlJrK8FfApOZPrLKBpLMhBCeEZOIXUWZMTEglFO9H7I-h7KxjuGKHCDhhzHHO7NPjxjuA9gEEmmEgXKCDnLAkXIoEiBAlXMgCKXqD5OkVvFFCtMPJ9OZNPP3GLNPIwCCnPKVBvNDHPhQDJFvIVJqB4SdOfMgmWDXNvC5I3L3NImlJvF6FWB3LrFAocEmggqimwmgoAn8FHLUTgs3PXOAuEmnJ3IwtyCQqUXiCdMoUglHGtPfMIitJQS8CKS+m4VlHIpoonMIuQmLMHJIguBZIAj2AIoeAfMpMLKrMpIFL5P9GQuLMYrZMpNZMwsMF-NJDdL7NvH4uAr3MkXPnkpfBQhkvWgcABhSC0oKE0oNB0oSHVCdJ1F5JKHjLKGPO5JDMrLdNstCEdKtM2l1CopcpMHwucrQngjIDQk9OLOqExCosCpMHFAItZLhm0qzJwlEtvPEHzNzNAjTNLNfIxCODmStJbPEHlJYSISdNIE5CooKtdGLOKswSfIGFOE-JzFtGZOhnkDAqtPqpRCouavtGqoODYrOlNGLO6pau0r6tgQIpeFgokoQq9LEp4uZJSnnAyu5MmraG9OmWLIrJFLdKyriCZILJBD5I2usvLIFLbPxHpKvJqsWuSAaq9IuuGFfPYXkDmoqo2FusqtGBPJqsQtys8AIrQkispJ+tgifJzHvmmuYnvgItBpwrSvusfPPKBoYgkDiprAIsRvhtyiYi2uuvUGlHYtCqFLEoXOmoiq+vDN-nPGZIjNbJhtem7EfHumEhUtMvptprMqsqFIcqiCoopo5q9OjJRFpt5qiBMqtIFqEnZK5tFuLK5oJvoCBrCq8vnPhqlpbDTMRpwqls8qlKFipAemlPjCpBXg0XVB0TcTcR8U8UNKsTMR0TMWJS2X8T0VYB8UvlvLwExPRLJt6FxOaFJPch9oWSGRGQMAaUXMvJIixNdPXKhnAD2BIGgH+jjqJLjukr0rDvROHG3OkvTpuCllwjgpTqJEXL3H-OlPztDoxPfP2pLr3LJvLqHP0vzpUt1txAZuTuUqKVrscrzuUoNA7sFuJAmVdo7B7ojtCBMqgUWUDpWW4E1qiAgCUQoA3LivtMNrkDhQRRNJJWaTaT0UXOWuZMgEWs9qmCFp3pAVkn3tyEboJCnomXjD3KITKBtFxJYAnuWTWCECOFZPzrysqA-ufm5qSt-vYiQgAd0AkiAdviyD3JDHfuAefHztLGwnUnYmiE7trBQlAdWjUD-ryCZOQe7F7Abv-mDv6u-kfqyDRCjqqjoATtjpWSgCTqPtIdCF7DSgof6W9pJPocklfrKXnqyDtlCHxCaADkMhAGNtNvNrMUttMWttMVtvtsdu4CAA)

  
#### Circuit Simulation: Program

Необходимые компоненты:
* [8-ми битный счетчик](turingcomplete_memory.html#counter)

Для корретного старта программы, первый адрес в памяти RAM должен быть больше чем 0, т.е. 1 или 2 и т.д. но не 0.
 
---

## Conditions


> Задача: 
> На этом уровне на вход подаётся значение и 3 бита условия (8 возможных комбинации).
> 
> 3 бита определяют условие, как показано ниже. 
> 
> Проверьте значение по выбранному условию и выведите 1 если оно выполняется, иначе 0.
> 
> ```
> V| OPCODE:    Выведите 1 когда: 
> -|-----------------------------------------
> 0| 0  0  0    Никогда т.е. ничего не делать
> 1| 0  0  1    Если значение = 0
> 2| 0  1  0    Если значение < 0
> 3| 0  1  1    Если значение ≤ 0
> 4| 1  0  0    Всегда, просто вывести 1 
> 5| 1  0  1    Если значение ≠ 0
> 6| 1  1  0    Если значение ≥ 0
> 7| 1  1  1    Если значение > 0 
> ```
 
Убедитесь, что вы находитесь в режиме -1 , а не в режиме +255.

**Связь с условными переходами (JMP / JZ / JN)**

Есть:
* VALUE — обычно результат ALU (последняя операция)
* OPCODE (3 бита) — код условия
* Выход Conditions — COND_OK 
    * COND_OK = 1 → условие выполнено → переход разрешён
    * COND_OK = 0 → переход запрещён

> [!SUCCESS]
> Нужны всего 2 флага:
> * ZERO (**Z**) это VALUE=0 получить можно так: все биты через OR потом NOT т.е. NOR
> * NEGATIVE (**N**) это просто старший бит VALUE (бит 7)
> 
> И далее нужно получить POSITIVE (**P**): `P = NOT N AND NOT Z`


Схема Conditions — это и есть “блок условий процессора”. Он отвечает на один вопрос: разрешён ли переход? (1 или 0)

| Ассемблер | Условие | OPCODE      | Проверка |
| --------- | ------- | ----------- | -------- |
|           | никогда | 000         | 0        |
| JMP       | всегда  | 100         | 1        |
| JZ        | == 0    | 001         | Z        |
| JNZ       | != 0    | 101         | !Z       |
| JN        | < 0     | 010         | N        |
| JP        | > 0     | 111         | P        |
| JLE       | ≤ 0     | 011         | N OR Z   |
| JGE       | ≥ 0     | 110         | !N       |

```
O0 = C0 & 0
O1 = C1 & Z
O2 = C2 & N
O3 = C3 & (N | Z)
O4 = C4 & 1
O5 = C5 & !Z
O6 = C6 & !N
O7 = C7 & P

COND_OK = O0 | O1 | O2 | O3 | O4 | O5 | O6 | O7
```

![Conditions](/Computer-Science-Bookshelf/img/tc/Conditions.png)

#### Circuit Simulation: Conditions

Необходимые компоненты:
* [3 Bit Decoder](turingcomplete_arithmetic.html#3-bit-decoder)
* [Bit Switch (tri-state buffer)](turingcomplete_memory#bit-switch-tri-state-buffer)



<div class="sim-wrapper" data-circuit-id="35">
  <button class="sim-fullscreen-btn" data-circuit-id="35">⛶</button>
  <iframe 
      id="35"
      data-circuit-id="35"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=true&startCircuit=/turingcomplete/35_conditions.txt"
      loading="lazy">
  </iframe>
</div> 

---

## Immediate values

Нам нужен способ напрямую передавать числа из нашей программы в регистры. 

Ключевая идея уровня - Инструкция сама является данными!
(Как в уровне [Calculations](turingcomplete_cpu_architecture.html#calculations), мы использовали младшие биты предназначенные для адреса Destination `xxxxxD2D1D0` в режиме MODE `01` не для адреса Destination, а для блока ALU)

> ```
> [ M1 M0 | S2 S1 S0 | D2  D1  D0  ]
> [ MODE  | Source   | Destination ]
>
> OPCODE MODE:
> ------------
> 0 0 Immediate values непосредственные значения
> ```

> Задача: В режиме "Immediate values" преобразовать 6 младших бит инструкции в байт и передать его в `REG0`
>
> Если линии остаются активными, когда не должны — будет короткое замыкание. 
> Это значит: В Immediate Mode: только Immediate должен писать в REG0. 
> В других режимах: Immediate вообще не должен влиять на шину
>
> `IMMEDIATE = NOT M1 AND NOT M0`

Если шесть бит инструкции после бит MODE и есть наши данные `(S2 S1 S0 D2  D1  D0)` то мы можем закодировать значение от 0 до 63 включительно (`xxxxxx = 2⁵ + 2⁴ + 2³ + 2² + 2¹ + 2⁰ = 32 + 16 + 8 + 4 + 2 + 1 = 64`)

Тогда мы берем шесть младших бит инструкции и превращаем их в байт, и записываем в `REG0`

Нам нужно:
* отключить декодеры для Source и Destination, так как в роли Source у нас данные инструкции,а в роли Destination `REG0`
* используя компонент `Switch 8 bit` (MUX) решить какие данные пустить в `REG0`
* используя компонент переключателя 1 bit (подойдет и `Switch 8 bit` (MUX)) разрешить регистру `REG0` сохранить данные, так как биты адресации Destination мы теперь используем как часть данных
 
исправил схему: 
* в режиме Immediate values необходимо так же отключить декодеры Source и Destination, как это сделано в режиме COPY
* так же нет необходимости в 8-ми битном MUX для сигналов LOW/HIGH, поставил gate OR для флага `Save` у `REG 0`

![Immediate values](/Computer-Science-Bookshelf/img/tc/Immediate_values.png)
 
 
#### Circuit Simulation: MODE Immediate values

Необходимые компоненты:
* [8 bit и 1 bit Multuplexers (MUX)](turingcomplete_memory.html#8-bit-switch-and-8-bit-multuplexers-mux-tri-state-buffer)

 
---

## Turing Complete

> [!TIP]
> Разблокирует компонент RAM (ПЗУ).
 
> [!NOTE]
> Полнота по Тьюрингу (Turing completeness) — это свойство вычислительной системы, означающее, что она может выполнить любую вычисляемую функцию, если дать ей достаточно времени и памяти.
> 
> Минимальный набор из четырёх вещей:
> 1. Память (неограниченная или достаточно большая)
> 2. Условные переходы (if/goto)
> 3. Изменение памяти (запись)
> 4. Бесконечный цикл (или возможность продолжать)
> 

В инструкции два старших бита `M1, M0` отвечают за режимы MODE, в котором нам нужно реализовать `11xxxxxx` Conditions. 

До этого момента все программы ограничивались выполнением байт за байтом.

До сих пор только код из Program влиял на память (REG0,...), теперь память должна влиять на код. 

До этого:
* PC (Program Counter счётчик) всегда делал PC = PC + 1
* Код → влиял на данные
 
С добавлением условной логики Conditions, наш компьютер теперь может выполнить любой алгоритм и вычислить все что вычислимо.

Теперь:
* Данные → могут влиять на код
* Мы можем изменить PC из инструкции
* Это и есть условные переходы (branch / jump)


Последний недостающий кусок CPU которую нам нужно добавить, это механизм для изменения текущего значения счётчика через **инструкции**, при определённых услових.

В режим Conditions мы попадаем при инструкции `11xxxxxx`

В этом режиме, значение `REG3` проверяется на условие заданное тремя младшими битами инструкции `xxxxxD2D1D0`.  
Если условие выполняется, мы записываем значение `REG0` в счётчик.
Изменение значения счетка с помощью условия означает, что мы можем пропускать инструкции основываясь на условиях или запускать инструкции в цикле.

Эти условия соответствуют компоненту Conditions который был сохранён в завод компонентов:
```
0 0 0 Никогда т.е. не должны ничего делать
0 0 1 Если REG3 = 0
0 1 0 Если REG3 < 0
0 1 1 Если REG3 ≤ 0 
1 0 0 Всегда
1 0 1 Если REG3 ≠ 0 
1 1 0 Если REG3 ≥ 0
1 1 1 Если REG3 > 0
```

Реализация:
* Отключаем блок декодера для Source и Destination, так как мы используем в роли Source `REG3`, а в роли Destination `REG0`
* Данные для проверки условий в компоненте Conditions. Берём `REG3` — это результат вычислений (для этого нужен 1 битный переключатель (подойдет и `Switch 8 bit` (MUX)) чтобы в компонент вычисления условий Conditions пошли данные из `REG3`)
* Если условие = 1 → берем данные с `REG0` и записываем в PC
* Иначе → PC просто увеличится на 1, как обычно


![Turing Complete](/Computer-Science-Bookshelf/img/tc/Turing_Complete.png)

#### Circuit Simulation: Priority input

<div class="sim-wrapper" data-circuit-id="47">
  <button class="sim-fullscreen-btn" data-circuit-id="47">⛶</button>
  <iframe 
      id="47"
      data-circuit-id="47"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=true&startCircuit=/turingcomplete/47_priority_input.txt"
      loading="lazy">
  </iframe>
</div> 


#### Circuit Simulation: Program Counter (PC): Controlled Counter

* [Альтернативная реализация Counter](turingcomplete_memory.html#counter)
* [Counter из JK-триггеров (рис. 6)](https://mrmcsoftware.wordpress.com/2017/03/07/testing-and-improving-my-cpu-design-with-logisim-and-digital-logic-basics/)

Необходимые компоненты:
* [Full Adder](turingcomplete_arithmetic.html#full-adder)
* [D-trigger](turingcomplete_memory.html#circuit-simulation-d-trigger)
* [Register 8 bit with Write Enable and Tri-state output](turingcomplete_memory.html#saving-bytes)
* [8-битный MUX (2→1)](turingcomplete_memory.html#8-bit-switch-and-8-bit-multuplexers-mux-tri-state-buffer)

PC (Program Counter ) — это не просто счётчик команд, это управляемый регистр.
* В каждом такте процессора, PC указывает, откуда читать инструкцию в памяти
* После выборки инструкции:
  * обычно происходит инкремент `PC = PC + 1`
  * иногда PC ← другое значение (jump, immediate, return), согласно блоку условий [Conditions](turingcomplete_cpu_architecture.html#conditions)

Program Counter обязан:
* Обновляться ровно один раз за такт (только по фронту clock). 
* Обновляться без enable. PC обязан обновляться каждый такт.
  * Остановка PC = остановка процессора.
* Иметь стабильное значение в течение всего такта.
* Всегда иметь валидное значение (никогда не должен быть Z). Адрес инструкции всегда должен существовать.
  * Даже при reset — он определён.
* Выбирать ровно одно следующее значение 
  * из набора:
    * PC + 1
    * Immediate
    * Jump target
    * Return address

```
┌────────────────────────────────┐
|       ┌────────────┐           │
PC ───► │  +1 Adder  │──┐        │
        └────────────┘  │        │
                        ├──► MUX ┘ ───► D[7:0]
Immediate ──────────────┘

```


Счетчик с управлением (Controlled Counter)

Имеет 3 управляющих сигнала:
* Increment (+1) - переход к следующей команде
* Load - загрузка нового адреса (для JUMP)
* Reset - сброс в начало программы

Основная задача:
* Хранить адрес следующей команды в памяти программ.


Три режима работы:
1. Последовательное выполнение (обычный режим)
  * PC указывает на текущую команду
  * После выполнения команды PC автоматически +1
  * Переход к следующей ячейке памяти

2. Прыжок/Переход (Jump)
  * Текущая команда говорит: "Перейди на адрес X"
  * PC загружает новый адрес X
  * Следующая команда будет по адресу X

3. Сброс (Reset)
  * PC устанавливается в начальный адрес (обычно 0x00)
  * Начинает выполнение программы сначала

> [!FAILURE]
> Program counter не должен инкрементировать значение с Load сразу после получения.
> Начинается счетчик с Full adder у которого второе слогаемое 1, результат идет в первый вход MUX, а второй это Load внешние данные, выход MUX сохраняется в register, выход с register идет на выход и во вход Full adder тем самым замыкая цикл. 
  

#### Circuit Simulation: Turing Complete

Необходимые компоненты:
* [Conditions](turingcomplete_cpu_architecture.html#conditions)
* [Program Counter (PC): Controlled Counter](turingcomplete_cpu_architecture.html#circuit-simulation-program-counter-pc-controlled-counter)
 

## [Digital simulator](https://github.com/hneemann/Digital/tree/master)


> В режиме calc, conditions и immediate values, декодеры sorce и destination не должны влиять на шину, 
> но если их перевести в состояние Z то шина имеет К.З.

Справка opcode:
```
[ x  x | S2  S1  S0 | D2  D1  D0  ]
[ MODE | Source     | Destination ]

OPCODE MODE:
------------------------- 
00xxxxxx Immediate values. Source 6 bit instruction, Destination REG 0
01xxxxxx CALC (ALU). Source REG 1 and REG 2, Destination REG 3 
10xxxxxx COPY
11xxxxxx Conditions. Source REG 3 and REG 0, Destination Program counter (PC)

OPCODE Source:

S2 S1 S0
--------------
0  0  0  REG 0
0  0  1  REG 1
0  1  0  REG 2
0  1  1  REG 3
1  0  0  REG 4
1  0  1  REG 5
1  1  0  INPUT # использовать внешний вход
1  1  1  UNUSED


OPCODE Destination:

D2 D1 D0 
--------------
0  0  0  REG 0
0  0  1  REG 1
0  1  0  REG 2
0  1  1  REG 3
1  0  0  REG 4
1  0  1  REG 5
1  1  0  OUTPUT # использовать внешний выход
1  1  1  UNUSED

 
Conditions:

0 0 0 Никогда т.е. не должны ничего делать
0 0 1 Если REG3 = 0
0 1 0 Если REG3 < 0
0 1 1 Если REG3 ≤ 0 
1 0 0 Всегда
1 0 1 Если REG3 ≠ 0 
1 1 0 Если REG3 ≥ 0
1 1 1 Если REG3 > 0

ALU:

V| OPCODE
-|---------------
0| xxxxx000   OR
1| xxxxx001   NAND
2| xxxxx010   NOR
3| xxxxx011   AND
4| xxxxx100   ADD
5| xxxxx101   SUB

```


Проверка всех режимов:
```
INPUT: 00000111 # 7

# tick=0 clk=0 addr=0 
# tick=1 clk=1 addr=1 

     Instruction: 10110001
        MODE COPY
        Source 110 (INPUT)
        Destination 001 (REG 1)

# tick=2 clk=0 addr=1
# tick=3 clk=1 addr=2

     Instruction: 00000001 # 1
        MODE Immediate values
        Source Instruction
        Destination (REG 0)

# tick=4 clk=0 addr=2
# tick=5 clk=1 addr=3

    Instruction: 10000010
        MODE COPY
        Source 000 (REG 0)
        Destination 010 (REG 2)
 
# tick=6 clk=0 addr=3
# tick=7 clk=1 addr=4

    Instruction: 01000100   
        MODE CALC
        Source REG 1 и REG 2
        Destination REG 3
        100 ADD

# tick=7 clk=0 addr=4
# tick=8 clk=1 addr=5

    Instruction: 10011110
        MODE COPY
        Source 011 (REG 3)
        Destination 110 (OUTPUT)

OUTPUT: 00001000 # 8


# tick=8 clk=0 addr=5
# tick=9 clk=1 addr=6

    Instruction: 11000110
    MODE Conditions
    Source (REG 3) данные из которого проверяются в блоке Conditions
        Условие для блока Conditions 110 означает "Выдать 1 если REG3 ≥ 0"
        Ожидаем выход блока условий 1, так как в REG 3 содержится значение 8
    Destination Program counter (PC)
        Program counter (PC) перезапишется данными из REG 0 т.е. 1
        В ROM по адресу 1 начинается снова выполнение этого цикла программы
        Ожидаем зацикленный вывод 00001000 # 8
```

File ROM.txt:
```
v2.0 raw
0
b1
1
82
44
9e
c6
ff
```

File turing_complete_cpu_rising.dig:

![Turing Complete](/Computer-Science-Bookshelf/img/tc/turing_complete.gif)


Все файлы симулятора Digital для сборки уровня "CPU Architecture":

* [dec_2_4.dig](/Computer-Science-Bookshelf/img/tc/digital_level_turing_complete_cpu/dec_2_4.dig)
* [dec_3_8.dig](/Computer-Science-Bookshelf/img/tc/digital_level_turing_complete_cpu/dec_3_8.dig)
* [dec_3_8_disable.dig](/Computer-Science-Bookshelf/img/tc/digital_level_turing_complete_cpu/dec_3_8_disable.dig)
* [full_adder_1bit.dig](/Computer-Science-Bookshelf/img/tc/digital_level_turing_complete_cpu/full_adder_1bit.dig)
* [full_adder_8bit.dig](/Computer-Science-Bookshelf/img/tc/digital_level_turing_complete_cpu/full_adder_8bit.dig)
* [mux_1bit.dig](/Computer-Science-Bookshelf/img/tc/digital_level_turing_complete_cpu/mux_1bit.dig)
* [mux_8bit.dig](/Computer-Science-Bookshelf/img/tc/digital_level_turing_complete_cpu/mux_8bit.dig)
* [mux_8bit_Z.dig](/Computer-Science-Bookshelf/img/tc/digital_level_turing_complete_cpu/mux_8bit_Z.dig)
* [switch_8bit.dig](/Computer-Science-Bookshelf/img/tc/digital_level_turing_complete_cpu/switch_8bit.dig)
* [neg_8bit.dig](/Computer-Science-Bookshelf/img/tc/digital_level_turing_complete_cpu/neg_8bit.dig)
* [d_trigger_falling_sync.dig](/Computer-Science-Bookshelf/img/tc/digital_level_turing_complete_cpu/d_trigger_falling_sync.dig)
* [d_trigger_rising_async.dig](/Computer-Science-Bookshelf/img/tc/digital_level_turing_complete_cpu/d_trigger_rising_async.dig)
* [register_8bit_falling.dig](/Computer-Science-Bookshelf/img/tc/digital_level_turing_complete_cpu/register_8bit_falling.dig)
* [register_8bit_falling_we_bus.dig](/Computer-Science-Bookshelf/img/tc/digital_level_turing_complete_cpu/register_8bit_falling_we_bus.dig)
* [register_8bit_rising.dig](/Computer-Science-Bookshelf/img/tc/digital_level_turing_complete_cpu/register_8bit_rising.dig)
* [register_8bit_rising_we_bus.dig](/Computer-Science-Bookshelf/img/tc/digital_level_turing_complete_cpu/register_8bit_rising_we_bus.dig)
* [alu.dig](/Computer-Science-Bookshelf/img/tc/digital_level_turing_complete_cpu/alu.dig)
* [conditions.dig](/Computer-Science-Bookshelf/img/tc/digital_level_turing_complete_cpu/conditions.dig)
* [program_counter_8bit.dig](/Computer-Science-Bookshelf/img/tc/digital_level_turing_complete_cpu/program_counter_8bit.dig)
* [turing_complete_cpu_rising.dig](/Computer-Science-Bookshelf/img/tc/digital_level_turing_complete_cpu/turing_complete_cpu_rising.dig)
* [ROM.hex](/Computer-Science-Bookshelf/img/tc/digital_level_turing_complete_cpu/ROM.hex)
  
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