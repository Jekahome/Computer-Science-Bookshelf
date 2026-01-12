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
 
#### Circuit Simulation ALU

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


#### Circuit Simulation Registers

Необходимые компоненты:
* [Register 8 bit](turingcomplete_memory.html#saving-bytes)
* [3 Bit Decoder](turingcomplete_arithmetic.html#3-bit-decoder)
* [8 Bit Switch (SWC)](turingcomplete_memory#bit-switch-tri-state-buffer)

 

Проверка:
```
#1
Input: 00000111 # 7
Instruction: 00110000
    Source 110 (INPUT)
    Destination 000 (REG 0)

#1
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

Для двух старших бит можно использовать декодер 2 на 4 который мы построили ранее на уровне [Instruction Decoder](#instruction-decoder) он тоже принимает 8 бит но реагирует только на первые два старших, в принципе его можно переделать, для этого нам есть завод компонентов, заменим 8-ми битный вход на 2-х битный. Или можно взять еще один декодер 3 на 8 (избыточно). 

Наш компонет ALU принимает 8 бит инструкцию, входом для нее будет служить младшие биты которые мы используем для адресации выхода Destination: `D2  D1  D0`

(COPY тут не реализовано)
![Calculations](/Computer-Science-Bookshelf/img/tc/Calculations1.png)

Либо можно вообще не заниматься двойным передокированием младших битов инструкции `D2  D1  D0`, и напрямую пустить в ALU все 8 бит инструкции. 
Также нужно верно использовать инструкцию COPY, которая должна управлять активацией Source, Destination

![Calculations](/Computer-Science-Bookshelf/img/tc/Calculations2.png)

А вот, переделанный декодер с двумя входными пинами вместо 8

![2bit_input_decoder_2_to_4](/Computer-Science-Bookshelf/img/tc/2bit_input_decoder_2_to_4.png)

---

## Program

Компонент ввода инструкций был удален. Он заменен компонентом программирования Program Memory. 

Раньше мы использовали ручной ввод инструкции (Instruction Input) прямой передачей 8 бит. 
Теперь нам выдают Program Memory (ПЗУ **ROM** (Read-Only Memory)) который содержит в своей постоянной памяти 8-ми битные инструкции.
Что бы их достать нам нужно их поочередно брать по индексу, для этого нам нужно использовать счетчик который будет с каждым такстом увеличиваться и мы будем получать следующую инструкцию из блока Program Memory.

В блоке Program Memory инструкции хранятся в определенной последовательности.
Это и есть последовательное выполнение программы.

Для этого уровня вам необходимо использовать компонент 8-ми битный счётчика, который вы разблокировали ранее.

![Program](/Computer-Science-Bookshelf/img/tc/Program.png)

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

---

## Immediate values

Нам нужен способ напрямую передавать числа из нашей программы в регистры. 

Ключевая идея уровня - Инструкция сама является данными!
(Как в уровне Calculations, мы использовали младшие биты предназначенные для адреса Destination `xxxxxD2D1D0` в режиме MODE `01` не для адреса Destination, а для блока ALU)

> ```
> [ M1 M0 | S2 S1 S0 | D2  D1  D0  ]
> [ MODE  | Source   | Destination ]
>
> OPCODE MODE:
> ------------
> 0 0 Immediate values непосредственные значения
> ```

> Задача: При режиме "Immediate values" преобразовать 6 младших бит инструкции в байт и передать его в `REG0`
>
>
> Если линии остаются активными, когда не должны — будет короткое замыкание. 
> Это значит: При Immediate Mode: только Immediate должен писать в REG0. 
> В других режимах: Immediate вообще не должен влиять на шину
>
> `IMMEDIATE = NOT M1 AND NOT M0`

Если 6 бит инструкции после бит MODE и есть наши данные `S2 S1 S0 | D2  D1  D0` то мы можем иметь значение от 0 до 63 включительно (`xxxxxx = 2⁵ + 2⁴ + 2³ + 2² + 2¹ + 2⁰ = 32 + 16 + 8 + 4 + 2 + 1 = 64`)

Тогда мы берем 6 младших бит инструкции и превращаем их в байт, и записываем в `REG0`

Нам нужно:
* используя компонент `Switch 8 bit` (MUX) решить какие данные пустить в `REG0`
* используя компонент переключателя 1 bit (подойдет и `Switch 8 bit` (MUX)) разрешить регистру `REG0` сохранить данные, так как биты адресации Destination мы теперь используем как часть данных
 
![Immediate values](/Computer-Science-Bookshelf/img/tc/Immediate_values.png)
 
---

## Turing Complete

> [!TIP]
> Разблокирует компонент RAM (ПЗУ).
 

В инструкции два старших бита M1 M0 отвечают за режимы MODE, в котором нам нужно реализовать `11xxxxxx` Conditions. 

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
* Данные для проверки условий в компоненте Conditions. Берём `REG3` — это результат вычислений (для этого нужен 1 битный переключатель (подойдет и `Switch 8 bit` (MUX)) чтобы в компонент вычисления условий Conditions пошли данные из `REG3`)
* Если условие = 1 → берем данные с `REG0` и записываем в PC
* Иначе → PC просто увеличится на 1, как обычно


![Turing Complete](/Computer-Science-Bookshelf/img/tc/Turing_Complete.png)


<details>

<summary>компонент RAM в circuitjs</summary>

[RAM](https://www.falstad.com/circuit/circuitjs.html?ctz=CQAgjCAMB0l3BWcMBMcUHYMGZIA4UA2ATmIxAUgoqoQFMBaMMAKABYxsQM82QG22FCDxhC-QcKp4R4EHzBUm-FCwCyIvEpRsqxMMIY6qVFNATrN2-CH2G0M0+ct4t-MBnF33nqCDMWADJWEkK2BqFScgBmAIYANgDOdNSQLm5GGMLemVEB6Uq4fDlFfvkaroXYXhEM2NVlzhUZ1TIlhI7+TSECcOGGuiZdQdy8KrrcDUYTQ3FJKUhpwTx8RjY44mudEHPJqSzLY0y+Gz7iVDsJe4sHo6uSk5sPFyC7C1C3KypZj99RVG99odVqVTnVIHxZld3ks7vx6uIwQi-ADoUC4XUOr9MZ1UfN0V9elQwYMUa80TcOMRwJwnmFmFiBGFpLdmFMHDSISobC9ATdgmzNsZOathbyKR8BbS-iKZeL8ZSUBAwGwdPwEBEVRMGBqop1Fn4DUh+eBVaKEOJOMR7BayXyPlTwAg0BI3GAEM7XUM8CwAEqmtU6zXYa3qzX+ZBOJABFHOAXO7Rq92eoxq+XXSUB0VJkP2NMxCWwlWBlCucC5lRlqEKzPFkEuq2GIRDPEZotm+Gexud-7kmvtwPYMbdupjatt1kdgRicuh6fnPz2gf3NgyEdsVd2wuTwOri4VgRuccwnerMChkfnqKXfushPuC8ewxXvw3icADxpGCQAgwypQCBcAIWJ8GqAAiACWiQAA7xLEACeLCflo1K-nweCEAMf7yP4fC+gAgmoSGmhw6qAU6uBkVQoF8AAwgA9gArgAdgALnQABOLBAA)

RAM:
A0, A1, A2 — это адресные входы (куда подаётся адрес ячейки)
D0, D1, D2... D7 — это выходы данных (откуда читается содержимое ячейки)

 

**Правильная логика управления RAM:**

| Сигнал | Значение | Режим работы |
|--------|----------|--------------|
| **WE = 1** | HIGH | **ЗАПИСЬ** — данные с входов D0-D7 записываются в память |
| **WE = 0** | LOW | **ЧТЕНИЕ** — данные из памяти выводятся на выходы D0-D7 |
| **OE = 1** | HIGH | **Разрешение выхода** — данные появляются на выходах |
| **OE = 0** | LOW | **Высокий импеданс** — выходы отключены (Z-состояние) |



RAM:
```
0: 0 0 0 0
7: 15
```

Вход это адрес ячейки:
```
A2=1 # 2²=4
A1=1 # 2¹=2
A0=1 # 2⁰=1 
 
Это означает адрес: 111 в двоичном = 7 в десятичном.

2² + 2¹ + 2⁰ = 4 + 2 + 1 = 7
```


Выход значения формируется из 8-ми бит:
```
D7=0
D6=0
D5=0
D4=0
D3=1 # 2³=8
D2=1 # 2²=4
D1=1 # 2¹=2
D0=1 # 2⁰=1
```

2³ + 2² + 2¹ + 2⁰ = 8 + 4 + 2 + 1 = 15

Для загрузки файла, RAM ожидает бинарный формат:
```
printf '\x00\x00\x00\x00\x00\x00\x00\x0F' > data.bin

Что создаст файл с:
0: 0 0 0 0 0 0 0 15

Что тоже самое:
7: 15

адрес: значение
```

Формат данных:
```
1: 4
2: 10

трансформируется в:
1: 4 10
```


</details>

 
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