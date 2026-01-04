
# Turing Complete. Basic logic

* [Turing Complete](https://turingcomplete.game/) — это игра о компьютерных науках.
* [Turing Complete youtube channel](https://www.youtube.com/@TuringCompleteGame)
* [strategywiki.org/wiki](https://strategywiki.org/wiki/Turing_Complete/Counter)
* [Nari youtube channel](https://www.youtube.com/@nari1774/videos)
* [Процессор RV32i](https://github.com/BenjaminSchaaf/turning-complete-riscv) полноценная реализация RISC-V CPU внутри игры, на который можно запускать реальный компилируемый код (например, Rust). Игра превращается в мини-симулятор архитектуры компьютера.
* [MegaIng/turing-complete-interface](https://github.com/MegaIng/turing-complete-interface) библиотека для взаимодействия со схемами, созданными в игре Turing Complete

> [!IMPORTANT]
> **Существа, которые смогут завершить создание компьютера, по закону считаются разумными.**
>

Всё в компьютере может быть построено из базового компонента, называемого логическим элементом NAND. Вам предстоит решить ряд головоломок, чтобы проложить путь от элементов NAND к арифметике, памяти и вплоть до полноценных архитектур центрального процессора. Пройдя эту игру, вы получите глубокое понимание взаимосвязи между ассемблером, наборами инструкций ЦП и базовыми компонентами. А также поймете, как работают такие концепции программирования, как условные операторы, циклы и функции, на уровне ассемблера и аппаратного обеспечения.

![you learn this](/Computer-Science-Bookshelf/img/tc/turingcomplete_1.png)

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

1. [Basic logic](turingcomplete/turingcomplete_basic_logic.md)
2. [Arithmetic](turingcomplete/turingcomplete_arithmetic.md)  
3. [Memory](turingcomplete/turingcomplete_memory.md)      
4. [CPU Architecture](turingcomplete/turingcomplete_cpu_architecture.md) 
 
