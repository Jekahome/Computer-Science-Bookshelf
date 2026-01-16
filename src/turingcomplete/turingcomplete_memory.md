# Memory

* [Circular Dependency](#circular-dependency)
* [Delayed lines](#delayed-lines)
* [Odd ticks](#odd-ticks)
* [Bit Inverter](#bit-inverter)
* [Bit Switch](#bit-switch-tri-state-buffer)
* [Input Selector](#input-selector)
* [The Bus](#the-bus)
* [Saving Gracefully](#saving-gracefully)
* [Saving Bytes](#saving-bytes)
* [Counter](#counter)
* [Little Box](#little-box)

## Circular Dependency

> Задача: Создайте "круговую зависимость". Это схема, в которой вход компонента зависит от его собственного выхода. 

В такой ситуации невозможно определить выход компонента, потому что сперва нужно определить вход, который сам ависит от выхода, образуя петлю.

```
A → B → C
↑       ↓
└───────┘
```

---

## Delayed lines

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

## Odd ticks

Нечетные такты. 

Мы не дорускаем круговых зависимостей. Но есть одно исключение. 

Линия задержки может зависеть от собственного входа. Это потому, что её вход не влияет на остальную схему до следующего такта. 

Квадратные контакты в игре никогда не влияют на вывод в тот же такт. Поэтому они никогда не вызывают круговых зависимостей.

> Задача: Выведите 0 для четных тактов и 1 для нечетных тактов.

Мы хотим сигнал, который чередуется каждый такт:

* Есть компонент Delay на 1 такт → он помнит прошлое состояние

![Odd ticks](/Computer-Science-Bookshelf/img/tc/Odd_ticks.png)

[Odd ticks (www.falstad.com/circuit)](https://www.falstad.com/circuit/circuitjs.html?ctz=CQAgjCCsIAwHQwCyQMwwBwE4ZgOwCZ0cA2TEfGKKSyAUwFowwAoAGRBQvMmO9-x6whAMwCGAGwDOtKjGYBZKJnzlElYgVWVK+OJGYAPcmH5rypVShCJVIAJIA7AA4BXAC7MwqDl3ybEJuSaOkL6XiqImKY0ylpCKtD6AO4+OoKcOvg2cikZ5FnWUfnZzCmIxDZ+EUUCvDnWFUERjWglZS1IqRyd9XmE6F2tsIYgFRD4YGS4YFb4c9a2APLurh5GiBOq0MjjiAM2YCoARi7CwrQATswbEYFVDZXB8XpQzEA)

<div class="sim-wrapper" data-circuit-id="11">
  <button class="sim-fullscreen-btn" data-circuit-id="11">⛶</button>
  <iframe 
      id="11"
      data-circuit-id="11"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=true&startCircuit=/turingcomplete/11_odd_ticks.txt"
      loading="lazy">
  </iframe>
</div> 

---

## Bit Inverter

Битовый инвертор

Задача: Имеем два входа: invert и value. Когда вход invert 1, выведите обратное от value. Иначе, просто выведите value как есть.

Найти решение которое соответвует такой таблице истинности. Это напоминает поведение XOR

|i/v| 0 | 1 |
|:--|:--|:--|
| 0 | 0 | 1 |
| 1 | 1 | 0 |

---

## Bit Switch (tri-state buffer)

> [!TIP]
> Разблокирует:
> * переключатель на 1 бит `S` 
> * 8-ми битный `SWC`

> [!IMPORTANT]
> Главная цель битового переключателя (Bit Switch / Tri-state) - дать возможность нескольким источникам делить **одну** линию, не вызывая короткого замыкания **к.з.** (в Falstad → singular matrix)
> ```
> A ─┐
>    ├─── BUS
> B ─┘
> ```

 
Если компоненты выдают разные значения по одному и тому же проводу, возникает ошибка (к.з.). Однако у некоторых компонентов выходные контакты могут иметь **состояние выхода Z (high-impedance)**, и они вообще не выдают сигналы, когда компонент не активен,т.е. Z состояние это не просто низкий сигнал LOW/pull-down, а имеено **электрическое отключение выхода/обрыв линии**. Это относится и к компоненту Bit Switch.

> [!IMPORTANT]
> На практике, состояние Z применяется в шинах (the bus)
> ```
> REG0 ──┐
> REG1 ──┼─── BUS ──► ALU
> REG2 ──┘
> ```

У Bit Switch выход “Z” — это значит:
* когда switch выключен → он НИЧЕГО не выдаёт (провод “в воздухе”)
* когда switch включен → он пропускает вход на выход

Поэтому несколько Z выходов можно соединять в один провод, если гарантировано, что включён только один из них.

> [!IMPORTANT]
> Bit Switch по сути это **управляемый буфер с тремя состояниями**
> | Enable | In | Out              |
> | ------ | -- | ---------------- |
> | 0      | X  | **Z** (отключён) |
> | 1      | 0  | 0                |
> | 1      | 1  | 1                |

 
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

![Bit Switch](/Computer-Science-Bookshelf/img/tc/Bit_Switch.png)
 
Из 8 штук битовых переключателей, можно сконструировать 8-ми битный переключатель самостоятельно.

![Byte Switch](/Computer-Science-Bookshelf/img/tc/Byte_Switch.png)

#### Circuit Simulation: gate XOR
(Собрать gate XOR используя два переключателя switch и два gate NOT)

Если учитывать состояние Z, то при A=1, B=1 оба tri-state буфера отключены, и линия оказывается в состоянии Z — никто не тянет её ни в 0, ни в 1.
Чтобы в этом случае получить 0, добавляют pull-down — подтяжку к нулю по умолчанию. Тогда при отсутствии активных драйверов (Z) выход принимает значение 0 по умолчанию.

[Bit Switch (www.falstad.com/circuit)](https://www.falstad.com/circuit/circuitjs.html?ctz=CQAgjCAMB0l3BWKsEA4wHYvoCwCZIBOVSVHcAZhCQUmoFMBaMMAKAA8Q88A2LwvF3z8k5POQBCrADIgKCQXlSouCPkpV0IAMwCGAGwDO9alFYB3OWS7iQOMOtuQLdnCoqRyON3IJmAklbkFH7e7n50MDSsgRQ8KnhqrglJkdDRlm50GnYkqnzOmXncXsWpLvbqynIKXNXOYHmVdSpZLXbIEGDwPfDgvX0IHNQUhFwUKjwsQhBi5ADyAK4ALgAOK8PcCXhj4mIhHbYAgqwAsrnZSWqXBVzpMr7ZtomO5FogekYmSM6co9kULzTPAUDAdbogABy8wAKsNRupCGN7GCPIJyBDoXDODgEEg8NMcBhsmAvP0QBJFtphrjFMS7DwwXgMFQMXRKdTGnRmiULjYyTAugMev1hdQXPJFNVJfkzJkfOJuQrypYJsE-CE6Jq5Y9ddreYVdby1fydTKXnJ4rLnEA)

<div class="sim-wrapper" data-circuit-id="12">
  <button class="sim-fullscreen-btn" data-circuit-id="12">⛶</button>
  <iframe 
      id="12"
      data-circuit-id="12"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=true&startCircuit=/turingcomplete/12_bit_switch.txt"
      loading="lazy">
  </iframe>
</div> 
 
#### Circuit Simulation: Bit Switch
(также часто называют "pass transistor logic", "передающий транзистор" или "ключ") 

В CircuitJS нет прямого “tri-state buffer” как в игре Turing Complete.

Но есть аналог из реальной электроники для правильного моделирования состояния Z:
* Управляемый ключ на N-MOSFET (или NMOS):
  * Сток (Drain) и Исток (Source) подключаются в разрыв вашей сигнальной линии
  * Затвор (Gate) — это управляющий вход
  * Принцип: Режим ключа предполагает заземлить подложку Body/Bulk, что бы диод не открылся когда на стоке напряжение выше, чем на истоке,что лишит нас возможности управлять затровором Gate. Когда на затвор Gate подается высокий уровень (логическая 1, напряжение, близкое к Vdd), транзистор открывается и проводит сигнал между стоком Drain и истоком Source. Когда на затворе низкий уровень (0), транзистор закрывается и разрывает цепь.
  * Недостаток: пороговые потери (Threshold Voltage, Vth) на преодоление барьра открытия транзистора, что снижает выходное напряжение, например если HIGH был 5 Вольт, то на выходе 3.5 Вольт, что в каскаде т.е. дальнейшее использование сигнала HIGH с выхода уже невозможно!
* Transmission Gate (Analog Switch)
  * Он состоит из параллельно соединенных N-MOSFET и P-MOSFET, управляемых инверсными сигналами.
  * Когда ключ открыт, он отлично передает как 0, так и 1 без потерь напряжения.
* Трёхстабильный буфер (Tri-state buffer)
  * По сути, трёхстабильный буфер — это "цифровая" версия Transmission Gate с гарантированными уровнями 0V/5V на выходе.

 
<div class="sim-wrapper" data-circuit-id="26">
  <button class="sim-fullscreen-btn" data-circuit-id="26">⛶</button>
  <iframe 
      id="26"
      data-circuit-id="26"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=true&startCircuit=/turingcomplete/26_bit_switch.txt"
      loading="lazy">
  </iframe>
</div> 

[Bit Switch NMOS,Transmission Gate,Tri-state buffer (www.falstad.com/circuit)](https://www.falstad.com/circuit/circuitjs.html?ctz=CQAgjCAMB0l3BWKsEA4wHYvoCwCZIBOVSVHcAZhCQUmoFMBaMMAKABlKdyEMA2cHhypq-KOBAAzAIYAbAM71qUVgHdKYhGDzgcpatpXqw3OgkI6TekH32RWASUq3q3XdYRu6MGmt0YdXgETANdye2McUK1LKMCvPxN9IK4eMXswEl1ksRDAsXDoCDB4UvhwMvKEVgAPcAQKCAQ8YPMmvEIQcgIQAHkAVwAXAAch2vAwNANO7UnqPB1yTJAHADtRwYAdeQBBVgBZf3ilhGaw8TxoarqwClRAheyqZqQlugA5XoAVcasMLsIMxwfECkEWFRAACF+pJftweFMTOZqHwIG8oTDWJk6EkRDEjgZwTBipVShVSdREqZ5iIrGYEpFrLZaY0dMyjJRDPjbppDBFOcdnPTwqxJCA7mkBHozGJCMErsgweM9HicJ1qadwSUQAARABO0gAlqttmw6jhDKjyDgzqj-uiAMoAe36eoAxvRTeMKGCbNqKBQ2bcuhCAOLSQae+RsTh4VB4wxCMx8iQyBRKJD2Q7NER8Jn6PPeECXa7gDBNTDFuPzOhLERrDbbPY3DBIPhiPCBmwYdXgevrIbbSEcYsIATtnRJ7s6ItpxTKex1H3kDA+8UUFcBkPa8OR7Z4ZWGLDWs5Ye0Q52uj375X6DBELrWctauj6o0m+QHsUSkCrqVwX8KAEOVwAVGAlXUaUUVrAD8X5KD8TjBMZz8KCAhgugJw5KCsLwMdpxUJwkNEAQAwEFJvAVap1DIkjxTIOj+TwaIO3jRi-GY-IBE4wCBCY0I-3ozc+NQ2CLBpahxP5HNJIeXNrHggDWzk38zmk6tCwk5SVAAcy6JSgK6LtBO8LFkVQLtiNQEDiMnHFSXKEssRcdBSIY6y3O6Yt7IcugnLqe4RCEGZIBBYs+F7EwQAdAAFB0vm2AAKB1jR02Qoxip10u2FLVjSqMvgACz1J1VAASm2TZNlWQA8EEAFhBAF4QQA+EEAfhAmsAbhBADkQQBBEAawBpEEAThAABptiaxqmoawAGEBawABEEmwAmEHawBWEAa-qBvGCzyADJASmRChTm3chYp1BL5GS1LsvkTLLp1F0ACNLqKkryqq2rxrarresGkb5Dq+bAGEQKbZoW5bVsG34ey6XgbCELoXG6Oh9n6WRBn6YZ0pqeg9XkJL9gAVQADW2AB6bZABwQf72sAGRBACEQQAOEBa5bAC4Qbqlpmir5HeRh9l6B0ADEAFEfju8B4a7PhT1QTM-Fo2iLVIwymLYwM6BV8Uen5WXIC2wziPsb8ejV2WehAsAwNgA8l3wcVIAgICkGXbdXwNY1vSi-aRA3B3USd6KXXdehxgIf5AyQIRSOrOsQF3QPDmBHXyLcWi-KuEdaNs-C9fEOcMxUG4JeLQgICEMPCGtPsVgHLZ5AAEwjaQR2ItW8IjosIBzhdWD043Vb4T2Nd0-SZSlLsKJUEWwHhyebDVUWRLqPDLEIZ4XDAEDulpaAh0NauHVUHe3UK7YuZ5-mhdYYY4YTq+hPFcQfLKLuumnn8oJ-UzIJfwz4-XETOAsycbF+BBTYjiKQch5zS0OB5YsbFCCsRECnao-88w2xXH3NB4h24QNzvYf+L8GLANvmAjuUCQAwKAiIeBpEMFIIOOQ9eAFqFDwuKnf+whxQYKIZQrB4D0ydzqPeHaRcJgAQWK8WBxYt7yEhDvHK+9BiH22F8A0qx5AAFtDTyHkIaJ0H4Y6vUSjsVYcgnQ6XkQfQqZVWAi3bDiPgwRtBSjXnnch1tVxUGXkgQSUcT680Fj8AKG5qD7RABYcidxtwiBitzfx59BG+htDtIgrx0Ahh6LI3eCilHyFivFJKuV8rbGulGQpj1iqlQ5q9eqzUPo9TWj9MazUgZzUWitNa4xVw6HvJ0LAEBnxROjhGQOdQLBtjSWXXMDF0RvldgFC0Ng0moFCH3cu2pLwB3GMvToEsqE+wcWiCEkInQ1wAJ4X3JLmfQJQCzWCoG3Uk9CblXNpJAfC7I6H-yyOyDADEPmphwZ3MUMD7zkBgRLYC8pMwW1FAw8iLkQLsngaBaFEEwmWmuXAF5HJwX6HBbDfk+LyBl3ICCEU6hwX4XBQpPw1K6AkpnqZPSDLNJykwgpDkLLYYMooqwAASmEtwKQeVeHEB4e+yBqLkO+Xi75+F+RFzZPoRVNh5UjgAb+bWv5CFapIYC6WTgNVjjoDA4198qK0uMlqo1Lh7B6WeYs15d5dUqD1Jcxl7q+6IPJISkCoLyFWpFG6n+r8CHhB9Vsg6Y4dDwP+GOCR2pjlnK2VFWwMabQ2Deb7DZHoNqz1sFQZZSqpQQlmasGxBEcBdhYJ2bV896IOx6HoEOtb0RJvOeaWeKRhAykinQdtXpO29O-lkVcfaoQnNOTeAKBcLCFrgS0X2ZatlrgsDGn2Yzs3+1zaM350q3gAXjOeRNk6-AqpKLDMgryCW0o4ResFHD4ymUNY+rIKqn3mt8OoCwVDqw-rCekWlIF-0wOYfYAVzDQMuVQUWcVRYfBntlG4SDIE7WCuJS4BlYHOWGDZehsJtqz24dlLhwj37cPIaQ+S9FMa-0Ueo-+-996wnVn1jR6V9LcN-v+GbVFX52PMZVcw025slT-yYdWbS-69X8LIcxko1gSihAU0Wfygr-hyiQHKTocoRBttPQFaGNyIDxidWsugOaRlhMMiUFgYTkS2fM7qF25a6h92AluDAVbrMluWA2QcuwNpefIRu4EIW1nkBiafAJQXknW0mRUDNSxyB+LPoE8h95BAOMFVQJxkUUuxLS0F3LpAIBl2SdM3QIAotxPS+5iY+EvO5cmJFfsjYZFbIWdoPCBGlgLD06WlzG0jPiOlbSfrW6rxWYS9oJhmXZtUH08m-+1bJjdLOCwNSALZMqDFOeoQVDDCzfpVCxU-Hz0KYE2tgDKKztnurJtywYIljXYVUd67F3aFEcsJPKhuRfs4csAQTjP2GJvdBxhnEmQRTMtXtDgDjiwc4tNtdmBx3OUPY+w94HhE3G0kuxdl1PhKQUrvQdvHoiGNHfJ1ewQHDFzWah2FuUxQTDjssxtAuLAohjYa7WQb75EjPZY9et4NLjDC8exUWGKnkeOLOBqqXhKALaC7CQHE5Plca8fWT+ntKVedh0Or6X1HjcgeA6xja1ypwWXG959EBjPwbTswpzoZASRnAd8Mr036qMI4qKEBV-2MHMKUyhBJRueglEIEge4i3vIgCRijNGGMsY4zOvjIm8hSbyAptTemjMloszZhzPJPxxP2VhtpZjMnIG4+N8x2nNfJWUjc2ki9w6QpdijgAYT0YMEqsgzJu9QTyYCD2KAdgTzchyUjqgtfC+4elD3qR2R9RSJyKCliT4EA3nfvDSEqC30v8hKvqS19weq6tKpT84hvxfgRv4ZcWm6fcXQcxe-98H50xdPJekWUoDfyjn82rmbCf2CG4H6QAKsDWTawC2HEOCYxv1D3vzn3uxZFlBXzsHQJPyQOwJfVpDiDhQJEoi-WILyHIJtBEgpWvzxVNnvyA2CAoHEjRwYNJ0INCEVyIMJVoJEC4MD0bhYm4g8HSG2zrwZ0wECDSU7DMEq2WD71WAHyygrSwgb1V0ECwHHgIm0BRzwGKC0HuU6RVzVmj1y0DHjx6CT1RnRnoExmxlxkJhJnJkplpgZmZlZnZhyjinS1mx42XnqFnjXhLWrAoGkUyQsUUSPnkBUUNEYHkEGG93kDuhhEkCxipH0GIlxEwQyCyDH2LGU1X1CEKBJF8nXzJFLBMDmDwiBDXBaBfAxFhE4DyJ4m0A4R4gfzk0KMsGaD8kDzQJuAaEnE0Ja0AX5x6AGBGDGBuEmH+GYh407FmKyGAKrm2DrgSJHCyMdlaM9hdWwR2wkLXi2gfFVyoFZEGRjl+BMEWAc1REWDCgRkT2RmsNT3sIz0cOz2cPzzcKLw8I5hiLiISL3CSJSLSKaLOM8D6w4QhIP31VcU23IloEECilOHj3kK-2UOmOhjQCB2RKlkGRAKHESHBLcFuAxUQQrRYG-mrRBCgJFAnh5FFlpBBFeD0NcUBGS2DF2g5OCLoAmI2F+EgEn2l3smRQvQsN5KGH5JuG1lkPxzEkOXGMlKmJRCoFRHHFrVRF7EVMmMGHGBeHFDmDQAdmWAeL5OVPbFLjoFbFLlRLgOrjWIbkEUMB9H+FbDtm1CjguKdNONQAEG0juFayGUjHGHbBoV6Xwkny1DtNWPrg2PBOpyhJTDoEPwZwilVLsywHTNgMrnawdNYCAA)


#### 8 bit Switch and 8 bit Multuplexers (MUX) Tri-state buffer:

<div class="sim-wrapper" data-circuit-id="27">
  <button class="sim-fullscreen-btn" data-circuit-id="27">⛶</button>
  <iframe 
      id="27"
      data-circuit-id="27"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=true&startCircuit=/turingcomplete/27_8_bit_switch_and_mux_tri_state_buffer.txt"
      loading="lazy">
  </iframe>
</div> 

---

## Input Selector

> [!TIP]
> Разблокирует 8-ми битный MUX / Мультиплексер

Multuplexers (MUX / Мультиплексер)

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
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=true&startCircuit=/turingcomplete/15_input_selector_1bit.txt"
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
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=true&startCircuit=/turingcomplete/16_input_selector_8bit.txt"
      loading="lazy">
  </iframe>
</div> 

 
![Input Selector](/Computer-Science-Bookshelf/img/tc/Input_Selector.png)

Но есть уже готовый компонет, 8-ми битный переключатель

![Input Selector](/Computer-Science-Bookshelf/img/tc/Input_Selector2.png)

---

## The Bus 

Шина.

> Задача: 
> На этом уровне имеется 2 байтовых входа и 2 байтовых выхода. 
>
> Ваша задача — скопировать верные данные с одного из входов на один из выходов.
> 
> Первый входной бит определяет, с какого входа следует копировать данные. Второй входной бит определяет, на какой выход следует копировать данные.

Используйте переключатели, чтобы подключить входы к одному проводу.

<div class="sim-wrapper" data-circuit-id="17">
  <button class="sim-fullscreen-btn" data-circuit-id="17">⛶</button>
  <iframe 
      id="17"
      data-circuit-id="17"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=true&startCircuit=/turingcomplete/17_the_bus.txt"
      loading="lazy">
  </iframe>
</div> 


![The Bus (Шина)](/Computer-Science-Bookshelf/img/tc/The_Bus.png)
 
---

## Saving Gracefully

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
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=true&startCircuit=/turingcomplete/18_saving_gracefully.txt"
      loading="lazy">
  </iframe>
</div> 

 

![Saving Gracefully](/Computer-Science-Bookshelf/img/tc/Saving_Gracefully.png)

![Saving Gracefully 2](/Computer-Science-Bookshelf/img/tc/Saving_Gracefully2.png)

---

## Saving Bytes

> [!TIP]
> Разблокирует компонент 1 ячейка памяти: `Register 8 bit`

> Задача:
> Создать схему, которая может SAVE или LOAD байт.
> 
> Когда первый входной бит 1, LOAD память и пошлите её на выход.
> 
> Когда второй входной бит 1, SAVE входной байт.
> 
> У выхода есть активирующий контакт, активируйте его только если LOAD.

Это байтовая память на 1 ячейку (ОЗУ **RAM** (Random Access Memory) запись/чтение) с управлением:

Входы
* LOAD (бит) — читать сохранённое значение
* SAVE (бит) — записать новое значение
* DATA IN (байт) — что сохраняем

Выходы
* DATA OUT (байт) — сохранённое значение
* ENABLE (бит) — выход активен только при LOAD
 
![Saving Bytes](/Computer-Science-Bookshelf/img/tc/Saving_Bytes.png)

#### Circuit Simulation: Saving Bytes

Необходимые компоненты:
* [Delayed Lines](turingcomplete_memory.html#delayed-lines)

Пример построение однобитной памяти с помощью D-триггеров (D Flip-Flop) - хранит 1 бит при тактировании.

Ячейка памяти через D-триггеры (D Flip-Flop). 
Это качественно и правильно для синхронного регистра, потому что запись происходит строго по фронту такта (строго в момент), нет проблем с гонками, как у защёлки/latch (пока Enable=1 - прозрачность, риск гонок). В реальных процессорах и ОЗУ ячейки строят на триггерах, а не на защёлках, именно для синхронности.

Компонент D-триггер (D Flip-Flop)
* вход `D` бит данных
* вход `>` тактирования
* вход `S` асинхронный Set (часто обозначается `S, PRE, PSET` — Preset). Когда на этот вход подаётся активный уровень (обычно 1), выход Q немедленно устанавливается в 1, независимо от тактового сигнала и входа `D`
  * Асинхронный - значит действуют мгновенно, без ожидания такта.
* вход `R` асинхронный Reset (обозначается `R, CLR, RST` — Clear). При активном уровне на R выход Q немедленно сбрасывается в 0, независимо от такта и `D`
* выход `Q`
* выход `!Q` инверсный выход

Запрещённая комбинация. Если `S=1` и `R=1` одновременно для одного триггера — состояние выхода неопределённое (может быть 0, 1 или колебания). В реальных схемах это не допускается.


Логика LOAD/SAVE это логика WE/RD, реализуется через 2-to-1 Multiplexer (MUX)

WE — Write Enable (Разрешение записи)
* WE = 1 → запись новых данных в регистр из шины (BUS)
* WE = 0 → запись новых запрещена (старые данные записываются в регистр из Q выхода D-trigger, так как D-trigger предполагает по такту тактирования запись иначе может хранить мусор)

RD или OE — Read Enable / Output Enable (Разрешение чтения на общую шину)
* RD = 1 → чтение данных из памяти/регистра на общую шину
* RD = 0 → выход в высокоимпедансном состоянии (Z) на общую шину

CS (Chip Select) нужен для выбора конкретного устройства из множества на общей шине.
* Так как одновременно читать в шину может только один исчточник, то это можно обеспечить внешней логикой, не полагаясь на корретные инструкции.
* Роль уникального читателя на шину выполняет декодер адреса 3 на 8



> Зачем нужен Multiplexer перед D-триггером?
> 
> У нас есть два источника данных для входа D:
> * Внешние новые данные (когда SAVE=1)
> * Текущие сохранённые данные с выхода Q (когда LOAD=1 — сохраняем старое)
> 
> Мультиплексор — это просто переключатель между двумя источниками.
> * режим WE = 1 соответвует режиму SAVE=1, мы записываем новые данные в регистр с DataIn. 
> * режим WE = 0 соответвует режиму SAVE=0, мы не записываем новые данные, мы обновляем старыми
>
> Реализация Multiplexer перед D-trigger не нуждается в Z состоянии. D-trigger не умееет хранить Z состояние.
> Поэтому в реализации Multiplexer нет буффера, достаточно простой логики (CMOS-логика):
> 
> `D = (WE AND BUS) OR (NOT WE AND Q)`
>
> WE = 1 (режим записи):
> * D = (1 AND BUS) OR (0 AND Q) = BUS
> * → На D идут данные с шины (новые данные).
> 
> WE = 0 (режим сохранения):
> * D = (0 AND BUS) OR (1 AND Q) = Q
> * → На D идёт текущее значение Q (старые данные).

 
> Зачем нужен Tri-state buf на выходе с D-trigger?
> Так как такие регистры будут использоваться вместе, что предполагает общую шину данных для входа и выхода, то на выходе иногда потребуется Z состояние.

> [!IMPORTANT]
> В один момент времени только **одно** устройство может выдавать данные на шину.
> 
> Но читать с шины могут **многие** одновременно. Но у нас декодер 3 на 8 жестко кодирует только однин адрес.

<div class="sim-wrapper" data-circuit-id="20">
  <button class="sim-fullscreen-btn" data-circuit-id="20">⛶</button>
  <iframe 
      id="20"
      data-circuit-id="20"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=true&startCircuit=/turingcomplete/20_saving_bytes.txt"
      loading="lazy">
  </iframe>
</div> 

---

## Counter

> [!TIP]
> Разблокирует компонент: 8-bit Counter


> Задача:
> Создайте счётчик, который будет увеличиваться на 1 при каждом такте.
> 
> Кроме того, должна быть предусмотрена возможность перезаписи счетчика заданным значением.
> 
> Вам даны два входных параметра: бит (MODE) и байт (INPUT_BYTE).
> 
> Входной бит (MODE) должен переключаться между подсчетом и перезаписью значением байта.


Каждый такт:
* если MODE = 0 → counter = counter + 1
* если MODE = 1 → counter = INPUT_BYTE

Где:
* MODE (бит) — выбирает режим
* INPUT_BYTE — значение для перезаписи
* counter хранится в регистре (delay byte + обратная связь)

Есть 3 сущности:
* Регистр → хранит counter
* Инкрементер → считает counter + 1
* MUX → выбирает, что писать в регистр

MUX
* Если select = 0 → на выход идёт первый вход (counter)
* Если select = 1 → на выход идёт второй вход (INPUT_BYTE)

[Counter (www.falstad.com/circuit)](https://www.falstad.com/circuit/circuitjs.html?ctz=CQAgjCAMB0l3BWcMBMcUHYMGZIA4UA2ATmIxAUgoqoQFMBaMMAKDDwBZwxIvtC83fiH6CqrAB4gOCLg1kVCKEPIhcUXAJIA7AA4BXAC4AdAM4AhAJ6G6LKR0JcOGCAmHOnIDSACyAeQARAFEWABkQRyomMGUEYmVo5SoqADMAQwAbUzpqSDCIjioHLjjlYqgK9KycpDzwyJViQgp4xubkkCrs3PyGhjxaVv6O1Mzu2t7ClUcWhJmOrpqoSaiOQVKVNYrR6p76qYZsWKGj7c6xpbqCqLBmjaZ2you964qNjohFnqkEARVmWicf6QchcHjgFB4OyKQQMYglDDERpqZAFaG-WFYCiIlQYTzgo7ov4MQiuHHyQRg8SEImwjjYbEMhhTKkgKE-YlERkqFAM1kcWn-MlMymolCChkIHARaSo1gAd2QeEpMwa5TyYCa4GVdNu4GIvE2euUUPC7GV-1a7AOmqS4HOuwmZp1PO81qiKG8C2eTu1FoYKF+2oOgceO3Gy2d-shgndPIt3sdkb9sKO4k4UTTZy+vvNqYQyjjhwL2Z9ybzKmwQKLVa4iYjVwrzL1RY4evrl2hYAwkMrYAZ3eIqfB6i4AEEAgEzKwo4ItXGtR2emAEEh+4QqKJwBxvFvkmxqzxR+J114puIWIqeBbytfBNhhHkr-gVWDbq-ltbuIbmIXT7+KlYL8ATkFtTweQCDzfQ0mA4N9hFgutwCg785EwP8EPQyDgKPTY4CEZpmXwi8cJggtY3A8jIOfC0H2aZg-jo5YaPvYQ7y8fBmKVQQ0Aov5eK4hjBAGbi2X3FixNEgSn24YktnY5kxEvUTFNEkSZKE-4MHol8tPaZSFO7HT-S2DT30rH9dMOQ0NKs3AqX9IzBPM5lTgU+kkgMqyPJU+yuIaKg1WIgzcMKQKWWCq9cKwwggywmS+j1WLCPbZTkv+Zp0vikKfzgiIe23Os0oOU5CAKw5PMVMqElKgqwDysyf0yi1Uqqg4tgEOklKqkzBE68B9Lam4YleZlFxyuQOC1PopsG1DKz+dLDj+BKg2Wvq1vGqrNsyg4tvmgMpmqnlzzS8qNEC8r1OKqIROO4ZnJggD+sSfzHJGl75hu6YuE+oqVzXfgqAu7dvBB-cAfAIGMtB0CwzYVcoY3bhCx3N87Q1RH1w9Oq0Z5AqIax6HENh-4Gs-InkfkXt6u8amxApwGqbC0miJGAy0E3P5sZEFaOfwkGefB-mT0LaGAI0zmYZ5iDJfwkmZfqoqrylgM6uJ7KVfllmZZZuWoiomWqKuBoqI2Y37RzQToZ16HNaR4H1eRpXredvKedah2Bq916NOJgCZeFq9idmn29wM4mt0D06rxkQjDcIGZ6cExO5ATpPbnElMMupDPkY0uO8OpLY2ZTkudccWEYm67OmBpyv-l7DSXThejCFOVvBMLtXC3bhJ7duDusMHhJ9vqtaXduPUSeb-0ZinwivqvUkUvdlefq7ie2+nz2F8rXud7mveKtr04C6DAP14lgyXWpdsU-vvfr9juKphHs8s4rNy+5ESqBtOb+pwg7Z3ng3Je24gxRwbuHZeHUwTQL5leIcchsAlk5pmEsMl0GVjQfAFQvxP64WTgpY2Xl-SGysgQwSlDkbYPkPnAyyD97A0gNPLMWDWGESzGgaeUo5o8PjtpZA5U+HUJEUIng5U9ZeSkfhECRdBJMIDBaNAQ8EzKVUf3FRkAO7vA0TohIGweBrT0c+ExVpIBrQHpYwiw8iEDyUaGDiJcnEcJcXFXgsINw1zQCXbxyADj+NsoEziR4bgNS8jad2RCXYaUcWDSASdPTKw4kkhJSclApLQBkt0kA6aZOofk3JdNd55LhgdXeSjbQcREkwVoHDanVN4lEAK+jakNB4JxEkMdkBdI6XACif9OmDPms-fUQIHgsK1JMriaBpmZw4tM9KHCllBlIHMIMMl1nTDWa0YB2zBbRR6ZqCZ1cvAATrj4i5ZyUAXOOlgu5BUyCLwJspZ50wnlCJgSAd5UdcLfN5MSApgK6TgN5BM4FW4SRehYJoLwTiSScV5GgaYnFkjQCQAgDRtZ8F7JxdCrJ+KqHgrTk3DRcFYRURJfgslioNAlypWsA2tKvBZhJHFNlQTsUosRTxNlVCsFEtofyhhdL8VZU5ZshG5BqXsuaBoaeyyQDkCziCzYMw1XWUJRM8omr1SwvhWtClrKeXGvRZi7FOqgH4v1WKnVAt8UAuNetVlEynUly3BoWpAK2Wh2RasUy3KA18tNZFV1k0HWhtVTarU-rNjjRXDKmN8q2yEVDlQFVFMk2nKEQqlKQiM3bEtaBXNbKnKComSJOND1BU8puWy66Yq62lrrSy+lXUzy1MDXS51WxZXdvDSoKtNqfH4oArK2JBrPQT17NWs55qKDFrJuoKFk6xXEhdmq1KibB2HGEHmysj5lVFrtSg-dbKmIVrQlMatwDb0qLZXei956eUxlmc6gSXqPRovJS4pF+LpJLsOsDADP7T342ULKvyeQ4XTtsQ+19CYoAYsXeBrVrLiTQexZhw0mrJ07upZu1NhUKiZqvSRuNa7B0gzjU+lFTTuE9Mo3lSj9Tf08QKl+rwrye19s4zi+21KaMCZ49R-91YC1TqcQx+j9TkMWvA05Ldkn12xlLYxR80rd3GmI7LY90bTnyrLfwnFw6UWNpNZuSDWYLNzqMyis+7GUadtFu++SI1CMjXI8Ow8f9qV9pxalWDTjrUOYxihrFimjPcxMzFjDan9IEZxbq4j6p9OzOSxqrMtrB3EqzAKoNFAPP5dFZZ6QWWUUIC805jYXHTFni2LVnF9XqV5aBC15rs7kvjWC0GKr1nKtefk6hwdfqtzpq09SvyB7oPpfI9NrMWHwMszjdIptm40HZbDXGhbDnMFOZ8lxnyWDjWHeS455bDqgTHaXZ6nFl6pOQM23t8LCnB1MTVQ91TIhz1-Am3SvBxZIMgyB7MhF-jkktLA-CkqaCQdcrpQi41cHmGzM4ajlHFVVUGPjDxBFb63GwicZj1xGiEU3Px1KxHkDgdTAuzDqzDPXRZMSWhMGIPklg4vnszn1SsG855yUtjAPGmC9AsLw1zRUHA84tLsHsu4ddIWSslKtCBIzP531jz1T+uzJ19roY86NHXNndUy5yZZWQoheeS2ZY8g+HGZNFsSi0soAxV2fgoI8D0TWGUPASJRwgACAwAAKgAJU0AAcSCGHrsMRYisO4BgTFhpA8+AAKoAA1oTYDbBQXPFB6T59BGeEAABhUIfgy8AGkzAAAptB0AkCYUwpgbC6AAJTfUNtDUhwdka22RqQqQ5okS3HomQAcJBZTeAnMEWPUhsmESxLc6eeIZ9cD8OnkPLAgA)

<div class="sim-wrapper" data-circuit-id="19">
  <button class="sim-fullscreen-btn" data-circuit-id="19">⛶</button>
  <iframe 
      id="19"
      data-circuit-id="19"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=true&startCircuit=/turingcomplete/19_counter.txt"
      loading="lazy">
  </iframe>
</div> 


![Counter](/Computer-Science-Bookshelf/img/tc/Counter.png)


> [!TIP]
> Этот уровень суммирует наши предущие полученные знания для построения счетчика PC (Program Counter счётчик)
> 
> Необходимые знания для построния счетчика:
> * полусумматор
> * полный сумматор
> * 8-битное сложение
> * линии задержки
> * D-триггеры
> * регистры
> * SAVE / LOAD
> * счётчик (логически)



---

## Little Box

> [!TIP]
> Разблокирует компонент памяти: 256 байтная ОЗУ **RAM** (Random Access Memory)

Можно ли уместить 4 байта в этом ограниченном пространстве?

> Задача:
> 
> На этом уровне вы должны построить схему которая может [SAVE] или [LOAD] (показать (выдать) значение выбранного регистра на выход) из 4-х разных байтов памяти.
> 
> Вам даётся один быт который определяет нужно ли вам [LOAD]. 
> Другой бит определяет нужно ли вам загружать [SAVE] и к нему прилагается значение.
> 
> Кроме того у вас есть 2 бита для адресас, что дает 4 комбинации, одна на каждый байт на этом уровне.

Для этого уровня вам, по сути, нужно создать четырехбайтовую оперативную память ((ОЗУ **RAM** (Random Access Memory) запись/чтение)) в ограниченном пространстве.

**RAM 4×8 бит:**
* 4 ячейки памяти
* каждая по 8 бит
* выбор ячейки по 2-битному адресу ADDR[1:0] — (00, 01, 10, 11)
* операции, декодер выбирает ровно один регистр:
  * SAVE — при 1 записать байт в выбранную ячейку, при 0 не записывать
  * LOAD — при 1 вывести байт из выбранной ячейки, при 0 не выводить


Для начала вам понадобятся четыре 8-битных регистра для независимого хранения и извлечения байтов.
* Соедините все выводы сохранения значений с входом, а все выводы вывода — с выходом.
* Подключите вывод загрузки к выводу разрешения выхода.

Далее соберите 2-битный декодер для активации одного из регистров за раз.

Наконец, используя восемь переключателей, создайте две шины, которые будут управлять выводами загрузки и сохранения регистров, и подключите их к соответствующим входам.

Соедините пары переключателей с соответствующими выходами декодера, и готово!

![Little Box](/Computer-Science-Bookshelf/img/tc/Little_Box.png)

(Так же можно использовать компонент Switch вместо NOT + AND. Узел Switch можно использовать как логическое И, поскольку для его включения необходимо выполнение двух условий.)

**2 bit decoder (2 to 4)** нужен для выбора одного из четырех регистров

```
A B | addr
-------------
0 0 | 0001 D0  
0 1 | 0010 D1  
1 0 | 0100 D2  
1 1 | 1000 D3  
```

<div class="sim-wrapper" data-circuit-id="22">
  <button class="sim-fullscreen-btn" data-circuit-id="22">⛶</button>
  <iframe 
      id="22"
      data-circuit-id="22"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=true&startCircuit=/turingcomplete/22_2_bit_decoder_2_to_4.txt"
      loading="lazy">
  </iframe>
</div> 


**RAM 4×8 бит:**

<div class="sim-wrapper" data-circuit-id="23">
  <button class="sim-fullscreen-btn" data-circuit-id="23">⛶</button>
  <iframe 
      id="23"
      data-circuit-id="23"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=true&startCircuit=/turingcomplete/23_ram_4_byte.txt"
      loading="lazy">
  </iframe>
</div> 







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