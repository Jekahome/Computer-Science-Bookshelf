# Turing Complete. Memory

### Circular Dependency

> Задача: Создайте "круговую зависимость". Это схема, в которой вход компонента зависит от его собственного выхода. 

В такой ситуации невозможно определить выход компонента, потому что сперва нужно определить вход, который сам ависит от выхода, образуя петлю.

```
A → B → C
↑       ↓
└───────┘
```

### Delayed lines

Линия задержки. В задаче предоставлен компонент с задержкой в один tick.

Компонент с задержкой «1 такт» не отдаёт свой вход сразу, а только на следующий шаг симуляции.

```
tick = 0: вход = A → Delay1 хранит A, Delay2 хранит старое значение
tick = 1: вход = B → Delay1 выдаёт A, Delay2 выдаёт старое значение
tick = 2: вход = C → Delay1 выдаёт B, Delay2 выдаёт A
```

Синхронизация здесь нужна «логически», чтобы симулятор понимал, что значения обновляются только по тактам, а не **мгновенно**. 
Так как в реальном мире электроники, компоненты работают только по тактам.

> Задача: Создайте цепь которая выводит свой собственный вход с задержкой в 2 такта.

Решение простое: последовательно два компонента с задержкой 1

```
Input → [Delay 1] → [Delay 2] → Output

```

---

### Odd ticks

Нечетные такты. 

Мы не дорускаем круговых зависимостей. Но есть одно исключение. 

Линия задержки может зависеть от собственного входа. Это потому, что её вход не влияет на остальную схему до следующего такта. 

Квадратные контакты в игре никогда не влияют на вывод в тот же такт. Поэтому они никогда не вызывают круговых зависимостей.

> Задача: Выведите 0 для четных тактов и 1 для нечетных тактов.

Мы хотим сигнал, который чередуется каждый такт:

* Есть компонент Delay на 1 такт → он помнит прошлое состояние

![Odd ticks](/Computer-Science-Bookshelf/img/Odd_ticks.png)

[Odd ticks (www.falstad.com/circuit)](https://www.falstad.com/circuit/circuitjs.html?ctz=CQAgjCCsIAwHQwCyQMwwBwE4ZgOwCZ0cA2TEfGKKSyAUwFowwAoAGRBQvMmO9-x6whAMwCGAGwDOtKjGYBZKJnzlElYgVWVK+OJGYAPcmH5rypVShCJVIAJIA7AA4BXAC7MwqDl3ybEJuSaOkL6XiqImKY0ylpCKtD6AO4+OoKcOvg2cikZ5FnWUfnZzCmIxDZ+EUUCvDnWFUERjWglZS1IqRyd9XmE6F2tsIYgFRD4YGS4YFb4c9a2APLurh5GiBOq0MjjiAM2YCoARi7CwrQATswbEYFVDZXB8XpQzEA)

<div class="sim-wrapper" data-circuit-id="11">
  <button class="sim-fullscreen-btn" data-circuit-id="11">⛶</button>
  <iframe 
      id="11"
      data-circuit-id="11"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=1&startCircuit=/turingcomplete/11_odd_ticks.txt"
      loading="lazy">
  </iframe>
</div> 

---

### Bit Inverter

Битовый инвертор

Задача: Имеем два входа: invert и value. Когда вход invert 1, выведите обратное от value. Иначе, просто выведите value как есть.

Найти решение которое соответвует такой таблице истинности. Это напоминает поведение XOR

|i/v| 0 | 1 |
|:--|:--|:--|
| 0 | 0 | 1 |
| 1 | 1 | 0 |


### Bit Switch

Битовый переключатель

Если компоненты выдают разные значения по одному и тому же проводу, возникает ошибка. Однако у некоторых компонентов выходные контакты серые, и они вообще не выдают сигналы, когда компонент не активен. Это относится и к компоненту Bit Switch

Switch: Когда верхний контакт компонента активирован, выходной сигнал отключен. Это означает, что к одному и тому же проводу может быть подключено более одного из этих серых выходных контактов, и это не вызовет ошибки, если одновременно активировать только один из них.

У Bit Switch выход “серый” — это значит:
* когда switch выключен → он НИЧЕГО не выдаёт (провод “в воздухе”)
* когда switch включен → он пропускает вход на выход

Поэтому несколько серых выходов можно соединять в один провод, если гарантировано, что включён только один из них.

> Задача: Собрать gate XOR используя два переключателя switch и два gate NOT

* Один switch отвечает за случай A=1, B=0
* Второй switch — за A=0, B=1
* Их выходы соединены в один провод
* Одновременно они никогда не активны

|XOR| 0 | 1 |
|:--|:--|:--|
| 0 | 0 | 1 |
| 1 | 1 | 0 |

```
(A AND !B) ─▶ buffer ┐
                     ├──▶ OUT
(!A AND B) ─▶ buffer ┘

```

![Bit Switch](/Computer-Science-Bookshelf/img/Bit_Switch.png)
 

У буфера с 3-мя состояниями выход бывает трёх типов:
* 0
* 1
* Z (отключён, «ничего не выдаёт», это не 0, это отсутствие сигнала)

Если учитывать состояние Z, то при A=1, B=1 оба tri-state буфера отключены, и линия оказывается в состоянии Z — никто не тянет её ни в 0, ни в 1.
Чтобы в этом случае получить 0, добавляют pull-down — подтяжку к нулю по умолчанию. Тогда при отсутствии активных драйверов (Z) выход принимает значение 0.


[Bit Switch (www.falstad.com/circuit)](https://www.falstad.com/circuit/circuitjs.html?ctz=CQAgjCAMB0l3BWKsEA4wHYvoCwCZIBOVSVHcAZhCQUmoFMBaMMAKAA8Q88A2LwvF3z8k5POQBCrADIgKCQXlSouCPkpV0IAMwCGAGwDO9alFYB3OWS7iQOMOtuQLdnCoqRyON3IJmAklbkFH7e7n50MDSsgRQ8KnhqrglJkdDRlm50GnYkqnzOmXncXsWpLvbqynIKXNXOYHmVdSpZLXbIEGDwPfDgvX0IHNQUhFwUKjwsQhBi5ADyAK4ALgAOK8PcCXhj4mIhHbYAgqwAsrnZSWqXBVzpMr7ZtomO5FogekYmSM6co9kULzTPAUDAdbogABy8wAKsNRupCGN7GCPIJyBDoXDODgEEg8NMcBhsmAvP0QBJFtphrjFMS7DwwXgMFQMXRKdTGnRmiULjYyTAugMev1hdQXPJFNVJfkzJkfOJuQrypYJsE-CE6Jq5Y9ddreYVdby1fydTKXnJ4rLnEA)

<div class="sim-wrapper" data-circuit-id="12">
  <button class="sim-fullscreen-btn" data-circuit-id="12">⛶</button>
  <iframe 
      id="12"
      data-circuit-id="12"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=1&startCircuit=/turingcomplete/12_bit_switch.txt"
      loading="lazy">
  </iframe>
</div> 
 

### Input Selector

Multuplexers (Мкльтиплексер)

> Задача: Если бит selector input установлен в 0, вывод Byte A, иначе вывод Byte B

Условие задачи (человеческим языком)

Есть:
* Byte A (8 бит)
* Byte B (8 бит)
* Selector (1 бит)

Нужно использовать Selector:
* если Selector = 0 → вывести Byte A и полностью игнорировать B
* если Selector = 1 → вывести Byte B и полностью игнорировать A

Мультиплексер для одного бита выглядит так: `OUT = (A & !S) | (B & S)`

| S | A | B | OUT |
| - | - | - | --- |
| 0 | 0 | 0 | 0   |
| 0 | 1 | 1 | 1   |
| 1 | 0 | 1 | 1   |
| 1 | 1 | 0 | 0   |

 
**Input Selector 1 bit**

<div class="sim-wrapper" data-circuit-id="15">
  <button class="sim-fullscreen-btn" data-circuit-id="15">⛶</button>
  <iframe 
      id="15"
      data-circuit-id="15"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=1&startCircuit=/turingcomplete/15_input_selector_1bit.txt"
      loading="lazy">
  </iframe>
</div> 


Для байта — то же самое, но 8 раз, по одному биту.

<div class="sim-wrapper" data-circuit-id="16">
  <button class="sim-fullscreen-btn" data-circuit-id="16">⛶</button>
  <iframe 
      id="16"
      data-circuit-id="16"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=1&startCircuit=/turingcomplete/16_input_selector_8bit.txt"
      loading="lazy">
  </iframe>
</div> 

 
![Input Selector](/Computer-Science-Bookshelf/img/Input_Selector.png)

Но есть уже готовый компонет, 8-ми битный переключатель

![Input Selector](/Computer-Science-Bookshelf/img/Input_Selector2.png)


### The Bus (Шина)

На этом уровне имеется 2 байтовых входа и 2 байтовых выхода. Ваша задача — скопировать верные данные с одного из входов на один из выходов.

Первый входной бит определяет, с какого входа следует копировать данные. Второй входной бит определяет, на какой выход следует копировать данные.

Используйте переключатели, чтобы подключить входы к одному проводу.

<div class="sim-wrapper" data-circuit-id="17">
  <button class="sim-fullscreen-btn" data-circuit-id="17">⛶</button>
  <iframe 
      id="17"
      data-circuit-id="17"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=1&startCircuit=/turingcomplete/17_the_bus.txt"
      loading="lazy">
  </iframe>
</div> 


![The Bus (Шина)](/Computer-Science-Bookshelf/img/The_Bus.png)
 
### Saving Gracefully

Линия задержки позволяет нам использовать значение на 1 такт позже. 

Нам нужно создать компонент, который позволит нам использовать значение, когда мы захотим, не важно на сколько тактов позже. 

Такое значение называется сохранённым.

> Задача:
> 
> На этом уровне 2 входа.
> 
> (Input Save) Только в случае если первый вход 1, обновите сохранённое значение.
> 
> (Input Value) Второй вход указывает какое значение нужно сохранить.
> 
> Всегда выводите сохранённое значение.
> 
> На этом уровне вам нужно использовать 1 линию задержки. Составьте таблицу истинности для того, что должно поступать на линию задержки. Рассматривайте вход линии задержки как выход для вашей таблицы.
> 
> Входами являются два входных сигнала уровня И выход линии задержки, поскольку выход играет роль во входном сигнале. В общей сложности у вас есть 3 «входа» для этой таблицы, что дает вам 8 комбинаций. Как только у вас будут требования к уровням в виде таблицы, решить задачу станет намного проще.
> 

Реализовать поведение согласно таблице истинности:
|            |    |    |    |    |    |    |    |    |    |    |    |    |    |
| ---------  |--- |--- |--- |--- |--- |--- |--- |--- |--- |--- |--- |--- |--- |
|Input Save  | 1  | 1  | 1  | 1  | 0  | 0  | 1  | 1  | 1  | 0  | 0  | 1  | 1  |
|Input Value | 1₁ | 1₂ | 0₃ | 0₄ | 0  | 1  | 0₅ | 1₆ | 1₇ | 1  | 0  | 0₈ | 0₉ |
|Output      | 0₀ | 1₁ | 1₂ | 0₃ | 0₄ | 0₄ | 0₄ | 0₅ | 1₆ | 1₇ | 1₇ | 1₇ | 0₈ |


Эта задача — про запоминание (1-битная память), реализованную через одну линию задержки. Это однобитный регистр с разрешением записи (write enable).

* Input Save — управляющий сигнал
* Input Value — значение, которое можно сохранить
* Delay — задержка на 1 такт (это и есть «память»)
* Output — всегда должен быть сохранённым значением

Поведение:
* Если Save = 1 → обновить сохранённое значение (запомни Value)
* Если Save = 0 → оставить старое значение
* Выход всегда показывает то, что сохранено

Линия задержки не держит значение “сама по себе”. Она каждый такт перезаписывается. Т.е. у нее нет постоянного хранилища, она каждый такт перезаписывается своим же значением но при условии что мы не даем сигнал сохранить новое значение.

Мы не вычисляем Output напрямую. Мы вычисляем что подать на вход Delay на следующем такте.

Обозначим:
* S = Save
* V = Value
* O = OldIn (выход delay)
* D = вход DelayIn (то, что считаем)

Таблица истинности для того, что должно поступать на линию задержки. Рассматриваем вход линии задержки как выход для вашей таблицы.

| Save (S) | Value (V) | OldIn (O) | DelayIn (D) |
| -------- | --------- | ------- | ------------ |
| 0        | 0         | 0       | 0            |
| 0        | 0         | 1       | 1            |
| 0        | 1         | 0       | 0            |
| 0        | 1         | 1       | 1            |
| 1        | 0         | 0       | 0            |
| 1        | 0         | 1       | 0            |
| 1        | 1         | 0       | 1            |
| 1        | 1         | 1       | 1            |


Из таблицы напрямую получается логическая функция:
* при Save=0 → DelayIn = OldIn
* при Save=1 → DelayIn = Value

```
DelayIn = (Save AND Value) OR (NOT Save AND OldIn)
```


<div class="sim-wrapper" data-circuit-id="18">
  <button class="sim-fullscreen-btn" data-circuit-id="18">⛶</button>
  <iframe 
      id="18"
      data-circuit-id="18"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=1&startCircuit=/turingcomplete/18_saving_gracefully.txt"
      loading="lazy">
  </iframe>
</div> 

 

![Saving Gracefully](/Computer-Science-Bookshelf/img/Saving_Gracefully.png)

![Saving Gracefully 2](/Computer-Science-Bookshelf/img/Saving_Gracefully2.png)













---

<style>
table {
  margin: 0px !important;  
  border-collapse: collapse;
}
</style>