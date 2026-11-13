
# Gangs of Four (GoF) Design Patterns
* [Порождающие паттерны](#%D0%BF%D0%BE%D1%80%D0%BE%D0%B6%D0%B4%D0%B0%D1%8E%D1%89%D0%B8%D0%B5-%D0%BF%D0%B0%D1%82%D1%82%D0%B5%D1%80%D0%BD%D1%8B)
* [Структурирующие паттерны](#%D1%81%D1%82%D1%80%D1%83%D0%BA%D1%82%D1%83%D1%80%D0%B8%D1%80%D1%83%D1%8E%D1%89%D0%B8%D0%B5-%D0%BF%D0%B0%D1%82%D1%82%D0%B5%D1%80%D0%BD%D1%8B)
* [Паттерны поведения](#%D0%BF%D0%B0%D1%82%D1%82%D0%B5%D1%80%D0%BD%D1%8B-%D0%BF%D0%BE%D0%B2%D0%B5%D0%B4%D0%B5%D0%BD%D0%B8%D1%8F)

---

* [Rust Design Patterns](https://chercher.tech/rust/observer-design-pattern-rust)
* [Паттерны проектирования](http://design-pattern.ru/patterns/)
* [Каталог шаблонов архитектуры корпоративных приложений](https://martinfowler.com/eaaCatalog/index.html)
* [Паттерны ООП в метафорах](https://habr.com/ru/articles/136766/)

> Методы решения распространенных проблем при кодировании.

Шаблоны проектирования `GoF` делятся на три категории:
- **Порождающие** паттерны связанны с созданием объекта. `Singleton, Builder, Factory, Fabric Method, Prototype, Fold`
- **Структурные** паттерны связаны со структурой классов, такой как наследование и композиция. `Adapter, Bridge, Composite, Decorator, Facade, Proxy, Flyweight`
- **Поведенческие** паттерны обеспечивают решение для лучшего взаимодействия между объектами, обеспечения потери связнности и гибкости для легкого расширения в будущем. `Chain of Responsibility, Command, Interpreter, Iterator, Mediator, Memento, Observer, State, Strategy, Template Method, Visitor`

## <ins>Порождающие паттерны</ins>
Паттерны которые создают новые объекты, или позволяют получить доступ к уже существующим. 
То есть те шаблоны, по которым можно создать новый автомобиль и как это лучше сделать.

<br>
<details>
<summary><b>Builder</b></summary>

`Pattern Builder` — это шаблон проектирования, который позволяет шаг за шагом создавать сложные объекты. Он позволяет создавать разные типы и представления объекта, используя один и тот же код построения, отделяя построение сложного объекта от его представления.

Создайте объект с помощью вызовов помощника-строителя.
`Pattern Builder` — это порождающий паттерн проектирования, который позволяет создавать сложные объекты пошагово. 
`Pattern Builder` даёт возможность использовать один и тот же код строительства для получения разных представлений объектов.
`Pattern Builder` особенно подходит, когда при построении T есть побочные эффекты, такие как создание потока или запуск процесса.
Полезно, когда в противном случае вам потребовалось бы много конструкторов или когда конструкция имеет побочные эффекты.

Преимущества
Отделяет методы построения от других методов.
Предотвращает распространение конструкторов.
Может использоваться для однострочной инициализации, а также для более сложной конструкции.

Этот шаблон чаще встречается в Rust (и для более простых объектов), чем во многих других языках, поскольку в Rust отсутствует перегрузка. 
Поскольку у вас может быть только один метод с заданным именем, иметь несколько конструкторов в Rust менее удобно, чем в C++, Java или других.

Этот шаблон часто используется там, где объект-строитель полезен сам по себе, а не просто является строителем. 
Например, см [std::process::Command](https://doc.rust-lang.org/std/process/struct.Command.html)
```
    Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process")

```

Пример:

```rust
#[derive(Debug, PartialEq)]
pub struct TargetObject {
    // Много сложных полей.
    a: i32,
    b: i32,
    c: Option<i32>,
    d: Option<(bool, usize)>,
    sum: usize,
}
impl TargetObject {
    // Этот метод поможет пользователям найти builder
    pub fn builder() -> Builder {
        Builder::default()
    }
}
pub struct Builder {
    // Вероятно, много необязательных полей.
    a: i32,
    b: i32,
    c: Option<i32>,
    d: Option<(bool, usize)>,
}
impl Default for Builder {
    fn default() -> Self {
        Self {
            a: 0,
            b: 0,
            c: None,
            d: None,
        }
    }
}
impl Builder {
    pub fn new(a: i32, b: i32) -> Builder {
        // Установите минимально необходимые поля TargetObject.
        Builder {
            a,
            b,
            c: None,
            d: None,
        }
    }
    pub fn c(&mut self, c: i32) -> &mut Builder {
        // Задайте имя и верните построитель по значению.
        self.c = Some(c);
        self
    }
    pub fn d(&mut self, left: bool, right: usize) -> &mut Builder {
        // Задайте имя и верните построитель по значению.
        self.d = Some((left, right));
        self
    }
    // Создание TargetObject применив все настройки Builder
    pub fn build(&self) -> Result<TargetObject, String> {
        let mut sum: usize = (self.a + self.b) as usize;
        if let Some(c) = self.c {
            sum += c as usize;
        }
        if let Some(d) = self.d {
            sum = sum + d.1;
        }
        Ok(TargetObject {
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            sum,
        })
    }
}
fn main() {
    // без Builder
    let target = {
        let (a, b, c, d) = (1, 2, 2, 1);
        TargetObject {
            a: a,
            b: b,
            c: Some(c),
            d: Some((true, d as usize)),
            sum: (a + b + c + d) as usize,
        }
    };

    // с Builder
    let target_2: TargetObject = Builder::new(1, 2).c(2).d(true, 1).build().unwrap();
    assert_eq!(target, target_2);
}
```

Еще пример:

```rust
// Step 1: Define the product
pub struct Meal {
    starter: Option<String>,
    main_course: Option<String>,
    dessert: Option<String>,
}
impl Meal {
    pub fn new() -> Self {
        Meal {
            starter: None,
            main_course: None,
            dessert: None,
        }
    }
}
// Step 2: Builder interface
pub trait MealBuilder {
    fn add_starter(&mut self, starter: String);
    fn add_main_course(&mut self, main_course: String);
    fn add_dessert(&mut self, dessert: String);
    fn build(&self) -> Meal;
}
// Step 3: Concrete builder
pub struct FullMealBuilder {
    meal: Meal,
}
impl FullMealBuilder {
    pub fn new() -> Self {
        FullMealBuilder { meal: Meal::new() }
    }
}
impl MealBuilder for FullMealBuilder {
    fn add_starter(&mut self, starter: String) {
        self.meal.starter = Some(starter);
    }
    fn add_main_course(&mut self, main_course: String) {
        self.meal.main_course = Some(main_course);
    }
    fn add_dessert(&mut self, dessert: String) {
        self.meal.dessert = Some(dessert);
    }
    fn build(&self) -> Meal {
        Meal {
            starter: self.meal.starter.clone(),
            main_course: self.meal.main_course.clone(),
            dessert: self.meal.dessert.clone(),
        }
    }
}
// Step 4: Director
pub fn construct_meal(builder: &mut dyn MealBuilder) -> Meal {
    builder.add_starter("Soup".to_string());
    builder.add_main_course("Steak".to_string());
    builder.add_dessert("Ice cream".to_string());
    builder.build()
}
// Step 5: Client code
fn main() {
    let mut builder = FullMealBuilder::new();
    let meal = construct_meal(&mut builder);
    println!("Starter: {:?}", meal.starter);
    println!("Main Course: {:?}", meal.main_course);
    println!("Dessert: {:?}", meal.dessert);
}
```

 

* [Builder rust-unofficial.github.io](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html)
* [Builder web.archive.org](https://web.archive.org/web/20210104103100/https://doc.rust-lang.org/1.12.0/style/ownership/builders.html)
* [Builder www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2023/10/builder-design-pattern-in-rust.html)
* [Builder www.lurklurk.org](https://www.lurklurk.org/effective-rust/builders.html)

---

</details>


<br>
<details>
<summary><b>Fold</b></summary>
 
Создание новой коллекции посредством изменения элементов применив соответсвующий реализованный алгоритм.

`Pattern Fold` аналогичен `Pattern Visitor`, но создает новую версию посещенной структуры данных.
Они разделяют концепцию обхода структуры данных, выполняя операцию на каждом узле. 
Однако посетитель не создает новую структуру данных и не использует старую.

**Мотивация**

Обычно требуется отобразить структуру данных, выполнив некоторую операцию над каждым узлом структуры. 
Для простых операций с простыми структурами данных это можно сделать с помощью `Iterator::map`. 
Для более сложных операций, возможно, когда более ранние узлы могут повлиять на операцию на более поздних 
узлах или где итерация по структуре данных нетривиальна, использование шаблона свертывания является более подходящим.

Как и `Pattern Visitor`, `Pattern Fold` позволяет нам отделить обход структуры данных от операций, выполняемых с каждым узлом.

**Обсуждение**

Такое отображение структур данных распространено в функциональных языках. 
В объектно-ориентированных языках более распространено изменение структуры данных на месте. 
«Функциональный» подход распространен в Rust, в основном из-за предпочтения неизменяемости. 
Использование новых структур данных вместо изменения старых в большинстве случаев упрощает анализ кода.

Компромисс между эффективностью и возможностью повторного использования можно изменить, изменив способ приема узлов методами fold_*.

В приведенном выше примере мы работаем с Boxуказателями. Поскольку они владеют исключительно своими данными, 
исходную копию структуры данных нельзя использовать повторно. 
С другой стороны, если узел не изменяется, его повторное использование очень эффективно.

Если бы нам пришлось работать с заимствованными ссылками, исходную структуру данных можно было бы использовать повторно; 
однако узел необходимо клонировать, даже если он не изменился, что может быть дорогостоящим.

Использование указателя с подсчетом ссылок дает лучшее из обоих миров: мы можем повторно использовать исходную структуру данных, 
и нам не нужно клонировать неизмененные узлы. 
Однако они менее эргономичны в использовании и означают, что структуры данных не могут быть изменены.

У итераторов есть fold метод, однако он сворачивает структуру данных в значение, а не в новую структуру данных. 

Пример:

```rust
// Данные AST.
pub mod ast {
    #[derive(Debug)]
    pub enum Stmt {
        Expr(Box<Expr>),
        Let(Box<Name>, Box<Expr>),
    }
    #[derive(Debug)]
    pub struct Name {
        value: String,
    }
    impl Name {
        pub fn new(value: String) -> Self {
            Self { value }
        }
    }
    #[derive(Debug)]
    pub enum Expr {
        IntLit(i64),
        Add(Box<Expr>, Box<Expr>),
        Sub(Box<Expr>, Box<Expr>),
    }
}
// The abstract folder
pub mod fold {
    use crate::ast::*;
    pub trait Folder {
        // узелы просто возвращает сами узлы.
        fn fold_name(&mut self, n: Box<Name>) -> Box<Name> {
            n
        }
        // Создайте новый внутренний узел, сложив его дочерние элементы.
        fn fold_stmt(&mut self, s: Box<Stmt>) -> Box<Stmt> {
            match *s {
                Stmt::Expr(e) => Box::new(Stmt::Expr(self.fold_expr(e))),
                Stmt::Let(n, e) => Box::new(Stmt::Let(self.fold_name(n), self.fold_expr(e))),
            }
        }
        fn fold_expr(&mut self, e: Box<Expr>) -> Box<Expr> {
            e
        }
    }
}
use ast::*;
use fold::*;
// Пример конкретной реализации — переименовывает каждое имя в 'foo'.
struct Renamer;
impl Folder for Renamer {
    fn fold_name(&mut self, n: Box<Name>) -> Box<Name> {
        Box::new(Name::new("foo".to_owned()))
    } // Используйте методы по умолчанию для других узлов.
}
struct IntLitChange;
impl Folder for IntLitChange {
    fn fold_expr(&mut self, n: Box<Expr>) -> Box<Expr> {
        match *n {
            Expr::IntLit(v) if v > 10 => {
                let v = 0;
                Box::new(Expr::IntLit(v))
            }
            _ => n,
        }
    } // Используйте методы по умолчанию для других узлов.
}
fn main() {
    let mut rename_logic = Renamer;
    let exp: Box<Stmt> = Box::new(Stmt::Let(
        Box::new(Name::new(String::from("ABCD"))),
        Box::new(Expr::IntLit(18)),
    ));
    let res: Box<Stmt> = rename_logic.fold_stmt(exp);
    println!("{:?}", *res); // Let(Name { value: "foo" }, IntLit(18))

    let mut intlit_logic = IntLitChange;
    let exp: Box<Stmt> = Box::new(Stmt::Let(
        Box::new(Name::new(String::from("ABCD"))),
        Box::new(Expr::IntLit(18)),
    ));
    let res: Box<Stmt> = intlit_logic.fold_stmt(exp);
    println!("{:?}", *res); // Let(Name { value: "ABCD" }, IntLit(0))
}
```




[Fold rust-unofficial.github.io](https://rust-unofficial.github.io/patterns/patterns/creational/fold.html)


---

</details>

 
<br>
<details>
<summary><b>Abstract factory</b></summary>

Его основное назначение - предоставить интерфейс для создания семейства взаимосвязанных объектов, не специфицируя их классы.
Используется в тех случаях, когда необходимо изменять поведение системы, варьируя создаваемыми объектами, при этом сохраняя интерфейсы. 
Он позволяет создавать группы взаимосвязанных объектов, реализующих общее поведение. 
Например, в зависимости от конкретных условий

```rust
// Step 1: Abstract products
trait Button {
    fn render(&self);
}
trait Checkbox {
    fn check(&self);
}
// Step 2: Concrete products
struct WindowsButton;
impl Button for WindowsButton {
    fn render(&self) {
        println!("Rendering a Windows button");
    }
}
struct MacButton;
impl Button for MacButton {
    fn render(&self) {
        println!("Rendering a Mac button");
    }
}
struct WindowsCheckbox;
impl Checkbox for WindowsCheckbox {
    fn check(&self) {
        println!("Checking a Windows checkbox");
    }
}
struct MacCheckbox;
impl Checkbox for MacCheckbox {
    fn check(&self) {
        println!("Checking a Mac checkbox");
    }
}
// Step 3 & 4: Abstract factory and its concrete implementations
trait GUIFactory {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_checkbox(&self) -> Box<dyn Checkbox>;
}
struct WindowsFactory;
impl GUIFactory for WindowsFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton)
    }
    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(WindowsCheckbox)
    }
}
struct MacFactory;
impl GUIFactory for MacFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacButton)
    }
    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(MacCheckbox)
    }
}
// Step 5: Client code
fn main() {
    let factory: Box<dyn GUIFactory> = if cfg!(target_os = "windows") {
        Box::new(WindowsFactory)
    } else {
        Box::new(MacFactory)
    };
    let button = factory.create_button();
    button.render();
    let checkbox = factory.create_checkbox();
    checkbox.check();
}
```


[Abstract factory www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2023/10/abstract-factory-design-pattern-in-rust.html)

---

</details>

<br>
<details>
<summary><b>Factory method</b></summary>

Вместо прямого вызова конструктора объекта для создания объекта используется фабричный метод, что обеспечивает большую гибкость и разделение обязанностей.
В оличии от `Abstract Factory` которая использует фабрику для создания всего набора обьектов, то `Factory Method` использует только один метод для создания единственного обьекта.

`Factory Method`
фабрике заранее неизвестно, объекты каких подклассов ему нужно создавать.
фабрика проектируется так, чтобы объекты, которые она создаёт, определялись ее подклассами.
т.е. делегирует свои обязанности одному из нескольких вспомогательных подклассов.

`Factory Method` избавляют проектировщика от необходимости встраивать в код классы выаолняющие создание кокнретных реализаций.
Также используется когда клиент не имеет прав или доступа или к деталям реализации классов.(инкапсуляция создания конкретных типов обьектов)
Фабричный метод используется, когда продуктам не нужно знать, как они созданы.

**Применимость**
- Когда заранее неизвестны типы и зависимости объектов, с которыми должен работать ваш код.
    `Factory Method` отделяет код производства продуктов от остального кода, который эти продукты использует.
- Когда вы хотите экономить системные ресурсы, повторно используя уже созданные объекты, вместо порождения новых.
    Т.е. `Factory Method` может еще управлять логикой создания обьектов, взаимодействуя с хранилищем `Redis,Json,...` этих обьектов.

**Отношения с другими паттернами**

Многие архитектуры начинаются с применения `Factory Method` (более простого и расширяемого через подклассы) 
и эволюционируют в сторону `Abstract Factory`, `Pattern Prototype` или `Pattern Builder` (более гибких, но и более сложных).

Классы `Abstract Factory` чаще всего реализуются с помощью `Factory Method`, хотя они могут быть построены и на основе `Pattern Prototype`.

Пример:

```rust
use gui::{Button, DialogFactory};
pub mod gui {
    pub trait Button {
        fn render(&self);
        fn on_click(&self);
    }
    pub trait DialogFactory {
        type B: Button;
        /// The factory method. It must be overridden with a concrete implementation.
        fn create_button_factory_method(&self) -> Self::B;

        fn render(&self) {
            let button = self.create_button_factory_method();
            button.render();
            self.refresh();
        }
        fn refresh(&self) {
            println!("Dialog - Refresh");
        }
    }
}
use html_gui::*;
pub mod html_gui {
    use super::*;
    pub struct HtmlButton;
    impl Button for HtmlButton {
        fn render(&self) {
            println!("<button>Test Button</button>");
            self.on_click();
        }
        fn on_click(&self) {
            println!("Click! Button says - 'Hello World!'");
        }
    }
    pub struct HtmlConcreteFactory;
    impl DialogFactory for HtmlConcreteFactory {
        type B = HtmlButton;
        fn create_button_factory_method(&self) -> Self::B {
            HtmlButton
        }
    }
}
use windows_gui::*;
pub mod windows_gui {
    use super::*;
    pub struct WindowsButton;
    impl Button for WindowsButton {
        fn render(&self) {
            println!("Drawing a Windows button");
            self.on_click();
        }
        fn on_click(&self) {
            println!("Click! Hello, Windows!");
        }
    }
    pub struct WindowsConcreteFactory;
    impl DialogFactory for WindowsConcreteFactory {
        type B = WindowsButton;
        fn create_button_factory_method(&self) -> Self::B {
            WindowsButton
        }
    }
}
use linux_gui::*;
pub mod linux_gui {
    use super::*;
    pub struct LinuxButton;
    impl Button for LinuxButton {
        fn render(&self) {
            println!("Drawing a Linux button");
            self.on_click();
        }
        fn on_click(&self) {
            println!("Click! Hello, Linux!");
        }
    }
    pub struct LinuxConcreteFactory;
    impl DialogFactory for LinuxConcreteFactory {
        type B = LinuxButton;
        fn create_button_factory_method(&self) -> Self::B {
            LinuxButton
        }
    }
}
struct Client<B> {
    btn: B,
}
impl<B: Button> Client<B> {
    fn new(btn: B) -> Self {
        Self { btn }
    }
    fn use_btn(&self) {
        self.btn.render();
    }
}
enum Variants {
    Windows(Box<dyn DialogFactory<B = WindowsButton>>),
    Linux(Box<dyn DialogFactory<B = LinuxButton>>),
    Html(Box<dyn DialogFactory<B = HtmlButton>>),
}
fn use_client(dialog: Variants) {
    match dialog {
        Variants::Windows(dialog) => {
            let client = Client::new(dialog.create_button_factory_method());
            client.use_btn();
        }
        Variants::Linux(dialog) => {
            let client = Client::new(dialog.create_button_factory_method());
            client.use_btn();
        }
        Variants::Html(dialog) => {
            let client = Client::new(dialog.create_button_factory_method());
            client.use_btn();
        }
    }
}
fn main() {
    let mut line = String::new();
    println!("Enter type OS: (Windows, Linux, HTML (default: HTML): ");
    let _ = std::io::stdin().read_line(&mut line).unwrap();
    let dialog: Variants = match line.trim().to_lowercase().as_str() {
        "windows" => Variants::Windows(Box::new(WindowsConcreteFactory)),
        "linux" => Variants::Linux(Box::new(LinuxConcreteFactory)),
        "html" => Variants::Html(Box::new(HtmlConcreteFactory)),
        _ => Variants::Html(Box::new(HtmlConcreteFactory)),
    };
    use_client(dialog);
    /*
        if cfg!(windows) {
             ...
        }else if cfg!(target_os = "linux") {
             ...
        } else {
             ...
        }
    */
}
```

Еще пример:

```rust
#[derive(Clone, Copy)]
enum PizzaType {
    CHEESE,
    PEPPERONI,
}
// Классы создатели --------------------------------------------------------------------------------
trait PizzaFactory {
    fn order_pizza(&mut self, types: PizzaType) -> Box<dyn Pizza> {
        let mut pizza: Box<dyn Pizza> = self.create_pizza(types);
        pizza.prepare();
        pizza.bake();
        pizza.cut();
        pizza.boxes();
        pizza
    }
    fn create_pizza(&self, types: PizzaType) -> Box<dyn Pizza>;
}
struct ChicagoPizzaConcreteFactory;
impl PizzaFactory for ChicagoPizzaConcreteFactory {
    fn create_pizza(&self, types: PizzaType) -> Box<dyn Pizza> {
        match types {
            PizzaType::CHEESE => Box::new(ChicagoStyleSheesePizza::default()),
            PizzaType::PEPPERONI => Box::new(ChicagoStylePepperoniPizza::default()),
        }
    }
}
struct NYPizzaConcreteFactory;
impl PizzaFactory for NYPizzaConcreteFactory {
    fn create_pizza(&self, types: PizzaType) -> Box<dyn Pizza> {
        match types {
            PizzaType::CHEESE => Box::new(NYStyleSheesePizza::default()),
            PizzaType::PEPPERONI => Box::new(NYStylePepperoniPizza::default()),
        }
    }
}

// Классы продукты ---------------------------------------------------------------------------------
trait Pizza {
    fn prepare(&mut self) {
        println!("prepare {}", self.get_name());
    }
    fn bake(&mut self) {
        println!("bake {}", self.get_name());
    }
    fn cut(&mut self) {
        println!("cut {}", self.get_name());
    }
    fn boxes(&mut self) {
        println!("box {}", self.get_name());
    }
    fn get_name(&self) -> String;
}

struct ChicagoStyleSheesePizza {
    name: String,
}
impl Default for ChicagoStyleSheesePizza {
    fn default() -> Self {
        Self {
            name: String::from("ChicagoStyleSheesePizza"),
        }
    }
}
impl Pizza for ChicagoStyleSheesePizza {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

struct ChicagoStylePepperoniPizza {
    name: String,
}
impl Default for ChicagoStylePepperoniPizza {
    fn default() -> Self {
        Self {
            name: String::from("ChicagoStylePepperoniPizza"),
        }
    }
}
impl Pizza for ChicagoStylePepperoniPizza {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

struct NYStyleSheesePizza {
    name: String,
}
impl Default for NYStyleSheesePizza {
    fn default() -> Self {
        Self {
            name: String::from("NYStyleSheesePizza"),
        }
    }
}
impl Pizza for NYStyleSheesePizza {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

struct NYStylePepperoniPizza {
    name: String,
}
impl Default for NYStylePepperoniPizza {
    fn default() -> Self {
        Self {
            name: String::from("NYStylePepperoniPizza"),
        }
    }
}
impl Pizza for NYStylePepperoniPizza {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
fn new_pizza(creator: &mut Box<dyn PizzaFactory>, types: PizzaType) -> Box<dyn Pizza> {
    creator.order_pizza(types)
}
fn main() {
    let mut pizza = new_pizza(
        &mut (Box::new(NYPizzaConcreteFactory) as Box<PizzaFactory>),
        PizzaType::CHEESE,
    );
    pizza = new_pizza(
        &mut (Box::new(NYPizzaConcreteFactory) as Box<PizzaFactory>),
        PizzaType::PEPPERONI,
    );

    pizza = new_pizza(
        &mut (Box::new(ChicagoPizzaConcreteFactory) as Box<PizzaFactory>),
        PizzaType::CHEESE,
    );
    pizza = new_pizza(
        &mut (Box::new(ChicagoPizzaConcreteFactory) as Box<PizzaFactory>),
        PizzaType::PEPPERONI,
    );
}
```

Еще пример:

```rust
trait Product {
    fn use_product(&self);
}
struct IDCard {
    owner: String,
}
impl IDCard {
    fn new(owner: String) -> IDCard {
        println!("{}のカードを作ります。", owner);
        IDCard { owner: owner }
    }
    fn get_owner(&self) -> String {
        self.owner.clone()
    }
}

impl Product for IDCard {
    fn use_product(&self) {
        println!("{}のカードを使います", self.owner);
    }
}
trait Factory {
    type Object;
    fn create(&mut self, owner: String) -> Box<Self::Object> {
        let product = self.create_product(owner);
        self.register_product(&product);
        product
    }
    fn create_product(&self, owner: String) -> Box<Self::Object>;
    fn register_product(&mut self, product: &Box<Self::Object>);
}

struct IDCardFactory {
    owners: Vec<String>,
}
impl IDCardFactory {
    fn new() -> IDCardFactory {
        IDCardFactory { owners: Vec::new() }
    }
}
impl Factory for IDCardFactory {
    type Object = IDCard;

    fn create_product(&self, owner: String) -> Box<Self::Object> {
        Box::new(IDCard::new(owner))
    }
    fn register_product(&mut self, product: &Box<Self::Object>) {
        self.owners.push(product.get_owner());
    }
}
fn main() {
    let mut factory = IDCardFactory::new();
    let card1 = factory.create("結城浩".to_string());
    let card2 = factory.create("とむら".to_string());
    let card3 = factory.create("佐藤花子".to_string());
    card1.use_product();
    card2.use_product();
    card3.use_product();
}
```

Еще пример:

```rust
// Step 1 & 3: Product interface and its concrete implementation
trait Button {
    fn render(&self);
}
struct WindowsButton;
impl Button for WindowsButton {
    fn render(&self) {
        println!("Rendering a Windows button");
    }
}
struct MacButton;
impl Button for MacButton {
    fn render(&self) {
        println!("Rendering a Mac button");
    }
}
// Step 2: Creator interface and its concrete implementations
trait ButtonFactory {
    fn create_button(&self) -> Box<dyn Button>;
}
struct WindowsButtonFactory;
impl ButtonFactory for WindowsButtonFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton)
    }
}
struct MacButtonFactory;
impl ButtonFactory for MacButtonFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacButton)
    }
}
// Step 5: Using the factory method
fn main() {
    let factory: Box<dyn ButtonFactory> = if cfg!(target_os = "windows") {
        Box::new(WindowsButtonFactory)
    } else {
        Box::new(MacButtonFactory)
    };
    let button = factory.create_button();
    button.render();
}
```


* [Factory method refactoring.guru](https://refactoring.guru/ru/design-patterns/factory-method)
* [Factory method sourcemaking.com](https://sourcemaking.com/design_patterns/factory_method)
* [Factory method chercher.tech](https://chercher.tech/rust/factory-design-pattern-rust)
* [Factory method www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2023/10/factory-method-design-pattern-in-rust.html)

---

</details>


<br>
<details>
<summary><b>Prototype</b></summary>
  
`Pattern Prototype` инкапсуляция создания клона. Создание объектов на основе шаблона существующего объекта посредством клонирования, не делая систему зависимой от его классов.

**Проблема**

У вас есть объект, который нужно скопировать. 
Как это сделать? Нужно создать пустой объект такого же класса, а затем поочерёдно скопировать значения всех полей из старого объекта в новый.
Но у нас может не быть доступа к приватных полям. И копирующий код становиться жество зависим от конкретного копируемого класса.

`Pattern Prototype` хочет что бы обьекты создавали свои копии самостоятельно и использовали эдиный интерфейс `method clone()`
Объект, который копируют, называется прототипом (откуда и название паттерна)
Также есть место для инкапсуляции логики создания клона

Если создание объекта требует много времени и средств, и у вас уже есть наиболее похожий экземпляр объекта, 
тогда вы клонируете уже приготовленный с его текущим состоянием.

Если вам нужна глубокая копия, вы можете использовать сериализацию в качестве хитрости, чтобы выполнить глубокую копию. 

Экземпляры класса могут иметь лишь несколько различных комбинаций состояний, и создание новых экземпляров обходится дороже, чем копирование существующего.

Пример:
```rust
#![allow(unused_variables)]
#![allow(dead_code)]

use cell::{Prototype, SomeCell};
use std::fmt::Debug;
mod cell {
    use std::cmp::PartialEq;
    use std::fmt::Debug;
    use std::time::{SystemTime, UNIX_EPOCH};

    pub trait Prototype: Clone {}
    pub struct SomeCell {
        pub cytoplasm: Vec<u8>,
        pub mitochondria: Vec<u8>,
        pub chloroplasts: Vec<u8>,
        secret: u128,
    }
    impl Clone for SomeCell {
        fn clone(&self) -> Self {
            let _clone @ SomeCell {
                cytoplasm,
                mitochondria,
                chloroplasts,
                ..
            } = self;
            let mut cytoplasm_cap: Vec<u8> = Vec::with_capacity(cytoplasm.capacity());
            let mut mitochondria_cap: Vec<u8> = Vec::with_capacity(mitochondria.capacity());
            let mut chloroplasts_cap: Vec<u8> = Vec::with_capacity(chloroplasts.capacity());
            cytoplasm_cap.clone_from(&cytoplasm);
            mitochondria_cap.clone_from(&mitochondria);
            chloroplasts_cap.clone_from(&chloroplasts);
            Self {
                cytoplasm: cytoplasm_cap,
                mitochondria: mitochondria_cap,
                chloroplasts: chloroplasts_cap,
                secret: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
            }
        }
    }
    impl Prototype for SomeCell {}
    impl Debug for SomeCell {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                f,
                "SomeCell {{ cytoplasm: {:?}, mitochondria: {:?}, chloroplasts: {:?} }}",
                self.cytoplasm, self.mitochondria, self.chloroplasts
            )
        }
    }
    impl<'a> PartialEq<SomeCell> for &'a SomeCell {
        fn eq(&self, other: &SomeCell) -> bool {
            let _clone @ SomeCell {
                cytoplasm,
                mitochondria,
                chloroplasts,
                ..
            } = other;
            self.cytoplasm == *cytoplasm
                && self.mitochondria == *mitochondria
                && self.chloroplasts == *chloroplasts
        }
    }
    impl<'a> PartialEq<SomeCell> for SomeCell {
        fn eq(&self, other: &SomeCell) -> bool {
            let _clone @ SomeCell {
                cytoplasm,
                mitochondria,
                chloroplasts,
                ..
            } = other;
            self.cytoplasm == *cytoplasm
                && self.mitochondria == *mitochondria
                && self.chloroplasts == *chloroplasts
        }
    }
    impl Default for SomeCell {
        fn default() -> Self {
            Self {
                cytoplasm: Vec::default(),
                mitochondria: Vec::default(),
                chloroplasts: Vec::default(),
                secret: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
            }
        }
    }
    impl SomeCell {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
fn client<T: Prototype + Debug>(cell: &T)
where
    for<'a> &'a T: PartialEq<T>,
{
    let clone_cell_2 = cell.clone();
    assert_eq!(cell, clone_cell_2)
}
fn main() {
    let mut clone_cell_1 = SomeCell::new();
    clone_cell_1.cytoplasm = vec![1, 2, 3]; // продолжить настраивать с текущего состояния
    client(&clone_cell_1);
}
```



* [Prototype refactoring.guru](https://refactoring.guru/ru/design-patterns/prototype)
* [Prototype chercher.tech](https://chercher.tech/rust/prototype-design-pattern-rust)

---

</details>

<br>
<details>
<summary><b>Singelton</b></summary>
 
`Pattern Singleton` (Одиночка) применяется в том случае, когда какой-либо класс может иметь только один экземпляр (или не иметь ни одного) и легко доступен из глобальной видимости.
    
`Pattern Singleton` нарушает принцип единственной ответственности класса (SRP), так как он доступен глобально для различных частей программы следовательно есть вероятность изменения в одной из частей.

`Pattern Singleton` решает сразу две проблемы:

1. Гарантирует наличие единственного экземпляра класса. Чаще всего это полезно для доступа к какому-то общему ресурсу, например, базе данных.

Представьте, что вы создали объект, а через некоторое время пробуете создать ещё один. В этом случае хотелось бы получить старый объект, вместо создания нового.

Такое поведение невозможно реализовать с помощью обычного конструктора, так как конструктор класса всегда возвращает новый объект.

2. Предоставляет глобальную точку доступа. Это не просто глобальная переменная, через которую можно достучаться к определённому объекту.  


#### Недостатки Singelton

Паттерн Синглтон (Singleton) часто считается антипаттерном (anti-pattern) не потому, что он всегда плох, а из-за его **частого злоупотребления** и ряда серьезных недостатков, которые он привносит в код. По сути, он маскирует **глобальное состояние** и **скрытые зависимости**, что противоречит принципам хорошего объектно-ориентированного дизайна (ООП).

Вот ключевые причины, по которым Синглтон критикуют:

**1. Глобальное состояние и скрытые зависимости**

Синглтон обеспечивает **глобальную точку доступа** к своему единственному экземпляру.

* Глобальное состояние: Экземпляр Синглтона по сути ведет себя как **глобальная переменная**. Любая часть кода может изменить его внутреннее состояние, и это изменение отразится на всей системе. Это делает поведение программы **непредсказуемым** и **затрудняет отладку**, поскольку сложно определить, какая именно часть кода и когда изменила состояние.
* Скрытые зависимости: Классы, использующие Синглтон, не объявляют эту зависимость явно (например, через конструктор или параметры метода), а просто вызывают статический метод `getInstance()`. Это создает **скрытые зависимости**, которые не видны в публичном контракте класса, что усложняет понимание структуры кода и его модификацию.
 
**2. Проблемы с тестированием**

Синглтон критически усложняет **модульное (юнит) тестирование**.

* Нарушение изоляции: Юнит-тесты должны быть независимыми и запускаться в изоляции. Поскольку Синглтон поддерживает глобальное состояние, тесты одного класса могут влиять на состояние Синглтона, которое затем будет использовано в тестах другого класса. Это означает, что **порядок выполнения тестов имеет значение**, а результаты могут быть нестабильными.
* Сложность подмены (Mocking): Большинство тестовых фреймворков для создания тестовых заглушек (mock-объектов) полагаются на механизмы, требующие передачи зависимостей (например, через интерфейсы или конструкторы). Синглтон с его приватным конструктором и статическим доступом делает **подмену или имитацию** его поведения крайне сложной, если не невозможной, без сложных хаков.

**3. Нарушение принципов SOLID**

Синглтон часто нарушает основные принципы ООП, в частности:

* **Принцип единственной ответственности (SRP):** Класс Синглтона берет на себя **две** обязанности:
    1.  Непосредственная **бизнес-логика** класса.
    2.  Логика **управления своим жизненным циклом** (гарантия единственного экземпляра и предоставление глобального доступа).
    Это нарушает SRP, который гласит, что у класса должна быть только **одна причина для изменения**.
* **Принцип открытости/закрытости (OCP):** Класс Синглтона, как правило, тесно связан со своей реализацией, что **снижает гибкость**. Расширение функциональности или изменение поведения может потребовать модификации существующего кода во многих местах, где он используется.

**4. Проблемы многопоточности**

В многопоточной среде реализация Синглтона требует дополнительных усилий для обеспечения **потокобезопасности**.

* Если несколько потоков одновременно попытаются создать экземпляр Синглтона (в случае ленивой инициализации), может возникнуть **состояние гонки**, и в результате будет создано несколько экземпляров, что нарушит основную цель паттерна. Для предотвращения этого необходимо использовать механизмы синхронизации (например, блокировки), которые, в свою очередь, могут **снизить производительность**.

Пример:

```rust
use std::sync::{Arc, Mutex, Once};
use std::time::Duration;
use std::{mem, thread};

#[derive(Clone)]
struct SingletonReader {
    // Since we will be used in many threads, we need to protect
    // concurrent access
    inner: Arc<Mutex<u8>>,
}
fn singleton() -> SingletonReader {
    // Initialize it to a null value
    static mut SINGLETON: *const SingletonReader = 0 as *const SingletonReader;
    static ONCE: Once = Once::new();
    unsafe {
        ONCE.call_once(|| {
            println!("--ONCE--");
            // Make it
            let singleton = SingletonReader {
                inner: Arc::new(Mutex::new(0)),
            };

            // Put it in the heap so it can outlive this call
            SINGLETON = mem::transmute(Box::new(singleton));
        });
        // Now we give out a copy of the data that is safe to use concurrently.
        (*SINGLETON).clone()
    }
}
fn main() {
    // Let's use the singleton in a few threads
    let threads: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(i * 10));
                let s = singleton();
                let mut data = s.inner.lock().unwrap();
                *data = i as u8;
            })
        })
        .collect();

    // And let's check the singleton every so often
    for _ in 0u8..20 {
        thread::sleep(Duration::from_millis(5));

        let s = singleton();
        let data = s.inner.lock().unwrap();
        println!("It is: {}", *data);
    }
    for thread in threads.into_iter() {
        thread.join().unwrap();
    }
}
```

Еще пример с `atomic`:

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);
fn do_a_call() {
    CALL_COUNT.fetch_add(1, Ordering::SeqCst);
}
fn main() {
    do_a_call();
    do_a_call();
    do_a_call();

    println!("called {}", CALL_COUNT.load(Ordering::SeqCst));
}
```

Еще пример с `Mutex`:

```rust
//! ructc 1.63
//! https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
//!
//! Starting with Rust 1.63, it can be easier to work with global mutable
//! singletons, although it's still preferable to avoid global variables in most
//! cases.
//!
//! Now that `Mutex::new` is `const`, you can use global static `Mutex` locks
//! without needing lazy initialization.

use std::sync::Mutex;
static ARRAY: Mutex<Vec<i32>> = Mutex::new(Vec::new());
fn do_a_call() {
    ARRAY.lock().unwrap().push(1);
}
fn main() {
    do_a_call();
    do_a_call();
    do_a_call();

    println!("Called {} times", ARRAY.lock().unwrap().len());
}
```

Еще пример с `lazy_static`:

```rust
use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;
lazy_static! {
    static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
}
fn do_a_call() {
    ARRAY.lock().unwrap().push(1);
}
fn main() {
    do_a_call();
    do_a_call();
    do_a_call();

    println!("called {}", ARRAY.lock().unwrap().len());
}
```

Еще пример с `once_cell`:

```rust
use once_cell::sync::Lazy; // 1.3.1
use std::sync::Mutex;
static ARRAY: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| Mutex::new(vec![]));
fn do_a_call() {
    ARRAY.lock().unwrap().push(1);
}
fn main() {
    do_a_call();
    do_a_call();
    do_a_call();

    println!("called {}", ARRAY.lock().unwrap().len());
}
```
 
[singleton-rs](https://github.com/crlf0710/singleton-rs/blob/master/src/singleton.rs)

[peripherals/singletons](https://rust-embedded.github.io/book/peripherals/singletons.html)

[Singleton refactoring.guru](https://refactoring.guru/design-patterns/singleton/rust/example#example-1)

[how-do-i-create-a-global-mutable-singleton](https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton)

---

</details>

 
## <ins>Структурирующие паттерны</ins>

Данные паттерны помогают внести порядок и научить разные объекты более правильно взаимодействовать друг с другом.
 
<br>
<details>
<summary><b>Adapter</b></summary>

`Pattern Adapter` работает как мост между двумя несовместимыми интерфейсами. 
Он действует как мост между двумя несовместимыми интерфейсами, предоставляя оболочку, позволяющую использовать один объект так, как если бы это был другой.
Эту стратегию можно реализовать с помощью наследования, с помощью агрегации, или композиции, то есть делегирования работы другим объектам.
`Pattern Adapter` делает два несовместимых интерфейса совместимыми без изменения их существующего кода позволяя им работать вместе.
Adapter это объект-переводчик, который трансформирует интерфейс или данные одного объекта в такой вид, чтобы он стал понятен другому объекту.
При этом адаптер оборачивает один из объектов, так что другой объект даже не знает о наличии первого. 
Например, вы можете обернуть объект, работающий в метрах, адаптером, который бы конвертировал данные в футы.

Адаптеры могут не только переводить данные из одного формата в другой, но и помогать объектам с разными интерфейсами работать сообща. 

Это работает так:
1. `Pattern Adapter` имеет интерфейс, который совместим с одним из объектов.
2. Поэтому этот объект может свободно вызывать методы адаптера.
3. `Pattern Adapter` получает эти вызовы и перенаправляет их второму объекту, но уже в том формате и последовательности, которые понятны второму объекту.


Применимость
- Когда вы хотите использовать сторонний класс, но его интерфейс не соответствует остальному коду приложения.
- `Pattern Adapter` позволяет создать объект-прокладку, который будет превращать вызовы приложения в формат, понятный стороннему классу.

**Отношения с другими паттернами**

`Pattern Brige` проектируют загодя, чтобы развивать большие части приложения отдельно друг от друга. 
`Pattern Adapter` применяется постфактум, чтобы заставить несовместимые классы работать вместе.
`Pattern Adapter` заставляет вещи работать после того, как они были спроектированы; `Pattern Brige` заставляет их работать изначально.

`Pattern Facade` задаёт новый интерфейс, тогда как `Pattern Adapter` повторно использует старый. 
`Pattern Adapter` оборачивает только один класс, а `Pattern Facade` оборачивает целую подсистему. 
Кроме того, `Pattern Adapter` позволяет двум существующим интерфейсам работать сообща, вместо того, чтобы задать полностью новый.

`Pattern Adapter` предоставляет другой интерфейс для своего объекта. 
`Pattern Proxy` предоставляет тот же интерфейс. 
`Pattern Decorator` предоставляет улучшенный интерфейс.

`Pattern Adapter` предназначен для изменения интерфейса существующего объекта. 
`Pattern Decorator` улучшает другой объект, не меняя его интерфейс. 
Таким образом, декоратор более прозрачен для приложения, чем адаптер. 
Как следствие, `Pattern Decorator` поддерживает рекурсивную композицию, что невозможно при использовании чистых адаптеров.

![Adapter](/Computer-Science-Bookshelf/img/software_architecture/patterns/Adapter.png "Adapter")

Пример:

```rust
pub mod tools_a {
    pub async fn add_wrap_start(arg: String) -> String {
        std::future::ready(format!("~~~{}", arg)).await
    }
}
pub mod tools_b {
    use std::string::FromUtf8Error;
    pub fn add_wrap_end(arg: &[u8]) -> Result<String, FromUtf8Error> {
        let s = String::from_utf8(arg.to_vec())?;
        let res = format!("{}~~~", s);
        Ok(res)
    }
}

use adapter::*;
pub mod adapter {
    use crate::tools_b;

    use futures::Future;
    pub trait AdapterAsyncToSync {
        fn add_wrap_end(fut: impl Future<Output = String>) -> String;
    }
    pub struct ImplAdapter;
    impl AdapterAsyncToSync for ImplAdapter {
        fn add_wrap_end(fut: impl Future<Output = String>) -> String {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let result = rt.block_on(fut);
            if let Ok(res) = tools_b::add_wrap_end(result.as_bytes()) {
                return res;
            }
            result
        }
    }
}
fn main() {
    let input = "hello";
    let input_with_wrap = {
        let input_a = tools_a::add_wrap_start(input.into());
        let input_b: String = ImplAdapter::add_wrap_end(input_a);
        input_b
    };
    assert_eq!("~~~hello~~~", input_with_wrap)
}
```

Еще Пример:

```rust
// Step 1: Define the target interface
pub trait JsonLogger {
    fn log(&self, data: &str);
}
// The existing class with an incompatible interface
pub struct XmlLogger;
impl XmlLogger {
    pub fn log_xml(&self, xml_data: &str) {
        println!("Logging XML: {}", xml_data);
    }
}
// Step 2: Create the adapter class
pub struct LoggerAdapter {
    xml_logger: XmlLogger,
}
impl LoggerAdapter {
    pub fn new(xml_logger: XmlLogger) -> Self {
        LoggerAdapter { xml_logger }
    }
}
// Step 3 & 4: Implement the target interface using the adapted object's methods
impl JsonLogger for LoggerAdapter {
    fn log(&self, data: &str) {
        // Here, we're simply passing the JSON data to the XML logger
        // In a real-world scenario, you would convert the JSON data to XML format
        self.xml_logger.log_xml(data);
    }
}
fn main() {
    let xml_logger = XmlLogger;
    let logger = LoggerAdapter::new(xml_logger);
    logger.log("{\"message\": \"This is a log.\"}");
}
```

 
* [Adapter sourcemaking.com](https://sourcemaking.com/design_patterns/adapter)
* [Adapter refactoring.guru](https://refactoring.guru/ru/design-patterns/adapter)
* [Adapter www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2023/10/adapter-design-pattern-in-rust.html)

---

</details>

<br>
<details>
<summary><b>Bridge</b></summary>


`Pattern Brige` - отделение интерфейса/абстракции (группа поведений) от реализации (конкретное поведение из группы), позволяя им изменяться независимо. 
Предполагает разбиение монолитной системы на две отдельные иерархии: абстракцию и реализацию.

`Pattern Brige` используется, чтобы избежать увеличения количества подклассов, к которому в конечном итоге могут привести механизмы наследования. 
Итак, если у вас есть, скажем, 2 ортогональные обязанности, то вместо создания 2**2 подклассов вы используете композицию для объединения этих обязанностей.

Учитывая, что любое изменение, внесенное в абстракцию, повлияет на все классы, которые ее реализуют, 
`Pattern Brige` предлагает добавить **новый уровень абстракции** между обоими элементами, который позволяет разрабатывать каждый из них независимо. 

**Отношения с другими паттернами**

Основное различие между `Pattern Brige` и `Pattern Adapter` заключается в том, что `Pattern Adapter` используется 
для унификации уже существующих интерфейсов, а `Pattern Brige` используется, когда есть подозрение, что реализация интерфейса со временем изменится.
Т.е. своевременное использование `Pattern Brige` избавит нас от необходимости внедрять `Pattern Adapter`

Bridge Before:

![BridgeBefore](/Computer-Science-Bookshelf/img/software_architecture/patterns/BridgeBefore.png "BridgeBefore")

Bridge After:

![BridgeAfter](/Computer-Science-Bookshelf/img/software_architecture/patterns/BridgeAfter.png "BridgeAfter")


Пример:
Система устройств в которой используется кнопки (это и есть абстракция) для включения/выключения.
Кнопки могут быть (это первая реализация) на пружине, сенсорные, bluetooth,
также (это вторая реализация) они могут иметь не только два состояния вкл/выкл но еще нейтральное или промежуточное,
также (это третья реализация) они могут иметь применение в помещения или снаружи или в космосе, под водой ...
И мы как следсвие создаем под каждый новый тип свою абстракцию т.е. на пружине, сенсорные, bluetooth это все абстракции и про состояния и применение.
Вообшем количество расширений может расти и количество создаваемых вариантов растет в геометрической прогрессии.
Но вместо создания конкретных абстракций сдедует их выделить в абстракную группу

Кнопка может иметь различные виды поведения:
- Принцип действия: пружина, сенсор, bluetooth,...
- Состояния: вкл/выкл, нейтральное,...
- Применение: в помещения, снаружи,...
- ...


```rust
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Clone, Copy)]
pub struct User;
impl User {
    fn verif_sensor(&self, sensor: &str) -> bool {
        true
    }
}

use btn::{Button1, Button2};
pub mod btn {
    use super::User;
    pub struct Button1 {
        // Кнопка на пружине с двумя состояниями
        status: bool,
        spring: bool,
    }
    impl Button1 {
        pub fn new() -> Self {
            Self {
                status: false,
                spring: false,
            }
        }
        pub fn turn_on(&mut self, _user: Option<User>) -> bool {
            if !self.spring_state() {
                self.spring_on();
                self.status = true;
                println!("Кнопка нажата");
                return true;
            }
            false
        }
        pub fn turn_off(&mut self, _user: Option<User>) -> bool {
            if self.spring_state() {
                self.spring_off();
                self.status = false;
                println!("Кнопка отжата");
                return true;
            }
            false
        }
        fn spring_state(&self) -> bool {
            if self.spring {
                return true;
            }
            false
        }
        fn spring_on(&mut self) {
            self.spring = true;
            println!("Пружина сжата");
        }
        fn spring_off(&mut self) {
            self.spring = false;
            println!("Пружина расжата");
        }
    }

    pub struct Button2 {
        // Кнопка на сенсоре с двумя состояниями
        status: bool,
        sensor: String,
    }
    impl Button2 {
        fn new(sensor: String) -> Self {
            Self {
                status: false,
                sensor,
            }
        }
        pub fn turn_on(&mut self, user: Option<User>) -> bool {
            if user.unwrap().verif_sensor(&self.sensor) {
                if !self.status {
                    self.status = true;
                    println!("Кнопка нажата");
                    return true;
                }
            }
            false
        }
        pub fn turn_off(&mut self, user: Option<User>) -> bool {
            if user.unwrap().verif_sensor(&self.sensor) {
                if self.status {
                    self.status = false;
                    println!("Кнопка отжата");
                    return true;
                }
            }
            false
        }
    }
}
// Что будет, если добавить еще один тип состония?
// Придется создавать еще две структуры для пружины и сенсора.
// И чем это мешает? Много структур?
// Да! при добавлении нового типа поведения придется реализовывать каждую комбинацию поведений
use client::Client;
pub mod client {
    use super::*;
    pub struct Client {
        btn: Button1,
    }
    impl Client {
        pub fn new() -> Self {
            Client {
                btn: Button1::new(),
            }
        }
        pub fn turn_on(&mut self) {
            if self.btn.turn_on(Some(User)) {
                println!("Включено\n");
            }
        }
        pub fn turn_off(&mut self) {
            if self.btn.turn_off(Some(User)) {
                println!("Выключено\n");
            }
        }
    }
}
//------------------------------------------------------------------------------------------------
// Решение - выделить возможные новые поведения в группы, эти группы будут абстракции, а их реализации - новые поведения.
// Такмим образом, группа вариантов отвечающих за внутреннее устройство - пружина или сенсор или еще что-то теперь в группе `trait OperatingPrinciple`
// а все виды поведения по стостояния кнопки теперь в группе `trait ModeState`
// И все новые их варианты не будут увеличивать количество реализаций
// И мы сможем разделить код для независимой разработки групп поведения
use btn2::*;
mod btn2 {
    use super::*;
    pub trait Button {
        fn turn_on(&mut self, user: Option<User>) -> bool;
        fn turn_off(&mut self, user: Option<User>) -> bool;
    }
    pub trait ModeState {
        fn on(&mut self);
        fn off(&mut self);
        fn state(&self) -> Option<bool>;
    }
    pub struct StateOnOff {
        state: bool,
    }
    pub enum VariantState {
        On,
        Off,
        Zero,
    }
    pub struct StateOnZeroOff {
        state: VariantState,
    }
    impl StateOnZeroOff {
        pub fn new() -> Self {
            Self {
                state: VariantState::Off,
            }
        }
    }
    impl ModeState for StateOnOff {
        fn on(&mut self) {}
        fn off(&mut self) {}
        fn state(&self) -> Option<bool> {
            Some(self.state)
        }
    }
    impl ModeState for StateOnZeroOff {
        fn on(&mut self) {}
        fn off(&mut self) {}
        fn state(&self) -> Option<bool> {
            match self.state {
                VariantState::On => Some(true),
                VariantState::Off => Some(false),
                VariantState::Zero => None,
            }
        }
    }

    pub trait OperatingPrinciple {
        fn check(&self, user: Option<&User>) -> bool;
        fn on(&mut self);
        fn off(&mut self);
    }
    pub struct Spring {
        spring: bool,
    }
    pub struct Sensor;
    impl OperatingPrinciple for Spring {
        fn check(&self, user: Option<&User>) -> bool {
            self.spring_state()
        }
        fn on(&mut self) {
            self.spring = false;
        }
        fn off(&mut self) {
            self.spring = true;
        }
    }
    impl Spring {
        fn spring_state(&self) -> bool {
            if self.spring {
                return true;
            }
            false
        }
    }
    impl OperatingPrinciple for Sensor {
        fn check(&self, user: Option<&User>) -> bool {
            if let Some(u) = user {
                return u.verif_sensor("");
            }
            false
        }
        fn on(&mut self) {}
        fn off(&mut self) {}
    }
    pub struct ButtonV2<M, C> {
        status: M,
        core: C,
    }
    impl<M: ModeState, C: OperatingPrinciple> ButtonV2<M, C> {
        pub fn new(status: M, core: C) -> Self {
            Self { status, core }
        }
    }

    impl<M: ModeState, C: OperatingPrinciple> Button for ButtonV2<M, C> {
        fn turn_on(&mut self, user: Option<User>) -> bool {
            if self.core.check(user.as_ref()) {
                self.core.on();
                self.status.on();
                println!("Кнопка нажата");
                return true;
            }
            false
        }
        fn turn_off(&mut self, user: Option<User>) -> bool {
            if self.core.check(user.as_ref()) {
                self.core.off();
                self.status.off();
                println!("Кнопка отжата");
                return true;
            }
            false
        }
    }
}

use client2::Client2;
mod client2 {
    use super::*;
    pub struct Client2<B> {
        btn: B,
    }
    impl<B: Button> Client2<B> {
        pub fn new(btn: B) -> Self {
            Self { btn }
        }
        pub fn turn_on(&mut self) {
            if self.btn.turn_on(Some(User)) {
                println!("Включено\n");
            }
        }
        pub fn turn_off(&mut self) {
            if self.btn.turn_off(Some(User)) {
                println!("Выключено\n");
            }
        }
    }
}
fn main() {
    let mut client = Client::new();
    client.turn_on();
    client.turn_off();
    println!("-----------------------");
    let mut client = Client2::new(ButtonV2::new(StateOnZeroOff::new(), Sensor));
    client.turn_on();
    client.turn_off();
}
```

Еще пример:

```rust
// Абстракция обьектом которой будут манипулировать разные реализации печати
trait DisplayImpl {
    fn raw_open(&self);
    fn raw_print(&self);
    fn raw_close(&self);
}

// Отделяем реализацию
// Первый вариант реализации печати
struct Display {
    imple: Box<DisplayImpl>,
}
impl Display {
    fn new(imple: Box<DisplayImpl>) -> Display {
        Display { imple: imple }
    }
    fn open(&self) {
        self.imple.raw_open();
    }
    fn print(&self) {
        self.imple.raw_print();
    }
    fn close(&self) {
        self.imple.raw_close();
    }
    fn display(&self) {
        self.open();
        self.print();
        self.close();
    }
}
// Отделяем реализацию
// Второй вариант реализации печати
struct CountDisplay {
    display: Box<DisplayImpl>,
}
impl CountDisplay {
    fn new(display: Box<DisplayImpl>) -> CountDisplay {
        CountDisplay { display: display }
    }
    fn multi_display(&self, times: u32) {
        self.display.raw_open();
        for _ in 0..times {
            self.display.raw_print();
        }
        self.display.raw_close();
    }
    fn display(&self) {
        self.display.raw_open();
        self.display.raw_print();
        self.display.raw_close();
    }
}

//----------------------------------------------------------------
// Отделяем абстракцию
struct StringDisplayImpl {
    string: String,
    width: usize,
}
impl StringDisplayImpl {
    fn new(string: String) -> StringDisplayImpl {
        StringDisplayImpl {
            string: string.clone(),
            width: string.chars().count(),
        }
    }
    fn print_line(&self) {
        print!("+");
        for _ in 0..self.width {
            print!("-");
        }
        println!("+");
    }
}
impl DisplayImpl for StringDisplayImpl {
    fn raw_open(&self) {
        self.print_line();
    }
    fn raw_print(&self) {
        println!("|{}|", self.string);
    }
    fn raw_close(&self) {
        self.print_line();
    }
}
fn main() {
    let d1 = Display::new(Box::new(StringDisplayImpl::new("Hello, Japan".to_string())));
    let d2 = CountDisplay::new(Box::new(StringDisplayImpl::new(
        "Hello, World.".to_string(),
    )));
    let d3 = CountDisplay::new(Box::new(StringDisplayImpl::new(
        "Hello, Universe.".to_string(),
    )));
    d1.display();
    d2.display();
    d3.display();
    d3.multi_display(5);
}
```



Еще пример:

```rust
// Step 1 & 2: Define the abstraction and the implementor interface
// Implementor
pub trait Renderer {
    fn render_circle(&self, radius: f32);
}
// Abstraction
pub struct Circle {
    radius: f32,
    renderer: Box<dyn Renderer>,
}
impl Circle {
    pub fn new(radius: f32, renderer: Box<dyn Renderer>) -> Self {
        Circle { radius, renderer }
    }
    pub fn draw(&self) {
        self.renderer.render_circle(self.radius);
    }
}
// Step 3: Concrete implementations
pub struct OpenGLRenderer;
impl Renderer for OpenGLRenderer {
    fn render_circle(&self, radius: f32) {
        println!("OpenGL: Drawing a circle of radius {}", radius);
    }
}
pub struct DirectXRenderer;
impl Renderer for DirectXRenderer {
    fn render_circle(&self, radius: f32) {
        println!("DirectX: Drawing a circle of radius {}", radius);
    }
}
fn main() {
    let opengl = OpenGLRenderer;
    let directx = DirectXRenderer;
    let circle_opengl = Circle::new(5.0, Box::new(opengl));
    let circle_directx = Circle::new(10.0, Box::new(directx));
    circle_opengl.draw();
    circle_directx.draw();
}
```



* [Bridge](http://dron.by/post/pattern-proektirovaniya-bridge-most-na-php.html)
* [Bridge sourcemaking.com](https://sourcemaking.com/design_patterns/bridge)
* [Bridge refactoring.guru](https://refactoring.guru/ru/design-patterns/bridge)
* [Bridge chercher.tech](https://chercher.tech/rust/bridge-design-pattern-rust)
* [Bridge www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2023/10/bridge-design-pattern-in-rust.html)

---

</details>

<br>
<details>
<summary><b>Decorator</b></summary>
 

`Pattern Decorator` (обёртка) - суть работы паттерна заключается в "оборачивании" готового объекта новым "функционалом", 
при этом весь оригинальный интерфейс объекта остается доступным (декоратор переадресует все запросы объекту). 
Смысл заключается в том, чтобы можно было безболезненно комбинировать различные декораторы в произвольном порядке, навешивая их на различные объекты. 

Оба объекта имеют общий интерфейс, поэтому для пользователя нет никакой разницы, с каким объектом работать — чистым или обёрнутым. 
Вы можете использовать несколько разных обёрток одновременно — результат будет иметь объединённое поведение всех обёрток сразу.

**Аналогия**
Уличный аниматор наряжается создавая образ из различных аксессуаров - макияж, одежда, больщие уши, перчатки, обувь, головной убор, кегли, трость, ....
Акссесуары - это все декораторы, комбинируй в любой последовательности

Поскольку этот шаблон решает проблему динамического добавления функций во время выполнения, 
он решает проблему сложного требования к созданию подклассов при расширении функциональности базового класса.

Если есть требования четкой последовательности накидывания поведения то стоит рассмотреть вариации `Pattern Builder` или `Pattern Strategy`

**Эмпирические правила**

- Адаптер предоставляет другой интерфейс для своего объекта. Прокси предоставляет тот же интерфейс. Декоратор предоставляет улучшенный интерфейс.

- Адаптер меняет интерфейс объекта, Декоратор расширяет возможности объекта. Таким образом, Decorator становится более прозрачным для клиента. 
Как следствие, Decorator поддерживает рекурсивную композицию, что невозможно при использовании чистых адаптеров.

- Composite и Decorator имеют схожие структурные диаграммы, отражающие тот факт, что оба используют рекурсивную композицию для организации н
еограниченного количества объектов.

- Декоратор можно рассматривать как вырожденный композит, содержащий только один компонент. 
Однако декоратор добавляет дополнительные обязанности — он не предназначен для агрегации объектов.

- Декоратор предназначен для того, чтобы вы могли добавлять обязанности к объектам без создания подклассов. 
Основное внимание Composite уделяется не украшению, а репрезентации. Эти намерения различны, но дополняют друг друга. 
Следовательно, Composite и Decorator часто используются совместно.

- Composite может использовать цепочку ответственности, чтобы позволить компонентам получать доступ к глобальным свойствам через своего родителя. 
Он также может использовать Decorator для переопределения этих свойств частей композиции.

- Декоратор и Прокси имеют разные цели, но схожую структуру. 
Оба описывают, как обеспечить уровень косвенности к другому объекту, и реализации сохраняют ссылку на объект, которому они перенаправляют запросы.

- Декоратор позволяет изменить внешний вид объекта. Стратегия позволяет вам изменить внутренности.

**Пример**
Построить конструктор фильтров для input полей формы. Помимо множества типов полей, есть еще правила валидации
Так, накидывая различные декораторы на input, мы получаем желаемый результат

**Пример**
Приложение оборачивает класс данных в шифрующую и сжимающую обёртки, которые при чтении выдают оригинальные данные, а при записи — зашифрованные и сжатые.

![Decorator](/Computer-Science-Bookshelf/img/software_architecture/patterns/Decorator.jpg "Decorator")


Пример:

```rust
#![allow(dead_code)]
#![allow(unreachable_patterns)]

// Конструктор коктейлей
#[derive(Clone, Copy)]
enum Size {
    TALL,
    GRANDE,
    VENTI,
}
impl Default for Size {
    fn default() -> Size {
        Size::TALL
    }
}

// Напиток
trait Beverage {
    fn get_description(&self) -> String;
    fn set_size(&mut self, size: Size);
    fn get_size(&self) -> Size;
    fn cost(&self) -> f32;
}

// Виды напитка -------------------------------------------------------- 
struct HouseBlend {
    size: Size,
    description: String,
    default_cost: f32,
}
impl Default for HouseBlend {
    fn default() -> Self {
        Self {
            size: Size::default(),
            description: String::from("House Blend Coffee"),
            default_cost: 0.89f32,
        }
    }
}
impl HouseBlend {
    fn new(size: Size, default_cost: f32, description: String) -> Self {
        Self {
            size,
            description,
            default_cost,
        }
    }
    fn new_with_size_default(size: Size) -> Self {
        let def = Self::default();
        Self {
            size,
            description: def.description,
            default_cost: def.default_cost,
        }
    }
}
impl Beverage for HouseBlend {
    fn get_description(&self) -> String {
        self.description.clone()
    }
    fn set_size(&mut self, size: Size) {
        self.size = size;
    }
    fn get_size(&self) -> Size {
        self.size
    }
    fn cost(&self) -> f32 {
        match self.get_size() {
            Size::TALL => self.default_cost,
            Size::GRANDE => self.default_cost + 0.35,
            Size::VENTI => self.default_cost + 0.40,
            _ => self.default_cost,
        }
    }
}
// Виды напитка
struct DarkRoast {
    size: Size,
    description: String,
    default_cost: f32,
}
impl Default for DarkRoast {
    fn default() -> Self {
        Self {
            size: Size::default(),
            description: String::from("DarkRoast"),
            default_cost: 1.0f32,
        }
    }
}
impl DarkRoast {
    fn new(size: Size, default_cost: f32, description: String) -> Self {
        Self {
            size,
            description,
            default_cost,
        }
    }
    fn new_with_size_default(size: Size) -> Self {
        let def = Self::default();
        Self {
            size,
            description: def.description,
            default_cost: def.default_cost,
        }
    }
}
impl Beverage for DarkRoast {
    fn get_description(&self) -> String {
        self.description.clone()
    }
    fn set_size(&mut self, size: Size) {
        self.size = size;
    }
    fn get_size(&self) -> Size {
        self.size
    }
    fn cost(&self) -> f32 {
        match self.get_size() {
            Size::TALL => self.default_cost,
            Size::GRANDE => 0.35,
            Size::VENTI => 0.40,
            _ => self.default_cost,
        }
    }
}
// Виды напитка
struct Espresso {
    size: Size,
    description: String,
    default_cost: f32,
}
impl Default for Espresso {
    fn default() -> Self {
        Self {
            size: Size::default(),
            description: String::from("Espresso"),
            default_cost: 1.99f32,
        }
    }
}
impl Espresso {
    fn new(size: Size, default_cost: f32, description: String) -> Self {
        Self {
            size,
            description,
            default_cost,
        }
    }
    fn new_with_size_default(size: Size) -> Self {
        let def = Self::default();
        Self {
            size,
            description: def.description,
            default_cost: def.default_cost,
        }
    }
}
impl Beverage for Espresso {
    fn get_description(&self) -> String {
        self.description.clone()
    }
    fn set_size(&mut self, size: Size) {
        self.size = size;
    }
    fn get_size(&self) -> Size {
        self.size
    }
    fn cost(&self) -> f32 {
        match self.get_size() {
            Size::TALL => self.default_cost,
            Size::GRANDE => self.default_cost + 0.40,
            Size::VENTI => self.default_cost + 0.45,
            _ => self.default_cost,
        }
    }
}
// Виды напитка
struct Decaf {
    size: Size,
    description: String,
    default_cost: f32,
}
impl Default for Decaf {
    fn default() -> Self {
        Self {
            size: Size::default(),
            description: String::from("Decaf"),
            default_cost: 1.99f32,
        }
    }
}
impl Decaf {
    fn new(size: Size, default_cost: f32, description: String) -> Self {
        Self {
            size,
            description,
            default_cost,
        }
    }
}
impl Beverage for Decaf {
    fn get_description(&self) -> String {
        self.description.clone()
    }
    fn set_size(&mut self, size: Size) {
        self.size = size;
    }
    fn get_size(&self) -> Size {
        self.size
    }
    fn cost(&self) -> f32 {
        match self.get_size() {
            Size::TALL => self.default_cost,
            Size::GRANDE => self.default_cost + 0.35,
            Size::VENTI => self.default_cost + 0.40,
            _ => self.default_cost,
        }
    }
}

trait New {
    fn new(beverage: Box<dyn Beverage>) -> Self;
}
// Дополнения к напитку --------------------------------------------------- 
struct Mocha {
    beverage: Box<dyn Beverage>,
}
impl New for Mocha {
    fn new(beverage: Box<dyn Beverage>) -> Self {
        Self { beverage }
    }
}
impl Beverage for Mocha {
    fn get_description(&self) -> String {
        format!("{}, Mocha", self.beverage.get_description())
    }
    fn set_size(&mut self, size: Size) {
        self.beverage.set_size(size);
    }
    fn get_size(&self) -> Size {
        self.beverage.get_size()
    }
    fn cost(&self) -> f32 {
        let mut cost: f32 = self.beverage.cost();
        cost += match self.beverage.get_size() {
            Size::TALL => 0.20,
            Size::GRANDE => 0.35,
            Size::VENTI => 0.40,
            _ => 0.20,
        };
        cost
    }
}
// Дополнения к напитку
struct Milk {
    beverage: Box<dyn Beverage>,
}
impl New for Milk {
    fn new(beverage: Box<dyn Beverage>) -> Self {
        Self { beverage }
    }
}
impl Beverage for Milk {
    fn get_description(&self) -> String {
        format!("{}, Milk", self.beverage.get_description())
    }
    fn set_size(&mut self, size: Size) {
        self.beverage.set_size(size);
    }
    fn get_size(&self) -> Size {
        self.beverage.get_size()
    }
    fn cost(&self) -> f32 {
        let mut cost: f32 = self.beverage.cost();
        cost += match self.beverage.get_size() {
            Size::TALL => 0.15,
            Size::GRANDE => 0.25,
            Size::VENTI => 0.30,
            _ => 0.15,
        };
        cost
    }
}
// Дополнения к напитку
struct Soy {
    beverage: Box<dyn Beverage>,
}
impl New for Soy {
    fn new(beverage: Box<dyn Beverage>) -> Self {
        Self { beverage }
    }
}
impl Beverage for Soy {
    fn get_description(&self) -> String {
        format!("{}, Soy", self.beverage.get_description())
    }
    fn set_size(&mut self, size: Size) {
        self.beverage.set_size(size);
    }
    fn get_size(&self) -> Size {
        self.beverage.get_size()
    }
    fn cost(&self) -> f32 {
        let mut cost: f32 = self.beverage.cost();
        cost += match self.beverage.get_size() {
            Size::TALL => 0.10,
            Size::GRANDE => 0.15,
            Size::VENTI => 0.20,
            _ => 0.10,
        };
        cost
    }
}
// Дополнения к напитку
struct Whip {
    beverage: Box<dyn Beverage>,
}
impl New for Whip {
    fn new(beverage: Box<dyn Beverage>) -> Self {
        Self { beverage }
    }
}
impl Beverage for Whip {
    fn get_description(&self) -> String {
        format!("{}, Whip", self.beverage.get_description())
    }
    fn set_size(&mut self, size: Size) {
        self.beverage.set_size(size);
    }
    fn get_size(&self) -> Size {
        self.beverage.get_size()
    }
    fn cost(&self) -> f32 {
        let mut cost: f32 = self.beverage.cost();
        cost += match self.beverage.get_size() {
            Size::TALL => 0.10,
            Size::GRANDE => 0.15,
            Size::VENTI => 0.20,
            _ => 0.10,
        };
        cost
    }
}
fn add<T: Beverage + New>(beverage: Box<dyn Beverage>) -> T {
    T::new(beverage)
}
fn main() {
    // Кофе Espresso
    let espresso: Espresso = Espresso::new_with_size_default(Size::VENTI);
    println!(
        "{:?} ${:?}",
        espresso.get_description(),
        espresso.cost() as f32
    );

    // Кофе с двойным шоколадом и взбитыми сливками
    let dark_roast: DarkRoast = DarkRoast::default();
    let mocha = add::<Mocha>(Box::new(dark_roast));
    let mocha = add::<Mocha>(Box::new(mocha));
    let milk = add::<Milk>(Box::new(mocha));
    println!("{:?} ${:?}", milk.get_description(), milk.cost() as f32);

    // Кофе "Домашняя смесь" с соей,шоколадом и взбитыми сливками
    let house_blend: HouseBlend = HouseBlend::new(Size::VENTI, 0.89, String::from("HouseBlend"));
    let soy = add::<Soy>(Box::new(house_blend));
    let mocha = add::<Mocha>(Box::new(soy));
    let whip = add::<Whip>(Box::new(mocha));
    println!("{:?} ${:?}", whip.get_description(), whip.cost() as f32);

    /*
    "Espresso" $2.44
    "DarkRoast, Mocha, Mocha, Milk" $1.55
    "HouseBlend, Soy, Mocha, Whip" $2.08
    */
}
```


Еще пример:

```rust
/*
Пример шаблона проектирования декоратора
Это все равно, что добавить сырную начинку в пиццу или добавить начинку из грецких орехов и шоколада в мороженое.
Вы можете добавить в пиццу любой сыр или соус непосредственно перед подачей клиенту, но основа пиццы останется неизменной.

Точно так же вы можете заказать грецкие орехи или вишню в ванильном мороженом,
а можете передумать и попросить добавить только шоколадную начинку и вишневый соус.
База ванильного мороженого не изменится, и вы можете добавлять к мороженому любую начинку, какую захотите.

Если в группе 20 человек, они могут заказать 20 ванильных мороженых с разными начинками, орехами и фруктами во время подачи.
*/

// Decorator pattern
trait Component {
    fn do_something(&self);
}
trait Decorator: Component {
    fn do_something_more(&self);
}
struct BaseObject {
    value: usize,
}
impl Component for BaseObject {
    fn do_something(&self) {
        println!("calculate value: {}", self.value);
    }
}
struct DecoratorObject {
    base: BaseObject,
    more_value: usize,
}
impl Component for DecoratorObject {
    fn do_something(&self) {
        self.base.do_something()
    }
}
impl Decorator for DecoratorObject {
    fn do_something_more(&self) {
        println!("calculate more: {}", self.more_value);
    }
}
fn process(c: &Component) {
    c.do_something();
}
fn main() {
    let obj = BaseObject { value: 100 };
    process(&obj);

    let decorator_obj = DecoratorObject {
        base: obj,
        more_value: 999,
    };
    process(&decorator_obj);

    decorator_obj.do_something_more();
}
```


Еще пример:

```rust
// Step 1: Common interface for core and decorators
pub trait Beverage {
    fn cost(&self) -> f32;
    fn description(&self) -> String;
}
// Step 2: Concrete core object
pub struct Coffee;
impl Beverage for Coffee {
    fn cost(&self) -> f32 {
        2.0
    }
    fn description(&self) -> String {
        "Coffee".to_string()
    }
}
// Step 3: Decorator classes
pub struct Milk<'a> {
    beverage: &'a dyn Beverage,
}
impl<'a> Milk<'a> {
    pub fn new(beverage: &'a dyn Beverage) -> Self {
        Milk { beverage }
    }
}
impl<'a> Beverage for Milk<'a> {
    fn cost(&self) -> f32 {
        self.beverage.cost() + 0.5
    }
    fn description(&self) -> String {
        format!("{} with Milk", self.beverage.description())
    }
}
pub struct Sugar<'a> {
    beverage: &'a dyn Beverage,
}
impl<'a> Sugar<'a> {
    pub fn new(beverage: &'a dyn Beverage) -> Self {
        Sugar { beverage }
    }
}
impl<'a> Beverage for Sugar<'a> {
    fn cost(&self) -> f32 {
        self.beverage.cost() + 0.25
    }
    fn description(&self) -> String {
        format!("{} with Sugar", self.beverage.description())
    }
}
fn main() {
    let my_coffee = Coffee;
    let my_coffee_with_milk = Milk::new(&my_coffee);
    let my_coffee_with_milk_and_sugar = Sugar::new(&my_coffee_with_milk);
    println!("{}", my_coffee_with_milk_and_sugar.description());
    println!("Cost: ${}", my_coffee_with_milk_and_sugar.cost());
}
```


* [Decorator chercher.tech](https://chercher.tech/rust/decorator-design-pattern-rust)
* [Decorator sourcemaking.com](https://sourcemaking.com/design_patterns/decorator)
* [Decorator refactoring.guru](https://refactoring.guru/ru/design-patterns/decorator)
* [Decorator www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2023/10/decorator-design-pattern-in-rust.html)

---

</details>

<br>
<details>
<summary><b>Facade</b></summary>
 
`Pattern Facade` скрывает сложности системы и предоставляет простой интерфейс к сложной системе классов, библиотеке или фреймворку.

Оберните сложную подсистему более простым интерфейсом.
Один класс Facade, представляющий всю подсистему. 

Объект Facade должен быть довольно простым защитником или посредником. Он не должен становиться всезнающим оракулом или объектом «бога».

**Проблема**
Вашему коду приходится работать с большим количеством объектов некой сложной библиотеки или фреймворка. 
Вы должны самостоятельно инициализировать эти объекты, следить за правильным порядком зависимостей и так далее.
В результате бизнес-логика ваших классов тесно переплетается с деталями реализации сторонних классов. 
Такой код довольно сложно понимать и поддерживать.

`Pattern Facade` может иметь урезанный интерфейс, не имеющий 100% функциональности, которой можно достичь, используя сложную подсистему напрямую. 
Но он предоставляет именно те фичи, которые нужны клиенту, и скрывает все остальные.

`Pattern Facade` полезен, если вы используете какую-то сложную библиотеку со множеством подвижных частей, но вам нужна только часть её возможностей.

Аналогия
Когда вы звоните в магазин и делаете заказ по телефону, сотрудник службы поддержки является вашим фасадом ко всем службам и отделам магазина. 
Он предоставляет вам упрощённый интерфейс к системе создания заказа, платёжной системе и отделу доставки.

**Эмпирические правила**

- `Pattern Facade` задаёт новый интерфейс, тогда как `Pattern Adapter` повторно использует старый. 
`Pattern Adapter` оборачивает только один класс, а `Pattern Facade` оборачивает целую подсистему. 
Кроме того, `Pattern Adapter` позволяет двум существующим интерфейсам работать сообща, вместо того, чтобы задать полностью новый.

- Abstract Factory может быть использована вместо `Pattern Facade` для того, чтобы скрыть платформо-зависимые классы.

- Объекты Facade часто являются синглтонами, поскольку требуется только один объект Facade.

![Facade](/Computer-Science-Bookshelf/img/software_architecture/patterns/Facade.png "Facade")

Пример:

```rust
#![allow(dead_code)]
#![allow(unused_imports)]

use wallet_facade::*;
mod wallet_facade {
    use crate::{
        account::Account, ledger::Ledger, notification::Notification, security_code::SecurityCode,
        wallet::Wallet,
    };
    /// Facade hides a complex logic behind the API.
    pub struct WalletFacade {
        account: Account,
        wallet: Wallet,
        code: SecurityCode,
        notification: Notification,
        ledger: Ledger,
    }
    impl WalletFacade {
        pub fn new(account_id: String, code: u32) -> Self {
            println!("Starting create account");
            let this = Self {
                account: Account::new(account_id),
                wallet: Wallet::new(),
                code: SecurityCode::new(code),
                notification: Notification,
                ledger: Ledger,
            };
            println!("Account created");
            this
        }
        pub fn add_money_to_wallet(
            &mut self,
            account_id: &String,
            security_code: u32,
            amount: u32,
        ) -> Result<(), String> {
            println!("Starting add money to wallet");
            self.account.check(account_id)?;
            self.code.check(security_code)?;
            self.wallet.credit_balance(amount);
            self.notification.send_wallet_credit_notification();
            self.ledger.make_entry(account_id, "credit".into(), amount);
            Ok(())
        }
        pub fn deduct_money_from_wallet(
            &mut self,
            account_id: &String,
            security_code: u32,
            amount: u32,
        ) -> Result<(), String> {
            println!("Starting debit money from wallet");
            self.account.check(account_id)?;
            self.code.check(security_code)?;
            self.wallet.debit_balance(amount);
            self.notification.send_wallet_debit_notification();
            self.ledger.make_entry(account_id, "debit".into(), amount);
            Ok(())
        }
    }
}
use wallet::*;
mod wallet {
    pub struct Wallet {
        balance: u32,
    }
    impl Wallet {
        pub fn new() -> Self {
            Self { balance: 0 }
        }
        pub fn credit_balance(&mut self, amount: u32) {
            self.balance += amount;
        }
        pub fn debit_balance(&mut self, amount: u32) {
            self.balance
                .checked_sub(amount)
                .expect("Balance is not sufficient");
        }
    }
}
use account::*;
mod account {
    pub struct Account {
        name: String,
    }
    impl Account {
        pub fn new(name: String) -> Self {
            Self { name }
        }
        pub fn check(&self, name: &String) -> Result<(), String> {
            if &self.name != name {
                return Err("Account name is incorrect".into());
            }
            println!("Account verified");
            Ok(())
        }
    }
}
use ledger::*;
mod ledger {
    pub struct Ledger;
    impl Ledger {
        pub fn make_entry(&mut self, account_id: &String, txn_type: String, amount: u32) {
            println!(
                "Make ledger entry for accountId {} with transaction type {} for amount {}",
                account_id, txn_type, amount
            );
        }
    }
}
use notification::*;
mod notification {
    pub struct Notification;
    impl Notification {
        pub fn send_wallet_credit_notification(&self) {
            println!("Sending wallet credit notification");
        }
        pub fn send_wallet_debit_notification(&self) {
            println!("Sending wallet debit notification");
        }
    }
}
use security_code::*;
mod security_code {
    pub struct SecurityCode {
        code: u32,
    }
    impl SecurityCode {
        pub fn new(code: u32) -> Self {
            Self { code }
        }
        pub fn check(&self, code: u32) -> Result<(), String> {
            if self.code != code {
                return Err("Security code is incorrect".into());
            }
            println!("Security code verified");
            Ok(())
        }
    }
}
fn main() -> Result<(), String> {
    let mut wallet = WalletFacade::new("abc".into(), 1234);
    println!();
    // Wallet Facade interacts with the account, code, wallet, notification and
    // ledger behind the scenes.
    wallet.add_money_to_wallet(&"abc".into(), 1234, 10)?;
    println!();
    wallet.deduct_money_from_wallet(&"abc".into(), 1234, 5)
}
```

Еще пример:

```rust
// Subsystem components
struct BIOS;
impl BIOS {
    pub fn execute(&self) {
        println!("Executing BIOS...");
    }
}
struct CPU;
impl CPU {
    pub fn load(&self) {
        println!("Loading CPU...");
    }
}
struct Memory;
impl Memory {
    pub fn check(&self) {
        println!("Checking memory...");
    }
}
struct HardDrive;
impl HardDrive {
    pub fn read(&self) {
        println!("Reading from hard drive...");
    }
}
// Facade
struct ComputerFacade {
    bios: BIOS,
    cpu: CPU,
    memory: Memory,
    hard_drive: HardDrive,
}
impl ComputerFacade {
    pub fn new() -> Self {
        ComputerFacade {
            bios: BIOS {},
            cpu: CPU {},
            memory: Memory {},
            hard_drive: HardDrive {},
        }
    }
    pub fn start(&self) {
        self.bios.execute();
        self.cpu.load();
        self.memory.check();
        self.hard_drive.read();
    }
}
fn main() {
    let computer = ComputerFacade::new();
    computer.start();
}
```



* [Facade refactoring.guru](https://refactoring.guru/ru/design-patterns/facade)
* [Facade sourcemaking.com](https://sourcemaking.com/design_patterns/facade)
* [Facade www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2023/10/facade-design-pattern-in-rust.html)

---

</details>

<br>
<details>
<summary><b>Composite</b></summary>

`Pattern Composite` позволяет объединять объекты в древовидную структуру и работать с ней, как если бы это был отдельный объект.
Замечательной особенностью `Pattern Composite` является возможность рекурсивного запуска методов по всей древовидной структуре и суммирования результатов.
Позволяя одинаково трактовать индивидуальные и составные объекты.

Использование `Pattern Composite` имеет смысл только в том случае, если базовую модель вашего приложения можно представить в виде дерева.
Решает проблему легкого доступа/обхода составных элементов

Пример:

Давайте попробуем понять шаблон Composite на примере файловой системы операционной системы. 
В файловой системе существует два типа объектов: файлы и папки. Бывают случаи, когда с файлами и папками следует обращаться одинаково. 
Вот тут-то и пригодится шаблон Composite.
File и Directory оба trait Component имеют один search метод. 
Для файла он просто просмотрит содержимое файла; для папки он просмотрит все файлы этой папки, чтобы найти это ключевое слово.

![Composite](/Computer-Science-Bookshelf/img/software_architecture/patterns/Composite.png "Composite")

```rust
pub trait Component {
    fn search(&self, keyword: &str);
}
pub mod dummy_file {
    use super::Component;
    pub struct File {
        name: &'static str,
    }
    impl File {
        pub fn new(name: &'static str) -> Self {
            Self { name }
        }
    }
    impl Component for File {
        fn search(&self, keyword: &str) {
            println!("Searching for keyword {} in file {}", keyword, self.name);
        }
    }
}
pub mod dummy_folder {
    use super::Component;
    pub struct Folder {
        name: &'static str,
        components: Vec<Box<dyn Component>>,
    }
    impl Folder {
        pub fn new(name: &'static str) -> Self {
            Self {
                name,
                components: vec![],
            }
        }
        pub fn add(&mut self, component: impl Component + 'static) {
            self.components.push(Box::new(component));
        }
    }
    impl Component for Folder {
        fn search(&self, keyword: &str) {
            println!(
                "Searching recursively for keyword {} in folder {}",
                keyword, self.name
            );
            for component in self.components.iter() {
                component.search(keyword);
            }
        }
    }
}
fn main() {
    let file1 = dummy_file::File::new("File 1");
    let file2 = dummy_file::File::new("File 2");
    let file3 = dummy_file::File::new("File 3");
    let mut folder1 = dummy_folder::Folder::new("Folder 1");
    folder1.add(file1);
    let mut folder2 = dummy_folder::Folder::new("Folder 2");
    folder2.add(file2);
    folder2.add(file3);
    folder2.add(folder1);
    folder2.search("rose");
}
```

Еще пример:
```rust
// Step 1: Define the component interface
pub trait Graphic {
    fn draw(&self);
}
// Step 2: Leaf object
pub struct Circle;
impl Graphic for Circle {
    fn draw(&self) {
        println!("Drawing a circle");
    }
}
pub struct Square;
impl Graphic for Square {
    fn draw(&self) {
        println!("Drawing a square");
    }
}
// Step 3: Composite object
pub struct GraphicGroup {
    graphics: Vec<Box<dyn Graphic>>,
}
impl GraphicGroup {
    pub fn new() -> Self {
        GraphicGroup {
            graphics: Vec::new(),
        }
    }
    pub fn add(&mut self, graphic: Box<dyn Graphic>) {
        self.graphics.push(graphic);
    }
}
impl Graphic for GraphicGroup {
    fn draw(&self) {
        for graphic in &self.graphics {
            graphic.draw();
        }
    }
}
fn main() {
    let circle1 = Circle;
    let square1 = Square;
    let mut group1 = GraphicGroup::new();
    group1.add(Box::new(circle1));
    group1.add(Box::new(square1));
    let circle2 = Circle;
    let mut group2 = GraphicGroup::new();
    group2.add(Box::new(circle2));
    group2.add(Box::new(group1));
    group2.draw();
}
```



* [Composite refactoring.guru](https://refactoring.guru/design-patterns/composite)
* [Composite www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2023/10/composite-design-pattern-in-rust.html)

---

</details>

<br>
<details>
<summary><b>Proxy</b></summary>
 
`Pattern Proxy` позволяет подставлять вместо реальных объектов специальные объекты-заменители. 
Эти объекты перехватывают вызовы к оригинальному объекту, позволяя сделать что-то до или после передачи вызова оригиналу.

Представьте, что у вас есть дорогостоящий объект, который потребляет много ресурсов при создании, и вы хотите создавать его только в случае крайней необходимости. Как можно отложить создание экземпляра или контролировать доступ к дорогостоящему объекту?

Применение:

1. Ленивая инициализация (виртуальный прокси). Когда у вас есть тяжёлый объект, грузящий данные из файловой системы или базы данных.
Вместо того, чтобы грузить данные сразу после старта программы, можно сэкономить ресурсы и создать объект тогда, когда он действительно понадобится.

2. Удаленный прокси-сервер предоставляет локального представителя объекта, который находится в другом адресном пространстве. 
Это то, что обеспечивает код-заглушка в RPC и CORBA.

3. Защитный прокси-сервер контролирует доступ к конфиденциальному главному объекту.
Защита доступа (защищающий прокси). Когда в программе есть разные типы пользователей, и вам хочется защищать объект от неавторизованного доступа. Например, если ваши объекты — это важная часть операционной системы, а пользователи — сторонние программы (хорошие или вредоносные).
Прокси может проверять доступ при каждом вызове и передавать выполнение служебному объекту, если доступ разрешён.

4. Умный прокси выполняет дополнительные действия при доступе к объекту. Типичное использование включает в себя:
Подсчет количества ссылок на реальный объект, чтобы его можно было автоматически освободить, когда ссылок больше нет (так называемый умный указатель),
Загрузка постоянного объекта в память при первом обращении к нему,
Проверка блокировки реального объекта перед доступом к нему, чтобы гарантировать, что никакой другой объект не сможет его изменить.

Эмпирические правила

`Pattern Decorator` и `Pattern Proxy` имеют схожие структуры, но разные назначения. Они похожи тем, что оба построены на принципе композиции и делегируют работу другим объектам. 
Паттерны отличаются тем, что `Pattern Proxy` сам управляет жизнью сервисного объекта, а обёртывание Декораторов контролируется клиентом.


Пример:

```rust
pub trait Server {
    fn handle_request(&mut self, url: &str, method: &str) -> (u16, String);
}
pub mod application {
    use super::Server;
    pub struct Application;
    impl Server for Application {
        fn handle_request(&mut self, url: &str, method: &str) -> (u16, String) {
            if url == "/app/status" && method == "GET" {
                return (200, "Ok".into());
            }
            if url == "/create/user" && method == "POST" {
                return (201, "User Created".into());
            }
            (404, "Not Ok".into())
        }
    }
}
use nginx::*;
pub mod nginx {
    use std::collections::HashMap;
    use super::{application::Application, Server};
    /// NGINX server is a proxy to an application server.
    pub struct NginxServer {
        application: Application,
        max_allowed_requests: u32,
        rate_limiter: HashMap<String, u32>,
    }
    impl NginxServer {
        pub fn new() -> Self {
            Self {
                application: Application,
                max_allowed_requests: 2,
                rate_limiter: HashMap::default(),
            }
        }
        pub fn check_rate_limiting(&mut self, url: &str) -> bool {
            let rate = self.rate_limiter.entry(url.to_string()).or_insert(1);
            if *rate > self.max_allowed_requests {
                return false;
            }
            *rate += 1;
            true
        }
    }
    impl Server for NginxServer {
        fn handle_request(&mut self, url: &str, method: &str) -> (u16, String) {
            if !self.check_rate_limiting(url) {
                return (403, "Not Allowed".into());
            }
            self.application.handle_request(url, method)
        }
    }
}
fn main() {
    let app_status = &"/app/status".to_string();
    let create_user = &"/create/user".to_string();
    let mut nginx = NginxServer::new();
    let (code, body) = nginx.handle_request(app_status, "GET");
    println!("Url: {}\nHttpCode: {}\nBody: {}\n", app_status, code, body);
    let (code, body) = nginx.handle_request(app_status, "GET");
    println!("Url: {}\nHttpCode: {}\nBody: {}\n", app_status, code, body);
    let (code, body) = nginx.handle_request(app_status, "GET");
    println!("Url: {}\nHttpCode: {}\nBody: {}\n", app_status, code, body);
    let (code, body) = nginx.handle_request(create_user, "POST");
    println!("Url: {}\nHttpCode: {}\nBody: {}\n", create_user, code, body);
    let (code, body) = nginx.handle_request(create_user, "GET");
    println!("Url: {}\nHttpCode: {}\nBody: {}\n", create_user, code, body);
}
```

Еще пример:
```rust
#![allow(dead_code)]
#![allow(unused_mut)]

// Abstract subject
trait Subject {
    fn request(&self);
}
// Real subject
struct RealSubject;
impl Subject for RealSubject {
    fn request(&self) {
        println!("RealSubject: Handling the request.");
    }
}
// Proxy
struct Proxy {
    real_subject: Option<RealSubject>,
}
impl Proxy {
    pub fn new() -> Self {
        Proxy { real_subject: None }
    }
    // Lazily instantiate the real subject
    fn ensure_real_subject(&mut self) {
        if self.real_subject.is_none() {
            self.real_subject = Some(RealSubject);
        }
    }
}
impl Subject for Proxy {
    fn request(&self) {
        println!("Proxy: Checking preconditions before accessing RealSubject.");
        if let Some(real_subject) = &self.real_subject {
            real_subject.request();
        }
        println!("Proxy: Additional tasks after accessing RealSubject.");
    }
}
fn main() {
    let mut proxy = Proxy::new();
    proxy.request();
}
```



* [Proxy refactoring.guru](https://refactoring.guru/ru/design-patterns/proxy/rust/example)
* [Proxy www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2023/10/proxy-design-pattern-in-rust.html)

---

</details>

<br>
<details>
<summary><b>Flyweight</b></summary>
 
`Pattern Flyweight` (Приспособленец, Кэш, Легковес) позволяет вместить бóльшее количество объектов в отведённую оперативную память.

Так же он экономит память, разделяя общее состояние объектов между собой, вместо хранения одинаковых данных в каждом объекте.

Так же он позволяет экономить память, кешируя одинаковые данные, используемые в разных объектах.

Так же это шаблон, который помогает минимизировать использование памяти за счет совместного использования и повторного использования данных.

Неизменяемые данные объекта принято называть «внутренним состоянием». Все остальные данные — это «внешнее состояние».
`Pattern Flyweight` предлагает не хранить в классе внешнее состояние, а передавать его в те или иные методы через параметры.
Таким образом, одни и те же объекты можно будет повторно использовать в различных контекстах.
Но главное — понадобится гораздо меньше объектов, ведь теперь они будут отличаться только внутренним состоянием, а оно имеет не так много вариаций.

**Структура**

Вы всегда должны помнить о том, что Легковес применяется в программе, имеющей громадное количество одинаковых объектов.
Этих объектов должно быть так много, чтобы они не помещались в доступную оперативную память без ухищрений.
Паттерн разделяет данные этих объектов на две части — легковесы и контексты.

`Pattern Flyweight` содержит состояние, которое повторялось во множестве первоначальных объектов.
Один и тот же легковес можно использовать в связке со множеством контекстов.
Состояние, которое хранится здесь, называется внутренним, а то, которое он получает извне — внешним.

Контекст содержит «внешнюю» часть состояния, уникальную для каждого объекта. Контекст связан с одним из объектов-легковесов, хранящих оставшееся состояние.

Поведение оригинального объекта чаще всего оставляют в Легковесе, передавая значения контекста через параметры методов.
Тем не менее, поведение можно поместить и в контекст, используя легковес как объект данных.

Клиент вычисляет или хранит контекст, то есть внешнее состояние легковесов.
Для клиента легковесы выглядят как шаблонные объекты, которые можно настроить во время использования, передав контекст через параметры.

Фабрика легковесов управляет созданием и повторным использованием легковесов. Фабрика получает запросы, в которых указано желаемое состояние легковеса.
Если легковес с таким состоянием уже создан, фабрика сразу его возвращает, а если нет — создаёт новый объект.

**Эмпирические правила**

`Pattern Composite` часто совмещают с `Pattern Flyweight`, чтобы реализовать общие ветки дерева и сэкономить при этом память.

`Pattern Flyweight` объясняет, когда и как можно совместно использовать объекты `Pattern State`.


Пример:
```rust
#![allow(dead_code)]
#![allow(unused_variables)]

use std::any::Any;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use flyweight::FlyweightTree;
pub mod flyweight {
    use super::{Any, Factory, Hash, Hasher};

    #[derive(Clone)]
    pub struct FlyweightTree {
        color: (u8, u8, u8),
        texture: Vec<u8>,
    }
    impl FlyweightTree {
        pub fn new(color: (u8, u8, u8), texture: Vec<u8>) -> Self {
            Self { color, texture }
        }
        pub fn draw(&self, canvas: &impl Any, context: (i32, i32)) {
            println!(
                "draw: {}",
                Factory::generate_key_hash(&(&self.color, &self.texture))
            );
        }
    }
    impl Hash for FlyweightTree {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.color.hash(state);
            self.texture.hash(state);
        }
    }
}
// TODO: Owner FlyweightTree
struct Factory {
    flyweight_trees: HashMap<u64, Rc<FlyweightTree>>,
}
impl Factory {
    fn new() -> Self {
        Self {
            flyweight_trees: HashMap::new(),
        }
    }
    fn get_tree_type(&mut self, color: (u8, u8, u8), texture: Vec<u8>) -> Rc<FlyweightTree> {
        let key = Factory::generate_key_hash(&(&color, &texture));
        if self.flyweight_trees.contains_key(&key) {
            return Rc::clone(&self.flyweight_trees.get(&key).unwrap());
        } else {
            let obj = Rc::new(FlyweightTree::new(color, texture));
            self.flyweight_trees.insert(key, obj);
            return Rc::clone(&self.flyweight_trees.get(&key).unwrap());
        }
    }
    fn generate_key_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}
// Контекстный объект, из которого мы выделили легковес FlyweightTree
// В программе могут быть тысячи объектов Tree, так как накладные расходы на их хранение совсем небольшие — в
// памяти нужно держать всего три целых числа (две координаты и Rc ссылка).
use tree::Tree;
pub mod tree {
    use super::{Any, FlyweightTree, Rc};

    #[derive(Clone)]
    pub struct Tree {
        x: i32,
        y: i32,
        flyweight_tree: Rc<FlyweightTree>,
    }
    impl Tree {
        pub fn new(x: i32, y: i32, flyweight_tree: Rc<FlyweightTree>) -> Self {
            Self {
                x,
                y,
                flyweight_tree,
            }
        }
        pub fn draw(&self, canvas: &impl Any) {
            let context: (i32, i32) = (self.x, self.y);
            self.flyweight_tree.draw(canvas, context);
        }
    }
}
// Классы Tree и Forest являются клиентами Легковеса. При желании их можно слить в один класс, если вам не нужно
// расширять класс Tree далее.
struct Forest {
    trees: Vec<Tree>,
    factory: Factory,
}
impl Forest {
    fn new() -> Self {
        Self {
            trees: vec![],
            factory: Factory::new(),
        }
    }
    fn add_tree(&mut self, x: i32, y: i32, color: (u8, u8, u8), texture: Vec<u8>) {
        let tree_type: Rc<FlyweightTree> = self.factory.get_tree_type(color, texture);
        let tree: Tree = Tree::new(x, y, tree_type);
        self.trees.push(tree);
    }
    fn draw(&self, canvas: &impl Any) {
        for tree in self.trees.iter() {
            tree.draw(canvas);
        }
    }
}
fn main() {
    let (x1, y1, color1, texture1) = (5, 4, (255, 0, 0), vec![1, 2, 3]);
    let (x2, y2, color2, texture2) = (1, 4, (255, 255, 0), vec![3, 3, 3]);
    let mut forest = Forest::new();
    forest.add_tree(x1, y1, color1, texture1.clone());
    forest.add_tree(x1, y1, color1, texture1.clone());
    forest.add_tree(x1, y1, color1, texture1.clone());
    forest.add_tree(x2, y2, color2, texture2.clone());
    forest.add_tree(x2, y2, color2, texture2.clone());
    forest.draw(&true);
}
```

Еще пример:
```rust
// Intrinsic state
struct TreeType {
    name: String,
    texture: String,
}
impl TreeType {
    pub fn new(name: &str, texture: &str) -> Self {
        TreeType {
            name: name.to_string(),
            texture: texture.to_string(),
        }
    }
    pub fn draw(&self, x: i32, y: i32) {
        println!(
            "Drawing {} at ({}, {}) with texture {}",
            self.name, x, y, self.texture
        );
    }
}
// Flyweight factory
struct TreeFactory {
    tree_types: Vec<TreeType>,
}
impl TreeFactory {
    pub fn new() -> Self {
        TreeFactory {
            tree_types: Vec::new(),
        }
    }
    pub fn get_tree_type(&mut self, name: &str, texture: &str) -> &TreeType {
        if self
            .tree_types
            .iter()
            .any(|t| t.name == name && t.texture == texture)
        {
            self.tree_types
                .iter()
                .find(|t| t.name == name && t.texture == texture)
                .unwrap()
        } else {
            let tree_type = TreeType::new(name, texture);
            self.tree_types.push(tree_type);

            return self.tree_types.last().unwrap();
        }
    }
}
fn main() {
    let mut factory = TreeFactory::new();
    let tree_type1: &TreeType = factory.get_tree_type("Pine", "PineTexture");
    tree_type1.draw(10, 20);
    tree_type1.draw(50, 60);
    let tree_type2: &TreeType = factory.get_tree_type("Oak", "OakTexture");
    tree_type2.draw(30, 40);
}
```


* [Flyweight refactoring.guru](https://refactoring.guru/ru/design-patterns/flyweight)
* [Flyweight + картинка](https://github.com/fadeevab/design-patterns-rust/blob/main/structural/flyweight/)

 
---

</details>

 
## <ins>Паттерны поведения</ins>

Эта группа паттернов позволяет структурировать подходы к обработке поведения и взаимодействия объектов. Проще говоря, как должны проходить процессы в которых существует несколько вариантов протекания событий.



<br>
<details>
<summary><b>Command</b></summary>
 

`Pattern Command` — это поведенческий паттерн проектирования, который превращает запросы в объекты, 
позволяя передавать их как аргументы при вызове методов, ставить запросы в очередь, логировать их, 
а также поддерживать отмену операций.

Когда использовать паттерн команды:

- Очередь. Когда запросы необходимо обрабатывать в определенные моменты времени и в соответствии с различными триггерными ситуациями.

- Слои. Когда необходимо разделить клиента и поставщика услуг (инкапсуляция получателя, для вызывающего не важно какая команда будет послана)

- Когда возникает необходимость в функции отката для определенных операций

- Когда необходима история запросов

- Когда есть необходимость добавить новые команды

- При необходимости параметризации объектов по действию

Мы хотим, чтобы эти действия или команды выполнялись или вызывались в определенном порядке позже и в другое время. 
Эти команды также могут быть вызваны в результате какого-либо события. 
Например, когда пользователь нажимает кнопку или при получении пакета данных. 
Кроме того, эти команды могут быть отменены. 
Это может оказаться полезным для работы редактора. 
Возможно, нам захочется хранить журналы выполненных команд, чтобы можно было повторно применить изменения позже, если система выйдет из строя.


Код с проблемами... 
Пользовательский слой (интерфейс) жестко зависит от бизнес слоя, напрямую используя его API (`add_str,read_str`). 
При изменении бизнес логики потребуется все вызовы в интерфейсе менять.
Так же, нет возможности фильтровать запросы или выполнять с задержкой, смотреть историю и воспроизводить последовательность дейсвий.
Вообщем, нет сохраненной истории запросов.

```rust
use std::fmt::Display;
use std::env;
 
pub mod business_layer {
    use std::io::{Write,Read};
    use std::fs::OpenOptions;
    use std::path::Path;
    use super::*;

    pub fn add_str<M: AsRef<str>, P: AsRef<Path> + Display>(msg: M, file: P) -> std::io::Result<()> {
        let display = format!("{}",file.as_ref().display());
        let path = Path::new(&display);
        let mut f = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)?;
        f.write_fmt(format_args!("{}\n",msg.as_ref()))?; 
        Ok(())
    }
    pub fn read_str<P: AsRef<Path>>(file: P) -> std::io::Result<()> {
        let display = format!("{}",file.as_ref().display());
        let path = Path::new(&display);
        let mut f = OpenOptions::new()
            .read(true)
            .open(path)?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
        let _ = std::io::stdout().write(contents.as_bytes());
        Ok(())
    }
}

// cargo run --bin command add 'msg of args' data.txt -- --bin main_problem  
// cargo run --bin command read data.txt -- --bin main_problem  
fn args_user_layer(args:Vec<String>) -> bool{
    if args.len() > 3{
        let action = &args[1];
        match action.as_str() {
            "add" => {
                let value = &args[2];
                let file = &args[3];
                let _ = business_layer::add_str(value,file);
                return true;
            },
            _ => {return false;}
        }
    } else if args.len() > 2 {
        let action = &args[1];
        match action.as_str() {
            "read" => {
                let file = &args[2];
                let _ = business_layer::read_str(file);
                return true;
            },
            _ => {return false;}
        }
    }
    false
}
// ENV_VAR_ACTION="add" ENV_VAR_VALUE='msg of env' ENV_VAR_FILE='data.txt' cargo run --bin command -- --bin main_problem 
// ENV_VAR_ACTION="read" ENV_VAR_FILE='data.txt' cargo run --bin command -- --bin main_problem
fn env_user_layer() -> bool{
    if let Ok(action) = env::var("ENV_VAR_ACTION"){
        match action.as_str() {
            "read" => {
                if let Ok(file) = env::var("ENV_VAR_FILE"){
                    let _ = business_layer::read_str(file);
                    return true;
                } 
            },
            "add" => {
                if let Ok(value) = env::var("ENV_VAR_VALUE"){
                    if let Ok(file) = env::var("ENV_VAR_FILE"){
                        let _ = business_layer::add_str(value,file);
                        return true;
                    }
                } 
            },
            _ => {return false;}
        }
    }
    false
}
fn main(){
    env_user_layer();
    let args: Vec<String> = env::args().collect();
    args_user_layer(args);
}
```

Пример:
Существует графический интерфейс с кнопками сохранения своих данных.
Проблема возникает если мы в слое пользовательского интерфейса будем реализовывать зависящий код от слоя бизнес-логики.
Тогда при изменении бизнес-логики придется переписывать слой пользовательского интерфейса, при чем во всех местах вызова.
Таким образом, команды станут гибкой прослойкой между пользовательским интерфейсом и бизнес-логикой.
И это лишь малая доля пользы, которую может принести паттерн Команда!

Решение:
Всю логику вынести в бизнес слой и унифицировать все вызова к интерфейсу команды.

```rust
use business_layer::Command;
use std::env;
pub mod business_layer {
    pub use base::Command;
    pub use base::MacroCommand;
    pub use command_add_str::CAddStr;
    pub use command_read::CRead;
    mod base {
        pub trait Command {
            fn execute(&self);
        }
        pub struct MacroCommand {
            stack: Vec<Box<dyn Command>>,
        }
        impl MacroCommand {
            pub fn new() -> MacroCommand {
                MacroCommand { stack: Vec::new() }
            }
            pub fn push(&mut self, cmd: Box<dyn Command>) {
                self.stack.push(cmd);
            }
            pub fn undo(&mut self) {
                self.stack.pop();
            }
            pub fn clear(&mut self) {
                self.stack.clear();
            }
            pub fn size(&self) -> usize {
                self.stack.len()
            }
        }
        impl Command for MacroCommand {
            fn execute(&self) {
                for command in &self.stack {
                    command.execute();
                }
            }
        }
    }
    mod command_read {
        use std::fmt::Display;
        use std::fs::OpenOptions;
        use std::io::{Read, Write};
        use std::path::Path;
        use super::*;
        pub struct CRead<P> {
            pub file: P,
        }
        impl<P: AsRef<Path> + Display> base::Command for CRead<P> {
            fn execute(&self) {
                let _ = self.read();
            }
        }
        impl<P: AsRef<Path> + Display> CRead<P> {
            pub fn new(file: P) -> Self {
                Self { file }
            }
            pub fn read(&self) -> std::io::Result<()> {
                println!("Read");
                let display = format!("{}", self.file.as_ref().display());
                let path = Path::new(&display);
                let mut f = OpenOptions::new().read(true).open(path)?;
                let mut contents = String::new();
                f.read_to_string(&mut contents)?;
                let _ = std::io::stdout().write(contents.as_bytes());
                Ok(())
            }
        }
    }
    mod command_add_str {
        use std::fmt::Display;
        use std::fs::OpenOptions;
        use std::io::Write;
        use std::path::Path;
        use super::*;
        pub struct CAddStr<M, P> {
            pub file: P,
            pub msg: M,
        }
        impl<M: AsRef<str>, P: AsRef<Path> + Display> base::Command for CAddStr<M, P> {
            fn execute(&self) {
                let _ = self.add_str();
            }
        }
        impl<M: AsRef<str>, P: AsRef<Path> + Display> CAddStr<M, P> {
            pub fn new(file: P, msg: M) -> Self {
                Self { file, msg }
            }
            pub fn add_str(&self) -> std::io::Result<()> {
                println!("Add");
                let display = format!("{}", self.file.as_ref().display());
                let path = Path::new(&display);
                let mut f = OpenOptions::new().append(true).create(true).open(path)?;
                f.write_fmt(format_args!("{}\n", self.msg.as_ref()))?;
                Ok(())
            }
        }
    }
}
// cargo run --bin command add 'msg of args' data.txt
// cargo run --bin command read data.txt
fn args_user_layer(history: &mut business_layer::MacroCommand, args: Vec<String>) -> bool {
    if args.len() > 3 {
        let action = &args[1];
        match action.as_str() {
            "add" => {
                let msg = args[2].to_owned();
                let file = args[3].to_owned();

                let cmd = Box::new(business_layer::CAddStr::new(file, msg));
                history.push(cmd);
                return true;
            }
            _ => {
                return false;
            }
        }
    } else if args.len() > 2 {
        let action = &args[1];
        match action.as_str() {
            "read" => {
                let file = args[2].to_owned();
                let cmd = Box::new(business_layer::CRead::new(file));
                history.push(cmd);
                return true;
            }
            _ => {
                return false;
            }
        }
    }
    false
}
// ENV_VAR_ACTION="add" ENV_VAR_VALUE='msg of env' ENV_VAR_FILE='data.txt' cargo run --bin command
// ENV_VAR_ACTION="read" ENV_VAR_FILE='data.txt' cargo run --bin command
fn env_user_layer(history: &mut business_layer::MacroCommand) -> bool {
    if let Ok(action) = env::var("ENV_VAR_ACTION") {
        match action.as_str() {
            "read" => {
                if let Ok(file) = env::var("ENV_VAR_FILE") {
                    let cmd = Box::new(business_layer::CRead::new(file));
                    history.push(cmd);
                    return true;
                }
            }
            "add" => {
                if let Ok(msg) = env::var("ENV_VAR_VALUE") {
                    if let Ok(file) = env::var("ENV_VAR_FILE") {
                        let cmd = Box::new(business_layer::CAddStr::new(file, msg));
                        history.push(cmd);
                        return true;
                    }
                }
            }
            _ => {
                return false;
            }
        }
    }
    false
}
fn main() {
    let mut history = business_layer::MacroCommand::new();
    env_user_layer(&mut history);
    let args: Vec<String> = env::args().collect();
    args_user_layer(&mut history, args);
    // + Своя логика обработки накопившихся команд
    if history.size() > 0 {
        history.execute();
        history.clear();
    }
}
```


Еще пример:
```rust
trait Command {
    fn execute(&self);
}
struct MacroCommand {
    stack: Vec<Box<Command>>,
}
impl MacroCommand {
    fn new() -> MacroCommand {
        MacroCommand { stack: Vec::new() }
    }
    fn push(&mut self, cmd: Box<Command>) {
        self.stack.push(cmd);
    }
    fn undo(&mut self) {
        self.stack.pop();
    }
    fn clear(&mut self) {
        self.stack.clear();
    }
}
impl Command for MacroCommand {
    fn execute(&self) {
        for command in &self.stack {
            command.execute();
        }
    }
}
struct DrawCommand {
    drawable: Box<Drawable>,
    x: u32,
    y: u32,
}
impl DrawCommand {
    fn new(drawable: Box<Drawable>, x: u32, y: u32) -> DrawCommand {
        DrawCommand {
            drawable: drawable,
            x: x,
            y: y,
        }
    }
}
impl Command for DrawCommand {
    fn execute(&self) {
        self.drawable.draw(self.x, self.y);
    }
}

trait Drawable {
    fn draw(&self, x: u32, y: u32);
}
#[derive(Clone)]
struct DrawCanvas {}
impl DrawCanvas {
    fn new() -> DrawCanvas {
        DrawCanvas {}
    }
}
impl Drawable for DrawCanvas {
    fn draw(&self, x: u32, y: u32) {
        println!("draw(x:{}, y:{})", x, y);
    }
}
fn main() {
    let mut history = MacroCommand::new();
    let canvas = Box::new(DrawCanvas::new());
    // TODO
    let cmd1 = Box::new(DrawCommand::new(canvas.clone(), 1, 1));
    let cmd2 = Box::new(DrawCommand::new(canvas.clone(), 2, 2));
    history.push(cmd1);
    history.push(cmd2);
    println!("----------");
    history.execute();
    println!();
    println!("---undo---");
    history.undo();
    history.execute();
    println!();
    println!("---clear---");
    history.clear();
    history.execute();
}
/*
----------
draw(x:1, y:1)
draw(x:2, y:2)

---undo---
draw(x:1, y:1)

---clear---

*/
```

Еще пример:
```rust
pub trait Migration {
    fn execute(&self) -> &str;
    fn rollback(&self) -> &str;
}
pub struct CreateTable;
impl Migration for CreateTable {
    fn execute(&self) -> &str {
        "create table"
    }
    fn rollback(&self) -> &str {
        "drop table"
    }
}
pub struct AddField;
impl Migration for AddField {
    fn execute(&self) -> &str {
        "add field"
    }
    fn rollback(&self) -> &str {
        "remove field"
    }
}
struct Schema {
    commands: Vec<Box<dyn Migration>>,
}
impl Schema {
    fn new() -> Self {
        Self { commands: vec![] }
    }
    fn add_migration(&mut self, cmd: Box<dyn Migration>) {
        self.commands.push(cmd);
    }
    fn execute(&self) -> Vec<&str> {
        self.commands.iter().map(|cmd| cmd.execute()).collect()
    }
    fn rollback(&self) -> Vec<&str> {
        self.commands
            .iter()
            .rev() // reverse iterator's direction
            .map(|cmd| cmd.rollback())
            .collect()
    }
}
fn main() {
    let mut schema = Schema::new();
    let cmd = Box::new(CreateTable);
    schema.add_migration(cmd);
    let cmd = Box::new(AddField);
    schema.add_migration(cmd);
    assert_eq!(vec!["create table", "add field"], schema.execute());
    assert_eq!(vec!["remove field", "drop table"], schema.rollback());
}
```


Еще пример:
```rust
// Command trait representing the command interface
trait Command {
    fn execute(&mut self);
}
// Concrete Command
struct LightOnCommand {
    light: Light,
}
impl LightOnCommand {
    pub fn new(light: Light) -> Self {
        LightOnCommand { light }
    }
}
impl Command for LightOnCommand {
    fn execute(&mut self) {
        self.light.turn_on();
    }
}
struct Light {
    is_on: bool,
}
impl Light {
    pub fn new() -> Self {
        Light { is_on: false }
    }
    pub fn turn_on(&mut self) {
        self.is_on = true;
        println!("Light is ON");
    }
}
// Invoker
struct RemoteControl {
    command: Box<dyn Command>,
}
impl RemoteControl {
    pub fn new(command: Box<dyn Command>) -> Self {
        RemoteControl { command }
    }
    pub fn press_button(&mut self) {
        self.command.execute();
    }
}
fn main() {
    let light = Light::new();
    let light_on = LightOnCommand::new(light);
    let mut remote = RemoteControl::new(Box::new(light_on));
    remote.press_button();
}
```



* [Command refactoring.guru](https://refactoring.guru/ru/design-patterns/command)
* [Command rust-unofficial.github.io](https://rust-unofficial.github.io/patterns/patterns/behavioural/command.html)
* [Command www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2023/10/command-design-pattern-in-rust.html)

---

 **Command + Composite** - Компоновка команд в блоки

 **Command + Composite + Chain Of Responsibilities** - Компоновка команд в блоки и делегирование другим командам

 **Command invoker** - Command и receiver исполнитель назначение команды и выполнение ее исполнителем

---

</details>

<br>
<details>
<summary><b>Interpreter</b></summary>
 
С помощью классов создать свой язык для гибкости и простоты использования пользователем.

Interpreter - поведенческий шаблон проектирования, решающий часто встречающуюся, но подверженную изменениям, задачу.
Также известен как Little (Small) Language, позволяет создать **свой язык** для гибкости и простоты использования пользователем
Сравнить данный паттерн можно с тем, как вы закладываете часто используемые действия в сокращенный набор слов, чтобы сам «интерпретатор» потом превратил этот набор в более комплексные осмысленные действия.

Шаблон интерпретатора используется для определения грамматического представления языка и предоставляет интерпретатор для работы с этой грамматикой.
Он предоставляет механизм интерпретации и оценки языковых выражений путем определения грамматики и интерпретатора.
Эффективно используя шаблон «Интерпретатор», вы можете добиться расширяемости, модульности кода и гибкости языковой интерпретации,
что в конечном итоге повышает функциональность и удобство использования ваших приложений языковой обработки.

Интерпретатор полагается на рекурсивный обход шаблона Composite для интерпретации «предложений», которые его просят обработать.

Эмпирические правила

Если рассматривать его в наиболее общей форме (т.е. операцию, распределенную по иерархии классов на основе шаблона Composite),
почти каждое использование шаблона Composite будет также содержать шаблон Interpreter.
Но шаблон Интерпретатора следует зарезервировать для тех случаев, когда вы хотите рассматривать эту иерархию классов как определение языка.
Interpreter может использовать State для определения контекстов синтаксического анализа.
Абстрактное синтаксическое дерево Интерпретатора является составным (поэтому также применимы Iterator и Visitor).
Terminal Expression в абстрактном синтаксическом дереве Interpreter могут использоваться совместно с Flyweight.
Шаблон не касается синтаксического анализа. Когда грамматика очень сложна, более подходящими являются другие методы (например, синтаксический анализатор).


Interpreter предоставляет механизм интерпретации и оценки языковых выражений путем определения грамматики и интерпретатора.
Шаблон представляет элементы языка как классы и использует рекурсивные алгоритмы для вычисления выражений.
Это способствует расширяемости и позволяет добавлять новые грамматические правила или языковые конструкции.

Design components:

Abstract Expression - объявляет операцию interpret(), которую переопределяют все узлы (терминальные и нетерминальные) в AST.
    класс абстрактного выражения определяет абстрактный интерфейс для языковых выражений. Он объявляет interpret()
    метод, определяющий логику интерпретации выражений.

Terminal Expression - (NumberExpression): реализует операцию interpret() для терминальных выражений.
    классы терминальных выражений представляют собой элементарные строительные блоки языка. Они реализуют интерфейс
    абстрактных выражений и обеспечивают логику интерпретации терминальных выражений.

Non-Terminal Expression - (AdditionExpression, SubtractionExpression, and MultiplicationExpression):
    реализует операцию interpret() для всех нетерминальных выражений.
    классы нетерминальных выражений представляют собой составные выражения, состоящие из нескольких подвыражений.
    Они также реализуют интерфейс абстрактных выражений и обеспечивают логику интерпретации составных выражений.

Context - содержит информацию, которая является глобальной для интерпретатора.
    класс контекста предоставляет любую необходимую информацию или состояние, необходимое для интерпретации выражений.
    Он сохраняет глобальную информацию, разделяемую между выражениями во время интерпретации.

Client - (ExpressionParser): строит (или предоставляет) AST, собранный из TerminalExpression и NonTerminalExpression.
    Клиент вызывает операцию interpret()

![Interpreter](/Computer-Science-Bookshelf/img/software_architecture/patterns/Interpreter.png "Interpreter")

Пример:
```rust
/// Интерпретатор asm синтаксиса
use dummy_interpreter_asm::*;
mod dummy_interpreter_asm {
    use std::collections::HashMap;
    // Terminal Expressions
    // обеспечивает логику интерпретации числового значения
    #[derive(Clone, Debug)]
    pub enum TerminalTypeExpressions {
        Number(usize),
        Str(String),
    }
    #[derive(Debug)]
    pub struct Context {
        stack: Vec<TerminalTypeExpressions>,
        map: HashMap<String, TerminalTypeExpressions>,
    }
    impl Context {
        pub fn new() -> Self {
            Self {
                stack: vec![],
                map: HashMap::new(),
            }
        }
        fn head(&mut self) -> Option<TerminalTypeExpressions> {
            if self.stack.len() > 0 {
                return self.stack.pop();
            }
            None
        }
        fn add(&mut self, t: TerminalTypeExpressions) {
            self.stack.push(t);
        }
        fn size(&self) -> usize {
            self.stack.len()
        }
        fn get_variable(&self, var_name: &str) -> Option<TerminalTypeExpressions> {
            if self.map.contains_key(var_name) {
                return Some(self.map.get(var_name).unwrap().clone());
            }
            None
        }
        fn create_variable(&mut self, var_name: &str, val: TerminalTypeExpressions) {
            self.map.insert(var_name.to_string(), val);
        }
    }
    // Abstract Expression
    pub trait AbstractExpression {
        fn interpret(&self, ctx: &mut Context);
    }
    // Non-Terminal Expression
    // предоставляет логику интерпретации для инструкции внесения в стек данных
    pub struct LoadDataExpression {
        data: TerminalTypeExpressions,
    }
    impl LoadDataExpression {
        pub fn new(data: TerminalTypeExpressions) -> Self {
            Self { data }
        }
    }
    impl AbstractExpression for LoadDataExpression {
        fn interpret(&self, ctx: &mut Context) {
            ctx.add(self.data.clone());
        }
    }
    // Non-Terminal Expression
    // предоставляет логику интерпретации для выражения присвоения переменной
    pub struct WriteVarExpression {
        data: TerminalTypeExpressions,
    }
    impl WriteVarExpression {
        pub fn new(data: TerminalTypeExpressions) -> Self {
            Self { data }
        }
    }
    impl AbstractExpression for WriteVarExpression {
        fn interpret(&self, ctx: &mut Context) {
            match self.data {
                TerminalTypeExpressions::Str(ref key) => {
                    if let Some(b) = ctx.head() {
                        match b {
                            TerminalTypeExpressions::Number(_) => {
                                ctx.create_variable(key, b);
                            }
                            TerminalTypeExpressions::Str(_) => {
                                ctx.create_variable(key, b);
                            }
                        }
                    } else {
                        panic!("WriteVarExpression:stack empty")
                    }
                }
                _ => {
                    panic!("WriteVarExpression:key must be a string")
                }
            }
        }
    }
    // Non-Terminal Expression
    // предоставляет логику интерпретации для выгрузки переменных в стек
    pub struct ReadVarExpression {
        data: TerminalTypeExpressions,
    }
    impl ReadVarExpression {
        pub fn new(data: TerminalTypeExpressions) -> Self {
            Self { data }
        }
    }
    impl AbstractExpression for ReadVarExpression {
        fn interpret(&self, ctx: &mut Context) {
            match self.data {
                TerminalTypeExpressions::Str(ref key) => {
                    if let Some(val) = ctx.get_variable(key) {
                        ctx.add(val);
                    } else {
                        panic!("ReadVarExpression:stack empty");
                    }
                }
                _ => {
                    panic!("ReadVarExpression:key must be a string")
                }
            }
        }
    }
    // Non-Terminal Expression
    // предоставляет логику интерпретации для выражения сложения
    pub struct AddExpression;
    impl AbstractExpression for AddExpression {
        fn interpret(&self, ctx: &mut Context) {
            if ctx.size() > 1 {
                if let Some(left) = ctx.head() {
                    if let Some(right) = ctx.head() {
                        match (left, right) {
                            (
                                TerminalTypeExpressions::Str(s1),
                                TerminalTypeExpressions::Str(s2),
                            ) => {
                                ctx.add(TerminalTypeExpressions::Str(format!("{s1}{s2}")));
                            }
                            (
                                TerminalTypeExpressions::Number(n1),
                                TerminalTypeExpressions::Number(n2),
                            ) => {
                                let (res, is_err) = n1.overflowing_add(n2);
                                if is_err {
                                    panic!("AddExpression:overflowing add");
                                }
                                ctx.add(TerminalTypeExpressions::Number(res));
                            }
                            _ => {
                                panic!("AddExpression:data type mismatch");
                            }
                        }
                    }
                }
            } else {
                panic!("AddExpression:stack empty");
            }
        }
    }
    // Non-Terminal Expression
    // предоставляет логику интерпретации для выражения произведения
    pub struct MultiplyExpression;
    impl AbstractExpression for MultiplyExpression {
        fn interpret(&self, ctx: &mut Context) {
            if ctx.size() > 1 {
                if let Some(left) = ctx.head() {
                    if let Some(right) = ctx.head() {
                        match (left, right) {
                            (
                                TerminalTypeExpressions::Str(s1),
                                TerminalTypeExpressions::Str(s2),
                            ) => {
                                ctx.add(TerminalTypeExpressions::Str(format!("{s1}{s2}")));
                            }
                            (
                                TerminalTypeExpressions::Number(n1),
                                TerminalTypeExpressions::Number(n2),
                            ) => {
                                let (res, is_err) = n1.overflowing_mul(n2);
                                if is_err {
                                    panic!("Multiply:overflowing mul");
                                }
                                ctx.add(TerminalTypeExpressions::Number(res));
                            }
                            _ => {
                                panic!("Multiply:data type mismatch");
                            }
                        }
                    }
                }
            } else {
                panic!("Multiply:stack empty");
            }
        }
    }

    pub struct Client {
        client_stack: Vec<Box<dyn AbstractExpression>>,
        ctx: Context,
    }
    impl Client {
        pub fn new(ctx: Context) -> Self {
            Self {
                client_stack: Vec::new(),
                ctx,
            }
        }
        pub fn add(&mut self, exp: Box<dyn AbstractExpression>) {
            self.client_stack.push(exp);
        }
        pub fn interpret(&mut self) {
            for exp in self.client_stack.iter() {
                exp.interpret(&mut self.ctx);
            }
        }
        pub fn ret_value_val(&mut self, var_name: &str) -> Option<TerminalTypeExpressions> {
            self.ctx.get_variable(var_name)
        }
        pub fn ret_value_stack(&mut self) -> Option<TerminalTypeExpressions> {
            self.ctx.head()
        }
    }
}
// cargo run --bin interpreter
fn main() {
    /*
    let input ="
        LOAD DATA 2
        WRITE VARIABLE ‘a’

        LOAD VALIABLE 3
        WRITE VARIABLE ‘b’

        READ VARIABLE ‘a’
        LOAD VALIABLE 1
        ADD

        READ VARIABLE ‘b’
        MULTIPLY

        RETURN_VALUE";
    */

    // TODO: ради удобства, все инструкции упаковать в enum и реализовать FromStr в Expressions
    let ctx: Context = Context::new();
    let mut client: Client = Client::new(ctx);

    let load_exp = LoadDataExpression::new(TerminalTypeExpressions::Number(2));
    client.add(Box::new(load_exp));

    let a = TerminalTypeExpressions::Str(String::from("a"));
    let write_exp = WriteVarExpression::new(a.clone());
    client.add(Box::new(write_exp));

    let load_exp = LoadDataExpression::new(TerminalTypeExpressions::Number(3));
    client.add(Box::new(load_exp));
    let b = TerminalTypeExpressions::Str(String::from("b"));
    let write_exp = WriteVarExpression::new(b.clone());
    client.add(Box::new(write_exp));

    let read_exp = ReadVarExpression::new(a.clone());
    client.add(Box::new(read_exp));
    let load_exp = LoadDataExpression::new(TerminalTypeExpressions::Number(1usize));
    client.add(Box::new(load_exp));
    let add_exp = AddExpression;
    client.add(Box::new(add_exp));

    let read_exp = ReadVarExpression::new(b);
    client.add(Box::new(read_exp));
    let mul_exp = MultiplyExpression;
    client.add(Box::new(mul_exp));

    client.interpret();

    println!("stack:{:?}", client.ret_value_stack());
    println!("a:{:?}", client.ret_value_val("a"));
    println!("b:{:?}", client.ret_value_val("b"));
}
```

Пример PHP:
```php
<?php
class Interpreter {  
    private $bookList;
    public function __construct($bookListIn) {
      $this->bookList = $bookListIn;
    }
    public function interpret($stringIn) {      
      $arrayIn = explode(" ",$stringIn);      
      $returnString = NULL;      
      // go through the array validating 
      // and if possible calling a book method
      // could use refactoring, some duplicate logic 
      if ('book' == $arrayIn[0]) {
        if ('author' == $arrayIn[1]) {
          if (is_numeric($arrayIn[2])) {
            $book = $this->bookList->getBook($arrayIn[2]);
            if (NULL == $book) {
              $returnString = 'Can not process, there is no book # '.$arrayIn[2]; 
            } else {
              $returnString = $book->getAuthor(); 
            }
          } elseif ('title' == $arrayIn[2]) {
            if (is_numeric($arrayIn[3])) {
              $book = $this->bookList->getBook($arrayIn[3]);
              if (NULL == $book) {
                $returnString = 'Can not process, there is no book # '.
                  $arrayIn[3]; 
              } else {
                $returnString = $book->getAuthorAndTitle(); 
              }
            } else {
              $returnString = 'Can not process, book # must be numeric.'; 
            }            
          } else {
            $returnString = 'Can not process, book # must be numeric.'; 
          }
        }
        if ('title' == $arrayIn[1]) {
          if (is_numeric($arrayIn[2])) {
            $book = $this->bookList->getBook($arrayIn[2]);
            if (NULL == $book) {
              $returnString = 'Can not process, there is no book # '.
                $arrayIn[2]; 
            } else {
              $returnString = $book->getTitle(); 
            }
          } else {
            $returnString = 'Can not process, book # must be numeric.'; 
          }
        }
      } else {
        $returnString = 'Can not process, can only process book author #,  book title #, or book author title #'; 
      }      
      return $returnString;  
    }
}

class BookList {
    private $books = array();
    private $bookCount = 0;
    public function __construct() {
    }
    public function getBookCount() {
        return $this->bookCount;
    }
    private function setBookCount($newCount) {
        $this->bookCount = $newCount;
    }
    public function getBook($bookNumberToGet) {
        if ( (is_numeric($bookNumberToGet)) && 
           ($bookNumberToGet <= $this->getBookCount())) {
           return $this->books[$bookNumberToGet];
        } else {
           return NULL;
        }
    }
    public function addBook(Book $book_in) {
        $this->setBookCount($this->getBookCount() + 1);
        $this->books[$this->getBookCount()] = $book_in;
        return $this->getBookCount();
    }
    public function removeBook(Book $book_in) {
      $counter = 0;
      while (++$counter <= $this->getBookCount()) {
        if ($book_in->getAuthorAndTitle() == 
          $this->books[$counter]->getAuthorAndTitle())
          {
            for ($x = $counter; $x < $this->getBookCount(); $x++) {
              $this->books[$x] = $this->books[$x + 1];
          }
          $this->setBookCount($this->getBookCount() - 1);
        }
      }
      return $this->getBookCount();
    }
}

class Book {
    private $author;
    private $title;
    function __construct($title_in, $author_in) {
        $this->author = $author_in;
        $this->title  = $title_in;
    }
    function getAuthor() {
        return $this->author;
    }
    function getTitle() {
        return $this->title;
    }
    function getAuthorAndTitle() {
        return $this->getTitle().' by '.$this->getAuthor();
    }
}

  writeln('BEGIN TESTING INTERPRETER PATTERN');
  writeln('');

  //load BookList for test data
  $bookList = new BookList();
  $inBook1 = new Book('PHP for Cats','Larry');
  $inBook2 = new Book('MySQL for Cats','Truett');
  $bookList->addBook($inBook1);
  $bookList->addBook($inBook2);
 
  $interpreter = new Interpreter($bookList);
 
  writeln('test 1 - invalid request missing "book"');
  writeln($interpreter->interpret('author 1'));
  writeln('');
 
  writeln('test 2 - valid book author request');
  writeln($interpreter->interpret('book author 1'));
  writeln('');

  writeln('test 3 - valid book title request');
  writeln($interpreter->interpret('book title 2'));
  writeln('');

  writeln('test 4 - valid book author title request');
  writeln($interpreter->interpret('book author title 1'));
  writeln('');

  writeln('test 5 - invalid request with invalid book number');
  writeln($interpreter->interpret('book title 3'));
  writeln('');

  writeln('test 6 - invalid request with nuo numeric book number');
  writeln($interpreter->interpret('book title one'));
  writeln('');

  writeln('END TESTING INTERPRETER PATTERN');
 
  function writeln($line_in) {
    echo $line_in."\n";
  }

?>
```

Еще пример:
```rust
#![allow(dead_code)]
// The Context keeps track of information that the interpreter needs
struct Context {
    input: String,
    output: i32,
}
// The Abstract Expression trait
trait Expression {
    fn interpret(&self, context: &mut Context);
}
// Terminal Expression
struct NumberExpression {
    number: i32,
}
impl NumberExpression {
    fn new(number: i32) -> Self {
        NumberExpression { number }
    }
}
impl Expression for NumberExpression {
    fn interpret(&self, context: &mut Context) {
        context.output = self.number;
    }
}
// Client Code
fn main() {
    let context = &mut Context {
        input: "Number: 5".to_string(),
        output: 0,
    };
    let number = NumberExpression::new(5);
    number.interpret(context);
    println!("Interpreted number: {}", context.output);
}
```

 
* [Interpreter sourcemaking.com](https://sourcemaking.com/design_patterns/interpreter)
* [Interpreter geeksforgeeks](https://www.geeksforgeeks.org/interpreter-design-pattern/)
* [Interpreter medium.com](https://medium.com/@rajeshvelmani/understanding-language-interpretation-with-the-interpreter-design-pattern-in-java-b2a3969eaf9)
* [Interpreter www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2023/10/interpreter-design-pattern-in-rust.html)
* [Abstract Syntax Tree, AST](https://ru.wikipedia.org/wiki/%D0%90%D0%B1%D1%81%D1%82%D1%80%D0%B0%D0%BA%D1%82%D0%BD%D0%BE%D0%B5_%D1%81%D0%B8%D0%BD%D1%82%D0%B0%D0%BA%D1%81%D0%B8%D1%87%D0%B5%D1%81%D0%BA%D0%BE%D0%B5_%D0%B4%D0%B5%D1%80%D0%B5%D0%B2%D0%BE). Абстрактное синтаксическое дерево (АСД, англ. abstract syntax tree, AST) — конечное помеченное ориентированное дерево,
в котором внутренние вершины сопоставлены (помечены) с операторами языка программирования, а листья — с соответствующими
операндами. Таким образом, листья являются пустыми операторами и представляют только переменные и константы.
* [Обработка древовидных структур и унифицированное AST](https://habr.com/ru/companies/pt/articles/210060/)

---

</details>



<br>
<details>
<summary><b>Strategy</b></summary>

Позволяет эффективно работать коду, благодаря наличию реализованных стратегий основываясь на входных данных.
Все стратегии реализуются по обшему интерфейсу и соответственно код становится гибким/взаимозаменяемым.
Это позволяет отказаться от использования переключателей и/или условных операторов.
К примеру выбор способа сортировки зависит от типа и размера данных, соответственно выбирая подходящий алгоритм мы используем паттерн стратегия.

Если используется правило, которое не подвержено изменениям, нет необходимости обращаться к `Pattern Strategy`.

Одно из преимуществ использования `Pattern Strategy` заключается в том, что мы можем избавиться от ветвления `if/else`.
Достигается это за счет того, что `Client` "знает",
какой алгоритм он хочет использовать и передает объект алгоритма в конструктор класса - `Context`. 
Согласно `Singl responsibility` разделяем классы на подклассы.

Один из принципов SOLID: open/close - предполагает сущность открыта для расширения но закрыта для модификаций. Таким образом при надобности расширить
поведение сущности мы просто добавляем еще одну стратегию, вместо изменения кода сущности.

Мотивы:
* Программа должна обеспечивать различные варианты алгоритма или поведения
* Нужно изменять поведение каждого экземпляра класса
* Необходимо изменять поведение объектов на стадии выполнения
* Введение интерфейса позволяет классам-клиентам ничего не знать о классах, реализующих этот интерфейс и инкапсулирующих в себе конкретные алгоритмы

Эмпирические правила:
- `Pattern Strategy` похожа на `Pattern Method`, за исключением степени детализации.
- `Pattern State` похож на `Pattern Strategy`, за исключением своего намерения.
- `Pattern State, Strategy, Bridge` (и в некоторой степени `Pattern Adapter`) имеют схожие структуры решений. 

Все они разделяют элементы идиомы «handle/body». Они различаются по назначению – то есть решают разные задачи.
- `Pattern Strategy` имеет две разные реализации, первая похожа на `Pattern State`. 
Разница заключается во времени привязки (`Pattern Strategy` — это шаблон с однократной привязкой, тогда как `Pattern State` более динамичен).
Объекты стратегии часто становятся хорошими легковесами.
- `Pattern Strategy` позволяет вам изменить внутренности объекта. `Pattern Decorator` позволяет менять скин.
И `Pattern Strategy`, и `Pattern Decorator` могут применяться для изменения поведения конкретных классов. 
Достоинство стратегии в том, что интерфейс кастомизации не совпадает с публичным интерфейсом и может быть куда более удобным, а недостаток в том, что для использования стратегии необходимо изначально проектировать класс с возможностью регистрации стратегий.

Состоит:
    - Strategy - абстрактная сущность 
    - ConcreteStrategy - конкретные реализации стратегии 
    - Context - содержит конкретную стратегию
    - Client - выбирает какую стратегию применять


Пример:
Представьте, что мы работаем над проектом, который генерирует отчеты каждый месяц.
Нам нужно, чтобы отчеты формировались в разных форматах (стратегиях), например в формате `JSON` или `Plain Text`.
Но со временем ситуация меняется, и мы не знаем, какие требования мы можем получить в будущем.
Например, нам может потребоваться создать отчет в совершенно новом формате или просто изменить один из существующих форматов.

Основное преимущество – разделение задач.
Например, в этом случае `Report` ничего не известно о конкретных реализациях `Jsonи Text`,
тогда как выходные реализации не заботятся о том, как данные предварительно обрабатываются, сохраняются и извлекаются.
Единственное, что им нужно знать, — это конкретный признак, который нужно реализовать,
и его метод, определяющий конкретную реализацию алгоритма, обрабатывающего результат.

```rust
use std::collections::HashMap;

type Data = HashMap<String, u32>;

// Strategy
trait Formatter {
    fn format(&self, data: &Data, buf: &mut String);
}
// ConcreteStrategy
struct Report;
impl Report {
    // Следует использовать trait Write, но мы используем String, чтобы игнорировать обработку ошибок.
    fn generate<T: Formatter>(g: T, s: &mut String) {
        // backend operations...
        let mut data = HashMap::new();
        data.insert("one".to_string(), 1);
        data.insert("two".to_string(), 2);
        // generate report
        g.format(&data, s);
    }
}
// ConcreteStrategy
struct Text;
impl Formatter for Text {
    fn format(&self, data: &Data, buf: &mut String) {
        for (k, v) in data {
            let entry = format!("{} {}\n", k, v);
            buf.push_str(&entry);
        }
    }
}
// ConcreteStrategy
struct Json;
impl Formatter for Json {
    fn format(&self, data: &Data, buf: &mut String) {
        buf.push('[');
        for (k, v) in data.into_iter() {
            let entry = format!(r#"{{"{}":"{}"}}"#, k, v);
            buf.push_str(&entry);
            buf.push(',');
        }
        if !data.is_empty() {
            buf.pop(); // remove extra, at the end
        }
        buf.push(']');
    }
}
fn main() {
    // Context/Client
    let mut s = String::from("");
    Report::generate(Text, &mut s);
    assert!(s.contains("one 1"));
    assert!(s.contains("two 2"));

    s.clear(); // reuse the same buffer
    Report::generate(Json, &mut s);
    assert!(s.contains(r#"{"one":"1"}"#));
    assert!(s.contains(r#"{"two":"2"}"#));
}
```

Еще пример:
```rust
// Strategy trait
trait CompressionStrategy {
    fn compress(&self, data: &str) -> String;
}
// Concrete Strategies
struct ZipCompression;
impl CompressionStrategy for ZipCompression {
    fn compress(&self, data: &str) -> String {
        format!("{} (ZIP compressed)", data)
    }
}
struct RarCompression;
impl CompressionStrategy for RarCompression {
    fn compress(&self, data: &str) -> String {
        format!("{} (RAR compressed)", data)
    }
}
// Context struct that uses a compression strategy
struct Compressor {
    strategy: Box<dyn CompressionStrategy>,
}
impl Compressor {
    fn new(strategy: Box<dyn CompressionStrategy>) -> Self {
        Compressor { strategy }
    }
    fn compress_data(&self, data: &str) -> String {
        self.strategy.compress(data)
    }
}
// Client Code
fn main() {
    let zip = ZipCompression;
    let rar = RarCompression;
    let mut compressor = Compressor::new(Box::new(zip));
    println!("{}", compressor.compress_data("MyData"));
    compressor = Compressor::new(Box::new(rar));
    println!("{}", compressor.compress_data("MyData"));
}
```





Ссылки:
* [Strategy rust-unofficial.github.io](https://rust-unofficial.github.io/patterns/patterns/behavioural/strategy.html)
* [Strategy sourcemaking.com](https://sourcemaking.com/design_patterns/strategy)
* [Strategy refactoring.guru](https://refactoring.guru/ru/design-patterns/strategy)
* [Strategy ru.wikipedia.org](https://ru.wikipedia.org/wiki/%D0%A1%D1%82%D1%80%D0%B0%D1%82%D0%B5%D0%B3%D0%B8%D1%8F_(%D1%88%D0%B0%D0%B1%D0%BB%D0%BE%D0%BD_%D0%BF%D1%80%D0%BE%D0%B5%D0%BA%D1%82%D0%B8%D1%80%D0%BE%D0%B2%D0%B0%D0%BD%D0%B8%D1%8F))
* [Strategy www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2023/10/strategy-design-pattern-in-rust.html)

---

</details>

<br>
<details>
<summary><b>Visitor</b></summary>

`Pattern Visitor` позволяет добавлять в программу новые операции, не изменяя разнородные классы объектов, 
над которыми эти операции могут выполняться.
(т.е. все струтуры или перечисления остаюся нетронутыми `...Industrial,Residential,Commercial,Build,LevelBuild`)

`Pattern Visitor` полезен везде, где вы хотите применить алгоритм к разнородным данным. 
Если данные однородны, просто применяем один метод. 
Использование объекта посетителя (а не функционального подхода) позволяет посетителю сохранять состояние и, 
таким образом, передавать информацию между узлами.

Т.е. в идеале мы бы могли имплементироваться от трейта и просто вызывая обший для всех метод решить свою задачу, но мы в силу каких-то
причин так не можем делать. 
Так-же, возможная причина, это неуместная логика в рамках этих структур или потенциальные изменения в новой требуемой логике.

`Pattern Fold` аналогичен `Pattern Visitor`, но создает новую версию посещенной структуры данных.

**Single-serving visitor** (одноразовый посетитель)

Является частным случаем использования `Pattern Visitor`. Если в случае с обычным «посетителем» у нас есть врач которого мы можем отправить к разным больным (и при желании по несколько раз), то в данном паттерне можно привести аналогию, что мы нанимаем врача, отправляем его к одному больному и после обследования сразу увольняем.

**Hierarchical visitor** (иерархический посетитель)

Тот же самый `Pattern Visitor`, однако в данном случае он отправляется к не одному больному, а в целую больницу и обходит там всех больных.

Пример:
Ваша команда разрабатывает приложение, работающее с геоданными в виде графа.
Узлами графа являются городские локации: памятники, театры, рестораны, важные предприятия и прочее.
Каждый узел имеет ссылки на другие, ближайшие к нему узлы.
Каждому типу узлов соответствует свой класс, а каждый узел представлен отдельным объектом.

Ваша задача — сделать экспорт этого графа в XML. Дело было бы плёвым, если бы вы могли редактировать классы узлов.
Достаточно было бы добавить метод экспорта в каждый тип узла, а затем, перебирая узлы графа, вызывать этот метод для каждого узла.
Благодаря полиморфизму, решение получилось бы изящным, так как вам не пришлось бы привязываться к конкретным классам узлов.
Но, к сожалению, классы узлов менять нельзя!

```rust
#![allow(dead_code)]
 
use std::any::Any;

// Узлы
// for trait-object variants
struct Industrial(String);
struct Residential(Vec<u8>);
struct Commercial([i32; 5]);

// Узлы
// for enum variants
use build::*;
mod build {
    pub enum Build {
        EIndustrial(Box<LevelBuild>),
        EResidential(Box<LevelBuild>),
        ECommercial(Box<LevelBuild>),
    }
    pub enum LevelBuild {
        EFrameLevel(u8),
        ENone,
    }
}
// Aggregator
struct CLientCity {
    fields_1: Vec<Box<dyn Any>>,
    fields_2: Vec<Build>,
}
// Необходимо имея неоднородную структуру типов, использовать конкретный метод для этого типа
// В случае с коллекцией можем использовать только trait objects и как следствие downcast
// Но в случае с enum просто matching
impl CLientCity {
    fn export_trait_objects(&self, export_visitor: impl VisitorForTraitObject) {
        for item in self.fields_1.iter() {
            if let Some(obj) = item.downcast_ref::<Industrial>() {
                export_visitor.for_industrial(obj);
            } else if let Some(obj) = item.downcast_ref::<Residential>() {
                export_visitor.for_residential(obj);
            } else if let Some(obj) = item.downcast_ref::<Commercial>() {
                export_visitor.for_commercial(obj);
            }
        }
    }
    fn export_enum(&self, export_visitor: impl VisitorForEnum) {
        for item in self.fields_2.iter() {
            match item {
                Build::EIndustrial(level) => {
                    export_visitor.for_industrial(item);
                    export_visitor.for_level(level);
                }
                Build::EResidential(level) => {
                    export_visitor.for_residential(item);
                    export_visitor.for_level(level);
                }
                Build::ECommercial(level) => {
                    export_visitor.for_commercial(item);
                    export_visitor.for_level(level);
                }
            }
        }
    }
}
//-------------------------------------------------------------------------------
trait VisitorForTraitObject {
    fn for_industrial(&self, obj: &Industrial);
    fn for_residential(&self, obj: &Residential);
    fn for_commercial(&self, obj: &Commercial);
}
struct ExportConcreteVisitor;
impl VisitorForTraitObject for ExportConcreteVisitor {
    fn for_industrial(&self, obj: &Industrial) {
        println!("trait-obj export for_industrial {}", obj.0);
    }
    fn for_residential(&self, obj: &Residential) {
        println!("trait-obj export for_residential {}", obj.0.len());
    }
    fn for_commercial(&self, obj: &Commercial) {
        println!("trait-obj export for_commercial {:?}", obj.0);
    }
}
//-------------------------------------------------------------------------------- 
trait VisitorForEnum {
    fn for_industrial(&self, _e: &Build);
    fn for_residential(&self, _e: &Build);
    fn for_commercial(&self, _e: &Build);
    fn for_level(&self, _e: &LevelBuild);
}
struct ExportConcreteVisitor2;
impl VisitorForEnum for ExportConcreteVisitor2 {
    fn for_industrial(&self, _e: &Build) {
        println!("enum export for_industrial");
    }
    fn for_residential(&self, _e: &Build) {
        println!("enum export for_residential");
    }
    fn for_commercial(&self, _e: &Build) {
        println!("enum export for_commercial");
    }
    fn for_level(&self, _e: &LevelBuild) {
        println!("enum export for_level");
    }
}
fn main() {
    let export_visitor = ExportConcreteVisitor;
    let export_visitor2 = ExportConcreteVisitor2;

    let fields_1: Vec<Box<dyn Any>> = {
        let some_city_object1 = Box::new(Industrial("Industrial".into()));
        let some_city_object2 = Box::new(Residential(vec![1]));
        let some_city_object3 = Box::new(Commercial([1, 2, 3, 4, 5]));
        vec![some_city_object1, some_city_object2, some_city_object3]
    };
    let fields_2: Vec<Build> = {
        vec![
            Build::EIndustrial(Box::new(LevelBuild::EFrameLevel(1))),
            Build::ECommercial(Box::new(LevelBuild::EFrameLevel(3))),
        ]
    };

    let nodes_city_object: CLientCity = CLientCity { fields_1, fields_2 };
    nodes_city_object.export_trait_objects(export_visitor);
    nodes_city_object.export_enum(export_visitor2);
}
```


Еще пример:
```rust
// Данные, которые мы не хотим изменять и поэтому будем их посещать
mod ast {
    pub enum Stmt {
        Expr(Expr),
        Let(Name, Expr),
    }
    pub struct Name {
        value: String,
    }
    pub enum Expr {
        IntLit(i64),
        Add(Box<Expr>, Box<Expr>),
        Sub(Box<Expr>, Box<Expr>),
    }
}
// The abstract visitor
// Интерфейс для всех типов используемых данных
mod visit {
    use super::*;
    use ast::*;

    pub trait Visitor<T> {
        fn visit_name(&mut self, n: &Name) -> T;
        fn visit_stmt(&mut self, s: &Stmt) -> T;
        fn visit_expr(&mut self, e: &Expr) -> T;
    }
}

use ast::*;
use visit::*;
// Пример конкретной реализации — обход AST, интерпретирующий его как код.
pub struct Interpreter;
impl Visitor<i64> for Interpreter {
    fn visit_name(&mut self, n: &Name) -> i64 {
        panic!()
    } // нет смысла String приводить к i64
    fn visit_stmt(&mut self, s: &Stmt) -> i64 {
        match *s {
            Stmt::Expr(ref e) => self.visit_expr(e),
            Stmt::Let(..) => unimplemented!(), // нет смысла String приводить к i64
        }
    }
    fn visit_expr(&mut self, e: &Expr) -> i64 {
        match *e {
            Expr::IntLit(n) => n,
            Expr::Add(ref lhs, ref rhs) => self.visit_expr(lhs) + self.visit_expr(rhs),
            Expr::Sub(ref lhs, ref rhs) => self.visit_expr(lhs) - self.visit_expr(rhs),
        }
    }
}
pub fn walk_expr(visitor: &mut Interpreter, e: &Expr) {
    match *e {
        Expr::IntLit(_) => {}
        Expr::Add(ref lhs, ref rhs) => {
            visitor.visit_expr(lhs);
            visitor.visit_expr(rhs);
        }
        Expr::Sub(ref lhs, ref rhs) => {
            visitor.visit_expr(lhs);
            visitor.visit_expr(rhs);
        }
    }
}
fn main() {
    let exp = Expr::Add(Box::new(Expr::IntLit(8)), Box::new(Expr::IntLit(3)));
    let mut interpreter = Interpreter;
    let res = interpreter.visit_expr(&exp);
    println!("{}", res);

    let mut interpreter = Interpreter;
    let exp = Expr::Add(Box::new(Expr::IntLit(8)), Box::new(Expr::IntLit(3)));
    walk_expr(&mut interpreter, &exp);
}
```

Еще пример:
```rust
// Element trait
trait Element {
    fn accept(&self, visitor: &dyn Visitor);
}
// Concrete elements
struct ElementA;
impl Element for ElementA {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_element_a(self);
    }
}
struct ElementB;
impl Element for ElementB {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_element_b(self);
    }
}
// Visitor trait
trait Visitor {
    fn visit_element_a(&self, element: &ElementA);
    fn visit_element_b(&self, element: &ElementB);
}
// Concrete visitor
struct ConcreteVisitor;
impl Visitor for ConcreteVisitor {
    fn visit_element_a(&self, _: &ElementA) {
        println!("Visited ElementA with ConcreteVisitor");
    }
    fn visit_element_b(&self, _: &ElementB) {
        println!("Visited ElementB with ConcreteVisitor");
    }
}
// Client code
fn main() {
    let elements: Vec<Box<dyn Element>> = vec![Box::new(ElementA), Box::new(ElementB)];
    let visitor = ConcreteVisitor;
    for element in &elements {
        element.accept(&visitor);
    }
}
```



* [Visitor refactoring.guru](https://refactoring.guru/ru/design-patterns/visitor)
* [Visitor sourcemaking.com](https://sourcemaking.com/design_patterns/visitor)
* [Visitor rust-unofficial.github.io](https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html)
* [Visitor habr.com](https://habr.com/ru/articles/332042/)
* [Visitor www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2023/10/visitor-design-pattern-in-rust.html)

---

</details>

<br>
<details>
<summary><b>Observer</b></summary>
 
`Pattern Observer` - полезен, когда вас интересует состояние объекта и вы хотите получать уведомления о любых изменениях.
Создаёт механизм подписки, позволяющий одним объектам следить и реагировать на события, происходящие в других объектах.

В `Pattern Observer` объект, который наблюдает за состоянием другого объекта, называется Observer,
а объект, за которым ведется наблюдение, называется Subject.

Определите зависимость «один ко многим» между объектами, чтобы при изменении состояния одного объекта
все его зависимые объекты автоматически уведомлялись и обновлялись.

Когда после изменения состояния одного объекта требуется что-то сделать в других, но вы не знаете наперёд, какие именно объекты должны отреагировать.

**Проблема**

Представьте, что вы имеете два объекта: Покупатель и Магазин. В магазин вот-вот должны завезти новый товар, который интересен покупателю.
Покупатель может каждый день ходить в магазин, чтобы проверить наличие товара. Но при этом он будет злиться, без толку тратя своё драгоценное время.
С другой стороны, магазин может разослать спам каждому своему покупателю. Многих это расстроит, так как товар специфический, и не всем он нужен.
Получается конфликт: либо покупатель тратит время на периодические проверки, либо магазин тратит ресурсы на бесполезные оповещения.

**Проблема**

Большая монолитная конструкция плохо масштабируется, поскольку предъявляются новые требования к графическому отображению или мониторингу.
Вообщем легче отправить сообщение нежели добраться в сложившейся иерархии классов до конкретного обьекта.

`Pattern Observer` предлагает хранить внутри объекта издателя `Observer` список ссылок на объекты подписчиков `Subject`, 
причём издатель не должен вести список подписки самостоятельно. 
Он предоставит методы, с помощью которых подписчики могли бы добавлять или убирать себя из списка.
Когда в издателе будет происходить важное событие, он будет проходиться по списку подписчиков и 
оповещать их об этом, вызывая определённый метод объектов-подписчиков.
Издателю безразлично, какой класс будет иметь тот или иной подписчик, так как все они должны следовать общему интерфейсу и иметь единый метод оповещения.

**Pattern Blackboard** (доска объявлений)

Данный паттерн служит для обеспечения взаимодействия между большим количеством объектов. Он является расширением `Pattern Observer` и позволяет централизованно обслуживать как «наблюдателей», так и «создателей событий». В аналогии подпиской на email уведомления, это будет сам сайт подписки, который обслуживает множество подписчиков и тех, кто для них создает информацию (сообщения).

Пример:
```rust
// Subject manages all Events
// Observer is called from Subject when Event occurs

// Event
trait Event {
    fn get_title(&self) -> &str;
}

#[derive(Debug, Clone)]
struct EventObject {
    title: String,
}
impl EventObject {
    fn new(title: String) -> EventObject {
        EventObject { title }
    }
}
impl Event for EventObject {
    fn get_title(&self) -> &str {
        &self.title
    }
}
// Subject
trait Subject<T: Clone> {
    fn notify_observers(&self, event: &T);
    fn register_observer(&mut self, observer: Box<dyn Observer<T>>) -> usize;
    fn unregister_observer(&mut self, index: usize);
}
struct ConcreteSubject {
    observers: Vec<(bool, Box<dyn Observer<EventObject>>)>,
}
impl ConcreteSubject {
    fn new() -> ConcreteSubject {
        ConcreteSubject {
            observers: Vec::new(),
        }
    }
}
impl Subject<EventObject> for ConcreteSubject {
    fn notify_observers(&self, event: &EventObject) {
        for observer in &self.observers {
            if observer.0 {
                observer.1.on_notify(event);
            }
        }
    }
    fn register_observer(&mut self, observer: Box<dyn Observer<EventObject>>) -> usize {
        self.observers.push((true, observer));
        self.observers.len() - 1
    }
    fn unregister_observer(&mut self, index: usize) {
        self.observers[index].0 = false;
    }
}
// Observer
trait Observer<T: Clone> {
    fn on_notify(&self, event: &T);
}
struct ConcreteObserver(usize);
impl Observer<EventObject> for ConcreteObserver {
    fn on_notify(&self, event: &EventObject) {
        println!(
            "ConcreteObserver {} gets event: [{}]",
            self.0,
            event.get_title()
        );
    }
}
fn main() {
    let mut subject = ConcreteSubject::new();
    subject.register_observer(Box::new(ConcreteObserver(1)));
    subject.register_observer(Box::new(ConcreteObserver(2)));
    subject.register_observer(Box::new(ConcreteObserver(3)));
    let event1 = EventObject::new(String::from("first event"));
    let event2 = EventObject::new(String::from("second event"));
    subject.notify_observers(&event1);
    subject.notify_observers(&event2);
}
```

Еще пример:
```rust
//use std::sync::{RwLock, RwLockWriteGuard};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;

//use std::borrow::{BorrowMut, Borrow}; не работает если подключить

// Есть вариант с каналами, оповещение подписчиков по каналу std::sync::mpsc::channel
// или crossbeam_channel для нескольких отправителей и приемников и гарантии жизни
// https://gist.github.com/mattgathu/73712cb7399834d6f8162d641830cbb7

// Есть вариант не хранить Uuid подписчика,а хранить ссылку на функцию  Vec<Box<dyn Fn(&E)>>
// https://stackoverflow.com/questions/37572734/how-can-i-implement-the-observer-pattern-in-rust

// Вариант с хранением ссыдки на субьект в каждом подписчике не жизнеспособен,
// нужно хранить субьект через слабый указатель Weak и при изменчивом заимствовании не сможем взять
// еще любую другую ссылку на подписчика так как получается циклическая ссылка

// Использую Rc<RefCell<Box<dyn ...>>> для возможности раскидать ссылки (Rc),
// возможность выборочно брать изменчивое/не изменчивое заимствование (RefCell),
// для пооддержки определенного интерфейса Box<dyn ...>

fn rand_string() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(30).collect()
}
trait SubjectImpl {
    fn register_observer(&mut self, obs: Rc<RefCell<Box<dyn ObserverImpl>>>) -> bool;
    fn remove_observer(&mut self, key: Uuid);
    fn notify_observers(&mut self);
    fn set_measurements(&mut self, temperature: f32, humidity: f32, pressure: i32); // фейковая загрузка
}
trait DataImpl {
    fn get_temperature(&self) -> f32;
    fn get_humidity(&self) -> f32;
    fn get_pressure(&self) -> i32;
    fn set_temperature(&mut self, temperature: f32);
    fn set_humidity(&mut self, humidity: f32);
    fn set_pressure(&mut self, pressure: i32);
}
trait ObserverImpl {
    fn update(&mut self, data: Ref<Box<dyn DataImpl>>); // Ref получение данных от интерфейса DataImpl по не изменяемой ссылке
    fn get_key(&self) -> Uuid;
    fn set_key(&mut self, key: Uuid);
    fn remove_observer(&mut self);
    fn get_subscription(&self) -> bool;
}
trait DisplayElementImpl {
    fn display(&self);
}

#[derive(Default)]
struct Data {
    temperature: f32,
    humidity: f32, //влажность
    pressure: i32, //давление
}
// Реализация запроса состояния
impl DataImpl for Data {
    fn get_temperature(&self) -> f32 {
        self.temperature
    }
    fn get_humidity(&self) -> f32 {
        self.humidity
    }
    fn get_pressure(&self) -> i32 {
        self.pressure
    }
    fn set_temperature(&mut self, temperature: f32) {
        self.temperature = temperature;
    }
    fn set_humidity(&mut self, humidity: f32) {
        self.humidity = humidity;
    }
    fn set_pressure(&mut self, pressure: i32) {
        self.pressure = pressure;
    }
}
// Управляющая структура, логика
struct WeatherData {
    observers: HashMap<Uuid, Rc<RefCell<Box<dyn ObserverImpl>>>>,
    data: Rc<RefCell<Box<dyn DataImpl>>>,
    changed: bool,
}
impl WeatherData {
    pub fn constructor(data: Rc<RefCell<Box<dyn DataImpl>>>) -> Self {
        WeatherData {
            observers: HashMap::new(),
            data: data,
            changed: false,
        }
    }
    fn set_changed(&mut self) {
        self.changed = true;
    }
    fn clear_changed(&mut self) {
        self.changed = false;
    }
    pub fn has_changed(&self) -> bool {
        self.changed
    }
    // Оповещение наблюдателей о появлении новых данных
    fn measurements_changed(&mut self) {
        self.set_changed(); // Если новые данные не значительные можно не оповещать
        self.notify_observers();
    }
    // удалить отписавшихся подписчиков по признаку отписку
    fn update_active_observer(&mut self) {
        self.observers.retain(|_, v| {
            if v.borrow().get_subscription() {
                return true;
            } else {
                println!("<<<Отписка>>> {:?}:", v.borrow().get_key());
                println!("-----------------------------------");
                return false;
            }
        });
    }
}
impl SubjectImpl for WeatherData {
    fn register_observer(&mut self, obs: Rc<RefCell<Box<dyn ObserverImpl>>>) -> bool {
        if !self.observers.contains_key(&obs.borrow().get_key()) {
            let key = Uuid::new_v5(&Uuid::NAMESPACE_OID, rand_string().as_bytes());
            obs.borrow_mut().set_key(key);
            self.observers.insert(key, obs);
            return true;
        }
        false
    }
    // удалить отписавшегося подписчика напрямую по Uuid
    fn remove_observer(&mut self, key: Uuid) {
        if self.observers.contains_key(&key) {
            println!("Отписка {:?}:", &key);
            println!("-----------------------------------");
            self.observers.remove(&key);
        }
    }
    fn notify_observers(&mut self) {
        self.update_active_observer();
        if self.has_changed() {
            //for el in self.observers.values_mut() {
            //   el.borrow_mut().update(self.data.borrow());
            //}
            for (_, el) in self.observers.iter_mut() {
                el.borrow_mut().update(self.data.borrow());
            }

            self.clear_changed();
        }
    }
    // Загрузка фейковых данных метеостанции с оповещением
    fn set_measurements(&mut self, temperature: f32, humidity: f32, pressure: i32) {
        {
            let mut data = self.data.borrow_mut();
            data.set_temperature(temperature);
            data.set_humidity(humidity);
            data.set_pressure(pressure);
        }
        // избавились от мутабельного заимствования
        self.measurements_changed();
    }
}
// Наблюдатели
//----------------------------------------------------------------------------
#[derive(Clone)]
struct CurrentConditionsDisplay {
    temperature: f32,
    humidity: f32, //влажность
    pressure: i32, //давление
    key: Uuid,
    subscription: bool,
}
impl CurrentConditionsDisplay {
    pub fn constructor() -> Self {
        CurrentConditionsDisplay {
            temperature: 0.0,
            humidity: 0.0,
            pressure: 0,
            key: Default::default(),
            subscription: false,
        }
    }
}
impl ObserverImpl for CurrentConditionsDisplay {
    fn update(&mut self, weather_data: Ref<Box<dyn DataImpl>>) {
        if self.subscription {
            //weatherData.set_humidity(1f32); не получиться так сделать
            self.temperature = weather_data.get_temperature();
            self.humidity = weather_data.get_humidity();

            self.display();
        }
    }
    fn get_key(&self) -> Uuid {
        self.key
    }
    fn set_key(&mut self, key: Uuid) {
        self.key = key;
        self.subscription = true;
    }
    fn remove_observer(&mut self) {
        //self.weatherData.upgrade().unwrap().borrow_mut().remove_observer( self.get_key());
        self.subscription = false;
    }
    fn get_subscription(&self) -> bool {
        self.subscription
    }
}
impl DisplayElementImpl for CurrentConditionsDisplay {
    fn display(&self) {
        println!("Текущие условия {:?}:", self.get_key());
        println!(
            "{:?}F degrees and, {:?}% humidity",
            self.temperature, self.humidity
        );
        println!("-----------------------------------");
    }
}
//--------------------------------------------------------------------------
struct StatisticsDisplay {
    max_temperature: f32,
    min_temperature: f32,
    sum_temperature: f32,
    num_readings: i32,
    key: Uuid,
    subscription: bool,
}
impl StatisticsDisplay {
    fn constructor() -> Self {
        StatisticsDisplay {
            max_temperature: 0.0,
            min_temperature: 0.0,
            sum_temperature: 0.0,
            num_readings: 0,
            key: Default::default(),
            subscription: false,
        }
    }
}
impl ObserverImpl for StatisticsDisplay {
    fn update(&mut self, weather_data: Ref<Box<dyn DataImpl>>) {
        if self.subscription {
            let temp_temperature = weather_data.get_temperature();
            self.sum_temperature += temp_temperature;
            self.num_readings += 1;
            if temp_temperature > self.max_temperature {
                self.max_temperature = temp_temperature;
            }

            if temp_temperature < self.min_temperature {
                self.min_temperature = temp_temperature;
            }

            self.display();
        }
    }
    fn get_key(&self) -> Uuid {
        self.key
    }
    fn set_key(&mut self, key: Uuid) {
        self.key = key;
        self.subscription = true;
    }
    fn remove_observer(&mut self) {
        self.subscription = false;
    }
    fn get_subscription(&self) -> bool {
        self.subscription
    }
}
impl DisplayElementImpl for StatisticsDisplay {
    fn display(&self) {
        println!("Статистика {:?}:", self.get_key());
        println!(
            "Avg/Max/Min temperature = {:?}/{:?}/{:?}",
            (self.sum_temperature / self.num_readings as f32),
            self.max_temperature,
            self.min_temperature
        );
        println!("-----------------------------------");
    }
}
//----------------------------------------------------------------------------------
struct ForecastDisplay {
    current_pressure: i32,
    last_pressure: i32,
    key: Uuid,
    subscription: bool,
}
impl ForecastDisplay {
    fn constructor() -> Self {
        ForecastDisplay {
            current_pressure: 0,
            last_pressure: 0,
            key: Default::default(),
            subscription: false,
        }
    }
}
impl ObserverImpl for ForecastDisplay {
    fn update(&mut self, weather_data: Ref<Box<dyn DataImpl>>) {
        if self.subscription {
            self.last_pressure = self.current_pressure;
            self.current_pressure = weather_data.get_pressure();

            self.display();
        }
    }
    fn get_key(&self) -> Uuid {
        self.key
    }
    fn set_key(&mut self, key: Uuid) {
        self.key = key;
        self.subscription = true;
    }
    fn remove_observer(&mut self) {
        self.subscription = false;
    }
    fn get_subscription(&self) -> bool {
        self.subscription
    }
}
impl DisplayElementImpl for ForecastDisplay {
    fn display(&self) {
        println!("Прогноз {:?}:", self.get_key());
        if self.current_pressure > self.last_pressure {
            println!("Скоро будет улучшение погоды!");
        } else if self.current_pressure == self.last_pressure {
            println!("Усиление погодных условий");
        } else if self.current_pressure < self.last_pressure {
            println!("Остерегайтесь прохладной, дождливой погоды");
        }
        println!("-----------------------------------");
    }
}
//-------------------------------------------------------------------------------
fn main() {
    let data: Rc<RefCell<Box<dyn DataImpl>>> = Rc::new(RefCell::new(Box::new(Data::default())));
    let weather_data: Box<dyn SubjectImpl> = Box::new(WeatherData::constructor(data));
    let weather_data_rc: Rc<RefCell<Box<dyn SubjectImpl>>> = Rc::new(RefCell::new(weather_data));

    // CurrentConditionsDisplay --------------------------------------------------
    let current_conditions_display = CurrentConditionsDisplay::constructor();
    let current_conditions_display_rc = Rc::new(RefCell::new(Box::new(
        current_conditions_display.clone(),
    ) as Box<dyn ObserverImpl>));
    weather_data_rc
        .borrow_mut()
        .register_observer(Rc::clone(&current_conditions_display_rc));

    // StatisticsDisplay ---------------------------------------------------------
    let statistics_display = StatisticsDisplay::constructor();
    let statistics_display_rc = Rc::new(RefCell::new(
        Box::new(statistics_display) as Box<dyn ObserverImpl>
    ));
    weather_data_rc
        .borrow_mut()
        .register_observer(Rc::clone(&statistics_display_rc));
    // ForecastDisplay -----------------------------------------------------------
    let forecast_display = ForecastDisplay::constructor();
    let forecast_display_rc = Rc::new(RefCell::new(
        Box::new(forecast_display) as Box<dyn ObserverImpl>
    ));
    weather_data_rc
        .borrow_mut()
        .register_observer(Rc::clone(&forecast_display_rc));
    // Заполнение данными и оповещение
    let mut rng = thread_rng();
    //№1
    weather_data_rc.borrow_mut().set_measurements(
        rng.gen_range(10.0, 100.0),
        rng.gen_range(10.0, 100.0),
        rng.gen_range(400, 800),
    );
    //№2
    weather_data_rc.borrow_mut().set_measurements(
        rng.gen_range(10.0, 100.0),
        rng.gen_range(10.0, 100.0),
        rng.gen_range(400, 800),
    );
    //отписка
    current_conditions_display_rc.borrow_mut().remove_observer();
    //№3
    weather_data_rc.borrow_mut().set_measurements(
        rng.gen_range(10.0, 100.0),
        rng.gen_range(10.0, 100.0),
        rng.gen_range(400, 800),
    );
}
```

 


* [Observer chercher.tech](https://chercher.tech/rust/observer-design-pattern-rust)
* [Observer refactoring.guru](https://refactoring.guru/ru/design-patterns/observer)

---

</details>


<br>
<details>
<summary><b>Iterator</b></summary>
  
`Pattern Iteratot` - Предоставьте способ последовательного доступа к элементам агрегатного объекта, не раскрывая его базовое представление.

Зачастую этот паттерн используется вместо массива объектов, чтобы не только предоставить доступ к элементам, но и наделить некоторой логикой. 
Это может быть ограничение доступа, сортировка или любая другая операция над множеством объектов.

Идея паттерна Итератор состоит в том, чтобы вынести поведение обхода коллекции из самой коллекции в отдельный класс.

Когда вам нужно иметь несколько вариантов обхода одной и той же структуры данных.

**Аналогия**

Вы планируете полететь в Рим и обойти все достопримечательности за пару дней. 
Но приехав, вы можете долго петлять узкими улочками, пытаясь найти Колизей.
Таким образом, Рим выступает коллекцией достопримечательностей, а ваш мозг, навигатор или гид — итератором по коллекции. 
Вы, как клиентский код, можете выбрать один из итераторов, отталкиваясь от решаемой задачи и доступных ресурсов.

**Проблема**

Также, возможный способ применения, когда вы используете экзотическую коллекцию данных т.е. не тривиально организованные данные.
Но каким способом следует перемещаться по сложной структуре данных? Например, сегодня может быть достаточным обход дерева в глубину,
    но завтра потребуется возможность перемещаться по дереву в ширину.
Добавляя всё новые алгоритмы в код коллекции, вы понемногу размываете её основную задачу, которая заключается в эффективном хранении данных. 
Некоторые алгоритмы могут быть и вовсе слишком «заточены» под определённое приложение и смотреться дико в общем классе коллекции.

**Проблема**

Необходимо «абстрагировать» обход совершенно разных структур данных, чтобы можно было определить алгоритмы, 
способные прозрачно взаимодействовать с каждой из них.

Агрегированный объект, такой как список, должен давать вам возможность доступа к его элементам, не раскрывая его внутреннюю структуру. 
Более того, вы можете захотеть перемещаться по списку разными способами, в зависимости от того, чего вам нужно достичь. 
Но вы, вероятно, не делаете этого. хотите раздуть интерфейс списка операциями для разных обходов, даже если вы можете предвидеть те, 
которые вам потребуются. Вам также может потребоваться иметь более одного ожидающего обхода в одном и том же списке». 
Кроме того, может оказаться полезным предоставление единого интерфейса для обхода многих типов агрегатных объектов (т. е. полиморфной итерации).

Абстракция `Iterator` является фундаментальной для новой технологии, называемой «обобщенным программированием». 
Эта стратегия стремится явно отделить понятие «алгоритм» от понятия «структура данных». 

Эмпирические правила
- Абстрактное синтаксическое дерево `Pattern Interpreter` является составным (поэтому также применимы `Pattern Iteratot` и `Pattern Visitor`).

- `Pattern Memento` часто используется вместе с `Pattern Iteratot`. Итератор может использовать Memento для фиксации состояния итерации. Итератор хранит Memento внутри себя.

В Rust итераторы ленивы, то есть они не действуют, пока вы не вызовете методы, которые используют итератор для его использования.

Пример:
```rust
#![allow(unused_imports)]

use std::iter::{IntoIterator, Iterator};
use words::WordsCollection2;
pub mod words {
    pub struct WordsCollection2 {
        items: Vec<String>,
        prev: usize,
        reverse: bool,
    }
    impl WordsCollection2 {
        pub fn new(items: Vec<String>, reverse: bool) -> Self {
            if !reverse {
                return Self {
                    items,
                    prev: 0,
                    reverse,
                };
            }
            let prev = items.len();
            Self {
                items,
                prev,
                reverse,
            }
        }
    }
    impl Iterator for WordsCollection2 {
        type Item = String;
        fn next(&mut self) -> Option<String> {
            if self.reverse {
                if self.prev > 0 {
                    self.prev -= 1;
                    return Some(self.items[self.prev].clone());
                }
                return None;
            } else {
                if self.prev < self.items.len() {
                    self.prev += 1;
                    return Some(self.items[self.prev - 1].clone());
                }
                return None;
            }
        }
    }
}
struct WordsCollection {
    items: Vec<String>,
}
impl IntoIterator for WordsCollection {
    type Item = String;
    type IntoIter = ::std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
fn main() {
    let w = WordsCollection {
        items: vec!["First".into(), "Second".into(), "Third".into()],
    };
    for i in w {
        println!("{}", i);
    }
    let w = WordsCollection2::new(vec!["First".into(), "Second".into(), "Third".into()], true);
    for i in w {
        println!("{}", i);
    }
}
```
 

* [Iterator sourcemaking.com](https://sourcemaking.com/design_patterns/iterator)
* [Iterator refactoring.guru](https://refactoring.guru/ru/design-patterns/iterator)
* [Rust std::iter](https://doc.rust-lang.org/std/iter/index.html)

---

</details>

<br>
<details>
<summary><b>Mediator</b></summary>
 
`Pattern Mediator` (посредник) позволяет уменьшить связанность множества классов между собой (многие ко многим), благодаря перемещению этих связей в один класс-посредник.
Устраняет зависимости между компонентами, позволяя повторно их использовать. Упрощает взаимодействие между компонентами. Централизует управление в одном месте.

Паттерн Посредник заставляет объекты общаться не напрямую друг с другом, а через отдельный объект-посредник, который знает, 
кому нужно перенаправить тот или иной запрос. 
Благодаря этому, компоненты системы будут зависеть только от посредника, а не от десятков других компонентов.

Объектам больше нет нужды вызывать друг друга напрямую. 
Это хорошая альтернатива `Pattern Observer`, если у вас есть “центр интеллекта” вроде контроллера (но не в смысле MVC)
Все компоненты (называемые «Коллеги») объединяются в интерфейс MediatorInterface.
Подписчики или объединенные компоненты делегируют управление медиатору.

Таким образом, посредник скрывает в себе все сложные связи и зависимости между классами отдельных компонентов программы. 
А чем меньше связей имеют классы, тем проще их изменять, расширять и повторно использовать.

**Аналогия**

Пилоты садящихся или улетающих самолётов не общаются напрямую с другими пилотами. 
Вместо этого они связываются с диспетчером, который координирует действия нескольких самолётов одновременно. 
Без диспетчера пилотам приходилось бы все время быть начеку и следить за всеми окружающими самолётами самостоятельно, а это приводило бы к частым катастрофам в небе.
Важно понимать, что диспетчер не нужен во время всего полёта. Он задействован только в зоне аэропорта, когда нужно координировать взаимодействие многих самолётов.

**Применимость**

1. Когда вам сложно менять некоторые классы из-за того, что они имеют множество хаотичных связей с другими классами.
Посредник позволяет поместить все эти связи в один класс, после чего вам будет легче их отрефакторить, сделать более понятными и гибкими.

2. Когда вы не можете повторно использовать класс, поскольку он зависит от уймы других классов.
После применения паттерна компоненты теряют прежние связи с другими компонентами, а всё их общение происходит косвенно, через объект-посредник.

3. Когда вам приходится создавать множество подклассов компонентов, чтобы использовать одни и те же компоненты в разных контекстах.
Если раньше изменение отношений в одном компоненте могли повлечь за собой лавину изменений во всех остальных компонентах, 
то теперь вам достаточно создать подкласс посредника и поменять в нём связи между компонентами.

**Эмпирические правила**

Разница между `Pattern Mediator` и `Pattern Observer` не всегда очевидна. Чаще всего они выступают как конкуренты, но иногда могут работать вместе.

Цель `Pattern Mediator` — убрать обоюдные зависимости между компонентами системы. Вместо этого они становятся зависимыми от самого посредника. 
С другой стороны, цель `Pattern Observer` — обеспечить динамическую одностороннюю связь, в которой одни объекты косвенно зависят от других.

![Mediator](/Computer-Science-Bookshelf/img/software_architecture/patterns/Mediator.png "Mediator")

![Mediator](/Computer-Science-Bookshelf/img/software_architecture/patterns/Mediator2.png "Mediator")

Пример:
Типичная реализация Mediator на других языках — это классический антишаблон в Rust: многие объекты содержат изменяемые перекрестные ссылки друг на друга, пытаясь изменить друг друга,
что в Rust является смертным грехом — компилятор не пропустит ваш первый наивный код. реализация, если она не упрощена.
По определению, Медиатор ограничивает прямую связь между объектами и заставляет их сотрудничать только через объект-посредник.
Это также означает контроллер в шаблоне MVC.

Владение сверху вниз. Ключевым моментом является мышление с точки зрения СОБСТВЕННОСТИ.

1. Посредник берет на себя ответственность за все компоненты.
2. Компонент не сохраняет ссылку на посредник. Вместо этого он получает ссылку через вызов метода.
3. Поток управления начинается с main(), где посредник получает внешние события/команды.
4. Свойство медиатора для взаимодействия между компонентами — это не то же самое, что его внешний API для получения внешних событий (команд из основного цикла).

Trait Mediator взаимодействия между компонентами (`notify_about_arrival`, `notify_about_departure`) не совпадает с его внешним API для получения внешних событий ( `accept​​`, `depart` команд из основного цикла).

```rust
// A train gets a mediator object by reference.
pub trait Train {
    fn name(&self) -> &String;
    fn arrive(&mut self, mediator: &mut dyn Mediator);
    fn depart(&mut self, mediator: &mut dyn Mediator);
}

// Nodes
use freight_train::FreightTrain;
pub mod freight_train {
    use super::Mediator;
    use super::Train;
    pub struct FreightTrain {
        name: String,
    }
    impl FreightTrain {
        pub fn new(name: &'static str) -> Self {
            Self { name: name.into() }
        }
    }
    impl Train for FreightTrain {
        fn name(&self) -> &String {
            &self.name
        }
        fn arrive(&mut self, mediator: &mut dyn Mediator) {
            if !mediator.notify_about_arrival(&self.name) {
                println!("Freight train {}: Arrival blocked, waiting", self.name);
                return;
            }
            println!("Freight train {}: Arrived", self.name);
        }
        fn depart(&mut self, mediator: &mut dyn Mediator) {
            println!("Freight train {}: Leaving", self.name);
            mediator.notify_about_departure(&self.name);
        }
    }
}
// Nodes
use passenger_train::PassengerTrain;
pub mod passenger_train {
    use super::*;
    pub struct PassengerTrain {
        name: String,
    }
    impl PassengerTrain {
        pub fn new(name: &'static str) -> Self {
            Self { name: name.into() }
        }
    }
    impl Train for PassengerTrain {
        fn name(&self) -> &String {
            &self.name
        }
        fn arrive(&mut self, mediator: &mut dyn Mediator) {
            if !mediator.notify_about_arrival(&self.name) {
                println!("Passenger train {}: Arrival blocked, waiting", self.name);
                return;
            }
            println!("Passenger train {}: Arrived", self.name);
        }
        fn depart(&mut self, mediator: &mut dyn Mediator) {
            println!("Passenger train {}: Leaving", self.name);
            mediator.notify_about_departure(&self.name);
        }
    }
}
// Mediator
use train_station::{Mediator, TrainStation};
pub mod train_station {
    use std::collections::{HashMap, VecDeque};
    use super::Train;
    // Mediator has notification methods.
    pub trait Mediator {
        fn notify_about_arrival(&mut self, train_name: &str) -> bool;
        fn notify_about_departure(&mut self, train_name: &str);
    }
    #[derive(Default)]
    pub struct TrainStation {
        trains: HashMap<String, Box<dyn Train>>,
        train_queue: VecDeque<String>,
        train_on_platform: Option<String>,
    }
    impl Mediator for TrainStation {
        fn notify_about_arrival(&mut self, train_name: &str) -> bool {
            if self.train_on_platform.is_some() {
                self.train_queue.push_back(train_name.into());
                return false;
            } else {
                self.train_on_platform.replace(train_name.into());
                return true;
            }
        }
        fn notify_about_departure(&mut self, train_name: &str) {
            if Some(train_name.into()) == self.train_on_platform {
                self.train_on_platform = None;
                if let Some(next_train_name) = self.train_queue.pop_front() {
                    let mut next_train = self.trains.remove(&next_train_name).unwrap();
                    next_train.arrive(self);
                    self.trains.insert(next_train_name.clone(), next_train);
                    self.train_on_platform = Some(next_train_name);
                }
            }
        }
    }
    impl TrainStation {
        pub fn accept(&mut self, mut train: impl Train + 'static) {
            if self.trains.contains_key(train.name()) {
                println!("{} has already arrived", train.name());
                return;
            }
            train.arrive(self);
            self.trains.insert(train.name().clone(), Box::new(train));
        }
        pub fn depart(&mut self, name: &'static str) {
            let train = self.trains.remove(name.into());
            if let Some(mut train) = train {
                train.depart(self);
            } else {
                println!("'{}' is not on the station!", name);
            }
        }
    }
}
fn main() {
    let train1 = PassengerTrain::new("Train 1"); // Пассажирский поезд
    let train2 = FreightTrain::new("Train 2"); // Грузовой поезд

    // Station has `accept` and `depart` methods,
    // but it also implements `Mediator`.
    let mut station = TrainStation::default();

    // Станция берет на себя управление поездами.
    station.accept(train1);
    station.accept(train2);

    // `train1` and `train2` have been moved inside,
    // but we can use train names to depart them.
    station.depart("Train 1");
    station.depart("Train 2");
    station.depart("Train 3");
}
```


 
* [Mediator refactoring.guru](https://refactoring.guru/ru/design-patterns/mediator)
* [Mediator github.com/fadeevab](https://github.com/fadeevab/mediator-pattern-rust/blob/main/README.md)

 
---

</details>

<br>
<details>
<summary><b>State</b></summary>
 
В зависимости от состояния изменяется поведение

`Typestate` делает неправильное использование объекта с состоянием ошибкой времени компиляции.

`Pattern State` невозможно рассматривать в отрыве от концепции машины состояний, также известной как стейт-машина или конечный автомат.
Основная идея в том, что программа может находиться в одном из нескольких состояний, которые всё время сменяют друг друга. 
Набор этих состояний, а также переходов между ними, предопределён и конечен. Находясь в разных состояниях, 
программа может по-разному реагировать на одни и те же события, которые происходят с ней.

`Pattern State` — это решение проблемы того, как заставить поведение зависеть от состояния.

**Аналогия**

Паттерн State позволяет объекту изменять свое поведение при изменении его внутреннего состояния. 
Эту картину можно наблюдать в торговом автомате. 
У торговых автоматов есть состояния, основанные на инвентаре, сумме внесенной валюты, возможности внесения сдачи, выбранном товаре и т. д. 
Когда валюта внесена и сделан выбор, торговый автомат либо доставит продукт без сдачи, либо доставит товар. 
продукт и изменить его, не доставить продукт из-за недостаточности валюты на депозите или не доставить продукт из-за истощения запасов.

**Проблема**

Машину состояний чаще всего реализуют с помощью множества условных операторов, `if` либо `switch`, 
которые проверяют текущее состояние объекта и выполняют соответствующее поведение. 
Основная проблема такой машины состояний проявится в том случае, если в обьект добавить ещё десяток состояний. 
Каждый метод будет состоять из увесистого условного оператора, перебирающего доступные состояния. Такой код крайне сложно поддерживать. 
Малейшее изменение логики переходов заставит вас перепроверять работу всех методов, которые содержат условные операторы машины состояний.
Путаница и нагромождение условий особенно сильно проявляется в старых проектах. 
Набор возможных состояний бывает трудно предопределить заранее, поэтому они всё время добавляются в процессе эволюции программы. 
Из-за этого решение, которое выглядело простым и эффективным в самом начале разработки, может впоследствии стать проекцией большого макаронного монстра.
`Pattern State` предлагает создать отдельные классы для каждого состояния, в котором может пребывать объект, 
а затем вынести туда поведения, соответствующие этим состояниям.

**Эмпирические правила**

Реализация `Pattern State` основана на шаблоне `Pattern Strategy`. Разница между `Pattern State` и `Pattern Strategy` заключается в намерении. 
При использовании `Strategy` выбор алгоритма достаточно стабилен. При использовании `State` изменение состояния объекта «контекст» приводит к 
выбору объектов стратегии из «палитры».

`Flyweight` объясняет, когда и как можно совместно использовать объекты `State`.

Пример:
```rust
use std::marker::PhantomData;

struct SoldState; // Состояние продано
struct SoldOutState; // Состояние все продано, нет шариков
struct NoQuarterState; // Состояние монетка не внесена
struct HasQuarterState; // Состояние монетка внесена

struct GumballMashine<S> {
    count: i32,
    state: PhantomData<S>,
}
impl GumballMashine<SoldState> {
    fn realese_ball(&mut self) {
        if self.count > 0 {
            self.count -= 1;
        }
    }
    fn get_count(&self) -> i32 {
        self.count
    }
}
impl std::fmt::Debug for GumballMashine<SoldState> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "GumballMashine<SoldState> {{ count: {} }}", self.count)
    }
}
impl std::fmt::Debug for GumballMashine<SoldOutState> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "GumballMashine<SoldOutState> {{ count: {} }}",
            self.count
        )
    }
}
impl std::fmt::Debug for GumballMashine<NoQuarterState> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "GumballMashine<NoQuarterState> {{ count: {} }}",
            self.count
        )
    }
}
impl std::fmt::Debug for GumballMashine<HasQuarterState> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "GumballMashine<HasQuarterState> {{ count: {} }}",
            self.count
        )
    }
}
//Переход из состояния монетка не внесена в состояние монетка внесена
/// NoQuarterState -- HasQuarterState
impl From<GumballMashine<NoQuarterState>> for GumballMashine<HasQuarterState> {
    fn from(_val: GumballMashine<NoQuarterState>) -> GumballMashine<HasQuarterState> {
        GumballMashine {
            count: _val.count,
            state: PhantomData,
        }
    }
}
//Переход из состояния монетка внесена в состояние монетка не внесена
/// NoQuarterState -- HasQuarterState
impl From<GumballMashine<HasQuarterState>> for GumballMashine<NoQuarterState> {
    fn from(_val: GumballMashine<HasQuarterState>) -> GumballMashine<NoQuarterState> {
        GumballMashine {
            count: _val.count,
            state: PhantomData,
        }
    }
}
//Переход из состояния монетка внесена в состояние продано
/// HasQuarterState -- SoldState
impl From<GumballMashine<HasQuarterState>> for GumballMashine<SoldState> {
    fn from(_val: GumballMashine<HasQuarterState>) -> GumballMashine<SoldState> {
        GumballMashine {
            count: _val.count,
            state: PhantomData,
        }
    }
}
//Переход из состояния продано в состояние шариков больше нет
/// SoldState -- SoldOutState
impl From<&mut GumballMashine<SoldState>> for GumballMashine<SoldOutState> {
    fn from(_val: &mut GumballMashine<SoldState>) -> GumballMashine<SoldOutState> {
        GumballMashine {
            count: _val.count,
            state: PhantomData,
        }
    }
}
//Переход из состояния продано в состояние монетка не внесена
/// SoldState -- NoQuarterState
impl From<&mut GumballMashine<SoldState>> for GumballMashine<NoQuarterState> {
    fn from(_val: &mut GumballMashine<SoldState>) -> GumballMashine<NoQuarterState> {
        GumballMashine {
            count: _val.count,
            state: PhantomData,
        }
    }
}

//------------------------------------------------------------------------------------------ 
fn new_gumball(count: i32) -> GumballMashine<NoQuarterState> {
    let gumball: GumballMashine<NoQuarterState> = GumballMashine {
        count,
        state: PhantomData,
    };
    gumball
}

fn insert_quarter(gumball: GumballMashine<NoQuarterState>) -> GumballMashine<HasQuarterState> {
    println!("Монетка внесена");
    gumball.into()
}
fn eject_quarter(gumball: GumballMashine<HasQuarterState>) -> GumballMashine<NoQuarterState> {
    println!("Возврат монетки");
    gumball.into()
}
fn turn_crank(gumball: GumballMashine<HasQuarterState>) -> GumballMashine<SoldState> {
    println!("Получение шарика");
    gumball.into()
}
fn dispense(
    gunball: &mut GumballMashine<SoldState>,
) -> Result<GumballMashine<NoQuarterState>, GumballMashine<SoldOutState>> {
    gunball.realese_ball();
    if gunball.get_count() > 0 {
        return Ok(gunball.into());
    } else {
        return Err(gunball.into());
    }
}
fn main() {
    /*
     let gumball_no_quarter_state = new_gumball(5);
     let gumball_has_quarter_state = insert_quarter(gumball_no_quarter_state);
     //let gumball_no_quarter_state = eject_quarter(gumball_has_quarter_state);
     let mut gumball_sold_state = turn_crank(gumball_has_quarter_state);
     let gumball  = dispense(&mut gumball_sold_state);
    */

    let gumball_no_quarter_state = new_gumball(3);
    let gumball_has_quarter_state = insert_quarter(gumball_no_quarter_state); //бросить монетку
    let mut gumball_sold_state = turn_crank(gumball_has_quarter_state); //опустить рычаг
    let _gumball = dispense(&mut gumball_sold_state);
    println!("{:?}\n", gumball_sold_state);

    let gumball_no_quarter_state = new_gumball(3);
    let gumball_has_quarter_state = insert_quarter(gumball_no_quarter_state); //бросить монетку
    let gumball_no_quarter_state = eject_quarter(gumball_has_quarter_state); //вернуть монетку
    println!("Опустить рычаг не получиться, нет реализованной функции!");
    println!("{:?}\n", gumball_no_quarter_state);

    println!("Бросить монетку два раза не получиться, первый вызов берет владение переменной!\n");

    let gumball_no_quarter_state = new_gumball(3);
    let gumball_has_quarter_state = insert_quarter(gumball_no_quarter_state); //бросить монетку
    println!("{:?}", gumball_has_quarter_state);
}
```
 

* [Typestate examples](https://github.com/Jekahome/Patterns/tree/main/src/idioms/1.Type_safety/1.2.Typestates)
* [Typestate developerlife.com](https://developerlife.com/2024/05/28/typestate-pattern-rust/)
* [Typestate willcrichton.net](https://willcrichton.net/rust-api-type-patterns/typestate.html)
* [Typestate cliffle.com](https://cliffle.com/blog/rust-typestate/)
* [State sourcemaking.com](https://sourcemaking.com/design_patterns/state)
* [State refactoring.guru](https://refactoring.guru/ru/design-patterns/state)
* [State doc.rust-lang.ru](https://doc.rust-lang.ru/book/ch17-03-oo-design-patterns.html)

---

</details>


<br>
<details>
<summary><b>Memento (Хранитель)</b></summary>
 
`Pattern Memento` (`Token`) позволяет сохранять и восстанавливать прошлые состояния объектов, не раскрывая подробностей их реализации.
По мере разработки вашего приложения вы можете захотеть сохранить контрольные точки в своем приложении и позже вернуться к этим контрольным точкам.
Предоставить возможность выполнить действие отмены, чтобы восстановить объект в предыдущее состояние.

**Аналония**

Отмена действия, возврат назад или сочетание клавиш Ctrl+Z — одна из наиболее часто используемых операций в редакторе. 
Для реализации операции отмены используется `Pattern Memento`. Это делается путем сохранения текущего состояния объекта по мере его изменения.

Одним из важных моментов, которые следует избегать при реализации `Pattern Memento`, является то, что инкапсуляция объекта не должна подвергаться риску.
Паттерн Снимок поручает создание копии состояния объекта самому объекту.
Паттерн предлагает держать копию состояния в специальном объекте-снимке с ограниченным интерфейсом, позволяющим, например, узнать дату изготовления или название снимка. 
Но, с другой стороны, снимок должен быть открыт для своего создателя, позволяя прочесть и восстановить его внутреннее состояние.
Такая схема позволяет создателям производить снимки и отдавать их для хранения другим объектам, называемым опекунами. 
Опекунам будет доступен только ограниченный интерфейс снимка, поэтому они никак не смогут повлиять на «внутренности» самого снимка. 
В нужный момент опекун может попросить создателя восстановить своё состояние, передав ему соответствующий снимок.

В некоторых языках (например, PHP, Python, JavaScript) сложно гарантировать, чтобы только исходный объект имел доступ к состоянию снимка.

**Эмпирические правила**

`Pattern Command` и `Pattern Memento` можно использовать сообща для реализации отмены операций. 
В этом случае объекты команд будут отвечать за выполнение действия над объектом, а снимки будут хранить резервную копию состояния этого объекта, 
сделанную перед самым запуском команды.

`Pattern Memento` иногда можно заменить Прототипом, если объект, состояние которого требуется сохранять в истории, довольно простой, 
не имеет активных ссылок на внешние ресурсы либо их можно легко восстановить.

**Участники:**

1. Originator (Создатель) может производить снимки своего состояния, а также воспроизводить прошлое состояние, если подать в него готовый снимок.

2. Memento (Снимок) — это простой объект данных, содержащий состояние создателя. Надёжнее всего сделать объекты снимков неизменяемыми, передавая в них состояние только через конструктор.

3. Caretaker (Смотритель/Опекун/Хранитель) должен знать, когда делать снимок создателя и когда его нужно восстанавливать.
    Опекун может хранить историю прошлых состояний создателя в виде стека из снимков. 
    Когда понадобится отменить выполненную операцию, он возьмёт «верхний» снимок из стека и передаст его создателю для восстановления.
    
![Memento](/Computer-Science-Bookshelf/img/software_architecture/patterns/Memento.png "Memento")

Пример:
```rust
use originator::{Memento, Originator, Point};
pub mod originator {
    use std::marker::PhantomData;
    pub trait Originator {
        fn generate_memento(&self) -> Box<dyn Memento>;
        fn restore_from_memento(&mut self, _: &dyn Memento);
    }
    // Concrete Originator
    #[derive(Debug)]
    pub struct Point {
        x: i32,
        y: i32,
        step: i32,
    }
    impl Point {
        pub fn new(x: i32, y: i32, step: u8) -> Self {
            Self {
                x,
                y,
                step: step as i32,
            }
        }
        pub fn up(&mut self) {
            self.y += self.step;
        }
        pub fn down(&mut self) {
            self.y -= self.step;
        }
        pub fn left(&mut self) {
            self.x -= self.step;
        }
        pub fn right(&mut self) {
            self.x += self.step;
        }
    }
    impl Originator for Point {
        fn generate_memento(&self) -> Box<dyn Memento> {
            Box::new(PointMemento(self.x, self.y, self.step))
        }
        fn restore_from_memento(&mut self, m: &dyn Memento) {
            let (x, y, step) = m.get_value(&Lock(PhantomData));
            self.x = x;
            self.y = y;
            self.step = step;
        }
    }
    // TODO: доступ к данным имееет только originator
    pub struct Lock(PhantomData<Self>);
    pub trait Memento {
        fn get_value(&self, _: &Lock) -> (i32, i32, i32);
    }
    // Concrete Memento
    struct PointMemento(i32, i32, i32);
    impl Memento for PointMemento {
        fn get_value(&self, _: &Lock) -> (i32, i32, i32) {
            (self.0, self.1, self.2)
        }
    }
}
// Смотритель/Опекун/Хранитель
use caretaker::ConcreteCaretaker;
pub mod caretaker {
    use super::originator::Memento;

    pub struct ConcreteCaretaker {
        history: Vec<Box<dyn Memento>>,
    }
    impl ConcreteCaretaker {
        pub fn new() -> ConcreteCaretaker {
            ConcreteCaretaker {
                history: Vec::new(),
            }
        }
        pub fn add_memento(&mut self, m: Box<dyn Memento>) {
            self.history.push(m)
        }
        pub fn get_memento(&mut self, index: usize) -> &dyn Memento {
            &*self.history[index]
        }
    }
}
pub fn main() {
    let mut caretaker = ConcreteCaretaker::new();
    {
        let mut originator = Point::new(10, 0, 1);
        println!("{:?}", &originator);
        caretaker.add_memento(originator.generate_memento());

        originator.up();
        println!("{:?}", &originator);
        caretaker.add_memento(originator.generate_memento());

        originator.right();
        println!("{:?}", &originator);
        caretaker.add_memento(originator.generate_memento());
    }
    println!("\n");
    let mut originator = Point::new(0, 0, 0);
    for indx in 0..3 {
        let m: &dyn Memento = caretaker.get_memento(indx);
        originator.restore_from_memento(&*m);
        println!("{:?}", originator);
    }
}
```

* [Memento refactoring.guru](https://refactoring.guru/ru/design-patterns/memento)
* [Memento chercher.tech](https://chercher.tech/rust/momento-design-pattern-rust)

---

</details>


<br>
<details>
<summary><b>Chain Of Responsibilities</b></summary>
  
`Pattern Chain Of Responsibilities` используется для достижения слабой связи отправителя запроса с получателем. Делегировать задачу следующему. 

Представьте себе систему, в которой запрос может обрабатываться несколькими типами обработчиков, но действовать над ним должен только один на основе определенных критериев. Вместо того, чтобы жестко запрограммировать логику принятия решений, как мы можем эффективно делегировать запрос через ряд обработчиков?

Позволяет передавать запросы последовательно по цепочке обработчиков. 
Каждый последующий обработчик решает, может ли он обработать запрос сам и стоит ли передавать запрос дальше по цепи.
Механизм цепочки использует рекурсивную композицию, позволяющую связывать неограниченное количество обработчиков.

`Pattern Chain Of Responsibilities` позволяет избежать связывания отправителя запроса с получателем, предоставляя более чем одному объекту возможность обработать запрос.

По сути это набор обработчиков, которые по очереди получают запрос, а затем решают обрабатывать его или нет. 
Если запрос не обработан, то он передается дальше по цепочке. 
Если же он обработан, то паттерн сам решает передавать его дальше или нет.
middleware

Необходимость эффективной обработки запросов без жесткого связывания отношений и приоритета обработчиков или сопоставлений запросов с обработчиками.

Паттерн предлагает связать объекты обработчиков в одну цепь. 
Каждый из них будет иметь ссылку на следующий обработчик в цепи. 
Таким образом, при получении запроса обработчик сможет не только сам что-то с ним сделать, но и передать обработку следующему объекту в цепочке.

Не используйте `Pattern Chain Of Responsibilities`, если каждый запрос обрабатывается только одним обработчиком или когда клиентский объект знает, 
какой объект службы должен обрабатывать запрос.

**Применимость**

1. Когда программа должна обрабатывать разнообразные запросы несколькими способами, но заранее неизвестно, какие конкретно запросы будут приходить и какие обработчики для них понадобятся.
    С помощью `Pattern Chain Of Responsibilities` вы можете связать потенциальных обработчиков в одну цепь и при получении запроса поочерёдно спрашивать каждого из них, не хочет ли он обработать запрос.

2. Когда важно, чтобы обработчики выполнялись один за другим в строгом порядке.
    `Pattern Chain Of Responsibilities` позволяет запускать обработчиков последовательно один за другим в том порядке, в котором они находятся в цепочке.

3. Когда набор объектов, способных обработать запрос, должен задаваться динамически.
    В любой момент вы можете вмешаться в существующую цепочку и переназначить связи так, чтобы убрать или добавить новое звено.

4. Объектно-ориентированный связанный список с рекурсивным обходом.

**Реальные варианты использования**

1. Обработка событий в библиотеках графического интерфейса, где события могут обрабатываться несколькими компонентами (например, кнопкой, диалогом, окном).

2. Промежуточное программное обеспечение на веб-серверах, где запрос может обрабатываться несколькими функциями промежуточного программного обеспечения, прежде чем он достигнет конечного обработчика.

3. Проверка входных данных в конвейерах обработки.

**Аналогия**

Банкоматы используют цепочку ответственности в механизме выдачи денег.
Для сдачи подойдут все купюры из которых возможно составить сумму и они есть в наличии.

Эмпирические правила

`Pattern Chain Of Responsibilities` часто используют вместе с Компоновщиком. В этом случае запрос передаётся от дочерних компонентов к их родителям.

Обработчики в `Pattern Chain Of Responsibilities` могут быть выполнены в виде `Pattern Command`. В этом случае множество разных операций может быть выполнено над одним и тем же контекстом, коим является запрос.
Но есть и другой подход, в котором сам запрос является  `Pattern Command`, посланной по цепочке объектов. В этом случае одна и та же операция может быть выполнена над множеством разных контекстов, представленных в виде цепочки.

`Pattern Chain Of Responsibilities` и `Pattern Decorator` имеют очень похожие структуры. 
Оба паттерна базируются на принципе рекурсивного выполнения операции через серию связанных объектов. Но `Pattern Decorator` не прерывает ход выполнения.

![Сhain-of-responsibility](/Computer-Science-Bookshelf/img/software_architecture/patterns/chain-of-responsibility.png "Сhain-of-responsibility")

Пример:
```rust
// обработка пациента через цепочку отделений
// Patient -> Reception -> Doctor -> Medical -> Cashier
use request::Patient;
pub mod request {
    #[derive(Default)]
    pub struct Patient {
        pub name: String,
        pub registration_done: bool,
        pub doctor_check_up_done: bool,
        pub medicine_done: bool,
        pub payment_done: bool,
    }
}
use handlers::*;
pub mod handlers {
    use super::*;
    /// A single role of objects that make up a chain.
    /// A typical trait implementation must have `handle` and `next` methods,
    /// while `execute` is implemented by default and contains a proper chaining
    /// logic.
    pub trait Department {
        fn execute(&mut self, patient: &mut Patient) {
            self.handle(patient);

            if let Some(next) = &mut self.next() {
                next.execute(patient);
            }
        }
        fn handle(&mut self, patient: &mut Patient);
        fn next(&mut self) -> &mut Option<Box<dyn Department>>;
    }
    /// Helps to wrap an object into a boxed type.
    pub(crate) fn into_next(
        department: impl Department + Sized + 'static,
    ) -> Option<Box<dyn Department>> {
        Some(Box::new(department))
    }
}
use cashier::*;
pub mod cashier {
    use super::*;
    #[derive(Default)]
    pub struct Cashier {
        next: Option<Box<dyn Department>>,
    }
    impl Department for Cashier {
        fn handle(&mut self, patient: &mut Patient) {
            if patient.payment_done {
                println!("Payment done");
            } else {
                println!("Cashier getting money from a patient {}", patient.name);
                patient.payment_done = true;
            }
        }
        fn next(&mut self) -> &mut Option<Box<dyn Department>> {
            &mut self.next
        }
    }
}
use doctor::*;
pub mod doctor {
    use super::*;
    pub struct Doctor {
        next: Option<Box<dyn Department>>,
    }
    impl Doctor {
        pub fn new(next: impl Department + 'static) -> Self {
            Self {
                next: into_next(next),
            }
        }
    }
    impl Department for Doctor {
        fn handle(&mut self, patient: &mut Patient) {
            if patient.doctor_check_up_done {
                println!("A doctor checkup is already done");
            } else {
                println!("Doctor checking a patient {}", patient.name);
                patient.doctor_check_up_done = true;
            }
        }
        fn next(&mut self) -> &mut Option<Box<dyn Department>> {
            &mut self.next
        }
    }
}
use medical::*;
pub mod medical {
    use super::*;
    pub struct Medical {
        next: Option<Box<dyn Department>>,
    }
    impl Medical {
        pub fn new(next: impl Department + 'static) -> Self {
            Self {
                next: into_next(next),
            }
        }
    }
    impl Department for Medical {
        fn handle(&mut self, patient: &mut Patient) {
            if patient.medicine_done {
                println!("Medicine is already given to a patient");
            } else {
                println!("Medical giving medicine to a patient {}", patient.name);
                patient.medicine_done = true;
            }
        }
        fn next(&mut self) -> &mut Option<Box<dyn Department>> {
            &mut self.next
        }
    }
}
use reception::*;
pub mod reception {
    use super::*;
    #[derive(Default)]
    pub struct Reception {
        next: Option<Box<dyn Department>>,
    }
    impl Reception {
        pub fn new(next: impl Department + 'static) -> Self {
            Self {
                next: into_next(next),
            }
        }
    }
    impl Department for Reception {
        fn handle(&mut self, patient: &mut Patient) {
            if patient.registration_done {
                println!("Patient registration is already done");
            } else {
                println!("Reception registering a patient {}", patient.name);
                patient.registration_done = true;
            }
        }
        fn next(&mut self) -> &mut Option<Box<dyn Department>> {
            &mut self.next
        }
    }
}
fn main() {
    let cashier = Cashier::default();
    let medical = Medical::new(cashier);
    let doctor = Doctor::new(medical);
    let mut reception = Reception::new(doctor);

    let mut patient = Patient {
        name: "John".into(),
        ..Patient::default()
    };

    // Reception handles a patient passing him to the next link in the chain.
    // Reception -> Doctor -> Medical -> Cashier.
    reception.execute(&mut patient);

    println!("\nThe patient has been already handled:\n");

    reception.execute(&mut patient);
}
```

Еще пример:
```rust
// Chain of Responsibility (= CoR) pattern
trait CoR {
    fn process_request(&self, req: &mut Request);
}
trait Request {
    fn get_level(&self) -> &Level;
    fn get_something(&self) -> usize;
}
struct ConcreteRequest {
    level: Level,
    v: usize,
}
impl ConcreteRequest {
    fn new(level: Level, v: usize) -> Self {
        Self { level, v }
    }
}
impl Request for ConcreteRequest {
    fn get_level(&self) -> &Level {
        &self.level
    }
    fn get_something(&self) -> usize {
        self.v
    }
}
#[derive(Debug, PartialEq)]
enum Level {
    High,
    Middle,
    Low,
}
struct ImplCoR {
    next: Option<Box<CoR>>,
    allowable_level: Level,
}
impl ImplCoR {
    fn new(level: Level, next: Option<Box<CoR>>) -> Self {
        Self {
            next,
            allowable_level: level,
        }
    }
}
impl CoR for ImplCoR {
    fn process_request(&self, req: &mut Request) {
        print!("{:?} ", self.allowable_level);
        if self.allowable_level == *req.get_level() {
            println!("Request accepted: v = {}", req.get_something());
        } else {
            if let Some(ref next) = self.next {
                println!("Pass to the next");
                next.process_request(req);
            } else {
                println!("Chain finished.");
            }
        }
    }
}
pub fn main() {
    let high = ImplCoR::new(Level::High, None);
    let middle = ImplCoR::new(Level::Middle, Some(Box::new(high)));
    let low = ImplCoR::new(Level::Low, Some(Box::new(middle)));

    let mut r1 = ConcreteRequest::new(Level::High, 1);
    let mut r2 = ConcreteRequest::new(Level::Middle, 2);
    let mut r3 = ConcreteRequest::new(Level::Low, 3);

    low.process_request(&mut r3);
    low.process_request(&mut r2);
    low.process_request(&mut r1);
}
```

Еще пример:
```rust
// Abstract handler
trait Handler {
    fn set_next(&mut self, next: Box<dyn Handler>);
    fn handle(&self, request: &str);
}
// Concrete handler 1
struct Handler1 {
    next: Option<Box<dyn Handler>>,
}
impl Handler1 {
    pub fn new() -> Self {
        Handler1 { next: None }
    }
}
impl Handler for Handler1 {
    fn set_next(&mut self, next: Box<dyn Handler>) {
        self.next = Some(next);
    }
    fn handle(&self, request: &str) {
        if request == "Request1" {
            println!("Handler1: Handling {}", request);
        } else if let Some(next) = &self.next {
            next.handle(request);
        }
    }
}
// Concrete handler 2
struct Handler2 {
    next: Option<Box<dyn Handler>>,
}
impl Handler2 {
    pub fn new() -> Self {
        Handler2 { next: None }
    }
}
impl Handler for Handler2 {
    fn set_next(&mut self, next: Box<dyn Handler>) {
        self.next = Some(next);
    }
    fn handle(&self, request: &str) {
        if request == "Request2" {
            println!("Handler2: Handling {}", request);
        } else if let Some(next) = &self.next {
            next.handle(request);
        }
    }
}
// Client code
fn main() {
    let mut handler1 = Handler1::new();
    let handler2 = Handler2::new();
    handler1.set_next(Box::new(handler2));
    handler1.handle("Request1");
    handler1.handle("Request2");
}
```


* [Chain Of Responsibilities refactoring.guru](https://refactoring.guru/ru/design-patterns/chain-of-responsibility/rust/example)
* [Chain Of Responsibilities chercher.tech](https://chercher.tech/rust/chain-of-responsibility-design-pattern-rust)
 
---

</details>


<br>
<details>
<summary><b>Template method</b></summary>

`Template method` - определяет скелет алгоритма, перекладывая ответственность за некоторые его шаги на подклассы. 
Паттерн позволяет подклассам переопределять шаги алгоритма, не меняя его общей структуры.
Идея состоит в том, чтобы позволить наследникам абстрактного
шаблона переопределить поведение алгоритмов родителя.
    
Это простой способ изолировать логику в конкретные классы и уменьшить копипаст,
поэтому вы повсеместно встретите его в том или ином виде.

`Template method` предлагает разбить алгоритм на последовательность шагов, описать эти шаги в отдельных методах и 
вызывать их в одном шаблонном методе друг за другом.

Это позволит подклассам переопределять некоторые шаги алгоритма, оставляя без изменений его структуру и 
остальные шаги, которые для этого подкласса не так важны.

Пример:
```rust
trait Drink {
    // Общий алгоритм приготовления, а отличные детали переопределенны
    // Шаблонный метод определяет скелет алгоритма.
    fn template_method(&mut self) {
        self.step_cook_1();
        self.step_cook_2();
        self.step_cook_3();
        self.step_cook_4();
    }
    fn step_cook_2(&mut self);
    fn step_cook_4(&mut self);

    fn step_cook_1(&mut self) {
        println!("Drink step_cook_1");
    }
    fn step_cook_3(&mut self) {
        println!("Drink step_cook_3");
    }
}
struct Coffe;
impl Drink for Coffe {
    fn step_cook_2(&mut self) {
        println!("Coffe step_cook_2");
    }
    fn step_cook_4(&mut self) {
        println!("Coffe step_cook_4");
    }
}
struct Tea;
impl Drink for Tea {
    fn step_cook_1(&mut self) {
        println!("Tea step_cook_1");
    }
    fn step_cook_2(&mut self) {
        println!("Tea step_cook_2");
    }
    fn step_cook_4(&mut self) {
        println!("Tea step_cook_4");
    }
}
fn cook<T: Drink>(obj: Box<&mut T>) {
    obj.template_method();
    println!("");
}
fn main() {
    cook(Box::new(&mut (Tea)));
    cook(Box::new(&mut (Coffe)));
}
/*
Tea step_cook_1
Tea step_cook_2
Drink step_cook_3
Tea step_cook_4

Drink step_cook_1
Coffe step_cook_2
Drink step_cook_3
Coffe step_cook_4
*/
```

Еще пример:
```rust
// Base class defining the template method
trait Algorithm {
    // Template method
    fn execute(&self) {
        self.step_one();
        self.step_two();
        self.step_three();
    }
    fn step_one(&self) {
        println!("Step 1: Common Initialization");
    }
    // Placeholder methods to be overridden in subclasses
    fn step_two(&self);
    fn step_three(&self) {
        println!("Step 3: Common Cleanup");
    }
}
// Concrete implementation
struct ConcreteAlgorithmA;
impl Algorithm for ConcreteAlgorithmA {
    fn step_two(&self) {
        println!("Step 2A: Specific Processing for A");
    }
}
struct ConcreteAlgorithmB;
impl Algorithm for ConcreteAlgorithmB {
    fn step_two(&self) {
        println!("Step 2B: Specific Processing for B");
    }
}
// Client code
fn main() {
    let algo_a = ConcreteAlgorithmA;
    let algo_b = ConcreteAlgorithmB;
    println!("Executing Algorithm A:");
    algo_a.execute();
    println!("\nExecuting Algorithm B:");
    algo_b.execute();
}
```




[Template method refactoring.guru](https://refactoring.guru/ru/design-patterns/template-method)

[Template method www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2023/10/template-method-design-pattern-in-rust.html)

---

</details>

 
---

# Other Design Patterns
- Object Pool Pattern
- Private Class Data	
- Specification
- Delegation
- Service Locator
 
 
## Object Pool Pattern	

`Object Pool Pattern`  предоставляет метод повторного использования инициализированных объектов вместо создания новых.
Из соображений эффективности может быть весьма удобно держать наготове пул (т.е. непустой набор) инициализированных объектов. 
Это может произойти, например, когда у вас есть соединения с базой данных, создание которых требует больших затрат времени и ресурсов.
Это позволяет вызвать определенный объект из пула для использования в течение определенного периода времени, 
а затем вернуть его обратно в пул после выполнения задания. 
Во время отсутствия этого объекта никакие другие компоненты не могут использовать его, пока он не будет возвращен обратно в пул.

Есть несколько crate, таких как [lockfree-object-pool, object-pool](https://crates.io/crates/lockfree-object-pool) и особенно [opool](https://crates.io/crates/opool), которые также реализуют и расширяют эту функциональность.

Пример:
```rust
use std::sync::{Arc, Mutex};
pub trait Identifiable {
    fn get_id(&self) -> i16;
}
struct ConcreteConnection {
    id: i16,
}
impl ConcreteConnection {
    fn new(value: i16) -> Self {
        ConcreteConnection { id: value }
    }
}
impl Identifiable for ConcreteConnection {
    fn get_id(&self) -> i16 {
        self.id
    }
}
impl Drop for ConcreteConnection {
    fn drop(&mut self) {
        println!("Drop ConcreteConnection id:{:?}", self.get_id())
    }
}
pub struct Pool<T> {
    pool: Arc<Mutex<Vec<T>>>,
}
impl<T> Pool<T> {
    pub fn new() -> Self {
        Pool {
            pool: Arc::new(Mutex::new(Vec::new())),
        }
    }
    pub fn add(&mut self, value: T) {
        self.pool.lock().unwrap().push(value);
    }
    pub fn release(&mut self, value: T) {
        self.pool.lock().unwrap().push(value);
    }
    pub fn get(&mut self) -> Option<T> {
        let mut pool = self.pool.lock().unwrap();
        if pool.len() > 0 {
            return pool.pop();
        }
        None
    }
}
impl<T> Drop for Pool<T> {
    fn drop(&mut self) {
        println!("Drop Pool len:{:?}", self.pool.lock().unwrap().len())
    }
}
fn main() {
    let mut pool: Pool<Box<dyn Identifiable>> = Pool::<Box<dyn Identifiable>>::new();
    pool.add(Box::new(ConcreteConnection::new(1)));
    pool.add(Box::new(ConcreteConnection::new(2)));
    pool.add(Box::new(ConcreteConnection::new(3)));
    {
        let connection = pool.get().unwrap();
        let connection2 = pool.get().unwrap();

        println!("Connection id: {}", connection.get_id());
        println!("Connection id: {}", connection2.get_id());

        pool.release(connection2);
    }
    println!("---------------------------");
}
```

[Object Pool www.hackingwithrust.net](https://www.hackingwithrust.net/2023/10/15/an-object-pool-in-rust-two-implementations/)

---

## Private Class Data	

`Pattern Private Class Data` - инкапсуляция аттрибутов класса.

**Намерение**

1. Управление доступом на запись к атрибутам класса
2. Отделяйте данные от методов, которые их используют.
3. Инкапсулировать инициализацию данных класса.

Мотивация для этого шаблона проектирования исходит из цели проектирования — защитить состояние класса путем минимизации видимости его атрибутов (данных).

Шаблон проектирования данных частного класса направлен на уменьшение раскрытия атрибутов за счет ограничения их видимости.

Это уменьшает количество атрибутов класса, инкапсулируя их в один объект данных. 
Это позволяет разработчику класса лишить права записи атрибутов, которые предназначены для установки только во время создания, даже из методов целевого класса.

P.S.
В Rust'е есть возможность "запечатать" [Sealing](https://github.com/Jekahome/Patterns/tree/main/idioms/6.Sealing) реализацию.

Пример:
```rust
#![allow(dead_code)]
use private_samples::Circle;
mod private_samples {
    use private_class_data::CircleSpecs;
    mod private_class_data {
        pub(in crate::private_samples) struct CircleSpecs {
            radius: f64,
            point: String,
        }
        impl CircleSpecs {
            pub fn new(radius: f64, point: String) -> Self {
                Self { radius, point }
            }
            pub fn radius(&self) -> f64 {
                self.radius
            }
            pub fn point(&self) -> &str {
                self.point.as_str()
            }
        }
    }
    // TODO: Circle не сможет получить доступ к приватным полям CircleSpecs и изменить его состояние после создания
    pub struct Circle {
        circle_specs: CircleSpecs,
    }
    impl Circle {
        pub fn new(radius: f64, point: String) -> Self {
            Self {
                circle_specs: CircleSpecs::new(radius, point),
            }
        }
        pub fn circumference(&self) -> f64 {
            self.circle_specs.radius() * std::f64::consts::PI
        }
        pub fn diameter(&self) -> f64 {
            self.circle_specs.radius() * 2.0
        }
    }
}
fn main() {
    let c = Circle::new(4.0, "center".into());
    println!("circumference: {:.2}", c.circumference());
    println!("diameter: {}", c.diameter());
}
```

[Private Class Data sourcemaking.com](https://sourcemaking.com/design_patterns/private_class_data)

---

## Specification
 
`Pattern Specification` предлагает решение, позволяющее создавать многократно используемые бизнес-правила, которые можно комбинировать с использованием логики.

Пример:
```rust
pub trait EmailValidator {
    fn is_valid(&self, email: &EmailAddress) -> bool;
}
use client::EmailAddress;
mod client {
    pub struct EmailAddress {
        user_name: String,
        domain: String,
    }
    impl EmailAddress {
        pub fn new(user_name: String, domain: String) -> Self {
            Self { user_name, domain }
        }
        pub fn user_name(&self) -> &str {
            self.user_name.as_str()
        }
        pub fn domain(&self) -> &str {
            self.domain.as_str()
        }
    }
}
use validators::{DomainValidator, UsernameValidator};
mod validators {
    use super::client::EmailAddress;
    use super::EmailValidator;
    pub struct UsernameValidator {
        pub min_length: usize,
    }
    impl EmailValidator for UsernameValidator {
        fn is_valid(&self, email: &EmailAddress) -> bool {
            email.user_name().len() >= self.min_length
        }
    }
    pub struct DomainValidator {
        pub min_length: usize,
        pub allowed_domains: Vec<String>,
    }
    impl EmailValidator for DomainValidator {
        fn is_valid(&self, email: &EmailAddress) -> bool {
            email.domain().len() >= self.min_length
                && self.allowed_domains.contains(&email.domain().to_owned())
        }
    }
}
use logic::EmailValidatorImpl;
mod logic {
    use super::client::EmailAddress;
    use super::EmailValidator;
    pub struct EmailValidatorImpl {
        validators: Vec<Box<dyn EmailValidator>>,
    }
    impl EmailValidatorImpl {
        pub fn new(validators: Vec<Box<dyn EmailValidator>>) -> Self {
            Self { validators }
        }
        pub fn is_valid(&self, email: &EmailAddress) -> bool {
            self.validators
                .iter()
                .all(|validator| validator.is_valid(email))
        }
    }
}
fn main() {
    let my_email = EmailAddress::new("test".to_string(), "gmail.com".to_string());
    let local_part_validator = UsernameValidator { min_length: 3 };
    let domain_validator = DomainValidator {
        min_length: 3,
        allowed_domains: vec!["gmail.com".to_string()],
    };
    let email_validator = EmailValidatorImpl::new(vec![
        Box::new(local_part_validator),
        Box::new(domain_validator),
    ]);
    println!("{}", email_validator.is_valid(&my_email));
}
```

[Specification www.hackingwithrust.net](https://www.hackingwithrust.net/2023/11/12/simplified-precision-unraveling-the-simple-specification-pattern-in-rust-for-expressive-code-design/)

---

## Delegation	 

`TODO: добавить больше информации, пример, пояснение`

Объект, вместо того чтобы выполнять одну из своих поставленных задач, поручает её связанному вспомогательному объекту.

[Delegation](https://snoekiede.medium.com/easy-delegation-in-rust-the-delegation-pattern-hacking-with-rust-9366f10bf7f2?source=user_profile---------19----------------------------)

---

## Service Locator	

`TODO: добавить больше информации, пример, пояснение`

Для реализации слабосвязанной архитектуры, чтобы получить хорошо тестируемый, сопровождаемый и расширяемый код. 
Паттерн Dependency injection (DI) и Service Locator — это реализация паттерна Inversion of Control (IoC)
Анти-паттерн так как нарушает принцип SOLID инверсии зависимости т.е. он избавляет другие классы от этого но сам зависим от конкретных классов.
В случае изменения данных зависимостей мы рискуем сломать функционал классов, которые их используют, вследствие чего затрудняется поддержка системы.

 