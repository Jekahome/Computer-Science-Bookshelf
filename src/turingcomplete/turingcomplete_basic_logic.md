
# Turing Complete. Basic logic

* [Turing Complete](https://turingcomplete.game/) — это игра о компьютерных науках.
* [Turing Complete youtube channel](https://www.youtube.com/@TuringCompleteGame)
* [Nari youtube channel](https://www.youtube.com/@nari1774/videos)
* [Процессор RV32i](https://github.com/BenjaminSchaaf/turning-complete-riscv) Это полноценная реализация RISC-V CPU внутри игры, на который можно запускать реальный компилируемый код (например, Rust). Игра превращается в мини-симулятор архитектуры компьютера.
* [MegaIng/turing-complete-interface](https://github.com/MegaIng/turing-complete-interface) Это библиотека для взаимодействия со схемами, созданными в игре Turing Complete

> [!IMPORTANT]
> **Существа, которые смогут завершить создание компьютера, по закону считаются разумными.**
>

Всё в компьютере может быть построено из базового компонента, называемого логическим элементом NAND. Вам предстоит решить ряд головоломок, чтобы проложить путь от элементов NAND к арифметике, памяти и вплоть до полноценных архитектур центрального процессора. Пройдя эту игру, вы получите глубокое понимание взаимосвязи между ассемблером, наборами инструкций ЦП и базовыми компонентами. А также поймете, как работают такие концепции программирования, как условные операторы, циклы и функции, на уровне ассемблера и аппаратного обеспечения.

![you learn this](/Computer-Science-Bookshelf/img/turingcomplete_1.png)

Игра построена на мощном симуляторе, который предоставляет вам полную свободу в прохождении уровней или создании собственных компьютеров. Подключайте экраны, таймеры, звук, ввод с клавиатуры и сетевые компоненты, чтобы создать все, что захотите. Вы даже можете разработать уникальный язык ассемблера для своего компьютера.

Режим кампании шаг за шагом проведет игрока к созданию самого простейшего 8-ми битного АЛУ с возможностью программировать.

А при желании в режиме песочница можно от одного транзистора развиться до создания полноценного марио, тетриса, сапера, змеек и прочих радостей эпохи денди. Что собственно уникумы тут и делают.

>
> Path save files for linux mint:
>
> `/home/$USER/.wine/drive_c/users/$USER/AppData/Roaming/Godot/app_userdata/Turing Complete`
>
> [tcsaveeditor](https://github.com/narikiro/tcsaveeditor.py) инструмент для работы с сохранениями Turing Complete
>

---

# Базовая логика

## Логические вентили

Логические вентили можно комбинировать для решения более сложных задач, например для выполнения простых арифметических операций.

![Логические вентили](/img/Логические_вентили.png)

[Логические вентили OR, AND, XOR (www.falstad.com/circuit)](https://www.falstad.com/circuit/circuitjs.html?ctz=CQAgjCAMB0l3BWcMBMcUHYMGZIA4UA2ATmIxAUgpABZsKBTAWjDACgAPEJwjYimkh4p+CGnlogUNEADkA8gBU2YDBKY1ihChm1NsCFDu3aIqKFFhIw8eJchG09ozERQ2Ac24GjhfN8MQQhoqULYAJW5NbQRdKK0KMBcLEOpQywQ2ACdvSBkEP1yZQmkLG0hPIqDS-Tyg1LCuHjx6bGx1QmwIbBoIGVKADXlwzm5eIxYSMew9MEJtfplh0eaZMB7ptexyRZAAQVkAERUS7jAaGSY8KhYLqIkjABMGADMAQwBXABsAFyYvhiPcAWUKwdhzCZgYhGc6XKEw6FSEDPd7fP4AoFmEFgk4TaRUGbqfG0OZIlGfX7-QHA9KucFqMY1WHcMRwu6mSxY1zWWxwZwUUEOeyQJAVVTqOiQu76QK3Yo06BcqzIXn8yj81wi9zi7goApnaWqWbshVKrXlWxqwUuWBaiqRYRsy6EfVykG0Kii7GZLxMVlnJIs6XXEGVGWQwPh7ghsK+o0ByFxK60tgAd0ZcMjLtmgYq6Y02AjExowQT7nzepzeLw6lYLjYABkxq66829IX3e8vgBnBhpRtRUssSMly4d9Jd3v9ptMFA1suz+f6ZJUSd90VsACyBrhc+4CRYe6oKGgmRybsPteD6XKYeZGivY5XESiHYvo537tSXtBmQdlc-RdHy-T13RgTJ839S8WUjJIJDzGDIT3P1YMRBCUMhRFlxueFy28JwWCwwscKPNN8JI9QfDOUj82Is4sPvXD0MYhjDRos4kzgwCuPQlBSgPXCEgqONEVwwjHH8MJz0E2ZRLuKhb23cSQAEu4hKkU8B30GZwEjNpTDQiw12ne4FJHEMmKMt4e3XdwZxKNYs1KSzV2sqcN1oupW200xczIjRxF04sLL8yDRKcxz619PjIt1Zz5PcX1CSC7xCAkZlY0QlKMPABL0NwGQQ1qGQhP85KDwKlTtHQi4JCK2qqrwnKDwa0qHQQREiuCKgYxSUDaU0nVipAIqYpGqgORgM0eVVVwjHVObhQ3IaGqK5KQ0mxVsXNXlrQFDVbWWhk-U6m4EEDDbTW2mbLUWhbYBtNwKiU4k6OUujj006LUl6PRiUlUNz1egjXrSso4DKpw3uBsIZznCQqKA2hx2BYyNxnWJtERzHkeSCA0aaujEfhkAqPQ-1sbiMmw1+0nZVp5LMoChH6cCxn3Hau43pxj6+rSb1-Ia-SolJSqarZuoNFFnSxWOomCIZpxNrMWBSDV9X1YwFB6G5FVboe-bFs1awVGOimFbZpWrtcDXbdILWdeVC07DuvbjfALToiCfwTt8SSrJs-sIQCebCmGgpj2RV4KXRakuXsdglMqiPdX1FPPsyHViVbOjW2V+w7dth37Bul2DfuoV3fpBG8Fij88+t1XC817WS71suhQrx7zRfbPA1w1t0m-MDNPTXO93aNZ2MnlKZ54sj673Qx0tIrxc65px40y+u7nruIwnTbO7jQdL5LIueucCDKBxP3L+n8MaFJAAmKibKjmSox-Ubc2znpALWepUF0HVdIJ5M6nHGONf+TgQxPGjmiKkmIR6QHYF4ABd9oEKUKFJTBUD0EZRVC+ZejViEoyoMPAaPogha3AHuSBcwUw6mIQkSBLCFQ2mkHgDABQUBgDwMQEUFxKCOx2rNA2TgjaHQ9lwKEEAxBSA6joSQpRZAHGOOmYhw4jBey0XhYWLA4iaK3ufHSuj9HGLHpLHOpjB4vlptBGe0Eh79QFm-Wu+4sbuMmGUZ+P8TJiQSGJUkT8X4L1JEwBIjjSqHxqIErCpUvA6ISDo0smVKoRKxpLVJHMpBMj3EEo8fMfwZFNhIexXEdFcXzjbZuati662dnyV2B03DV1ybuBGVi6GN0gLUuprcGm7StC0nuMi8CyWPNgfgLAuHKJkAoEYTCw7MgwFYk0KsOHRC4fwlB2xcCQFSU7IZi0JEGyrpUVZmYjCXOjCmSIKc3QPNvs4-mv5sj-1lAwj5Ex1K3jQbKBI3CJi9TFKcG5kwgGfMKHA1ElIMQ0m2onb5ZxCjXBwtgjSEEKDIVbA82xGicWGJxX5LOqQqLCyovnbAioxAHK1qoPZcxiDMCSI9UuTTy5uykbLMppZyVg0pT0vp9sBlHLEZ3LlrSTFYx0slRm0rSYCoFTLXuZLAi3zJkUkeVCKXuOFiEFMa8BV6rBga0Mh8yXuI1e4hCH4Z5exnq-KQ-gZ63w-BOPx6NaCRPcV7d139A4eVoHywIFNAgVByGG+au8mmEK8P6BqCaQW4gEDIM1CbAowpjoghFgp2AyJwFIcZUgaCOH4XMkAAwFmlOMKTJw6C6KbU2eMyAuBeDEA6lgJI5BBniokiMk2aCoaBAbTanJPMnA40wCuD0rySk5GAaTOoi72Z-P-kqiQK6x2guuVDZdAq6hZoQfC+OdItzrs6YVUWdQM5kSjbW6mYB1TeK4oEwpyRMjbjfafKe8FMVaVJPeUsBD8aersii7KwHcwBvctqKF1EJACXfRYT9jVoKMXfZpGc4SgNOh8aE7DVYxhXPw2BioAAjMY85APQuwWwSjXzwlpWmbwjcX7wowjyGZZIYCBzjN-SNQDhTQOBvA-xxqeBSS-N8aJsU50RpQaMKQGE0GP3nuUwh5AqQnH-qbDWHCdwWhShkCEsjfGiqtiM2WUzsn1OeLWFzMdvGZwdjNTwNNsbXK2Zc0YZK7nFV-pE7Bij4BCgpNmXqfgIXGOmHwPQWcT73AMehfqGw7jEvRbC4E-A-R9SZfIQeNK2h6VwafKTdxPqTNIg+AAOxRAASxq9STOCB6D6c0xpnTRhasNaa0CTIABJfC3tEMmH9uBCgdmRu0IkH4P9znhtzeI9Nj1tmgA)


### Законы де Моргана: Позволяют упрощать и инвертировать сложные условия.

Исходное тождество `!(A && B)` эквивалентно ≡ `!A || !B`
* Если теперь отрицать обе части, получим:
    * `!(!(A && B))` ≡ `!(!A || !B)`
    * убираем двойное отрицание слева, получим:
    *  `A && B` ≡ `!(!A || !B)`

Исходное тождество `!(A || B)` ≡ `!A && !B`
* Если теперь отрицать обе части, получим: 
    * `A || B` ≡ `!(!A && !B)`

---

### AND — `A && B`

**AND** в логике и программировании означает И (логическое умножение или конъюнкция). Это логическая операция, которая выдает истину (True), только если оба операнда истинны. Если хотя бы один из операндов ложен, то результат — ложь. Так же объединение [NAND + NOT эквивалентно AND](https://www.falstad.com/circuit/circuitjs.html?ctz=CQAgjCAMB0l3BWcMBMcUHYMGZIA4UA2ATmIxAUgoqoQFMBaMMAKDAQj0IBZwNCQpFHwFVhtFgFlwYNCJkJhYflBApoCFgElBxJSuZzloqBoosAMiDzZhTARjy97qiADMAhgBsAznWqQlta24A5Ooa4gnr7+SIHSZCbMiqpiGmyU1jyqialq1JoAHtbEEAyE2ILcZYRUvCi8AIIAcgAiLMV4GMTISmCQlf1I9bzNLa0AOj4TAA5TzQDyACpTEwCOU+MsAO5ZvGB6wUqHgbs2SgLn4Cc7RxFc+6K3DyAuTnivYE9nIW-cHy5TtZ-p9LiCvlQgS8IcCPjDAkA).

* AND можно получить через NAND `A && B ≡ !(A NAND B)`
* AND можно получить через NOR `A && B ≡ !(!A || !B) ≡ (A NOR A) NOR (B NOR B)`

|AND| 0 | 1 |
|:--|:--|:--|
| 0 | 0 | 0 |
| 1 | 0 | 1 |

```rust
# const NAME_WIDTH: usize = 4;
# fn print_truth_table(name: &str, gate: fn(u8, u8) -> u8) {
#     println!("|{:^width$}| 0 | 1 |", name, width = NAME_WIDTH);
#     println!("|{:-^width$}|---|---|", "", width = NAME_WIDTH);
#
#     for b in 0..=1 {
#        println!(
#            "|{:^width$}| {} | {} |",
#            b,
#            gate(0, b),
#            gate(1, b),
#            width = NAME_WIDTH
#        );
#     }
#     println!("");
# }
#
# fn or(a: u8, b: u8) -> u8 { a | b }
# fn not(x: u8) -> u8 {!x & 1}

fn and(a: u8, b: u8) -> u8 {
    a & b
}

fn and_by_using_or(a: u8, b: u8) ->u8{
    not(or(not(a),not(b)))
}

fn main() {
    print_truth_table("AND", and);

    print_truth_table("AND", and_by_using_or);
}
```


<div class="sim-wrapper" data-circuit-id="6">
  <button class="sim-fullscreen-btn" data-circuit-id="6">⛶</button>
  <iframe 
      id="6"
      data-circuit-id="6"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=1&startCircuit=/6_and.txt"
      loading="lazy">
  </iframe>
</div> 

---

### OR — `A || B`

**OR** Это логическая операция, которая выдает истину (True), если хотя бы один из операндов истинен. Мат. синтаксис **`A + B`**

* OR можно получить через AND `(A || B) ≡ !(!A && !B)`
* OR можно получить через NAND `(A || B) ≡ (A NAND A) NAND (B NAND B)`
* OR можно получить через NOR `(A || B) ≡ !(!(A || B))` 

|OR | 0 | 1 |
|:--|:--|:--|
| 0 | 0 | 1 |
| 1 | 1 | 1 |

```rust
# const NAME_WIDTH: usize = 4;
# fn print_truth_table(name: &str, gate: fn(u8, u8) -> u8) {
#     println!("|{:^width$}| 0 | 1 |", name, width = NAME_WIDTH);
#     println!("|{:-^width$}|---|---|", "", width = NAME_WIDTH);
#
#     for b in 0..=1 {
#        println!(
#            "|{:^width$}| {} | {} |",
#            b,
#            gate(0, b),
#            gate(1, b),
#            width = NAME_WIDTH
#        );
#     }
#     println!("");
# }
# fn and(a: u8, b: u8) -> u8 { a & b }
# fn nand(a: u8, b: u8) -> u8 { not(a & b)}
# fn not(x: u8) -> u8 { !x & 1 }

fn or(a: u8, b: u8) -> u8 {
    a | b
}
 
fn or_by_using_and(a: u8, b: u8) -> u8 { 
    not(and(not(a),not(b)))
}

fn or_by_using_nand(a: u8, b: u8) -> u8 { 
    nand(nand(a, a), nand(b, b))
}

fn main() {
    print_truth_table("OR", or);
    print_truth_table("OR", or_by_using_and);    
    print_truth_table("OR", or_by_using_nand);
}
```

<div class="sim-wrapper" data-circuit-id="7">
  <button class="sim-fullscreen-btn" data-circuit-id="7">⛶</button>
  <iframe 
      id="7"
      data-circuit-id="7"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=1&startCircuit=/7_or.txt"
      loading="lazy">
  </iframe>
</div> 

---

### XOR — `A XOR B`

**XOR** (от англ. Exclusive OR/Исключающее ИЛИ). Это логическая операция, которая выдает истину (True), если только один из операндов истинен, и ложь (False), если оба операнда одинаковы (оба истинны или оба ложны). Математически записывается как `A ⊕ B`

* XOR можно получить через AND + OR + NOT: `A XOR B = (A && !B) || (!A && B)`
* XOR можно получить через NAND: `A XOR B = (A NAND (A NAND B)) NAND (B NAND (A NAND B))`
* XOR можно получить через NOR: `A XOR B = (A NOR B) NOR ((A NOR A) NOR (B NOR B))`
* XOR можно получить через NAND + AND + OR: `A XOR B = (A NAND B) && (A || B)`

|XOR| 0 | 1 |
|:--|:--|:--|
| 0 | 0 | 1 |
| 1 | 1 | 0 |

 
```rust
# const NAME_WIDTH: usize = 4;
# fn print_truth_table(name: &str, gate: fn(u8, u8) -> u8) {
#     println!("|{:^width$}| 0 | 1 |", name, width = NAME_WIDTH);
#     println!("|{:-^width$}|---|---|", "", width = NAME_WIDTH);
#
#     for b in 0..=1 {
#        println!(
#            "|{:^width$}| {} | {} |",
#            b,
#            gate(0, b),
#            gate(1, b),
#            width = NAME_WIDTH
#        );
#     }
#     println!("");
# }
#
# fn and(a: u8, b: u8) -> u8 { a & b }
# fn or(a: u8, b: u8) -> u8 { a | b }
# fn nand(a: u8, b: u8) -> u8 { 
#   not(a & b)  
# }
# fn not(x: u8) -> u8 {!x & 1}

fn xor(a: u8, b: u8) -> u8 {
    a ^ b
}
 
fn xor_by_using_nand_and_or(a: u8, b: u8) -> u8 {
    and(nand(a, b), or(a, b))
}

fn main() {
    print_truth_table("XOR", xor);
    print_truth_table("XOR", xor_by_using_nand_and_or);    
}
```

<div class="sim-wrapper" data-circuit-id="9">
  <button class="sim-fullscreen-btn" data-circuit-id="9">⛶</button>
  <iframe 
      id="9"
      data-circuit-id="9"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=1&startCircuit=/9_xor.txt"
      loading="lazy">
  </iframe>
</div> 

---

### NOT — `!A`

**NOT** (Инвертор) Инвертирует входное значение. Истина становится Ложью, и наоборот.

0 -> NOT -> 1

1 -> NOT -> 0

```rust
// Операция &1 означает нормализовать 8 бит к 0 или 1 
// т.е. оставить только младший бит, всё остальное выбросить
// Для типа bool нормализация не нужна
fn not(x: u8) -> u8 {
  !x & 1
}
 
fn main() {
    assert_eq!(not(1),0);
    assert_eq!(not(0),1);    
}
```

---

### NAND — `!(A && B)`

**NAND** (Not-AND/И-НЕ) Противоположность AND. Выдает Ложь, только если оба входа Истинны. Т.е. сперва применяется операция AND и к результату ее применяется операция NOT: 1 AND 1 = 1 NOT = 0

|NAND| 0 | 1 |
|:-- |:--|:--|
| 0  | 1 | 1 |
| 1  | 1 | 0 |

[Из AND можно получить NAND и наоборот](https://www.falstad.com/circuit/circuitjs.html?ctz=CQAgjCAMB0l3BWcMBMcUHYMGZIA4UA2ATmIxAUgpABZsKBTAWjDACgwEIUUaQUEhENkJ5+gqP0kIOlfr3AoxIsWCWSU1GQA9+hPoSTYwBsXwUBBAHIARNrsya0SGmDFoz-Ples2AOgDOfgAOgVYA8gAqbACywqKKZjSq6lQo0DJxrkICQjTJ4kJpGWwAMuCEVKya1eBqkhAAZgCGADYBDNSQZRVV+XU1-VUgLe2dSN0A7r0D4MQ19VPyfLVg87NLaws167xUSzzmNFW7x1Bs02CV4P1gGEImfJv3NysvjufTh-wYNe+-n2Wr3AL0e5wAkvF3BJsoVJDAkDIgA):
* NAND можно получить через AND, так как `A && B ≡ !(A NAND B)`
* NAND можно получить через OR `!(A && B) ≡ (!A) || (!B)`
* NAND можно получить через NOR `!(A && B) ≡ (A NOR A) OR (B NOR B)`

```rust
# const NAME_WIDTH: usize = 4;
# fn print_truth_table(name: &str, gate: fn(u8, u8) -> u8) {
#     println!("|{:^width$}| 0 | 1 |", name, width = NAME_WIDTH);
#     println!("|{:-^width$}|---|---|", "", width = NAME_WIDTH);
#
#     for b in 0..=1 {
#        println!(
#            "|{:^width$}| {} | {} |",
#            b,
#            gate(0, b),
#            gate(1, b),
#            width = NAME_WIDTH
#        );
#     }
#     println!("");
# }
#
# fn and(a: u8, b: u8) -> u8 { a & b }
# fn or(a: u8, b: u8) -> u8 { a | b }
# fn nor(a: u8, b: u8) -> u8 { not(a | b) }
# fn not(x: u8) -> u8 {!x & 1}

fn nand(a: u8, b: u8) -> u8 { 
  not((a & b)) 
}

fn nand_by_using_and(a: u8, b: u8) -> u8 { 
  not(and(a, b))
}

fn nand_by_using_or(a: u8, b: u8) -> u8 { 
  or(not(a), not(b))  
}

fn nand_by_using_nor(a: u8, b: u8) -> u8 { 
  or(nor(a, a), nor(b, b)) 
}

fn main() {
    print_truth_table("NAND", nand);
    print_truth_table("NAND", nand_by_using_and);
    print_truth_table("NAND", nand_by_using_or);
    print_truth_table("NAND", nand_by_using_nor);
}
```

<div class="sim-wrapper" data-circuit-id="8">
  <button class="sim-fullscreen-btn" data-circuit-id="8">⛶</button>
  <iframe 
      id="8"
      data-circuit-id="8"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=1&startCircuit=/8_nand.txt"
      loading="lazy">
  </iframe>
</div> 

---

### NOR — `!(A || B)`

**NOR** (Not-OR/ИЛИ-НЕ) Противоположность OR. Выдает Истину, только если оба входа Ложны. Т.е. сперва применяется операция OR и к результату ее применяется операция NOT: 0 OR 0 = 0 NOT = 1
[Так же можно получить NOR](https://www.falstad.com/circuit/circuitjs.html?ctz=CQAgjCAMB0l3BWcMBMcUHYMGZIA4UA2ATmIxAUgoqoQFMBaMMAKDAQmJT3A0OQT8wfKCBSiELALLgwkACy8h7ccP5UU0SQEkBQkcwVLRMJJIAyIPNnFN+GPIrsmQAMwCGAGwDOdapBZLa1V7R3B1UQ8fPyQAmVJxKmYERNFNSXYqLh4bJMhBEFy06kkADytiCAZCbBBieSdCKkUURQA5AHkAJRZyvAxiZFVuIXyQFvaOgBUAChnOqYAdbwBBAEpltpW2gBFl+enlgCE1je9FgEdN7pYAdytCRTAuKxtwF4D74PDX1Q+734-PCPH6fB5OMD8Rw8Zxg77OaEgWEAxEI+Q8SFUOEgzFWdHhLFsBC1YGKLIfYpmFi6UnvcTZOkmLQUanggl1bjsrHMsoVCCYWrMQjkTD8CYgA5LVZrQDkRJLjmdLtcejSQbhFAz1UyqarFNhsPxNQbtSyvvjcYitdi9UZLUY4W9nuJvvr1Ci1caXca4npCmhZCk-VixFoWEA)
* NOR можно получить через AND `!(A || B) ≡ (! A) && (! B)`
* NOR можно получить через NAND `!(A || B) ≡ (! A) && (! B) = !(!A NAND !B)`

Так же можно получить NOR если инвертировать выход OR, сравните таблицу истинности, если выход NOR `!(A || B)` инвертировать то получим OR `A || B`

|NOR| 0 | 1 |
|:--|:--|:--|
| 0 | 1 | 0 |
| 1 | 0 | 0 |

Задача: вывести NOR `!(A || B)` имея только NAND `!(A && B)` и NOT `!A`
* по закону де Моргана
  * `A || B ≡ !(!A && !B)`
* тогда через его отрицание получим NOR
  * `!(A || B) ≡ !(!(!A && !B))` 
* устраним двойное отрицание
  * `!(A || B) ≡ !A && !B`  
* преобразуем AND в NAND
  * `!(A || B) ≡ !(!A NAND !B)`     
 
```rust
# const NAME_WIDTH: usize = 4;
# fn print_truth_table(name: &str, gate: fn(u8, u8) -> u8) {
#     println!("|{:^width$}| 0 | 1 |", name, width = NAME_WIDTH);
#     println!("|{:-^width$}|---|---|", "", width = NAME_WIDTH);
#
#     for b in 0..=1 {
#        println!(
#            "|{:^width$}| {} | {} |",
#            b,
#            gate(0, b),
#            gate(1, b),
#            width = NAME_WIDTH
#        );
#     }
#     println!("");
# }
#
# fn and(a: u8, b: u8) -> u8 { a & b }
# fn not(x: u8) -> u8 {!x & 1}

fn nor(a: u8, b: u8) -> u8 {
    not(a | b)
}

fn nor_by_using_and(a: u8, b: u8) -> u8 { 
    and(not(a), not(b))  
}

fn main() {
    print_truth_table("NOR", nor);
    print_truth_table("NOR", nor_by_using_and);    
 
}
```

<div class="sim-wrapper" data-circuit-id="5">
  <button class="sim-fullscreen-btn" data-circuit-id="5">⛶</button>
  <iframe 
      id="5"
      data-circuit-id="5"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=1&startCircuit=/5_nor.txt"
      loading="lazy">
  </iframe>
</div> 

---

### XNOR — `!(A ⊕ B)`

**XNOR** (Exclusive-NOR/Исключающее ИЛИ-НЕ/NOT XOR). Противоположность XOR. Выдает Истину, если оба входа одинаковы (оба Ложны или оба Истинны).

* XNOR можно получить через XOR + NOT: `!(A ⊕ B) = !(A XOR B)`
* XNOR можно получить через AND + OR + NOT: `!(A ⊕ B) = (A && B) || (!A && !B)`
* XNOR можно получить через XOR + NAND: `!(A ⊕ B) = XOR NAND XOR`
* XNOR можно получить через NOR: `!(A ⊕ B) = ((A NOR B) NOR ((A NOR A) NOR (B NOR B))) NOR ((A NOR B) NOR ((A NOR A) NOR (B NOR B)))`

|XNOR| 0 | 1 |
|:-- |:--|:--|
| 0  | 1 | 0 |
| 1  | 0 | 1 |


```rust
# const NAME_WIDTH: usize = 4;
# fn print_truth_table(name: &str, gate: fn(u8, u8) -> u8) {
#     println!("|{:^width$}| 0 | 1 |", name, width = NAME_WIDTH);
#     println!("|{:-^width$}|---|---|", "", width = NAME_WIDTH);
#
#     for b in 0..=1 {
#        println!(
#            "|{:^width$}| {} | {} |",
#            b,
#            gate(0, b),
#            gate(1, b),
#            width = NAME_WIDTH
#        );
#     }
#     println!("");
# }
#
# fn not(x: u8) -> u8 {!x & 1}
# fn xor(a: u8, b: u8) -> u8 { a ^ b}

fn xnor(a: u8, b: u8) -> u8 {
    not(xor(a,b))  
}

fn main() {
    print_truth_table("XNOR", xnor);
}
```

<div class="sim-wrapper" data-circuit-id="10">
  <button class="sim-fullscreen-btn" data-circuit-id="10">⛶</button>
  <iframe 
      id="10"
      data-circuit-id="10"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=1&startCircuit=/10_xnor.txt"
      loading="lazy">
  </iframe>
</div> 

---

### Always ON
(Всегда включен)

Как получать на выходе всегда 1 при любом входе, имея в расспоряжении NOR, NAND, AND, OR, NOT

Один из вариантов, к результату NOR применить OR, так как их таблицы истинности наглядно показывают это поведение. Тогда мы сможем все нули NOR преобразовать в 1 через OR.

|NOR| 0 | 1 |
|:--|:--|:--|
| 0 | 1 | 0 |
| 1 | 0 | 0 |

|OR | 0 | 1 |
|:--|:--|:--|
| 0 | 0 | 1 |
| 1 | 1 | 1 |

<div class="sim-wrapper" data-circuit-id="1">
  <button class="sim-fullscreen-btn" data-circuit-id="1">⛶</button>
  <iframe 
      id="1"
      data-circuit-id="1"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=1&startCircuit=/turingcomplete/1_true.txt"
      loading="lazy">
  </iframe>
</div> 

---

### Второй тик

Ожидаемый выход 1 только при входе A=1, B=0

|tick| 0 | 1 |
|:--|:--|:--|
| 0 | 0 | 1 |
| 1 | 0 | 0 |

... решение напоминает таблицу истинности для XOR

|XOR| 0 | 1 |
|:--|:--|:--|
| 0 | 0 | 1 |
| 1 | 1 | 0 |

если для случая A=0, B=1 инвертировать результат через AND то можно получить трубуемое поведение.

Но более простой вариант это NOT + AND. Если мы вход B предварительно инвертируем, то при случаии A=1, B=0 мы инвертируем B и получим 1, а в остальных случаях результат через AND всегда будет 0 

Ожидаемое поведение, таблица истинности:
```
A   0 1 0 1
B   0 0 1 1
out 0 1 0 0

```

```rust
# const NAME_WIDTH: usize = 4;
# fn print_truth_table(name: &str, gate: fn(u8, u8) -> u8) {
#     println!("|{:^width$}| 0 | 1 |", name, width = NAME_WIDTH);
#     println!("|{:-^width$}|---|---|", "", width = NAME_WIDTH);
#
#     for b in 0..=1 {
#        println!(
#            "|{:^width$}| {} | {} |",
#            b,
#            gate(0, b),
#            gate(1, b),
#            width = NAME_WIDTH
#        );
#     }
#     println!("");
# }
# fn and(a: u8, b: u8) -> u8 { a & b }
# fn not(x: u8) -> u8 {!x & 1}

fn tick(a: u8, b: u8) -> u8 { 
    and(a, not(b))
}

fn main() {
    print_truth_table("tick", tick);
}
```

<div class="sim-wrapper" data-circuit-id="2">
  <button class="sim-fullscreen-btn" data-circuit-id="2">⛶</button>
  <iframe 
      id="2"
      data-circuit-id="2"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=1&startCircuit=/turingcomplete/2_second_tick.txt"
      loading="lazy">
  </iframe>
</div> 

---

### XOR элемент

Получить XOR

Таблица истинности XOR

|XOR| 0 | 1 |
|:--|:--|:--|
| 0 | 0 | 1 |
| 1 | 1 | 0 |


<div class="sim-wrapper" data-circuit-id="5">
  <button class="sim-fullscreen-btn" data-circuit-id="5">⛶</button>
  <iframe 
      id="5"
      data-circuit-id="5"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=1&startCircuit=/turingcomplete/5_xor.txt"
      loading="lazy">
  </iframe>
</div> 

---

### OR gate with 3 inputs

`(A || B) || (B || C)`

| A | B | C | A \|\| B \|\| C |
|---|---|---|---------------|
| 0 | 0 | 0 | 0 |
| 0 | 0 | 1 | 1 |
| 0 | 1 | 0 | 1 |
| 0 | 1 | 1 | 1 |
| 1 | 0 | 0 | 1 |
| 1 | 0 | 1 | 1 |
| 1 | 1 | 0 | 1 |
| 1 | 1 | 1 | 1 |

<div class="sim-wrapper" data-circuit-id="3">
  <button class="sim-fullscreen-btn" data-circuit-id="3">⛶</button>
  <iframe 
      id="3"
      data-circuit-id="3"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=1&startCircuit=/turingcomplete/3_big_or.txt"
      loading="lazy">
  </iframe>
</div> 

---

### AND gate with 3 inputs

`(A && B) && (B && C)`

| A | B | C | (A && B) && (B && C) |
| - | - | - | -------------------- |
| 0 | 0 | 0 | 0                    |
| 0 | 0 | 1 | 0                    |
| 0 | 1 | 0 | 0                    |
| 0 | 1 | 1 | 0                    |
| 1 | 0 | 0 | 0                    |
| 1 | 0 | 1 | 0                    |
| 1 | 1 | 0 | 0                    |
| 1 | 1 | 1 | 1                    |


<div class="sim-wrapper" data-circuit-id="4">
  <button class="sim-fullscreen-btn" data-circuit-id="4">⛶</button>
  <iframe 
      id="4"
      data-circuit-id="4"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=1&startCircuit=/turingcomplete/4_big_and.txt"
      loading="lazy">
  </iframe>
</div> 
 





 
---

<style>
table {
  margin: 0px !important;  
  border-collapse: collapse;
}
</style>