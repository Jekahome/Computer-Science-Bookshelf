
# Turing Complete 

* [Turing Complete](https://turingcomplete.game/) — это игра о компьютерных науках.
* [Turing Complete youtube channel](https://www.youtube.com/@TuringCompleteGame)
* [strategywiki.org/wiki](https://strategywiki.org/wiki/Turing_Complete/Counter)
* [Nari youtube channel](https://www.youtube.com/@nari1774/videos)
* [Процессор RV32i](https://github.com/BenjaminSchaaf/turning-complete-riscv) полноценная реализация RISC-V CPU внутри игры, на который можно запускать реальный компилируемый код (например, Rust). Игра превращается в мини-симулятор архитектуры компьютера.
* [MegaIng/turing-complete-interface](https://github.com/MegaIng/turing-complete-interface) библиотека для взаимодействия со схемами, созданными в игре Turing Complete

---

* [github.com/tomwhite/8-bit-computer](https://github.com/tomwhite/8-bit-computer)
* [Building an 8-bit breadboard computer!](https://www.youtube.com/playlist?list=PLowKtXNTBypGqImE405J2565dvjafglHU)

> [!IMPORTANT]
> **Существа, которые смогут завершить создание компьютера, по закону считаются разумными.**
>

Всё в компьютере может быть построено из базового компонента, называемого логическим элементом [NAND](turingcomplete_basic_logic.html#nand--a--b). Вам предстоит решить ряд головоломок, чтобы проложить путь от элементов NAND к арифметике, памяти и вплоть до полноценных архитектур центрального процессора. Пройдя эту игру, вы получите глубокое понимание взаимосвязи между ассемблером, наборами инструкций ЦП и базовыми компонентами. А также поймете, как работают такие концепции программирования, как условные операторы, циклы и функции, на уровне ассемблера и аппаратного обеспечения.

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

При первом знакомстве с игрой Turing Complete без опыта в схемотехнике и дискретной математике вам, скорее всего, будет сложно понять суть задачи. То есть, сначала нужно разгадать условие, и только потом приступать к решению. Именно в этом и заключается смысл слова игра применимый к Turing Complete, так как, по большому счету, это познавательный симулятор с багом - от игрока требуется предварительная подготовка.

С другой стороны, игра ведёт игрока пошагово к созданию компьютера, что является её сильной обучающей стороной.

Но так как наша главная цель — разобраться в устройстве компьютера, а не просто играть, нам стоит использовать подсказки из различных источников, чтобы понять, как работает тот или иной компонент или схема. Желательно также модифицировать решение самостоятельно, чтобы закрепить знания. Важен и сам процесс, поскольку в ходе самостоятельного решения мы параллельно приобретаем новые навыки, помимо достижения цели. **Таким образом, важно и понимание принципа работы компонента, и сам процесс его конструирования!**

Я постараюсь дублировать смысл поведения компонента/идеи в виде кода или реальной схемы, там где это уместно.

### Симуляторы электрических цепей

1. Симулятор электрических цепей [Falstad](https://www.falstad.com/circuit/) — это интерактивный, упрощённый SPICE-подобный симулятор **физических процессов**. Он полезен для использования в браузере. 
2. Симулятор электрических цепей [Digital](https://github.com/hneemann/Digital?tab=readme-ov-file) — это **цифровой** симулятор архитектуры ([Обзор. Hneemann Digital Tutorial](https://www.youtube.com/watch?v=agO4eO-bamk)).
   * Симулятор электрических цепей [Logisim-Evolution](https://github.com/logisim-evolution/logisim-evolution) (устарел) 
3. Другие [бесплатные симуляторы](https://crimmscoolclass.github.io/software.html) 

---

## Turing Complete levels

1. [Basic logic](turingcomplete_basic_logic.md)
2. [Arithmetic](turingcomplete_arithmetic.md)  
3. [Memory](turingcomplete_memory.md)      
4. [CPU Architecture](turingcomplete_cpu_architecture.md) 
 

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
 