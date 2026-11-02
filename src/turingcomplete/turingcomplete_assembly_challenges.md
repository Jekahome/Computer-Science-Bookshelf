# Assembly Challenges

* [Robot Racing](#robot-racing)
* [AI Showdown](#ai-showdown)
* [Unseen Fruit](#unseen-fruit)
* [Delicious Order](#delicious-order)
* [Dancing Machine](#dancing-machine)
* [Planet Names](#planet-names)
* [Tower of Alloy](#tower-of-alloy)
* [Water World](#water-world)

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

Лабиринт отличается от лабиринта на уровне [The Maze](turingcomplete_programming.html#the-maze) тем, что нет поворота робота и обратной связи.

### Вариант 1 (без математики)

* Простой способ, жестко задать все 64 шага, получим **<span style="color:purple">256 байт программу.</span>**

* Способ, учесть повторяющиеся действия
    <details>
    <summary>С такой разбивкой на функции, код программы получится 208 байт:</summary>

    ![Robot Racing](/Computer-Science-Bookshelf/img/tc/Robot_Racing2.png)

    </details>

* Способ, жестко задать только половину шагов:

    Лабиринт разбит на четыре секции но последовательности действий для его прохождения всего две. 
    * Вторая с третья секция имеют общий алгоритм, с одним отличием - выход из секции.
    * Стартовая секция и завершающая, это второй алгоритм последовательности шагов, оличаются выходом из секции и зеркальным отражением. 
        * Если в первой группе мы можем просто передать последовательность шагов то, при зеркальном раположении лабиринтов нам нужно переопределить направления
        * Если повернуть первую секцию так как мы смотрим на вторую и третью то, чтобы двигать влево нам нужно передать сигнал идти вверх и т.д. А для четвертой секции, чтобы идти влево нужно передать сигнал вниз

        |sec.|right|down|left|up |finish|
        |----|-----|----|----|---|------|
        |1|1|2|3|0|2|
        |2|0|1|2|3|2|
        |3|0|1|2|3|1|
        |4|3|0|1|2|2|

    >
    > Кривая Гильберта 3-го порядка состоит из четырех кривых 2-го порядка, соединенных между собой. Именно поэтому мы видим закономерности каждые 16 шагов

    ![Robot Racing](/Computer-Science-Bookshelf/img/tc/Robot_Racing.png)


    Для реализации ассемблера нам потребуется наличие команд `CALL`,`RET` и `MOV`. **<span style="color:purple">Количество тиков - 86, размер программы 152 байта.</span>**

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
    146        #4 destination PC

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

---

### Вариант 2 (с помощью математики?)

[akademeia.info](https://akademeia.info/?p=31869)

[en.wikipedia.org/wiki/Hilbert_curve](https://en.wikipedia.org/wiki/Hilbert_curve)

Использование последовательности формирующей кривую Гильберта именно потому, что движение робота соответствует пути по пространственно‑заполненной кривой, и её рекурсивная структура позволяет пройти трек без входных данных.

> [!IMPORTANT]
> Кривая Гильберта — это непрерывная фрактальная линия, которая проходит через каждую точку единичного квадрата, то есть заполняет плоскость. Мощный инструмент для упорядочивания многомерной информации, который находит применение в компьютерных науках, физике и инженерии.
> 
> Это один из самых известных примеров так называемых "space-filling curves" (кривых, заполняющих пространство), открытых Давидом Гильбертом в 1891 году.
>
> Парадоксальные свойства кривой Гильберта нашли практическое применение:
>
> * Сжатие изображений и обработка данных: Кривая Гильберта сохраняет "локальность". Близкие точки на кривой (одномерный индекс) оказываются близкими и в двумерном пространстве. Это используется в алгоритмах сжатия (например, в некоторых методах вейвлет-преобразования) и для преобразования многомерных данных в одномерные с минимальной потерей пространственной связности.
> 
> * Построение пространственных индексов в базах данных (Hilbert R-tree): Для эффективного поиска геоданных (например, "найти все объекты в этом прямоугольнике") многомерные координаты (широта, долгота) преобразуются в одномерный ключ с помощью кривой Гильберта. Близкие в пространстве объекты получают близкие ключи, что ускоряет запросы к базе данных.
> 
> * Проектирование микросхем (traversal order): При размещении элементов на чипе или маршрутизации соединений порядок обхода, заданный кривой Гильберта, может быть более эффективным, чем простой построчный (растровый) обход, из-за меньшей средней длины "прыжков".

В этом видео [Robot Racing (zoickx youtube)](https://youtu.be/FojkEeoPfcs?si=dAXdFiMMQkS24dL2&t=5833) решение на 803 тика и 116 байт кода.

Наша последовательность шагов — это **Кривая Гильберта 3-го порядка, с вертикальным отражением (Vertical Flip) т.е. инверсия Верх/Низ**

У кривой Гильберта нет единственной «стандартной» ориентации — их 8.
Если кривая Гильберта начинается с движения `Вправо`, а наш лабиринт требует начать с движения `Вверх`, это может значит, что вся последовательность была повернутая вертикально, отражена (Vertical Flip). 


<details>
<summary>Демонстрация стандартной последовательности кривой Гильберта 3-го порядка:</summary>

```rust,editable
fn vertical_flip(dir: u8) -> u8 {
    match dir {
        1 => 3,
        3 => 1,
        _ => dir,
    }
}

fn horizontal_flip(dir: u8) -> u8 {
    match dir {
        0 => 2,
        2 => 0,
        _ => dir,
    }
}

fn main() {
    let standard: [u8; 64] = [
        0,1,2,3,2,3,0,1,2,3,0,1,0,1,2,3,
        2,3,0,1,0,1,2,3,0,1,2,3,2,3,0,1,
        2,3,0,1,0,1,2,3,0,1,0,1,2,3,0,1,
        2,3,0,1,2,3,0,1,0,1,2,3,2,3,0,1,
    ];

    for rot in 0..4 {
        let r = rot as u8;

        let seq_rot: Vec<u8> = standard.iter().map(|&d| (d + r) % 4).collect();
        let seq_vflip: Vec<u8> = seq_rot.iter().map(|&d| vertical_flip(d)).collect();
        let seq_hflip: Vec<u8> = seq_rot.iter().map(|&d| horizontal_flip(d)).collect();
        let seq_both: Vec<u8> = seq_rot
            .iter()
            .map(|&d| vertical_flip(horizontal_flip(d)))
            .collect();

        println!("Rotation {}°: {:?}", rot*90, seq_rot);
        println!("Rotation {}° + VFlip: {:?}", rot*90, seq_vflip);
        println!("Rotation {}° + HFlip: {:?}", rot*90, seq_hflip);
        println!("Rotation {}° + VFlip+HFlip: {:?}", rot*90, seq_both);
        println!("---");
    }
}
```
</details>

<br>

<details>
<summary>Демонстрация модифицированной последовательности кривой Гильберта 3-го порядка:</summary>

Модифицированная кривая, которая учитывает локальные повороты квадрантов (rot)

```rust,editable
fn main() {
    let order = 3; // Кривая 3-го порядка (одна сторона квадрата сторона 2^3=8, 8*8=64 шага), с вертикальным отражением (Vertical Flip).
    let (mut current_x, mut current_y) = direct_to_xy(order, 0);
    println!("N Шаг: Стандартное | Отражение | (d_X,d_Y) ");
    for i in 1..64 {
        let (x, y) = direct_to_xy(order, i);
        let d_X = x as i8 - current_x as i8;
        let d_Y = y as i8 - current_y as i8;
        
        /*
        // Получаем стандартное направление
        // эту логику разности (if/else) заменить на одну формулу через XOR, чтобы сэкономить место
        let standard_dir = match (d_X, d_Y) {
            (1, 0)  => 0, // Вправо
            (0, 1)  => 1, // Вниз
            (-1, 0) => 2, // Влево
            (0, -1) => 3, // Вверх
            _ => 0,
        };
        */
 
        // Вместо match логики используем битовоую логику: 
        // 1. Сдвигаем и используем & 1, чтобы получить чистый 0 или 1
        let bit_1 = ((d_X >> 7) | (d_Y >> 7)) & 1; 
        // 2. Младший бит d_Y
        let bit_0 = d_Y & 1;
        // 3. Собираем итоговое двухбитное число 0-3
        let standard_dir = (bit_1 << 1) | bit_0;
 
        // 4. Вертикальное отражение от стандарта кривой
        let robot_dir = match standard_dir {
            1 => 3,
            3 => 1,
            _ => standard_dir,
        };

        let dir_name = match robot_dir {
            0 => "Вправо",
            1 => "Вниз",
            2 => "Влево",
            3 => "Вверх",
            _ => "",
        };

        println!("{:5}: {:11} | {} {:7} | ({},{}) ", i, standard_dir, robot_dir, dir_name, d_X, d_Y);

        current_x = x;
        current_y = y;
    }
}

/// Преобразует индекс (шаг) в координаты (x, y) для кривой Гильберта.
/// n - порядок кривой (сторона квадрата = 2^n).
/// d - индекс (от 0 до n^2 - 1).
fn direct_to_xy(n: u32, mut d: u32) -> (u32, u32) {
    let mut x = 0;
    let mut y = 0;
    let mut s = 1; // Текущий размер подквадрата (растет от 1 до 2^(n-1))
    
    // Итеративно строим координаты снизу вверх (от малых квадратов к большим)
    let side = 1 << n; // 2^n
    
    while s < side {
        // Определяем, в какой четверти находится точка (0, 1, 2 или 3)
        // Используем по 2 бита индекса на каждом уровне
        let rx = 1 & (d / 2);
        let ry = 1 & (d ^ rx);
        
        // Поворачиваем и зеркалим систему координат, если мы в боковых четвертях
        rot(s, &mut x, &mut y, rx, ry);
        
        // Смещаем координаты в текущий подквадрат
        x += s * rx;
        y += s * ry;
        
        d /= 4;
        s *= 2;
    }
    
    (x, y)
}

/// Функция поворота и отражения системы координат.
/// Это "магия" кривой Гильберта, которая обеспечивает её непрерывность.
fn rot(n: u32, x: &mut u32, y: &mut u32, rx: u32, ry: u32) {
    if ry == 0 {
        if rx == 1 {
            *x = n - 1 - *x;
            *y = n - 1 - *y;
        }
        // Меняем x и y местами
        let temp = *x;
        *x = *y;
        *y = temp;
    }
}
```

</details>

---

#### Как мы получаем направление движения из координат

У нас есть четыре возможных состояния вектора движения $(\Delta x, \Delta y)$ и мы хотим получить из них число (направление) $0, 1, 2, 3$

Значения координат delta_x, delta_y: 
* `1 = 0000 0001`
* `-1 = 1111 1111`
* `0 = 0000 0000`

Таблица состояния вектора движения $(\Delta x, \Delta y)$:

| Направление | $\Delta X$  (число) | $\Delta X$ (биты) | $\Delta Y$ (число) | $\Delta Y$ (биты) | Желаемый код |
| --- | --- | --- | --- | --- | --- |
| **Вправо** | 1 | `00000001` | 0 | `00000000` | **0** (`00`) |
| **Вниз** | 0 | `00000000` | 1 | `00000001` | **1** (`01`) |
| **Влево** | -1 | `11111111` | 0 | `00000000` | **2** (`10`) |
| **Вверх** | 0 | `00000000` | -1 | `11111111` | **3** (`11`) |

<br>
<details>
<summary>Из байтов $(\Delta x, \Delta y)$ получаем 2 бита направления (Желаемый код):</summary>
 

* Младший бит желаемого кода (`x1`) повторяет поведение младшего бита $\Delta Y$ 
    * Получить его можно через побитовое AND: 
        * Bit 0 = $\Delta Y$ & 1
        

```rust,editable
fn main() {
    let dY = 0b00000001;
    println!("Bit 0 = {:08b}", dY&1);

    let dY = 0b11111111;
    println!("Bit 0 = {:08b}", dY&1);

    let dY = 0b00000000;
    println!("Bit 0 = {:08b}", dY&1);

}
```
* Старший бит желаемого кода `1x` повторяет поведение старшего бита $\Delta X$ или $\Delta Y$
    *  Получить его можно предварительно сдвинув старший бит на позицию младшего и выполнив побитовое OR: 
        * Bit 1 = ($\Delta X >> 7$) | ($\Delta Y >> 7$)

```rust,editable
fn main() {
    let dX = 0b11111111;
    let dY = 0b00000000;
    println!("Bit 1 = {:08b}", (dX >> 7)|(dY >> 7));
    
    let dX = 0b00000000;
    let dY = 0b11111111;
    println!("Bit 1 = {:08b}", (dX >> 7)|(dY >> 7));
    
    let dX = 0b00000001;
    let dY = 0b00000000;
    println!("Bit 1 = {:08b}", (dX >> 7)|(dY >> 7));
}
```

Итог:
```rust,editable
fn main() {
    let dX = 0b00000001;
    let dY = 0b00000000;
    println!("dX = {:08b} dY = {:08b}", dX,dY);
    println!("Bit 0 = {:08b}", dY&1);
    println!("Bit 1 = {:08b}", (dX >> 7)|(dY >> 7));
    // формируем два бита, нужно результат сдвига старшего бита на младшую позицию сдвинуть обратно на старшую
    println!("standard direct = {:02b}\n", (((dX >> 7) | (dY >> 7)) << 1) | (dY & 1));// Вправо
    
    
    let dX = 0b00000000;
    let dY = 0b00000001;
    println!("dX = {:08b} dY = {:08b}", dX,dY);
    println!("Bit 0 = {:08b}", dY&1);
    println!("Bit 1 = {:08b}", (dX >> 7)|(dY >> 7));
    println!("standard direct = {:02b}\n", (((dX >> 7) | (dY >> 7)) << 1) | (dY & 1));// Вниз
    
    let dX = 0b11111111;
    let dY = 0b00000000;
    println!("dX = {:08b} dY = {:08b}", dX,dY);
    println!("Bit 0 = {:08b}", dY&1);
    println!("Bit 1 = {:08b}", (dX >> 7)|(dY >> 7));
    println!("standard direct = {:02b}\n", (((dX >> 7) | (dY >> 7)) << 1) | (dY & 1));// Влево
    
    let dX = 0b00000000;
    let dY = 0b11111111;
    println!("dX = {:08b} dY = {:08b}", dX,dY);
    println!("Bit 0 = {:08b}", dY&1);
    println!("Bit 1 = {:08b}", (dX >> 7)|(dY >> 7));
    println!("standard direct = {:02b}\n", (((dX >> 7) | (dY >> 7)) << 1) | (dY & 1));// Вверх
}
```
</details>

---

### Заменить вычисления координат $(x, y)$ на жесткое кодирование
 
> Через Код Грея не работает, так как последовательность кривой Гилберта модифицированна, а Код Грея основан на чистой последовательности.
>
> стандартная последовательность:    0,1,2,3,2,3,0,1,2,3,0,1,0,1,2,3,2,3,0,1,2,3,0,1,0,1,2,3,0,1,2,3,2,3,0,1,2,3,0,1,0,1,2,3,0,1,0,1,2,3,2,3,0,1,2,3,0,1,2,3,0,1,2,3
>
> наша модифицированная:             1,0,3,0,0,1,2,1,0,1,2,2,3,2,1,1,0,1,2,1,1,0,3,0,1,0,3,3,2,3,0,0,0,1,2,1,1,0,3,0,1,0,3,3,2,3,0,3,3,2,1,2,2,3,0,3,2,3,0,0,1,0,3,3
>
> наша модифицированная с инверсией: 3,0,1,0,0,3,2,3,0,3,2,2,1,2,3,3,0,3,2,3,3,0,1,0,3,0,1,1,2,1,0,0,0,3,2,3,3,0,1,0,3,0,1,1,2,1,0,1,1,2,3,2,2,1,0,1,2,1,0,0,3,0,1,1
>
> Код Грея — это способ представления чисел, при котором соседние значения отличаются **только в одном двоичном разряде**, что бы избежать ошибки переключения (фронт, дребезг).
> Например, $1_{10}=01_{2}$, а $2_{10}=10_{2}$ т.е. что изменить число нужно сразу два бита переключить, что даелает возможным появления ошибки при переключении на фронтах (falling edge, rising edge). В коде Грея $1_{10}=01_{2}$ , но $2_{10}=11_{2}$ т.е. что бы изменить число с один на два или наоборот, нужно всего один быт переключить.
 
Цель - получить требуемую последовательность не реализуя кривую Гилберта с модификацией и инверсией и не используя [архитектуру LEG](https://steamcommunity.com/sharedfiles/filedetails/?id=2824911652), так как кода будет больше чем 256 байт (*возможно*).
* Мы можем жестко закодировать последовательность шагов в виде двух бит упакованных в байты (**The Data Approach**), на весь лабиринт 16 байт, и в цикле пройтись по ним.
    * 2 бита на шаг * 64 шага = 128 бит = 16 байт, итого 16 инструкций MOV в stack RAM
    * логика иcпользования байта из stack? одна инструкция переместить байт (на 4 шага) в регистр и в цикле на 4 шага читаем последовательно по два бита и сдвигаем результат 

<details>
<summary>Что такое упаковка (The Data Approach):</summary>

```rust,editable
fn main() {
    // Упакованный байт: 4 значения по 2 бита каждое
    // Биты: [val3: 7-6] [val2: 5-4] [val1: 3-2] [val0: 1-0]
    let packed: u8 = 0b11_10_01_00; // значения: 3, 2, 1, 0

    println!("Упакованный байт: {:#010b} ({})\n", packed, packed);
    
    let bits_per_value = 2;
    let mask: u8 = 0b11; // маска для 2 бит

    // вариант 1 (двигаем значащие биты в младшие позиции через деление)
    println!("Значение [{}]: {:#04b} = {}", 0, packed & mask,  packed & mask);
    println!("Значение [{}]: {:#04b} = {}", 1, (packed/4) & mask,  (packed/4) & mask);
    println!("Значение [{}]: {:#04b} = {}", 2, (packed/16) & mask,  (packed/16) & mask);
    println!("Значение [{}]: {:#04b} = {}", 3, (packed/64) & mask,  (packed/64) & mask);
    println!();
    
    // вариант 2 (двигаем значащие биты в младшие позиции через правый сдвиг)
    println!("Значение [{}]: {:#04b} = {}", 0, packed & mask,  packed & mask);
    println!("Значение [{}]: {:#04b} = {}", 1, (packed >> 2) & mask,  (packed >> 2) & mask);
    println!("Значение [{}]: {:#04b} = {}", 2, (packed >> 4) & mask,  (packed >> 4) & mask);
    println!("Значение [{}]: {:#04b} = {}", 3, (packed >> 6) & mask,  (packed >> 6) & mask);
    println!();
    
    // вариант 3
    for i in 0..4 {
        let shift = i * bits_per_value;
        let value = (packed >> shift) & mask;
        println!("Значение [{}]: {:#04b} = {}", i, value, value);
    }
}
```

</details>
<br>
<details>
<summary>Упаковка по 4 значения в байт (2 бита на значение):</summary>
 
```rust,editable
fn pack_reversed() {
    let seq = vec![3,0,1,0,0,3,2,3,0,3,2,2,1,2,3,3,0,3,2,3,3,0,1,0,3,0,1,1,2,1,0,0,0,3,2,3,3,0,1,0,3,0,1,1,2,1,0,1,1,2,3,2,2,1,0,1,2,1,0,0,3,0,1];
    
    let mut packed = Vec::new();
    
    for chunk in seq.chunks(4) {
        let mut byte = 0u8;
        for (i, &val) in chunk.iter().enumerate() {
            byte |= (val & 0b11) << (i * 2);
        }
        packed.push(byte);
    }
    
    println!("const PACKED: [u8; 16] = [");
    for (i, &byte) in packed.iter().enumerate() {
        print!("    0b{:08b}", byte);
        if i < packed.len() - 1 {
            print!(",");
        }
        println!();
    }
    println!("];");
}

fn get_direction(step: usize) -> u8 {
    // Младшие биты = первые значения
    // 19,236,172,249,236,19,83,6,236,19,83,70,185,70,6,19
    const PACKED: [u8; 16] = [
        0b00010011,
        0b11101100,
        0b10101100,
        0b11111001,
        0b11101100,
        0b00010011,
        0b01010011,
        0b00000110,
        0b11101100,
        0b00010011,
        0b01010011,
        0b01000110,
        0b10111001,
        0b01000110,
        0b00000110,
        0b00010011
    ];
    
    let byte_idx = step / 4;
    let bit_pos = (step % 4) * 2;
    (PACKED[byte_idx] >> bit_pos) & 0b11
}

fn main() {
    pack_reversed();
    
    println!("\nПроверка:");
    let expected = vec![3,0,1,0,0,3,2,3,0,3,2,2,1,2,3,3,0,3,2,3,3,0,1,0,3,0,1,1,2,1,0,0,0,3,2,3,3,0,1,0,3,0,1,1,2,1,0,1,1,2,3,2,2,1,0,1,2,1,0,0,3,0,1];
    
    for i in 0..63 {
        let val = get_direction(i);
        print!("{val}");
        if val != expected[i] {
            panic!();
        }
    }
}
```


```rust,editable
fn print_direction(four_step: u8)   {

    let bit_pos = 0;//(step % 4) * 2;
    let res_1:u8 = four_step & 0b11; 
    print!("{res_1}\n");

    let bit_pos = 2;//(step % 4) * 2;
    let res_1:u8 = (four_step >> bit_pos) & 0b11; 
    print!("{res_1}\n");

    let bit_pos = 4;//(step % 4) * 2;
    let res_1:u8 = (four_step >> bit_pos) & 0b11; 
    print!("{res_1}\n");

    let bit_pos = 6;//(step % 4) * 2;
    let res_1:u8 = (four_step >> bit_pos) & 0b11; 
    print!("{res_1}\n");

}

fn main() {
    // 19,236,172,249,236,19,83,6,236,19,83,70,185,70,6,19
    const PACKED: [u8; 16] = [
        0b00010011,
        0b11101100,
        0b10101100,
        0b11111001,
        0b11101100,
        0b00010011,
        0b01010011,
        0b00000110,
        0b11101100,
        0b00010011,
        0b01010011,
        0b01000110,
        0b10111001,
        0b01000110,
        0b00000110,
        0b00010011 
    ];
    for i in 0..16 {
        print!("\nstep={i}\n");
        print_direction(PACKED[i]);
    }
}
```

</details>

**<span style="color:purple">Реализация на 2,500 тиков, размер кода 204 байта</span>** 💩

Идея, вызывать функцию для каждого байта оказалась избыточной.

<details>
<summary>Assembly Editor:</summary>

```bash
const CALL 0b00001000
const RET 0b00010000

# 19,236,172,249,236,19,83,6,236,19,83,70,185,70,6,19

# prepare steps
0b10001100 19 0 0 
CALL 0 0 jump_fn        
 
0b10001100 236 0 0 
CALL 0 0 jump_fn
    
0b10001100 172 0 0 
CALL 0 0 jump_fn
 
0b10001100 249 0 0 
CALL 0 0 jump_fn

0b10001100 236 0 0 
CALL 0 0 jump_fn

0b10001100 19 0 0 
CALL 0 0 jump_fn

0b10001100 83 0 0 
CALL 0 0 jump_fn

0b10001100 6 0 0 
CALL 0 0 jump_fn

0b10001100 236 0 0 
CALL 0 0 jump_fn

0b10001100 19 0 0 
CALL 0 0 jump_fn

0b10001100 83 0 0 
CALL 0 0 jump_fn

0b10001100 70 0 0 
CALL 0 0 jump_fn

0b10001100 185 0 0 
CALL 0 0 jump_fn

0b10001100 70 0 0 
CALL 0 0 jump_fn

0b10001100 6 0 0 
CALL 0 0 jump_fn

0b10001100 19 0 0 
CALL 0 0 jump_fn

# FUNCTION ----------------------
label jump_fn
# xxxxxx11---------------------
0b01000010 # ALU reg_0 AND 3
0b00000000 # reg_0
3 # mask для 2 младших бит
0b00000111 # output
# xxxx11xx----------------------
0b00001100 0 0 2  # MOV reg_0 reg_2
# ALU SUB reg_0 >> 2
0b10001100 4 0 3 # MOV 4 reg_3
CALL 0 0 jump_fn_div
# xx11xxxx----------------------
0b00001100 0 0 2  # MOV reg_0 reg_2
# ALU SUB reg_0 >> 4
0b10001100 16 0 3 # MOV 16 reg_3 
CALL 0 0 jump_fn_div
# 11xxxxxx----------------------
0b00001100 0 0 2  # MOV reg_0 reg_2
# ALU SUB reg_0 >> 6
0b10001100 64 0 3 # MOV 64 reg_3
CALL 0 0 jump_fn_div
 
RET 0 0 0         
 
# FN DIV ---------------------------
label jump_fn_div
# args reg_2, reg_3
# out reg_1 
0b10001100 0 0 1 # reset reg_1
 
label jump_div_if 
# IF_LESS  RET 
0b00100010   #1 opcode Cond IF_LESS
0b00000010   #2 arg1 source reg_2
0b00000011   #3 arg2 source reg_3
jump_div_output  #4 destination

# 5. SUB reg_2 - reg_3 = reg_2
0b00000001 #1 opcode SUB
0b00000010 #2 arg1 source reg_2
0b00000011 #3 arg2 source reg_3
0b00000010 #4 destination reg_2

# Quotient ADD 1
0b10000000 #1 opcode ADD 1 reg_1
0b00000001 #2 arg1 source ImVal
0b00000001 #3 arg2 source reg_1
0b00000001 #4 destination reg_1

0b10001100 jump_div_if 0 6

label jump_div_output
0b01000010 # ALU reg_1 AND 3
0b00000001 # Quotient reg_1
3
0b00000111 # output

RET 0 0 0 

# JMP Exit 
0b10001100 200 0 6  
```

</details>

---

Возможно и по другому выводить упакованные числа. Перенести логику выбора следующего байта упакованных шагов внутрь цикла.

**<span style="color:purple">Реализация на 2,857 тиков, размер кода 140 байт</span>** 
 
<details>
<summary>Прототип:</summary>

```rust,editable
fn main() {
    // 19,236,172,249,236,19,83,6,236,19,83,70,185,70,6,19
    const PACKED: [u8; 16] = [
        0b00010011, 0b11101100, 0b10101100, 0b11111001,
        0b11101100, 0b00010011, 0b01010011, 0b00000110,
        0b11101100, 0b00010011, 0b01010011, 0b01000110,
        0b10111001, 0b01000110, 0b00000110, 0b00010011
    ];
    
    let mut byte_idx = 0;
    let mut current_byte = PACKED[0];
    let mut shift_count = 0;
    
    for _step in 0..63 {
        // 1. Взять младшие 2 бита
        print!("{}", current_byte & 3);
        
        // 2. Сдвинуть байт вправо на 2 бита
        current_byte = current_byte >> 2;  // div 4
        
        // 3. Увеличить счётчик
        shift_count += 1;
        
        // 4. Если сдвинули 4 раза - загрузить новый байт
        if shift_count == 4 {
            byte_idx += 1;
            if byte_idx < 16 {
                current_byte = PACKED[byte_idx];
            }
            shift_count = 0;
        }
    }
}
```
</details>

<details>
<summary>Assembly Editor:</summary>

```bash
const CALL 0b00001000
const RET 0b00010000
    
# prepare steps 
# 19,236,172,249,236,19,83,6,236,19,83,70,185,70,6,19
 
# PUSH stack RAM
0b10010111 19 0 0 
0b10010111 6 0 0
0b10010111 70 0 0
0b10010111 185 0 0
0b10010111 70 0 0
0b10010111 83 0 0
0b10010111 19 0 0
0b10010111 236 0 0
0b10010111 6 0 0
0b10010111 83 0 0
0b10010111 19 0 0
0b10010111 236 0 0
0b10010111 249 0 0
0b10010111 172 0 0
0b10010111 236 0 0

# первое число сразу отдаем в REG_2
0b10001100 19 0 0b00000010  
#----------------------------

# byte_idx     REG_1
# current_byte REG_2
# shift_count  REG_3

label jmp_while  
# current_byte & 3
0b01000010 # ALU reg_0 AND 3
0b00000010 # reg_0
3          # mask для 2 младших бит
0b00000111 # output

# current_byte = current_byte >> 2;
0b00001100 0b00000010 0 0b00000100 # MOV reg_2 to reg_4
CALL 0 0 jump_fn_div

# shift_count += 1;
0b01000000 # ALU reg_3 ADD 1
0b00000011 # reg_3
1          
0b00000011 # destination reg_3

# if shift_count == 4
0b01100000   #1 opcode Cond IF_EQUAL
0b00000011   #2 arg1 source reg_3
4            #3 arg2 source ImVal
jump_inc     #4 destination

0b10001100 jmp_while 0 6 

# ------------------------------
label jump_inc
# byte_idx += 1;
0b01000000 #1 opcode ADD reg_1+1
0b00000001 #2 arg1 source reg_1 
1          #3 arg2 source ImVal
0b00000101 #4 destination reg_1

# shift_count = 0;
0b00001100 0 0 0b00000011 # MOV 0 to reg_3
 
# if byte_idx < 16
0b01100010    #1 opcode Cond IF_LESS if reg_1 < 16
0b00000001    #2 arg1 source reg_1
16            #3 arg2 source ImVal
jump_set_byte #4 destination
 
0b10001100 jmp_while 0 6 

label jump_set_byte
# current_byte = PACKED[byte_idx];
0b00010101 #1 opcode POP
0          #2 arg1 Unused
0          #3 arg2 Unused
0b00000010 #4 destination reg_2

0b10001100 jmp_while 0 6    
# end while
# -----------------------------

# FN DIV ------------------------
label jump_fn_div
# args Dividend=reg_4, Divisor=4   
# out reg_5 
0b10001100 0 0 5 # reset reg_5
 
label jump_div_if 
# IF_LESS  if reg_4 < 4 
0b01100010   #1 opcode Cond IF_LESS
0b00000100   #2 arg1 source reg_4
4            #3 arg2 source ImVal
jump_div_result  #4 destination

# 5. SUB reg_4 - 4 = reg_4
0b01000001 #1 opcode SUB
0b00000100 #2 arg1 source reg_4
4          #3 arg2 source ImVal
0b00000100 #4 destination reg_4

# Quotient ADD 1
0b10000000 #1 opcode ADD 1 reg_5
0b00000001 #2 arg1 source ImVal
0b00000101 #3 arg2 source reg_5
0b00000101 #4 destination reg_5

0b10001100 jump_div_if 0 6

label jump_div_result
# MOV to REG_2
0b00001100 # MOV reg_5 to REG_2
0b00000101 # Quotient reg_5
0
0b00000010 # REG_2
 
RET 0 0 0 

```
</details>


 
Если реализовать логику деления (`DIV`) в `ALU`, можно еще сократить программу.

**<span style="color:purple">Реализация на 387 тика, размер кода 108 байт</span>** 

<details>
<summary>Assembly Editor:</summary>

```bash
# prepare steps 
# 19,236,172,249,236,19,83,6,236,19,83,70,185,70,6,19
 
# PUSH stack RAM
0b10010111 19 0 0 
0b10010111 6 0 0
0b10010111 70 0 0
0b10010111 185 0 0
0b10010111 70 0 0
0b10010111 83 0 0
0b10010111 19 0 0
0b10010111 236 0 0
0b10010111 6 0 0
0b10010111 83 0 0
0b10010111 19 0 0
0b10010111 236 0 0
0b10010111 249 0 0
0b10010111 172 0 0
0b10010111 236 0 0

# первое число сразу отдаем в REG_2
0b10001100 19 0 0b00000010  
#-------------------------------

# byte_idx     REG_1
# current_byte REG_2
# shift_count  REG_3

label jmp_while  
# current_byte & 3
0b01000010 # ALU reg_2 AND 3
0b00000010 # reg_2
3          # mask для 2 младших бит
0b00000111 # output
 
# ALU DIV
0b01010100 # DIV reg_2=reg_2/4
0b00000010
4
0b00000010

# shift_count += 1;
0b01000000 # ALU reg_3 ADD 1
0b00000011 # reg_3
1          
0b00000011 # destination reg_3

# if shift_count == 4
0b01100000   #1 opcode Cond IF_EQUAL
0b00000011   #2 arg1 source reg_3
4            #3 arg2 source ImVal
jump_inc     #4 destination

0b10001100 jmp_while 0 6 

# -----------------------------
label jump_inc
# byte_idx += 1;
0b01000000 #1 opcode ADD reg_1+1
0b00000001 #2 arg1 source reg_1 
1          #3 arg2 source ImVal
0b00000101 #4 destination reg_1

# shift_count = 0;
0b00001100 0 0 0b00000011 # MOV 0 to reg_3
 
# if byte_idx < 16
0b01100010    #1 opcode Cond IF_LESS if reg_1 < 16
0b00000001    #2 arg1 source reg_1
16            #3 arg2 source ImVal
jump_set_byte #4 destination
 
0b10001100 jmp_while 0 6 

label jump_set_byte
# current_byte = PACKED[byte_idx];
0b00010101 #1 opcode POP
0          #2 arg1 Unused
0          #3 arg2 Unused
0b00000010 #4 destination reg_2

0b10001100 jmp_while 0 6    
# end while

```
</details>


![Hilbert curve](/Computer-Science-Bookshelf/img/tc/Hilbert_curve.gif)


#### Применим наш самописный ассемблер

```rust,editable,abraetable
use legassembly::assembly;
fn main(){

  static input: &str = "
## byte_idx     REG_1
## current_byte REG_2
## shift_count  REG_3

## PUSH stack RAM
PUSH 19 
PUSH 6 
PUSH 70 
PUSH 185
PUSH 70 
PUSH 83 
PUSH 19 
PUSH 236 
PUSH 6 
PUSH 83 
PUSH 19 
PUSH 236 
PUSH 249 
PUSH 172 
PUSH 236 

## первое число сразу отдаем в REG_2
MOV 19, r2  

jmp_while:
    AND r2, 3, io
    DIV r2, 4, r2
    ADD r3, 1, r3
    CJE r3, 4, jump_inc
    JMP jmp_while 

jump_inc:
    ADD r1, 1, r1
    MOV 0,r3
    CJL r1, 16, jump_set_byte 
    JMP jmp_while 

jump_set_byte:
    POP r2
    JMP jmp_while 
";
 
  let output_debug = true;
  assembly(input, output_debug);

}

{{#include ../../src/legassembly.rs}}

```



---

Наша кривая имеет паттерн Гилберта **DAAB** т.е. начало с `D` потом `A` снова `A` и последний `B`.


![Hilbert curve](/Computer-Science-Bookshelf/img/tc/Hilbert_curve.png)

Описание поворотов (но что бы это реализовать нужно больше кода чем 104 байта).

Начинается с первого блока четырех шагов (это начало паттерна D) `3010` что бы получить второй блок нужно (повернуть) поменять местами индексы значений теперь 0=3,1=2,2=1,3=0 и мы получим `0323`. Далее переход к третьему блоку, просто `-1` от предыдущего и получаем `0322` и теперь, что бы получить четвертый блок нужно снова (повернуть) поменять местами индексы значений, теперь 0=1,1=0,2=3,3=2 и мы получаем `1233`. 

Далее для перехода на пятый блок (это начало паттерна A) нам нужно `+1` к последнему значению и снова поменять местами индексы значений теперь 0=1,1=0,2=3,3=2 и мы получим `0323`. Что получить шестой блок нужно снова поменять местами индексы значений теперь 0=3,1=2,2=1,3=0 и мы получаем `3010`. Для седьмого блока выполняем просто `+1` и мы получаем `3011`, для восьмого блока выполняем снова поменять местами индексы значений теперь 0=1,1=0,2=3,3=2 и мы получаем `2100`. 

Для девятого блока (это начало паттерна A) выполняем `+1` и снова поменять местами индексы значений теперь 0=2,1=3,2=0,3=1 и мы получаем `0323`....

```
3010->0323->0322->1233->0323->3010->3011->2100->0323->3010->3011->2101->1232->2101->2100->3011

3010 -> 0323 # 0=3,1=2,2=1,3=0
          -1
        0322 -> 1233 # 0=1,1=0,2=3,3=2
                  +1
                0323 -> 3010 # 0=3,1=2,2=1,3=0
                          +1
                        3011 -> 2100
                                  +1
                                0323 -> 3010
                                          +1
                                        3011 +1 -> 2101
...
                               

```
  
[steamcommunity discussions](https://steamcommunity.com/app/1444480/discussions/0/3192487178171763816/)  


Реализация алгоритма построения кривой Гильберта через **взаимную рекурсию (Mutual Recursion)**, адаптированный под специфический обход плоскости. Каждая паттерн фцнкция `a(), b(), c(), и d()` представляят собой состояние (ориентацию) робота в пространстве. Кривая Гильберта фрактальна. Это значит, что большой квадрат состоит из четырех маленьких квадратов. Чтобы нарисовать большой квадрат (порядок 3), нам нужно нарисовать четыре маленьких квадрата (порядок 2), но повернутых определенным образом, чтобы их можно было соединить одной непрерывной линией. Каждая функция знает в каком порядке вызвать другие функции (под-квадраты).

*Адаптированная под специфический обход*:
* Зеркальное отражение (Vertical Flip): Мы инвертировали команды `up` и `down`.
* Измененная точка старта: Обычно кривая начинается в углу $(0,0)$, но порядок вызовов в наших данных берет начало обхода с определенной ориентации.
* Реверс: Мы перевернули порядок команд внутри функций, потому что наш робот проходит трассу от выхода к входу относительно стандартного алгоритма.


```rust,editable

# use std::cell::RefCell;
# thread_local! {
#     static RESULT: RefCell<Vec<i32>> = RefCell::new(Vec::new());
# }
# 
# fn check(){
#     let truth = vec![
#         3,0,1,0,0,3,2,3,0,3,2,2,1,2,3,3,0,3,2,3,3,0,1,0,3,0,1,1,2,1,0,0,
#         0,3,2,3,3,0,1,0,3,0,1,1,2,1,0,1,1,2,3,2,2,1,0,1,2,1,0,0,3,0,1,1
#     ];
#    
#     RESULT.with(|res| {
#         assert_eq!(*res.borrow(), truth, "Последовательности не совпадают!");
#     });
# }

fn main() {
    d(3);
    m(1);// finish
    
#    check();// test
}

fn m(cmd: i32) {
    print!("{}", cmd);
    
#    RESULT.with(|res| {res.borrow_mut().push(cmd);});
}

fn a(n: i32) {
    if n > 0 {
        b(n - 1); m(2); a(n - 1); m(1); a(n - 1); m(0); d(n - 1);
    }
}

fn b(n: i32) {
    if n > 0 {
        a(n - 1); m(1); b(n - 1); m(2); b(n - 1); m(3); c(n - 1);
    }
}

fn c(n: i32) {
    if n > 0 {
        d(n - 1); m(0); c(n - 1); m(3); c(n - 1); m(2); b(n - 1);
    }
}

fn d(n: i32) {
    if n > 0 {
        c(n - 1); m(3); d(n - 1); m(0); d(n - 1); m(1); a(n - 1);
    }
}
```

---

## AI Showdown

> Задача:
> 
> Правила игры: На столе лежат 12 карт, каждый игрок по очереди берет от 1 до 3 карт. Вы начинаете, и игрок, взявший последнюю карту (джокер), проигрывает.
> 
> При чтении входных данных отобразится текущее количество карт.
> 
> Отправка 1, 2 или 3 на выход позволит получить соответствующее количество карт. NAK 02 реагирует немедленно, поэтому вы можете считать входные данные сразу после вывода, чтобы получить результат его действия.


<details>
<summary>Assembly Editor:</summary>

```bash
# reg_0 Текущее количество

# MOV 3 to output
0b10001100 3 0 0b00000111 

# MOV INPUT to reg_0
0b00001100 0b00000111 0 0b00000000
 
# OUTPUT reg_0 - 5
0b01000001 # ALU SUB reg_0 SUB 5
0b00000000 # reg_0
5          
0b00000111 # destination OUTPUT

# MOV INPUT to reg_0
0b00001100 0b00000111 0 0b00000000

# OUTPUT reg_0 - 1
0b01000001 # ALU SUB reg_0 SUB 1
0b00000000 # reg_0
1          
0b00000111 # destination OUTPUT

```
</details>

---

## Unseen Fruit

> Задача:
> 
> Регулярно осматривайте конвейерную ленту на предмет поступающих фруктов. Как только вы увидите один и тот же вид фруктов дважды, повернитесь и нажмите на панель управления.

Управление роботом:
* 0 Turn left
* 1 Move forward
* 2 Turn right
* 3 Enjoy the moment
* 4 use action
* 5 Shoot laser

<details>
<summary>Прототип:</summary>

```rust,editable
fn main() {
    // Входящие фрукты
    let fruits = vec!["яблоко", "яблоко", "банан", "апельсин", "яблоко", "киви", "апельсин"];
    
    // уникальные фрукты
    let mut unique_fruits = Vec::new();
    
    for fruit in fruits {

        let mut found = false;
        
        // Проходим циклом по всему тому, что уже запомнили
        for seen_fruit in &unique_fruits {
            if seen_fruit == &fruit {
                found = true;
                break; // Нашли дубликат, выходим из внутреннего цикла
            }
        }

        if found {
            println!("Повтор: {}! Нажать кнопку.", fruit);
        } else {
            // Если не нашли, добавляем
            unique_fruits.push(fruit);
            println!("Пропускаем: {}", fruit);
        }
    }
}
```
</details>

<br>

<details>
<summary>Assembly Editor:</summary>

```bash
const jump_scan 44
const jump_contains_fruit 60
const jump_array_iteration 68
const jump_add_fruit 84
const jump_button_activation 100

# MOV go to control place
0b10001100 0 0 0b00000111 
0b10001100 1 0 0b00000111 
0b10001100 0 0 0b00000111
0b10001100 1 0 0b00000111 
0b10001100 1 0 0b00000111 
0b10001100 1 0 0b00000111 
0b10001100 1 0 0b00000111 
0b10001100 0 0 0b00000111 
0b10001100 1 0 0b00000111 
0b10001100 2 0 0b00000111 
0b10001100 1 0 0b00000111

# jump_scan-----------------------
# wait
0b10001100 3 0 0b00000111 
# MOV INPUT to reg_1
0b00001100 0b00000111 0 0b00000001

# if reg_1 != 92 
0b01100001 #1 cond !=
0b00000001 #2 arg1 source reg_0
92         #3 arg2 source ImmVal
jump_contains_fruit  #4

# else continue
0b10001100 jump_scan 0 6

# -------------------------------
# jump_contains_fruit
# save SP reg_0 to reg_2
0b00001100 0b00000000 0 0b00000010 # MOV
# reset reg_0=0
0b10001100 0 0 0b00000000 # MOV

# jump_array_iteration------------ 
# if reg_0 == reg_2
0b00100000 #1 cond ==
0b00000000 #2 arg1 source reg_0
0b00000010 #3 arg2 source reg_2
jump_add_fruit #4

# if contains RAM[reg_0] == reg_1
0b00100000 #1 cond ==
0b00001000 #2 arg1 source RAM[reg_0]
0b00000001 #3 arg2 source reg_1
jump_button_activation #4

# inc reg_0 if < reg_2
# inc reg_0
0b01000000 #1 opcode ADD reg_0+1 
0b00000000 #2 arg1 source reg_0
1          #3 arg2 source ImVal
0b00000000 #4 destination reg_0 

0b10001100 jump_array_iteration 0 6

# jump_add_fruit ----------------
# restore reg_0=reg_2
0b00001100 0b00000010 0 0b00000000 # MOV

# MOV reg_1 to RAM
0b00001100 #1 opcode MOV
0b00000001 #2 arg1 source INPUT
0          #3 unused
0b00001000 #4 destination RAM

# inc reg_0
0b01000000 #1 opcode ADD reg_0+1 
0b00000000 #2 arg1 source reg_0
1          #3 arg2 source ImVal
0b00000000 #4 destination reg_0

0b10001100 jump_scan 0 6
# -------------------------------
 
# jump_button_activation ------
# if contains
0b10001100 2 0 0b00000111 # MOV
0b10001100 4 0 0b00000111 # MOV
0b10001100 0 0 0b00000111 # MOV

# restore reg_0=reg_2
0b00001100 0b00000010 0 0b00000000 # MOV

0b10001100 jump_scan 0 6 
```
</details>

![Unseen Fruit](/Computer-Science-Bookshelf/img/tc/Unseen_Fruit.gif)

---

## Delicious Order

> Задача:
> 
> Мы обновляем галактическую энциклопедию еды, добавляя в неё продукты, предназначенные для людей. Поскольку у нас нет алфавита, статьи в энциклопедии упорядочены по степени вкусности.
> 
> Сначала поочередно считайте 15 оценок вкуса из входных данных. Ваша задача — вывести их в отсортированном порядке, от наименьшей к наибольшей.

Не будем скромничать, возьмем все 256 байт RAM для сортировки подсчетом.

<details>
<summary>Прототип:</summary>
 
Сортировка подсчетом (Counting Sort):

Она работает только если числа 8 бит (от 0 до 255)

```rust,editable
# fn display_ram_as_grid(counts: &[u8; 256]) {
#    println!("\n--- RAM GRID DUMP (256 bytes) ---");
#    println!("    00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F");
#    println!("  +------------------------------------------------");
#    for row in 0..16 {
#        print!("{:X}0| ", row);
#        for col in 0..16 {
#            let address = row * 16 + col;
#            let value = counts[address];
#            if value > 0 {
#                print!("{:02X} ", value);
#            } else {
#                print!(".. ");
#            }
#        }
#        println!();  
#    }
#    println!("-------------------------------------------------\n");
# }

fn main() {
    // Загрузка данных------------------------
    let input = vec![10, 5, 10, 10, 2, 100, 5, 42, 0, 8, 4, 3, 250, 3, 3];
    
    // Наша RAM. Каждая ячейка — это счетчик того, сколько раз встретилось число.
    // Если в задаче оценки от 0 до 255, массив должен быть на 256.
    let mut counts = [0u8; 256];

    // Сортировка -----------------------------
    // Читаем из Input и инкрементируем значение в RAM по этому адресу
    for &value in &input {
        let current_count = counts[value as usize];
        counts[value as usize] = current_count + 1;
    }
    display_ram_as_grid(&counts);

    // Результат-------------------------------
    // Просто проходим по всей памяти по порядку
    for score in 0..256 {
        // Если в ячейке не ноль, значит такое число было
        let mut how_many = counts[score as usize];
        while how_many > 0 {
            print!("{},", score); // Выводим само число (индекс ячейки)
            how_many -= 1;
        }
    }
}
```

</details>


<br>

<details>
<summary>Assembly Editor:</summary>

```bash
const jump_while 124
const jump_inc 140

0b00001100 7 0 0b00000000 #MOV INPUT to REG_0
0b01000000 8 1 0b00001000 #ALU ADD inc RAM[REG_0]

0b00001100 7 0 0b00000000 #MOV INPUT to REG_0
0b01000000 8 1 0b00001000 #ALU ADD inc RAM[REG_0]

0b00001100 7 0 0b00000000 #MOV INPUT to REG_0
0b01000000 8 1 0b00001000 #ALU ADD inc RAM[REG_0]

0b00001100 7 0 0b00000000 #MOV INPUT to REG_0
0b01000000 8 1 0b00001000 #ALU ADD inc RAM[REG_0]

0b00001100 7 0 0b00000000 #MOV INPUT to REG_0
0b01000000 8 1 0b00001000 #ALU ADD inc RAM[REG_0]

0b00001100 7 0 0b00000000 #MOV INPUT to REG_0
0b01000000 8 1 0b00001000 #ALU ADD inc RAM[REG_0]

0b00001100 7 0 0b00000000 #MOV INPUT to REG_0
0b01000000 8 1 0b00001000 #ALU ADD inc RAM[REG_0]

0b00001100 7 0 0b00000000 #MOV INPUT to REG_0
0b01000000 8 1 0b00001000 #ALU ADD inc RAM[REG_0]

0b00001100 7 0 0b00000000 #MOV INPUT to REG_0
0b01000000 8 1 0b00001000 #ALU ADD inc RAM[REG_0]

0b00001100 7 0 0b00000000 #MOV INPUT to REG_0
0b01000000 8 1 0b00001000 #ALU ADD inc RAM[REG_0]

0b00001100 7 0 0b00000000 #MOV INPUT to REG_0
0b01000000 8 1 0b00001000 #ALU ADD inc RAM[REG_0]

0b00001100 7 0 0b00000000 #MOV INPUT to REG_0
0b01000000 8 1 0b00001000 #ALU ADD inc RAM[REG_0]

0b00001100 7 0 0b00000000 #MOV INPUT to REG_0
0b01000000 8 1 0b00001000 #ALU ADD inc RAM[REG_0]

0b00001100 7 0 0b00000000 #MOV INPUT to REG_0
0b01000000 8 1 0b00001000 #ALU ADD inc RAM[REG_0]

0b00001100 7 0 0b00000000 #MOV INPUT to REG_0
0b01000000 8 1 0b00001000 #ALU ADD inc RAM[REG_0]

# -----------------------------------------------

0b10001100 0 0 0b00000000 #MOV 0 to REG_0

# jump_while-------------------------------------

# if RAM[REG_0] == 0
0b01100000   #1 opcode Cond IF_EQUAL
0b00001000   #2 arg1 source RAM[REG_0]
0            #3 arg2 source ImVal
jump_inc     #4 destination

0b00001100 0 0 0b00000111 #MOV REG_0 to OUTPUT
0b01000001 8 1 0b0001000  #ALU SUB dec RAM[REG_0]
0b10001100 jump_while 0 6 

# jump_inc---------------------------------------
0b01000000 0 1 0b00000000 #ALU ADD inc REG_0

0b10001100 jump_while 0 6 
#------------------------------------------------
```
</details>

 
![Delicious Order](/Computer-Science-Bookshelf/img/tc/Delicious_Order.gif)

---

## Dancing Machine

> Задача:
> 
> На этом уровне вы получите только один вход, мы называем это начальным семенем (`seed`). Семя проходит через следующие шаги для получения псевдо-случайного числа (ГПСЧ).
> 
> В алгоритме ниже:
> * "shl 1" означает сдвиг влево один раз
> * "shl 2" означает сдвиг влево дважды
> * "shr 1" означает сдвиг вправо один раз
> 
> ```
> 8-ми битовый xorshift RNG:
> 
> temp1     = seed xor (seed shr 1)
> temp2     = temp1 xor (temp1 sh 1)
> next_seed = temp2 xor (temp2 shr 2)
> ```
> 
> Затем, выведите `next_seed` по модулю 4, чтобы двигать робота. И наконец, используйте `next_seed` (до модуля) как `seed`, чтобы получить следующее число в танцевальной последовательности и повторите.
> 
> (Обратите внимание, что начальное семя никогда не будет 0)


<details>
<summary>Прототип:</summary>

```rust,editable
fn main() {
    let mut seed: u8 = 29; 
    
    println!("Начальное семя: {:08b} ({})", seed, seed);
    println!("---------------------------------------");

    for i in 1..=10 {
        // 1: temp1 = seed xor (seed shr 1) 
        let shr_1 = seed >> 1;// DIV 2
        let temp1 = seed ^ shr_1;

        // 2: temp2 = temp1 xor (temp1 shl 1) 
        // заменяем shl 1 на сложение (temp1 + temp1)
        // имитировать 8-битное переполнение как в LEG
        let shl_1 = temp1.wrapping_add(temp1); 
        let temp2 = temp1 ^ shl_1;

        // 3: next_seed = temp2 xor (temp2 shr 2) 
        let shr_2 = temp2 >> 2;// DIV 4
        seed = temp2 ^ shr_2;

        // 4: Получаем направление (next_seed % 4) 
        // Используем побитовое И (AND 3), чтобы оставить только 2 нижних бита
        let direction = seed & 3;

        println!("Шаг {}: Направление = {}, Новый seed = {:08b} ({})", 
                 i, direction, seed, seed);

        // По условию, если seed станет 0, стоп
        if seed == 0 { break; }
    }
}
```

</details>

<br>

<details>
<summary>Assembly Editor:</summary>

```bash
# seed REG_0
# temp1 REG_1
# temp2 REG_2

const jump_start 4

0b00001100 7 0 0b00000000 #MOV INPUT to REG_0

# 1: temp1 = seed xor (seed shr 1) 
# seed shr 1
 
0b01010100 # ALU DIV REG_1=REG_0/2
0b00000000
2
0b00000001

# temp1 = seed xor (seed shr 1) 
0b00000101 # ALU XOR
0b00000000 # seed 
0b00000001 # temp1
0b00000001 # temp1

# 2: temp2 = temp1 xor (temp1 shl 1) 
# temp1 shl 1
0b00000000 # ALU ADD
0b00000001 # temp1
0b00000001 # temp1
0b00000010 # temp2

# temp2 = temp1 xor (temp1 shl 1)
0b00000101 # ALU XOR
0b00000001 # temp1 
0b00000010 # temp2
0b00000010 # temp2

# 3: next_seed = temp2 xor (temp2 shr 2) 
# temp2 shr 2
0b01010100 # ALU DIV REG_0=REG_2/4
0b00000010
4
0b00000000 # next_seed

# next_seed = temp2 xor (temp2 shr 2) 
0b00000101 # ALU XOR
0b00000010 # temp2 
0b00000000 # next_seed
0b00000000 # next_seed

# 4: direction = (next_seed % 4) 
0b01000010 # ALU AND
0b00000000 # next_seed 
3          # mask 2 bit
0b00000111 # OUTPUT

0b10001100 jump_start 0 6 
```

</details>

<video controls width="100%" playsinline preload="metadata">
    <source src="/Computer-Science-Bookshelf/img/tc/Dancing_Machine.mp4" type="video/mp4">
    Ваш браузер не поддерживает видео.
</video>

---

## Planet Names

> Задача:
> 
> Мы попросили стажера напечатать названия планет обычным почерком. К сожалению, он забыл написать каждое название с заглавной буквы.
> 
> Входные данные на этом уровне представляют собой символы из списка названий планет, закодированные в формате ASCII (см. руководство). Каждое название разделено пробелом, имеющим числовое значение 32.
> 
> Замените первую букву каждого слова на соответствующую заглавную букву.
> 
> (Вводимые символы: строчные буквы от a до z, пробел, апостроф и тире)

 
![ASCII encoding](/Computer-Science-Bookshelf/img/tc/ASCII_encoding.png)

Входные данные содержат пробелы (32), апострофы (39) и тире (45). Их трогать нельзя!

Чтобы сделать букву заглавной, нужно вычесть 32 из ее кода.

<details>
<summary>Прототип:</summary>

```rust,editable
fn main() {
    let input = "mars earth jupiter's-moon alpha-9";
     
    let mut is_new_word = true; 

    for c in input.bytes() {
        let mut char_code = c;

        // Проверяем, не пробел ли это (ASCII 32)
        if char_code == 32 {
            is_new_word = true; // Следующий символ будет началом слова
            // Выводим пробел как есть
        } 
        // Если это начало слова, пытаемся сделать букву заглавной
        else if is_new_word {

            // Сбрасываем флаг, так как первая буква обработана
            is_new_word = false;

            // Проверяем, что это строчная буква (a-z: от 97 до 122)
            if char_code >= 97 {
                char_code -= 32; // Магия ASCII: 'a'(97) -> 'A'(65)
            }
        }
        print!("{}", char_code as char);
    }
}
```
</details>

<br>

<details>
<summary>Assembly Editor:</summary>

```bash
# REG_0 char_code
# REG_1 is_new_word

0b10001100 1 0 1 #MOV 0 to REG_1
label jump_start
0b00001100 7 0 0 #MOV INPUT to REG_0

# opcode Cond IF_EQUAL if char_code == 32
0b01100000 0 32 jump_new_word    

# opcode Cond IF_EQUAL if is_new_word == 0
0b01100000 1 0 jump_print 

0b10001100 0 0 1 #MOV 0 to REG_1

# opcode Cond 34 IF_LESS if char_code < 97 
0b01100010 0 97 jump_print  
0b01000001 0 32 0 #ALU SUB char_code=char_code-32 

0b10001100 jump_print 0 6 

#jump_new_word----------------------
label jump_new_word
0b10001100 1 0 1 #MOV 1 to REG_1

# jump_print-------------------------
label jump_print
0b00001100 0 0 7 #MOV REG_0 to OUTPUT
0b10001100 jump_start 0 6 
```
</details>

---

## Tower of Alloy

> Задача:
> 
> Нам нужно, чтобы вы переместили груды радиоактивных дисков из старого реактора. Только следите за тем, чтобы не класть больший диск  поверх меньшего, иначе весь корабль взорвется.
> 
> Первые 4 входных параметра дадут вам следующий результат в указанном порядке:
> * disk_nr - Наибольший номер диска в стопке (от 2 до 4)
> * source - Номер какого местоположения нужно переместить из пункта назначения 
> * destination - Куда переместить кучу
> * spare - третье место, которое не является ни источником, ни пунктом назначения.
> 
> Управляйте краном с помощью следующих выходных сигналов:
> 0. Переместите магнит в точку 0
> 1. Переместите магнит в точку 1
> 2. Переместите магнит в точку 2
> 5. Включите или выключите магнит
> 
> Управляйте магнитом вручную, используя клавиши со стрелками для перемещения и клавишу Enter для переключения.
> 
> Tower of Hanoi algorithm:
> 
> ```
> func hanoi(disk_nr, source, dest, spare):
>     if disk_nr is 0:
>         move disk from source to desk
>     else:
>         hanoi(disk_nr - 1, source, spare, dest)
>         move disk from source to dest
>         hanoi(disk-nr -1, spare, dest, source)
> ```
> 
> СОВЕТ: Сохраняйте значения регистров в стеке перед вызовом функции, которая их изменяет.

<details>
<summary>Рекурсия и стек:</summary>
 
Проблема глобальных данных при использовании с рекурсии в том, что рекурсия разматываясь назад должна использовать данные которые были при ее вызове. Функция, которая вызвала рекурсию, надеется продолжить работу со своими данными, но обнаруживает, что в глобальной переменной находится «мусор», оставленный последним, самым глубоким вызовом.


```rust,editable
#![allow(static_mut_refs)]

static mut VALUE: u8 = 0;
static mut STEP: i32 = 0;

unsafe fn command() {
    if STEP == 3 { 
        return;
    }
    
    STEP += 1;
    let local_step = STEP;
     
    println!("{}(STEP {}) VALUE {:?}","-".repeat(local_step as usize), local_step, VALUE);
    
    VALUE = VALUE + 10;
    command();
  
    println!("{}(STEP {}) Вернулись из рекурсии: VALUE {:?}","-".repeat(local_step as usize), local_step, VALUE);
 
    println!("{}(STEP {}) Отчет: Пришло VALUE {} ед.","-".repeat(local_step as usize), local_step, VALUE);
  
    if local_step == 1 {println!(">>> {VALUE} == 0 <<<");}
    if local_step == 2 {println!(">>> {VALUE} == 10 <<<");}
    if local_step == 3 {println!(">>> {VALUE} == 20 <<<");} 

    STEP -= 1; // Уменьшаем счетчик при выходе, чтобы другие ветки могли работать
}

fn main() {
    unsafe {
        VALUE = 0;
        command();
    }
}
```

Значит нам нужно сохранить состояние переменных **перед** вызовом рекурсивной функции. Эти сохраненные данные и формируют фрейм (кадр) стека для текущего вызова функции. Если бы не было команды `PUSH`, мы бы сохраняли данные в `RAM` по адресу, который увеличивается. А после рекурсивной функции восстановить обратно данные командой `POP`, которая вернет их в обраном порядке.

```rust,editable
#![allow(static_mut_refs)]

static mut VALUE: u8 = 0;
static mut STEP: i32 = 0;
static mut STACK: Vec<u8> = Vec::new();

unsafe fn command() {
    if STEP == 3 { 
        return;
    }
    
    STEP += 1;
    let local_step = STEP;
     
    // Сохраняем наше текущее состояние в стек
    STACK.push(VALUE);// 0->10 

    println!("{}(STEP {}) VALUE {:?}","-".repeat(local_step as usize), local_step, VALUE);
    
    println!("{}STACK {:?}","-".repeat(local_step as usize),STACK);
    VALUE = VALUE + 10;
    command();
 
    // Возвращаем наше состояние из стека
    VALUE = STACK.pop().unwrap();//<-20<-10<-0

    println!("{}(STEP {}) Вернулись из рекурсии: VALUE {:?}","-".repeat(local_step as usize), local_step, VALUE);
 
    println!("{}(STEP {}) >>>Отчет: Пришло VALUE {} ед.<<<","-".repeat(local_step as usize), local_step, VALUE);
  
    if local_step == 1 {println!(">>> {VALUE} == 0 <<<");}
    if local_step == 2 {println!(">>> {VALUE} == 10 <<<");}
    if local_step == 3 {println!(">>> {VALUE} == 20 <<<");} 

    STEP -= 1; // Уменьшаем счетчик при выходе, чтобы другие ветки могли работать
}

fn main() {
    unsafe {
        VALUE = 0;
        command();
    }
}
```
</details>
 
<br> 
<details>
<summary>Прототип:</summary>

```rust,editable
fn command(source: u8, dest: u8) {
    println!("OUT: {} (Едем к источнику)", source); // к источнику
    println!("OUT: 5 (ВКЛ магнит)");  // поднять диск
    println!("OUT: {} (Едем к цели)", dest); // к цели
    println!("OUT: 5 (ВЫКЛ магнит)");  // опустить диск
    println!("---");
}

fn hanoi(disk_nr: i32, source: u8, dest: u8, spare: u8) {
    println!("disk_nr={disk_nr}");
    if disk_nr == 0 {
        command(source, dest);
    } else {
        // 1. move(disk_nr - 1, source, spare, dest)
        // source=>source, spare=>dest, dest=>spare
        hanoi(disk_nr - 1, source, spare, dest);
        
        // 2. Перемещаем сам диск
        command(source, dest);
        
        // 3. move(disk_nr - 1, spare, dest, source)
        // spare=>source, dest=dest, source=>spare
        hanoi(disk_nr - 1, spare, dest, source);
    }
}

fn main() {
    // Входные данные 
    let disk_nr = 2; // Максимальный номер (от 2 до 4)
    let source = 0;
    let dest = 2;
    let spare = 1;
    
    // Запускаем рекурсию
    hanoi(disk_nr, source, dest, spare);
}
```
</details>

<br>
<details>
<summary>Assembly Editor:</summary>

```bash
const CALL 0b00001000
const RET 0b00010000
const move_to_0 0
const move_to_1 1
const move_to_2 2
const toggle_on_off 5

const disk_nr 0b00000001 # disk_nr 
const source 0b00000010 # source
const destination 0b00000011 # destination
const spare 0b0000100 # spare
const temp_var 0b00000000 # REG_0
 
# load instruction
0b00001100 7 0 disk_nr #MOV INPUT to disk_nr
0b00001100 7 0 source #MOV INPUT to source
0b00001100 7 0 destination #MOV INPUT to destination
0b00001100 7 0 spare #MOV INPUT to spare

#-------------------------------------------------------
label jump_hanoi
#Cond IF_EQUAL if disk_nr == 0 
0b01100000 disk_nr 0 jump_command  
 
# save local vars 
0b00010111 disk_nr 0 0 #PUSH disk_nr to stack 
0b00010111 source 0 0 #PUSH source to stack 
0b00010111 destination 0 0 #PUSH destination to stack 
0b00010111 spare 0 0 #PUSH spare to stack 

# set args
# 1. move(disk_nr - 1, source, spare, dest)
# source=source, dest=spare, spare=dest
0b01000001 disk_nr 1 disk_nr #ALU SUB dec disk_nr
0b00001100 destination 0 temp_var #MOV dest to temp_var
0b00001100 spare 0 destination #MOV spare to dest
0b00001100 temp_var 0 spare #MOV dest to spare

CALL 0 0 jump_hanoi

# restor local vars
0b00010101 0 0 spare #POP to spare
0b00010101 0 0 destination #POP to destination
0b00010101 0 0 source #POP to source
0b00010101 0 0 disk_nr #POP to disk_nr

# 2. Перемещаем сам диск
CALL 0 0 jump_command

# save local vars 
0b00010111 disk_nr 0 0 #PUSH disk_nr to stack 
0b00010111 source 0 0 #PUSH source to stack 
0b00010111 destination 0 0 #PUSH destination to stack 
0b00010111 spare 0 0 #PUSH spare to stack 

# set args
# 3. move(disk_nr - 1, spare, dest, source)
# spare=>source, dest=dest, source=>spare
0b01000001 disk_nr 1 disk_nr #ALU SUB dec disk_nr
0b00001100 spare 0 temp_var #MOV spare to temp_var
0b00001100 source 0 spare #MOV source to spare
0b00001100 temp_var 0 source #MOV spare to source
 
CALL 0 0 jump_hanoi

# restor local vars
0b00010101 0 0 spare #POP to spare
0b00010101 0 0 destination #POP to destination
0b00010101 0 0 source #POP to source
0b00010101 0 0 disk_nr #POP to disk_nr

RET 0 0 0 
#-------------------------------------------------------
label jump_command
0b00001100 source 0 7 #MOV source to OUTPUT
0b10001100 toggle_on_off 0 7 #MOV toggle_on_off to OUTPUT
0b00001100 destination 0 7 #MOV destination to OUTPUT
0b10001100 toggle_on_off 0 7 #MOV toggle_on_off to OUTPUT
RET 0 0 0 

```
</details>


![Tower of Alloy](/Computer-Science-Bookshelf/img/tc/Tower_of_Alloy.gif)

---

## Water World

> Задача:
> 
> Нам нужна ваша помощь в поиске подходящего места для водной горки «Пиратский спуск». В частности, нам нужно место, способное вместить большой объем воды.
> 
> Ландшафт состоит из 16 столбцов. Чтобы получить высоту ландшафта в каждом столбце слева направо, прочтите входные данные 16 раз.
> 
> Затем в качестве ответа выведите **общий объем**, который может вместить ландшафт.


Суть в том, что вода задерживается в "ямах". Чтобы понять, сколько воды над конкретным столбцом, нам нужно знать самый высокий столб слева от него и самый высокий столб справа от него.

Вода над столбцом не может быть выше, чем самый низкий из двух его "крайних" соседей (левого и правого максимумов). Двигаясь с двух сторон, мы всегда уверены, что если мы обрабатываем левую сторону, то где-то справа точно есть стена не ниже текущей max_L.

Алгоритм "Два указателя": 

Идти с двух сторон одновременно.

1. Ставим один указатель (`L`) на начало (0), другой (`R`) на конец (15).
2. Храним переменные `max_L` (максимальная высота слева) и `max_R` (максимальная высота справа).
3. **Логика:**
    * Если `height[L]` меньше `height[R]`:
        * Если `height[L]` больше или равен `max_L`, обновляем `max_L`.
        * Иначе (если текущий столб ниже максимума), добавляем в ответ разницу: `max_L - height[L]`.
        * Сдвигаем `L` вправо.
    * Иначе (если `height[R]` меньше или равен `height[L]`):
        * Если `height[R]` больше или равен `max_R`, обновляем `max_R`.
        * Иначе, добавляем в ответ разницу: `max_R - height[R]`.
        * Сдвигаем `R` влево.

<br> 
<details>
<summary>Прототип:</summary>

```rust,editable
# fn draw_world(landscape: &[u8; 16]) {
#     let n = landscape.len();
#     let mut water_level = [0u8; 16];
#     
#     // Предварительно рассчитываем уровень воды для визуализации
#     // (Используем ту же логику: min(max_L, max_R))
#     for i in 0..n {
#         let max_l = *landscape[..=i].iter().max().unwrap_or(&0);
#         let max_r = *landscape[i..].iter().max().unwrap_or(&0);
#         water_level[i] = std::cmp::min(max_l, max_r);
#     }
# 
#     let max_h = *landscape.iter().max().unwrap_or(&0);
# 
#     for level in (1..=max_h).rev() {
#         print!("{:2} | ", level); // Шкала высоты слева
#         for i in 0..n {
#             if landscape[i] >= level {
#                 print!("█ "); // Земля
#             } else if water_level[i] >= level {
#                 print!("≈ "); // Вода
#             } else {
#                 print!("  "); // Пустота
#             }
#         }
#         println!();
#     }
# 
#     println!("   +{}", "――".repeat(n));
#     print!("     ");
#     for i in 0..n { print!("{:X} ", i); } // Индексы в hex
#     println!("\n");
# }
fn solve_water_world(ram: [u8; 16]) -> u32 {
    let mut i_left = 0;           
    let mut i_right = 15;         
    let mut max_l = 0;          
    let mut max_r = 0;          
    let mut total_water = 0;    

    while i_left <= i_right {
        // Логика: обрабатываем ту сторону, где высота меньше, 
        // так как вода ограничена самым низким "краем"
        if ram[i_left] <= ram[i_right] {
            if ram[i_left] >= max_l {
                max_l = ram[i_left]; // Обновили левый пик
            } else {
                total_water += (max_l - ram[i_left]) as u32; // Нашли яму!
            }
            i_left += 1;
        } else {
            if ram[i_right] >= max_r {
                max_r = ram[i_right]; // Обновили правый пик
            } else {
                total_water += (max_r - ram[i_right]) as u32; // Нашли яму!
            }
            i_right -= 1;
        }
    }
    total_water
}

fn main() {
    // Пример ландшафта (16 столбцов)
    let landscape: [u8; 16] = [4,6,1,4,6,5,1,4,1,2,6,5,6,1,4,2];
    draw_world(&landscape);
    
    let result = solve_water_world(landscape);
    println!("Общий объем воды: {}", result);
}
```

</details>
<br>
<details>
<summary>Assembly Editor:</summary>

Для сохранения промежуточных расчетов нам нужен один из регистров, но данных заняли уже все регистры.
Поэтому хранить значение результата `total` будем в `RAM[16]`  

```bash
const CALL 0b00001000
const RET 0b00010000
const i_left 0b00000001  
const i_right 0b00000010
const max_l 0b00000011
const max_r 0b00000100
const temp_var 0b00000101
const i_total 16 # результат в RAM[i_total]
const ram 0b00001000
const sp 0b00000000

0b10001100 15 0 i_right #MOV 15 to i_right

label jump_load_data
0b01100100 sp 15 jump_reset_sp #Cond IF_GREATER sp > i_right
0b00001100 7 0 ram #MOV INPUT to RAM[SP]
0b01000000 sp 1 sp #ADD SP+=1   
0b10001100 jump_load_data 0 6 #jump

# reset SP
label jump_reset_sp
0b10001100 i_total 0 sp #MOV 0 to sp
0b10001100 0 0 ram #MOV 0 to RAM[i_total]
0b10001100 0 0 sp #MOV 0 to sp

label jump_while
0b00100100 i_left i_right jump_output #Cond IF_GREATER i_left > i_right

0b00001100 i_right 0 sp  #MOV i_right to sp
0b00001100 ram 0 temp_var #MOV RAM[i_right] to temp_var
0b00001100 i_left 0 sp  #MOV i_left to sp
0b00100100 ram temp_var jump_l_greater_r #Cond IF_GREATER RAM[i_left] > RAM[i_right]

0b00100010 ram max_l jump_inc_total_l #Cond IF_LESS RAM[i_left] < max_l
0b00001100 ram 0 max_l  #MOV RAM[i_left] to max_l
0b10001100 jump_inc_left 0 6 #jump

label jump_inc_total_l
0b00000001 max_l ram temp_var #ALU SUB temp_var=max_l-RAM[i_left] 
0b10001100 i_total 0 sp  #MOV i_total to sp
0b00000000 ram temp_var ram #ALU ADD RAM[i_total]+=max_l-RAM[i_left]

label jump_inc_left
0b01000000 i_left 1 i_left #ALU ADD i_left+=1 

0b10001100 jump_while 0 6 #jump

#----------------------
label jump_l_greater_r
0b00001100 i_right 0 sp  #MOV i_right to sp
0b00100010 ram max_r jump_inc_total_r #Cond IF_LESS RAM[i_right] < max_r
0b00001100 ram 0 max_r  #MOV RAM[i_right] to max_r
0b10001100 jump_dec_right 0 6 #jump

label jump_inc_total_r
0b00000001 max_r ram temp_var #ALU SUB temp_var=max_r-RAM[i_right] 
0b10001100 i_total 0 sp  #MOV i_total to sp
0b00000000 ram temp_var ram #ALU ADD RAM[i_total]+=max_r-RAM[i_right] 

label jump_dec_right
0b01000001 i_right 1 i_right #ALU SUB i_right-=1 

0b10001100 jump_while 0 6 #jump

label jump_output
0b10001100 i_total 0 sp #MOV i_total to sp
0b00001100 ram 0 7 #MOV RAM[i_total] to OUTPUT

```
</details>

![Water World](/Computer-Science-Bookshelf/img/tc/Water_World.gif)

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
 
<script>
document.addEventListener("DOMContentLoaded", function () {
  document.querySelectorAll('.abraetable.ace_editor').forEach(el => {
    const editor = ace.edit(el);
    const scroller = editor.container.querySelector('.ace_scroller');
    scroller.addEventListener('wheel', function(e) {
      e.preventDefault();
      scroller.scrollTop += e.deltaY;
    });
  });
});
</script>

<style>
table {
  margin: 0px !important;  
  border-collapse: collapse;
}
.abraetable.ace_editor {
  height: 1000px !important;
}
.abraetable  {
    border: 2px solid purple;  
    border-radius: 8px;      
    padding: 10px;          
    background-color: #f5f5f5;  
    box-shadow: 0 4px 6px rgba(0,0,0,0.1);  
    border-color: #8a2be2; 
}
</style> 



 