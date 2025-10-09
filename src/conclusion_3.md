# Чарльз Петцольд. КОД тайный язык информатики

## Глава 10. Логика и переключатели

Явившись в зоомагазин, вы сказали продавцу: «Мне нужен стерилизованный (С) кот (М) белого (Б) или рыжего (Р) окраса; **или** стерилизованная кошка (Ж) любого окраса, кроме белого; или я возьму любую из имеющихся у вас черных кошек (Ч)», — и продавец составил такое выражение:

Каждый символ «×» соответствует месту схемы, где два переключателя (или две группы переключателей) соединены последовательно, каждый символ «+» — месту схемы, в котором два переключателя (или две группы переключателей) соединены параллельно.
* символ `+` соответсвует параллельному соединению т.е. `ИЛИ`
  * т.е. кошка/кот не может быть двух полов одновременно, поэтому или так или иначе т.е. **что-то одно**
* символ `x` соответсвует параллельному соединению т.е. `И`
  * т.е. кошка/кот может быть белая/черная/рыжая и **одновременно** женского/мужского пола, тут нет противоречий

    `(М × С × (Б + Р)) + (Ж × С × (1 − Б)) + Ч`

Действительно, если вы просмотрите электрическую схему слева направо и сверху вниз, то столкнетесь с буквами в том же порядке (ну почти), в каком они представлены в выражении.

[схема www.falstad.com/circuit](https://www.falstad.com/circuit/circuitjs.html?ctz=CQAgjCAMB0l3BWcMBMcUHYMGZIA4UA2ATmIxAUgpABZsKBTAWjDACgB3WsQ2uCwlRr9IbAG7dewqrhp8qCvtUUwEbHigEyEmhIPkhNAEwYAzAIYBXADYAXJtYZHwUVzEjsATiFkGwGKRFkODYAZx80Hx1JKM0qCAtrUIY2AA8opGxsPB8MGRoIORQ5QHIQMPAAwwxNFDwczDjXROTy-15saLaqxqpmlK4u6Qr26NF0sBpiHzApnnaUTSK5QEIQTmHu9Zm41sqtwzrwYkaE8ySU8NqcniEecH14kD61y7uqF73RLnej-friqDWBV4eyBhn+YzB9BQxF4xSQ0KmSxAgDQQNJg+FgeE0IpgHJIwA4IGjsNjwDRMghZjQ8YY5IBEECJNHIxRy2EINQQhRpIHp6WJTImPkIUP8tC5gAIQNgAZV+G1Z9WqrhOZ1cKDWXQ6mmwkQ1AIuBwWclBBsVj1OLS4em0i1uOs+MRecpldtBe0th22Fv0rv0tsBt2uMQ+fth-yNB1EQA)


