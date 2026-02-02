# Functions

* [Hex Racer](#hex-racer)
* [Shift](#shift)
* [RAM](#ram)
* [Delay](#delay)



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
const start_save_to_ram 0b00000100 # 4
const start_output_ram 0b00010100 # 20
const start_continue_output 0b00011000 # 24
const iterations 0b00100000 # 32
 
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

> Задача: 
>
> Умножение двух 4-х битных чисел дает 8-и битное число. Создайте схему, которая это делает.

`Умножение = AND + сдвиг + сложение`



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
 