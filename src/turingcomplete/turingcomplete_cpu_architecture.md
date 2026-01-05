# CPU Architecture

## Arithmetic Engine

> Задача: 
>
> Добавить еще два варианта в схему [Logic Engine](turingcomplete_arithmetic.html#logic-engine):
> * ADD для сложения двух восьмибитных чисел
> * SUB для вычитания двух восьмибитных чисел
> ```
> 0 OR
> 1 NAND
> 2 NOR
> 3 AND
> 4 ADD
> 5 SUB
> ```
> т.е. нам хватит всего первых трех младших битов
> ```
> 000 OR
> 001 NAND
> 010 NOR
> 011 AND
> 100 ADD
> 101 SUB
> ```