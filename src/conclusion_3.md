# Чарльз Петцольд. КОД тайный язык информатики

## Глава 10. Логика и переключатели

Явившись в зоомагазин, вы сказали продавцу: «Мне нужен стерилизованный (С) кот (М) белого (Б) или рыжего (Р) окраса; **или** стерилизованная кошка (Ж) любого окраса, кроме белого; или я возьму любую из имеющихся у вас черных кошек (Ч)», — и продавец составил такое выражение:

Каждый символ «×» соответствует месту схемы, где два переключателя (или две группы переключателей) соединены последовательно, каждый символ «+» — месту схемы, в котором два переключателя (или две группы переключателей) соединены параллельно.


`(М × С × (Б + Р)) + (Ж × С × (1 − Б)) + Ч`

или так

`(С × (М × (Б + Р)) + (Ж × (1 − Б))) + Ч`

* символ `+` соответсвует параллельному соединению т.е. `ИЛИ`
  * т.е. кошка/кот не может быть двух полов одновременно, поэтому или так или иначе т.е. **что-то одно**
* символ `x` соответсвует параллельному соединению т.е. `И`
  * т.е. кошка/кот может быть белая/черная/рыжая и **одновременно** женского/мужского пола, тут нет противоречий
* `1 − Б` означает - не белая, т.е. `НЕ`

Действительно, если вы просмотрите электрическую схему слева направо и сверху вниз, то столкнетесь с буквами в том же порядке (ну почти), в каком они представлены в выражении.

![Глава 10. Логика и переключатели](img/10.png)

[схема www.falstad.com/circuit](https://www.falstad.com/circuit/circuitjs.html?ctz=CQAgjCAMB0l3BWcMBMcUHYMGZIA4UA2ATmIxAUgpABZsKBTAWjDACgB3WsQ2uCwlRr9IbAG7dewqrhp8qCvtUUwEbHigEyEmhIPkhNAEwYAzAIYBXADYAXJtYZHwUVzEjsATiFkGwGKRFkODYAZx80Hx1JKM0qCAtrUIY2AA8opGxsPB8MGRoIORQ5QHIQMPAAwwxNFDwczDjXROTy-15saLaqxqpmlK4u6Qr26NF0sBpiHzApnnaUTSK5QEIQTmHu9Zm41sqtwzrwYkaE8ySU8NqcniEecH14kD61y7uqF73RLnej-friqDWBV4eyBhn+YzB9BQxF4xSQ0KmSxAgDQQNJg+FgeE0IpgHJIwA4IGjsNjwDRMghZjQ8YY5IBEECJNHIxRy2EINQQhRpIHp6WJTImPkIUP8tC5gAIQNgAZV+G1Z9WqrhOZ1cKDWXQ6mmwkQ1AIuBwWclBBsVj1OLS4em0i1uOs+MRecpldtBe0th22Fv0rv0tsBt2uMQ+fth-yNB1EQA)

## Глава 11. Логические вентили

Логические вентили можно комбинировать для решения более сложных задач, например для выполнения простых арифметических операций.

![Логические вентили](img/Логические_вентили.png)

[Логические вентили OR, AND, XOR](https://www.falstad.com/circuit/circuitjs.html?ctz=CQAgjCAMB0l3BWcMBMcUHYMGZIA4UA2ATmIxAUgpABZsKBTAWjDACgAPEJwlFCjBCYIM-ETVogUEgHIB5ACpswGPNxphCFGhKaYxOkFoiooUWEjDx45yPzS3+MRFDYBzbvoFa9oo-jNINgAldU1tXQ0tGjwqONoqJHiYBDYAJ09pChQ1PSyEYidkII88iQL+Mv81OM5uXi1sbGJucRBsGn4JLIANOWC6nhVuDCphPCFCcm6JfsHCYbAC1qnwXkksgEEZABFldbHwqnD+ABMGADMAQwBXABsAFyY7hlPwQNt2TUrY8ELuX7SKhnS63R7PV7vZKwL7rFgYRp4XQqaLYewgc7Xe5PF5vEyBGHKVTqX4sMCVGhwbisfjGcz45yWaxU5xiOKwJywSBJIm5HK5GmeAJk2lQ6AMizFayOCjsuy2bmuFQC8nU1VMNFq0UmcUExVWZkyygy5yKoKhJgxQ7qhDqwXxSnUaGpUqUsakt0kwLuIW5UloXIxb2lTVMUmhoO1ADuJPdY1tP2jnjwfrGaLjrhjIoBh0MYdqABlWurSQmPlRrncAM4MJ1sIsahz57jp8sgSs1usNsB50k9iTJdtXau1nkAWTMTGI0Vy07MKGgqQyYznhy0xzgPuO6-aRXNyZVlVb2YdiXLi5CWqvLDz9rMjqSBNSWc9U582EIs-XbCzH6-Lc-alCCTDVAJYYDqQRICQJRaDIJ8Vcf3g7g5z0YVEKzWDwPibDM0nTQcIwltKlXFDv0w1VBWbe0fWbIEcykBAdyCDJqJLY5-g3IIJzYo9+DvBdUgbPgJHJPksgDD4O1HVwGz-cAcgAtRJPiaSu2LfjFOEVUVLMNSeUw-4xMyUTFKCX8PwU3J5OM8zYysjSHJKK9jOzAjg08JiHL0Lz3NqV0qVcz0-Lwlh-lQsAjNVOyfOMO0oqcJCNUs1CmjixKX0C9U0vAaLL0tX5UIQRTSKoB9z1SZVqXCnxBTnOkYAlfVmTlNkTS5HkquSrRUJQLz6rFJqmUNVlZXalwgi6q0QFQz0Bp1IapRsUbjVG01x2Q3BkUUrb5wvUpYLoSpDsjVxWNg3b4URAdiiSvrGkgZEoN2oJhKyHKqiiMwIH02THI+ssvuOIcRzrLN7vaSyqhymLpoB1UYdo7Bish98UeRvckaRVGW2xjHvQtOHHv+4nTydJ8kuwPGm3x-lQqp7oUxbdGHEm4lYvaYmNXR4mGt1ZxSEFoWhdEehGSWlkOTGtaOvAXl7MuhnOYkPmdUgYWNdIUXbGG6UVtahVLHrMItBEHwyzNqTh07Tq4S8YrxPKRSQSxcFcShPV2B4kTsgFKCHb2yriQhuccvm+lbE1jXtfFg09al1apfWuWqrmmcqHDxrI6jkWUDFyU4+WhODeT9hQhDrRJNKhJyfZZ8ccFYK8pjNOEmOZvaF+QUEEMGiPBy4yB4g-zO+ORTppCuyIeMn3bKQ4KdsAuei1n52JICYHfte9ol8X5SN-eLe2AnHAtEICRVBVm7BP2WkmfPkBT6MboMVBbEITxc9IHYDwn+Mv-DC1AyE-B+f8zK3VCD3CQvwoFrA+OVZ0PpPxqEFMg8AgClTElgb8NBOCxScmkAQGg58dB9UgOmPA+dmojSlqzJOst2BcFYC0VgEAwB4HIKwCQMwQAyG2HsGMTFrRiC0jRGM8kroUAgpIuyEjBRyKgrIqG8iwI0VCDoAU49wgsHAWVM8iCixfTfLQbRRxD7WxktvH2xjrFmJ+hYsGtAgzGKMURCGNisjGOcgHYxPiH4jw+qHKG-jXDly8jo5SnjbL3n0RTVOzjjI+OMqrbOOctZ5x1hLA2id5Sl3lu4weyjFIpIFmk9JVDdZF3lDkzkLhGG5RaGGYwfgwz0B4fIAYVVeDIkMAiAUhg+YEJoMQTQnRPw9ywAscgscWoyjobkhhPo+lXmWc2WooRunUgGZ43Seja7mCXCACYtUILHLIt9TcHgznGOub8Wo3xH6qNOaYiCLswQ4khAyT4x8jkvKoHgB+uFgQXkEdI1BUSO5COQlCk8+S-Ctnkq2FJ2BoANEgMBDG6tSDfwQMwMAFSslGhLrLNmahhmNAcJ6JFg1UnEEoKMKwxB0XECZjMmh1TiV1KQoi4mVLLKyMAjlKlpNLxeFbPRVsZNHx1x9Ii9GgrAIjypSjKliq8JipRhKlGdlyXtBRgHfGVjHS00dEdK2oMeRFgNfq8empN4OIMrQUODhLaSvSN4XciQoKei4j6S2GiPXEJupNdY-qqT+qDG89+7svnOHqeSRoCBLB0r1WSqQEgegdPlsBZSTMMCavvvg+kTLNDI0gPmvOKZhmZMLpLeU8zan6iWSzKgFa9WYw2QEOmOapBZClRVd1sRgR+CHTjX1VyqQ5VHYapU6w21eFHV4KNbtPlfy9kcqkC7I1+GBfXV1Dge1uqAA)


**AND** в логике и программировании означает И (логическое умножение или конъюнкция). Это логическая операция, которая выдает истину (True), только если оба операнда истинны. Если хотя бы один из операндов ложен, то результат — ложь.

|AND| 0 | 1 |
|:--|:--|:--|
| 0 | 0 | 0 |
| 1 | 0 | 1 |

**OR** Это логическая операция, которая выдает истину (True), если хотя бы один из операндов истинен.

|OR | 0 | 1 |
|:--|:--|:--|
| 0 | 0 | 1 |
| 1 | 1 | 1 |

**XOR** (от англ. eXclusive OR/Исключающее ИЛИ). Это логическая операция, которая выдает истину (True), если только один из операндов истинен, и ложь (False), если оба операнда одинаковы (оба истинны или оба ложны).

|XOR| 0 | 1 |
|:--|:--|:--|
| 0 | 0 | 1 |
| 1 | 1 | 0 |


**NOT** Инвертирует входное значение. Истина становится Ложью, и наоборот.

0 -> NOT -> 1

1 -> NOT -> 0

**NAND** (Not-AND) Противоположность AND. Выдает Ложь, только если оба входа Истинны. Т.е. сперва применяется операция AND и к результату ее применяется операция NOT: 1 AND 1 = 1 NOT = 0

|NAND| 0 | 1 |
|:-- |:--|:--|
| 0  | 1 | 1 |
| 1  | 1 | 0 |


**NOR** (Not-OR) Противоположность OR. Выдает Истину, только если оба входа Ложны.  Т.е. сперва применяется операция OR и к результату ее применяется операция NOT: 0 OR 0 = 0 NOT = 1

|NOR| 0 | 1 |
|:--|:--|:--|
| 0 | 1 | 0 |
| 1 | 0 | 0 |

**XNOR** (Exclusive-NOR/Исключающее ИЛИ-НЕ). Противоположность XOR. Выдает Истину, если оба входа одинаковы (оба Ложны или оба Истинны).

|XNOR| 0 | 1 |
|:-- |:--|:--|
| 0  | 1 | 0 |
| 1  | 0 | 1 |



