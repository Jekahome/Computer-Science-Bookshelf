# Подходят ли ООП паттерны для Rust?

* [ООП vs Rust: Ключевые отличия](#1-ООП-vs-rust-Ключевые-отличия)
* [Когда ООП-паттерны не нужны в Rust](#2-Когда-ООП-паттерны-не-нужны-в-rust)
* [Когда ООП-паттерны полезны в Rust](#3-Когда-ООП-паттерны-полезны-в-rust)
* [Функциональные альтернативы в Rust](#4-Функциональные-альтернативы-в-rust)
* [Виды полиморфизма](#5-Виды-полиморфизма)


Rust — это статически типизированный язык системного программирования, сочетающий элементы **функционального**, **процедурного** и **ограниченно ООП-стилей**. Хотя Rust не является классическим ООП-языком (нет наследования, виртуальных методов по умолчанию), он позволяет применять **принципы ООП** через:
- **Инкапсуляцию** (модули `mod`, `pub`-видимость).
- **Полиморфизм** (трейты, generics, `dyn`). В Rust полиморфизм реализуется несколько иначе, чем в классических ООП-языках (например, Java/C++), но основные виды поддерживаются с помощью **трэйтов, обобщённых типов (generics) и системы владения**. 
- **Композицию** (вместо наследования).

Но важнее не "можно ли", а **"нужно ли"** — Rust предлагает свои идиомы, часто более эффективные, чем классические ООП-паттерны.
  

## 1. ООП vs Rust: Ключевые отличия

### Наследование → Трейты + Композиция
В Rust **нет наследования реализации**, только:
- **Наследование интерфейсов** через `super-trait`:
  ```rust,editable
  trait Animal: Display {  // Требует реализации Display
      fn speak(&self);
  }
  ```
- **Композиция** (встраивание структур):
  ```rust,editable
  struct Engine { power: u32 }
  struct Car { engine: Engine }  // Вместо наследования
  ```

### Виртуальные методы → `dyn Trait`

Динамическая диспетчеризация возможна, но требует явного указания:

```rust,editable
trait Draw { fn draw(&self); }
impl Draw for Circle { ... }

let shapes: Vec<Box<dyn Draw>> = vec![Box::new(Circle)];  // Явное указание `dyn`
```
 
## 2. Когда ООП-паттерны **не нужны** в Rust

### Стратегия (Strategy)

Вместо отдельного класса стратегии — просто трейт:

```rust,editable
trait SortStrategy { fn sort(&self, data: &mut [i32]); }

struct QuickSort;
impl SortStrategy for QuickSort { ... }

struct Sorter<T: SortStrategy> { strategy: T }  // Статическая диспетчеризация
```

### Декоратор (Decorator)

Композиция + трейты:

```rust,editable
trait Coffee { fn cost(&self) -> f64; }
struct SimpleCoffee;
impl Coffee for SimpleCoffee { ... }

struct MilkDecorator<T: Coffee> { inner: T }  // Обёртка
impl<T: Coffee> Coffee for MilkDecorator<T> {
    fn cost(&self) -> f64 { self.inner.cost() + 0.5 }
}
```
 

## 3. Когда ООП-паттерны **полезны** в Rust

### Строитель (Builder)
Идеален для Rust из-за неизменяемости по умолчанию:

```rust,editable
#[derive(Default)]
struct ConfigBuilder { timeout: Option<u32> }

impl ConfigBuilder {
    fn timeout(mut self, secs: u32) -> Self {
        self.timeout = Some(secs);
        self
    }
}
```

### Посетитель (Visitor)

Реализуется через трейты и `match`:
```rust,editable
trait Visitor {
    fn visit_circle(&self, c: &Circle);
    fn visit_rect(&self, r: &Rect);
}
```
 

## 4. Функциональные альтернативы в Rust

### Итераторы и замыкания

Вместо ООП-шаблонов часто хватает комбинаторов:
```rust,editable
let sum: i32 = (1..=10).filter(|x| x % 2 == 0).sum();  // Декларативный стиль
```

### Алгебраические типы

`enum` + `match` заменяют полиморфизм подтипов:

```rust,editable
enum Shape { Circle(f64), Rect(f64, f64) }

fn area(s: Shape) -> f64 {
    match s {
        Shape::Circle(r) => r * r * PI,
        Shape::Rect(w, h) => w * h,
    }
}
```
 

Вывод: Rust `≠` ООП, но гибок

| Паттерн       | Альтернатива в Rust               | Когда использовать?           |
|---------------|-----------------------------------|-------------------------------|
| Наследование  | Трейты + композиция              | Всегда                        |
| Стратегия     | Generics + трейты                | Если поведение статично       |
| Декоратор     | Композиция + трейты              | Для расширения функциональности |
| Строитель     | Method chaining + `Default`      | Для сложных объектов          |
| Фабрика       | Функции + `impl Trait`           | Когда нужна динамическая генерация |

**Главное правило**: в Rust сначала смотрите на **функциональные возможности** (итераторы, `match`, трейты), и только если они не подходят — применяйте ООП-паттерны в адаптированном виде.
 
---

 
Также благодаря `static dispatch` мы можем вынести потенциальные ошибки использования типов на этап компиляции кода.  
Система типов Rust может превратить многие виды проблем программирования в проблемы `static dispatch`. 
Это одно из самых больших преимуществ при выборе функционального языка, и оно имеет решающее значение для многих гарантий времени компиляции Rust.
В Rust параметр универсального типа создает то, что в функциональных языках называется «**ограничением класса типа**».
Это называется **мономорфизацией**, когда разные типы создаются из полиморфного кода.


## 5. Виды полиморфизма

Паттерны: Null Object, Strategy, Adapter, Decorator, Composite, Proxy, State — все это просто разновидности применения полиморфизма.
  
- **Null Object** — избегание null проверок
- **Strategy** — смена алгоритмов
- **Adapter** — совместимость интерфейсов  
- **Decorator** — динамическое расширение функциональности
- **Composite** — работа с иерархиями
- **Proxy** — контроль доступа и ленивая инициализация
- **State** — изменение поведения объекта


<details>
<summary>1. Null Object — полиморфизм для отсутствия поведения</summary>

```rust,editable
trait Logger {
    fn log(&self, message: &str);
}

// Реальный логгер
struct RealLogger;
impl Logger for RealLogger {
    fn log(&self, message: &str) {
        println!("LOG: {}", message);
    }
}

// Null-объект
struct NullLogger;
impl Logger for NullLogger {
    fn log(&self, _message: &str) {
        // Ничего не делаем
    }
}

// Использование через полиморфизм
fn process(logger: &impl Logger) {
    logger.log("processing...");
}
```

---

</details>


<details>
<summary>2. Strategy — полиморфизм для замены алгоритмов</summary>

```rust,editable
trait SortingStrategy {
    fn sort(&self, data: &mut [i32]);
}

struct QuickSort;
impl SortingStrategy for QuickSort {
    fn sort(&self, data: &mut [i32]) {
        // Быстрая сортировка
    }
}

struct BubbleSort;
impl SortingStrategy for BubbleSort {
    fn sort(&self, data: &mut [i32]) {
        // Пузырьковая сортировка
    }
}

struct Sorter {
    strategy: Box<dyn SortingStrategy>,
}
```

---

</details>


<details>
<summary>3. Adapter — полиморфизм для совместимости интерфейсов</summary>

```rust,editable
// Старый интерфейс
trait OldService {
    fn old_method(&self);
}

// Новый интерфейс  
trait NewService {
    fn new_method(&self);
}

// Адаптер - реализует новый интерфейс через старый
struct Adapter {
    old_service: Box<dyn OldService>,
}

impl NewService for Adapter {
    fn new_method(&self) {
        self.old_service.old_method(); // Адаптация
    }
}
```

---

</details>


<details>
<summary>4. Decorator — полиморфизм для добавления функциональности</summary>

```rust,editable
trait Coffee {
    fn cost(&self) -> f64;
}

struct SimpleCoffee;
impl Coffee for SimpleCoffee {
    fn cost(&self) -> f64 {
        2.0
    }
}

struct MilkDecorator {
    coffee: Box<dyn Coffee>,
}

impl Coffee for MilkDecorator {
    fn cost(&self) -> f64 {
        self.coffee.cost() + 0.5 // Добавляем функциональность
    }
}
```

---

</details>


<details>
<summary>5. Composite — полиморфизм для древовидных структур</summary>

```rust,editable
trait Graphic {
    fn draw(&self);
}

// Лист
struct Circle;
impl Graphic for Circle {
    fn draw(&self) {
        println!("Drawing circle");
    }
}

// Композит - содержит другие Graphic
struct Picture {
    children: Vec<Box<dyn Graphic>>,
}

impl Graphic for Picture {
    fn draw(&self) {
        for child in &self.children {
            child.draw(); // Рекурсивный вызов
        }
    }
}
```

---

</details>


<details>
<summary>6. Proxy — полиморфизм для контроля доступа</summary>

```rust,editable
trait Database {
    fn query(&self, sql: &str) -> Result<(), String>;
}

struct RealDatabase;
impl Database for RealDatabase {
    fn query(&self, sql: &str) -> Result<(), String> {
        println!("Executing: {}", sql);
        Ok(())
    }
}

struct DatabaseProxy {
    database: RealDatabase,
    has_access: bool,
}

impl Database for DatabaseProxy {
    fn query(&self, sql: &str) -> Result<(), String> {
        if !self.has_access {
            return Err("Access denied".to_string());
        }
        self.database.query(sql) // Делегирование
    }
}
```

---

</details>


<details>
<summary>7. State — полиморфизм для изменения поведения</summary>
 
```rust,editable
trait State {
    fn handle(&self, context: &mut Context);
}

struct StateA;
impl State for StateA {
    fn handle(&self, context: &mut Context) {
        println!("State A handling");
        context.set_state(Box::new(StateB));
    }
}

struct StateB; 
impl State for StateB {
    fn handle(&self, context: &mut Context) {
        println!("State B handling");
        context.set_state(Box::new(StateA));
    }
}

struct Context {
    state: Box<dyn State>,
}
```

---

</details>



### 1. Параметрический полиморфизм (Generics)  
**Есть**  
- Rust поддерживает обобщённое программирование через **Generics**.  
- Позволяет писать код, работающий с разными типами.  
- Этот код создаёт monomorphization, компилятор создаёт **отдельную версию функции `max` для каждого типа**, с которым мы используем функцию, что раздувает бинарный файл.

```rust,editable
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
fn main(){
    let m1 = max(5, 10);       // Работает с i32
    let m2 = max(3.14, 2.71);  // Работает с f64
}
```


### 2. Полиморфизм подтипов (Subtyping / Inheritance)  
**Нет в классическом виде**  
- В Rust **нет наследования** (как в Java/C++).  
- Вместо этого используется **композиция и трейты**.
- Этот код создаёт monomorphization, компилятор создаёт **отдельную версию функции `animal_sound` для каждого типа**, с которым мы используем функцию, что раздувает бинарный файл.  

**Альтернатива в Rust:**
```rust,editable
trait Animal {
    fn make_sound(&self);
}

struct Dog;
impl Animal for Dog {
    fn make_sound(&self) { println!("Woof!"); }
}

struct Cat;
impl Animal for Cat {
    fn make_sound(&self) { println!("Meow!"); }
}

fn animal_sound(animal: &impl Animal) {
    animal.make_sound();
}
fn main(){
    let dog = Dog;
    let cat = Cat;
    animal_sound(&dog);  // Woof!
    animal_sound(&cat);  // Meow!
}
```
(Это **полиморфизм через трейты**, а не классический subtyping.)


### 3. Ad-hoc полиморфизм (перегрузка функций/операторов)  
**Частично**  
- Rust **не поддерживает перегрузку функций** (как в C++/Java).  
- Но можно **перегружать операторы** через трейты (`std::ops`).  

**Пример перегрузки оператора:**
```rust,editable
use std::ops::Add;

struct Point { x: i32, y: i32 }

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}
fn main(){
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;  // Point { x: 4, y: 6 }
}
```

### 4. Полиморфизм через приведение (Coercion Polymorphism)  
**Ограниченно**  
- Rust **не допускает неявных преобразований** (в отличие от C++/Java). 
- Но есть **явные приведения** (`as`) и трейты типа `From`/`Into`. 

```rust,editable
let x: i32 = 5;
let y: f64 = x as f64;  // Явное приведение

// Через трейт From
let z: f64 = f64::from(x);
```

### 5. Полиморфизм времени выполнения (Dynamic Dispatch / Runtime Polymorphism)  
**Есть** 
- Rust поддерживает динамическую диспетчеризацию через **трейт-объекты (`dyn Trait`)**. 
- Но требует явного указания (`Box<dyn Trait>`, `&dyn Trait`). 
- Динамическая диспетчеризация - есть накладные расходы (обычно 1 индиректный вызов).

**Что происходит**

В `Box<dyn Animal>` хранится: 
- Указатель на данные (Dog или Cat).
- Указатель на `vtable` (таблицу методов для конкретного типа). 

Вызов `animal.make_sound()` - идёт через виртуальную таблицу `vtable`.

```rust,editable
trait Animal {
    fn make_sound(&self);
}

struct Dog;
impl Animal for Dog { fn make_sound(&self) { println!("Woof!"); } }

struct Cat;
impl Animal for Cat { fn make_sound(&self) { println!("Meow!"); } }

fn main(){
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
    ];

    for animal in animals {
        animal.make_sound();  // Woof! Meow!
    }
}
```


### 6. Полиморфизм через интерфейсы (Interface Polymorphism)
**Есть (через трейты)**  
- В Rust **нет интерфейсов**, но их роль выполняют **трейты**.  
- Можно реализовывать для разных структур.  
- Этот код создаёт monomorphization, компилятор создаёт **отдельную версию функции `render` для каждого типа**, с которым мы используем функцию, что раздувает бинарный файл.  

```rust,editable
trait Drawable {
    fn draw(&self);
}

struct Circle;
impl Drawable for Circle { fn draw(&self) { println!("Drawing Circle"); } }

struct Square;
impl Drawable for Square { fn draw(&self) { println!("Drawing Square"); } }

fn render(item: &impl Drawable) {
    item.draw();
}

let circle = Circle;
let square = Square;
render(&circle);  // Drawing Circle
render(&square);  // Drawing Square
```


Вывод: Какие виды полиморфизма есть в Rust? 
| **Вид полиморфизма**          | **Поддержка в Rust** | **Как реализуется?**          |
|-------------------------------|----------------------|-------------------------------|
| **Параметрический (Generics)** | ✅ Да                | `fn foo<T>(x: T)`             |
| **Subtyping (наследование)**   | ❌ Нет               | Нет классов, только трейты    |
| **Ad-hoc (перегрузка)**        | ⚠️ Частично         | Только операторы (`std::ops`) |
| **Приведение типов**           | ⚠️ Ограниченно      | `as`, `From`/`Into`           |
| **Динамический (`dyn`)**       | ✅ Да                | `Box<dyn Trait>`, `&dyn Trait`|
| **Интерфейсный (трейты)**      | ✅ Да                | `trait Drawable { ... }`      |

Что уникально в Rust?
- **Нет наследования** → вместо него **композиция + трейты**.  
- **Нет неявных приведений** → только явные (`as`, `From`/`Into`).  
- **Динамический полиморфизм (`dyn Trait`)** требует явного указания.  
- **Перегрузка функций отсутствует**, но есть перегрузка операторов.  

Rust делает упор на **статический полиморфизм (Generics + трейты)**, но при необходимости позволяет и **динамическую диспетчеризацию (`dyn`)**.

 