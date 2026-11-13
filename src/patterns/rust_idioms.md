
# Rust idioms

Идиомы — это широко используемые стили, рекомендации и шаблоны, в значительной степени согласованные сообществом. Написание идиоматического кода позволяет другим разработчикам лучше понимать, что происходит.
 
Общая рекомендация, начинать проектировать данные с **ограниченной областью видимости**. Как только элемент crate становится публичным, его нельзя снова сделать приватным, не сломав код, который использует crate, что требует изменение MAJOR версии. Если вы раскрываете внутренние детали реализации структуры данных, предполагаемое будущее изменение для использования более эффективного алгоритма становится критическим изменением. Если вы раскрываете внутренние вспомогательные функции, это неизбежно, что некоторый внешний код станет зависеть от точных деталей этих функций.


## Type safety (Newtype, Typestate) 
  
"Безопасный тип" - это способ ограничить функционал программы конкретным типом, 
что бы компилятор фильтровал некорректные способы использования. 
Покрывая код "безопасными типами" мы увеличиваем шансы корректного использования.

  
<details>
<summary>Newtype</summary>


```rust,no_run,editable
/// 1.1 Newtype
mod no_correct{
    mod user{
        struct User{
            id:i32,
            age:i32,
            name:String
        }
    }
    fn get_user(id:i32) -> User{
        // search db ...
        User{id:id,age:0,name:"".into()}
    }
    fn main(){
        let age:i32 = 68;
        let user:User = get_user(age);// Некорректное использование ф-ции `get_user`
        // Вместо `id` передали `age`
    }
}
mod correct{
    mod user{
        struct UserId(i32);
        impl UserId{
            fn new(id:i32) -> Option<Self>{
                // TODO:валидация ...
                Some(Self(id))
            }
        }
        struct UserAge(i32);
        struct UserName(String);
        struct User{
            id:UserId,
            age:UserAge,
            name:UserName
        }
    }
    fn get_user(id:user::UserId) -> user::User{
        // search db ...
        user::User{id:id,age:user::UserAge(0),name:user::UserName("".into())}
    }
    fn main(){
        let age:user::UserId = user::UserId::new(68).expect("Invalid UserId");
        let user:user::User = get_user(age);// теперь ф-цию `get_user` мы не сможем использовать некорректно
    }
}
```


---

</details>


Предотвращает недопустимое использование данных.
Завернув обший тип в свою оболочку мы получаем свой собственный тип данных,
который неполучится спутать с его внутренным типом и подставить в неверное место,
так же вы можете обеспечить требуемые инварианты для значений типа, ограничив или расширив их.
Также newtype ближе к предметной области и следовательно лучше документировать и понять его применение.
Newtype не несет дополнительных накладных расходов во время выполнения (абстракция с нулевой стоимостью),
но для полноценного использования newtype требуется реализовать трейты (`Clone/Copy,From/Into,AsRef​​/AsMut...`)
 
<br>
<details>
<summary>Вариант основан на преобразовании From</summary>

```rust,editable
#![allow(dead_code)]
///Вариант основан на преобразовании From

/// Состояния
struct New;
struct Unmoderated;
struct Published;
struct Deleted;

/// New --  Unmoderated
impl From<New> for Unmoderated {
    fn from(_val: New) -> Unmoderated {
        Unmoderated
    }
}
/// Unmoderated --  Published
impl From<Unmoderated> for Published {
    fn from(_val: Unmoderated) -> Published {
        Published
    }
}
/// Unmoderated --  Deleted
impl From<Unmoderated> for Deleted {
    fn from(_val: Unmoderated) -> Deleted {
        Deleted
    }
}
/// Published --  Deleted
impl From<Published> for Deleted {
    fn from(_val: Published) -> Deleted {
        Deleted
    }
}
fn new() -> New {
    New
}
fn publish(a: New) -> Unmoderated {
    println!("New -- \"publish()\" --> Unmoderated");
    Unmoderated::from(a)
}
fn allow(a: Unmoderated) -> Published {
    println!("Unmoderated -- \"allow()\" --> Published");
    Published::from(a)
}
fn deny(a: Unmoderated) -> Deleted {
    println!("Unmoderated -- \"deny()\" --> Deleted");
    Deleted::from(a)
}
fn delete(a: Published) -> Deleted {
    println!("Published -- \"delete()\" --> Deleted");
    Deleted::from(a)
}
fn main() {
    let in_new: New = new();
    let in_unmoderated: Unmoderated = publish(in_new);
    let in_published: Published = allow(in_unmoderated);
    let _in_delete: Deleted = delete(in_published);

    println!("\n");

    let in_new: New = new();
    let in_unmoderated: Unmoderated = publish(in_new);
    let _in_delete: Deleted = deny(in_unmoderated);
}
```

---

</details>

<br>
<details>
<summary>Вариант 2, основан на преобразовании From</summary>

```rust,editable
#![allow(dead_code)]
///Вариант основан на преобразовании From
use std::string::String;
mod post {
    #[derive(Clone)]
    pub struct Id(pub u64);

    #[derive(Clone)]
    pub struct Title(pub String);

    #[derive(Clone)]
    pub struct Body(pub String);
}
mod user {
    #[derive(Clone)]
    pub struct Id(pub u64);
}
/// Состояния
struct New {
    post_id: post::Id,
    user_id: user::Id,
    title: post::Title,
    body: post::Body,
}
struct Unmoderated {
    post_id: post::Id,
}
struct Published {
    post_id: post::Id,
}
struct Deleted {
    post_id: post::Id,
}

/// New --  Unmoderated
impl From<New> for Unmoderated {
    fn from(_val: New) -> Unmoderated {
        Unmoderated {
            post_id: _val.post_id,
        }
    }
}
/// Unmoderated --  Published
impl From<Unmoderated> for Published {
    fn from(_val: Unmoderated) -> Published {
        Published {
            post_id: _val.post_id,
        }
    }
}
/// Unmoderated --  Deleted
impl From<Unmoderated> for Deleted {
    fn from(_val: Unmoderated) -> Deleted {
        Deleted {
            post_id: _val.post_id,
        }
    }
}
/// Published --  Deleted
impl From<Published> for Deleted {
    fn from(_val: Published) -> Deleted {
        Deleted {
            post_id: _val.post_id,
        }
    }
}
fn new(post_id: post::Id, user_id: user::Id, title: post::Title, body: post::Body) -> New {
    New {
        post_id: post_id,
        user_id: user_id,
        title: title,
        body: body,
    }
}
fn publish(post: New) -> Unmoderated {
    println!("New -- \"publish()\" --> Unmoderated");
    //Unmoderated::from(post)
    post.into()
}
fn allow(post: Unmoderated) -> Published {
    println!("Unmoderated -- \"allow()\" --> Published");
    //Published::from(post)
    post.into()
}
fn deny(post: Unmoderated) -> Deleted {
    println!("Unmoderated -- \"deny()\" --> Deleted");
    //Deleted::from(post)
    post.into()
}
fn delete(post: Published) -> Deleted {
    println!("Published -- \"delete()\" --> Deleted");
    // Deleted::from(post)
    post.into()
}
fn main() {
    let in_new: New = new(
        post::Id(1),
        user::Id(1),
        post::Title(String::from("Title")),
        post::Body(String::from("Body")),
    );
    let in_unmoderated: Unmoderated = publish(in_new);
    let in_published: Published = allow(in_unmoderated);
    let _in_delete: Deleted = delete(in_published);
    println!("\n");
    let in_new: New = new(
        post::Id(1),
        user::Id(2),
        post::Title(String::from("Title")),
        post::Body(String::from("Body")),
    );
    let in_unmoderated: Unmoderated = publish(in_new);
    let _in_delete: Deleted = deny(in_unmoderated);
}
```


---
 
</details>


<br>
<details>
<summary>Вариант основан на преобразовании From и PhantomData</summary>

```rust,editable
/*
struct Post {
    id: post::Id,
    user_id: user::Id,
    title: post::Title,
    body: post::Body,
}

struct ModerablePost<S>(Post, PhatomData<S>)
*/

#![allow(dead_code)]
///Вариант основан на преобразовании From и PhantomData
use std::marker::PhantomData;
use std::string::String;
/// Сущности
struct User {
    user_id: u64,
    full_name: String,
    email: String,
}
struct Post<S> {
    post_id: u64,
    user: User,
    title: String,
    body: String,
    state: PhantomData<S>,
}
/// Состояния
struct New;
struct Unmoderated;
struct Published;
struct Deleted;
/// New -- Unmoderated
impl From<Post<New>> for Post<Unmoderated> {
    fn from(_val: Post<New>) -> Post<Unmoderated> {
        Post {
            post_id: _val.post_id,
            user: _val.user,
            title: _val.title,
            body: _val.body,
            state: PhantomData,
        }
    }
}
/// Unmoderated -- Published
impl From<Post<Unmoderated>> for Post<Published> {
    fn from(_val: Post<Unmoderated>) -> Post<Published> {
        Post {
            post_id: _val.post_id,
            user: _val.user,
            title: _val.title,
            body: _val.body,
            state: PhantomData,
        }
    }
}
/// Unmoderated -- Deleted
impl From<Post<Unmoderated>> for Post<Deleted> {
    fn from(_val: Post<Unmoderated>) -> Post<Deleted> {
        Post {
            post_id: _val.post_id,
            user: _val.user,
            title: _val.title,
            body: _val.body,
            state: PhantomData,
        }
    }
}
/// Published -- Deleted
impl From<Post<Published>> for Post<Deleted> {
    fn from(_val: Post<Published>) -> Post<Deleted> {
        Post {
            post_id: _val.post_id,
            user: _val.user,
            title: _val.title,
            body: _val.body,
            state: PhantomData,
        }
    }
}
/// Create new Post
/// state New
fn new(user: User, title: String, body: String) -> Post<New> {
    let post: Post<New> = Post {
        post_id: 1u64,
        user: user,
        title: title, // String::from("title"),
        body: body,
        state: PhantomData,
    };
    post
}
fn publish(post: Post<New>) -> Post<Unmoderated> {
    println!("New -- \"publish()\" --> Unmoderated");
    post.into()
}
fn allow(post: Post<Unmoderated>) -> Post<Published> {
    println!("Unmoderated -- \"allow()\" --> Published");
    post.into()
}
fn deny(post: Post<Unmoderated>) -> Post<Deleted> {
    println!("Unmoderated -- \"deny()\" --> Deleted");
    post.into()
}
fn delete(post: Post<Published>) -> Post<Deleted> {
    println!("Published -- \"delete()\" --> Deleted");
    post.into()
}
fn main() {
    let user = User {
        user_id: 1u64,
        full_name: String::from("Egor Egorov"),
        email: String::from("email@mail.ru"),
    };
    let post_new: Post<New> = new(user, String::from("title"), String::from("body"));
    let post_unmoderated: Post<Unmoderated> = publish(post_new);
    let post_published: Post<Published> = allow(post_unmoderated);
    let _post_delete: Post<Deleted> = delete(post_published);
}
```

---

</details>


<br>
<details>
<summary>Вариант основан на преобразовании From и методах структуры</summary>

```rust,editable
///Вариант основан на преобразовании From
///
/// Базовая структура
/// Содержит текущее состояние
struct StateMachine<S> {
    state: S,
}
/// Состояния
/// Следующие состояния могут быть 'S' в StateMachine<S>
struct New;
struct Unmoderated;
struct Published;
struct Deleted;
/// New -- Unmoderated
impl From<StateMachine<New>> for StateMachine<Unmoderated> {
    fn from(_val: StateMachine<New>) -> StateMachine<Unmoderated> {
        println!("New -- Unmoderated");
        StateMachine {
            state: Unmoderated {},
        }
    }
}
/// Unmoderated -- Published
impl From<StateMachine<Unmoderated>> for StateMachine<Published> {
    fn from(_val: StateMachine<Unmoderated>) -> StateMachine<Published> {
        println!("Unmoderated -- Published");
        StateMachine {
            state: Published {},
        }
    }
}
/// Unmoderated -- Deleted
impl From<StateMachine<Unmoderated>> for StateMachine<Deleted> {
    fn from(_val: StateMachine<Unmoderated>) -> StateMachine<Deleted> {
        println!("Unmoderated -- Deleted");
        StateMachine { state: Deleted {} }
    }
}
/// Published -- Deleted
impl From<StateMachine<Published>> for StateMachine<Deleted> {
    fn from(_val: StateMachine<Published>) -> StateMachine<Deleted> {
        println!("Published -- Deleted");
        StateMachine { state: Deleted {} }
    }
}
/// Вариант основан на методах структуры
/// Передача владения в методах обеспечивает продвижение состояния
/// начальное состояние
impl StateMachine<New> {
    fn new() -> Self {
        StateMachine { state: New {} }
    }
    fn publish(self) -> StateMachine<Unmoderated> {
        println!("New -- \"publish()\" --> Unmoderated");
        StateMachine {
            state: Unmoderated {},
        }
    }
}
impl StateMachine<Unmoderated> {
    fn allow(self) -> StateMachine<Published> {
        println!("Unmoderated -- \"allow()\" --> Published");
        StateMachine {
            state: Published {},
        }
    }
    fn deny(self) -> StateMachine<Deleted> {
        println!("Unmoderated -- \"deny()\" --> Deleted");
        StateMachine { state: Deleted {} }
    }
}
impl StateMachine<Published> {
    fn delete(self) -> StateMachine<Deleted> {
        println!("Published -- \"delete()\" --> Deleted");
        StateMachine { state: Deleted {} }
    }
    //Вариант для реализованного типажа From
    fn _delete(val: StateMachine<Published>) -> StateMachine<Deleted> {
        println!("Published -- \"delete()\" --> Deleted");
        val.into()
    }
}
fn main() {
    println!("Через реализацию типажа From:\n");
    //let _in_delete = StateMachine::<Published>::delete_(in_published);

    let in_new: StateMachine<New> = StateMachine::<New>::new();
    let in_unmoderated: StateMachine<Unmoderated> = StateMachine::<Unmoderated>::from(in_new);
    let in_published: StateMachine<Published> = StateMachine::<Published>::from(in_unmoderated);
    //let in_deleted:StateMachine<Deleted> = StateMachine::<Deleted>::from(in_unmoderated);
    let _in_deleted: StateMachine<Deleted> = StateMachine::<Deleted>::from(in_published);

    println!("\nЧерез методы StateMachine<S>:\n");
    let in_new: StateMachine<New> = StateMachine::<New>::new();
    let in_unmoderated: StateMachine<Unmoderated> = in_new.publish();
    let in_published: StateMachine<Published> = in_unmoderated.allow();
    //let _in_delete = in_unmoderated.deny();
    let _in_delete: StateMachine<Deleted> = in_published.delete();
    println!("\nЦепочка:\n");
    StateMachine::<New>::new().publish().allow().delete();
    println!("\nЦепочка:\n");
    StateMachine::<New>::new().publish().deny();
}
```

---

</details>

<br>
<details>
<summary>Вариант основан на enum</summary>

```rust,editable
/// Продвижение состояния обеспечивает enum
/// Текущее состояние
#[derive(Debug)]
struct StateMachine<S> {
    shared_value: bool,
    state: S,
}
/// Состояния
#[derive(Debug)]
struct New;
#[derive(Debug)]
struct Unmoderated;
#[derive(Debug)]
struct Published;
#[derive(Debug)]
struct Deleted;
/// Родитель обеспечивает единственное состояние на текущий момент
#[derive(Debug)]
enum StateMachineWrapper {
    New(StateMachine<New>),
    Unmoderated(StateMachine<Unmoderated>),
    Published(StateMachine<Published>),
    Deleted(StateMachine<Deleted>),
}
#[derive(Debug)]
struct Factory {
    machine: StateMachineWrapper,
}
impl Factory {
    fn new() -> Self {
        Factory {
            machine: StateMachineWrapper::New(StateMachine::new(true)),
        }
    }
    fn get_state(&self) -> &StateMachineWrapper {
        &self.machine
    }
    fn get_state_str(&self) -> &str {
        match &self.machine {
            StateMachineWrapper::New(_) => "New",
            StateMachineWrapper::Unmoderated(_) => "Unmoderated",
            StateMachineWrapper::Published(_) => "Published",
            StateMachineWrapper::Deleted(_) => "Deleted",
        }
    }
    fn is_end(&self) -> bool {
        match &self.machine {
            StateMachineWrapper::Deleted(_val) => true,
            _ => false,
        }
    }
}
/// Переходы состояния
impl StateMachineWrapper {
    fn step(mut self) -> Self {
        self = match self {
            StateMachineWrapper::New(val) => StateMachineWrapper::Unmoderated(val.into()),
            StateMachineWrapper::Unmoderated(val) => {
                if val.shared_value == true {
                    StateMachineWrapper::Published(val.into())
                } else {
                    StateMachineWrapper::Deleted(val.into())
                }
            }
            StateMachineWrapper::Published(val) => StateMachineWrapper::Deleted(val.into()),
            StateMachineWrapper::Deleted(val) => StateMachineWrapper::Deleted(val.into()),
        };
        self
    }
    /// Изменить shared_value
    fn set_shared(&mut self, shared_value: bool) {
        match self {
            StateMachineWrapper::New(val) => {
                val.shared_value = shared_value;
            }
            StateMachineWrapper::Unmoderated(val) => {
                val.shared_value = shared_value;
            }
            StateMachineWrapper::Published(val) => {
                val.shared_value = shared_value;
            }
            StateMachineWrapper::Deleted(val) => {
                val.shared_value = shared_value;
            }
        };
    }
}
// New
impl StateMachine<New> {
    fn new(shared_value: bool) -> Self {
        StateMachine {
            shared_value: shared_value,
            state: New {},
        }
    }
}
// New -- Unmoderated
impl From<StateMachine<New>> for StateMachine<Unmoderated> {
    fn from(_val: StateMachine<New>) -> StateMachine<Unmoderated> {
        StateMachine {
            shared_value: _val.shared_value,
            state: Unmoderated {},
        }
    }
}
// Unmoderated -- Published
impl From<StateMachine<Unmoderated>> for StateMachine<Published> {
    fn from(_val: StateMachine<Unmoderated>) -> StateMachine<Published> {
        StateMachine {
            shared_value: _val.shared_value,
            state: Published {},
        }
    }
}
// Unmoderated -- Deleted
impl From<StateMachine<Unmoderated>> for StateMachine<Deleted> {
    fn from(_val: StateMachine<Unmoderated>) -> StateMachine<Deleted> {
        StateMachine {
            shared_value: _val.shared_value,
            state: Deleted {},
        }
    }
}
// Published -- Published
impl From<StateMachine<Published>> for StateMachine<Deleted> {
    fn from(_val: StateMachine<Published>) -> StateMachine<Deleted> {
        StateMachine {
            shared_value: _val.shared_value,
            state: Deleted {},
        }
    }
}
fn main() {
    let mut the_factory = Factory::new();

    println!("\nПроход №1\n");
    println!("state:{}\n", the_factory.get_state_str()); // state:New
    the_factory.machine = the_factory.machine.step();
    println!("state:{}\n", the_factory.get_state_str()); // state:Unmoderated
                                                         // the_factory.machine.set_shared(false);
    the_factory.machine = the_factory.machine.step();
    println!("state:{}\n", the_factory.get_state_str()); //state:Published
    the_factory.machine = the_factory.machine.step();
    println!("state:{}\n", the_factory.get_state_str()); // state:Deleted
    the_factory.machine = the_factory.machine.step();
    println!("state:{}\n", the_factory.get_state_str()); // state:Deleted

    println!("\nПроход №2\n");
    let mut the_factory = Factory::new();
    while the_factory.is_end() == false {
        println!("state:{}\n", the_factory.get_state_str());
        the_factory.machine = the_factory.machine.step();
    }
    println!("Step:{}\n", "Deleted");
}
```

---

</details>

Подход "состояние типа" позволяет контролировать последовательность смены состояний при выполнении программы еще на этапе компиляции. Мы можем описать поведение типа,  какие состояния он может принимать и в какой последовательности.

## Mem replace (hook lifetime)

Поскольку Rust подразумевает семантику перемещения по умолчанию и довольно строгие правила заимствования, часто возникают ситуации (особенно с большими `structs` и `enums`), когда изменение значения на месте или замена значений не могут быть разрешены средством проверки заимствований, что довольно сбивает с толку и приводит к к созданию ненужных клонов (что приводит к избыточным затратам на производительность).

<details>

<summary>...</summary>

### Завладеть данными не используя Clone

Средство проверки заимствований не позволит нам извлечь из name перечисления (потому что что- то должно быть там). 
Мы, конечно, могли бы `.clone()` назвать и поместить клон в наш `MyEnum::B`, но это будет экземпляр антипаттерна «Клон для проверки заимствований». 
В любом случае, мы можем избежать лишнего распределения, изменив `e` только изменяемое заимствование.

```rust,editable
use std::fmt::Debug;
use std::default::Default;
use std::mem;

#[derive(Debug,PartialEq)]
enum MyEnum {
    A { name: String, x: u8 },
    B { name: String }
}
fn a_to_b(e: &mut MyEnum) {
    if let MyEnum::A{ name, x:_x @ 3...7 } = e {
        // это извлекает наше имя и вставляет вместо него пустую строку 
        // (обратите внимание, что пустые строки не выделяются). 
        // Затем создаем новый вариант перечисления (который будет 
        // присваивается `* e`)
        
        *e = MyEnum::B{ name: mem::take(name) }
        //*e = MyEnum::B{ name: mem::replace(name,Default::default()) }
    }
}
fn main() {
    let mut e = MyEnum::A{name:"foo".to_owned(),x:4};
    a_to_b(&mut e);
    assert_eq!(e,MyEnum::B{name:"foo".to_owned()});
    println!("{:?}",e);
}
```

### Сохранение принадлежащих значений в измененных перечислениях

При работе с перечислениями нам может потребоваться изменить значение перечисления на месте, возможно, на другой вариант.

[Rust Design Patterns: mem::replace to keep owned values in changed enums](https://github.com/rust-unofficial/patterns/blob/main/src/idioms/mem-replace.md)

```rust,no_run,editable
enum MyEnum {
    A { name: String },
    B { name: String },
}
fn swizzle(e: &mut MyEnum) {
    use self::MyEnum::*;
    *e = match *e {
        // Ownership rules do not allow taking `name` by value, but we cannot
        // take the value out of a mutable reference, unless we replace it:
        A { ref mut name } => B { name: mem::replace(name, String::new()) },
        B { ref mut name } => A { name: mem::replace(name, String::new()) },
    }
}
```

### Обойти двойное заимствование

```rust,editable
use std::default::Default;
use std::mem;
use std::collections::HashSet;

#[derive(Debug,PartialEq)]
struct Names {
    exclusions: Vec<String>,
    names: HashSet<String>,
}
impl Names {
    //fn apply_exclusions(&mut self) {
    // error: for_each уже позаимствовал self
    //    self.exclusions.drain(..).for_each(|name| {
    //        self.names.remove(&name);
    //    })
    //}
     fn apply_exclusions(&mut self) {
        let mut names = mem::replace(&mut self.names, HashSet::new());
        self.exclusions.drain(..).for_each(|name| {
            names.remove(&name);
        });
        mem::replace(&mut self.names, names);
    }
}
fn main() {
    let mut e = Names{exclusions:vec!["Einar".to_owned(), "Olaf".to_owned(), "Harald".to_owned()],
    names: [ "Einar".to_owned(), "Olaf".to_owned(), "Harald".to_owned() ].iter().cloned().collect::<HashSet<String>>()};
    e.apply_exclusions();
    println!("{:?}",e);
}
```

### Мутация внутренней коллекции

```rust,no_run,editable
struct Names {
    exclusions: Vec<String>,
    names: HashSet<String>,
}
impl Names {
    fn apply_exclusions(&mut self) {
        self.exclusions.drain(..).for_each(|name| {
            self.names.remove(&name);
        })
    }
}
``` 

```text
который не компилируется из-за двух изменяемых заимствований:

error[E0500]: closure requires unique access to `self` but it is already borrowed
  --> src/main.rs:10:44
   |
10 |         self.exclusions.drain(..).for_each(|name| {
   |         ---------------           -------- ^^^^^^ closure construction occurs here
   |         |                         |
   |         |                         first borrow later used by call
   |         borrow occurs here
11 |             self.names.remove(&name);
   |             ---- second borrow occurs due to use of `self` in closure

```

```text
Использование mem::replaceздесь позволяет нам практически бесплатно избежать проблемы двух изменяемых заимствований ( HashSet::new() это без операций), заменяя значение во временной переменной:

impl Names {
    fn apply_exclusions(&mut self) {
        let mut names = mem::replace(&mut self.names, HashSet::new());
        self.exclusions.drain(..).for_each(|name| {
            names.remove(&name);
        });
        mem::replace(&mut self.names, names);
    }
}
```

Еще пример:

```rust,editable
#[derive(Clone, Debug, PartialEq, Default)]
struct Trinity<T> {
    a: T,
    b: T,
    c: T,
}
impl<T> Trinity<T> {
    fn rotate(&mut self) {
        std::mem::swap(&mut self.a, &mut self.b);
        std::mem::swap(&mut self.b, &mut self.c);
    }
}

#[derive(Debug)]
struct Solver<T> {
    expected: Trinity<T>,
    unsolved: Vec<Trinity<T>>,
}
impl<T: PartialEq + Default> Solver<T> {
    fn resolve(&mut self) {
        let mut unsolved = Vec::with_capacity(self.unsolved.len());
        'l: for t in self.unsolved.iter_mut() {
            for _ in 0..3 {
                if *t == self.expected {
                    continue 'l;
                }
                t.rotate();
            }
            unsolved.push(std::mem::take(t))
        }
        self.unsolved = unsolved;
    }
}
fn main() {
    let mut s = Solver {
        expected: Trinity { a: 1, b: 2, c: 3 },
        unsolved: vec![
            Trinity { a: 1, b: 2, c: 3 },
            Trinity { a: 2, b: 1, c: 3 },
            Trinity { a: 2, b: 3, c: 1 },
            Trinity { a: 3, b: 1, c: 2 },
        ],
    };
    s.resolve();
    println!("{:?}", s)
}
```

[rust-incubator](https://github.com/instrumentisto/rust-incubator/tree/main/2_idioms/2_2_mem_replace)

[Official mem::replace docs](https://doc.rust-lang.org/std/mem/fn.replace.html)

[Official mem::swap docs](https://doc.rust-lang.org/std/mem/fn.swap.html)

[Official mem::take docs](https://doc.rust-lang.org/std/mem/fn.take.html)

[Karol Kuczmarski: Moving out of a container in Rust](http://xion.io/post/code/rust-move-out-of-container.html)

---

</details>

## Bound impl (ограничение поведения)

Описывайте bond сигнатурой не данные, а поведение т.е. методы.
Размещение границ признаков на `impl` блоках, методах и функциях, а не на типах, уменьшает загрязнение границ признаков, уменьшает связанность частей кода и делает общий код более чистым, простым и эргономичным.

Структуры данных не должны дублировать производные границы признаков.
Вы должны попытаться максимально поднять границы признаков (особенно в коде библиотеки), поскольку это расширяет возможности использования типа.

<details>
<summary>...</summary>
 
### Конкретизиируйте/опысывайте поведение, а не данные

Описывайте bond сигнатурой не данные, а поведение т.е. методы

Размещение границ признаков на impl блоках, методах и функциях, а не на типах, уменьшает загрязнение границ признаков, уменьшает связанность частей кода и делает общий код более чистым, простым и эргономичным.

```rust,no_run,editable
trait UserRepo{}

// Плохо -------------------------------------------------------------------------------------
struct UserService<R: UserRepo> {
    repo: R,
}
```

Мы указываем `R: UserRepo` здесь `bound`, поскольку мы хотим ограничить типы в `repo` поле для реализации `UserRepo` поведения.
Однако такое ограничение непосредственно на тип приводит к так называемому «загрязнению границ признаков»: мы должны повторять эту границу в каждом отдельном случае `impl`, даже в тех, которые вообще не имеют отношения к UserRepo поведению.

```rust,no_run,editable
impl<R> Display for UserService<R>
where
    R: Display + UserRepo, // Здесь нас не интересует UserRepo,  все, что нам нужно, это просто Display.
{                          
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UserService with repo {}", self.repo)
    }
}
```

В сложной кодовой базе такое загрязнение умножается из-за разных типов и в какой-то момент может стать кошмаром.

Решением этой проблемы было бы понимание того, что черта представляет собой определенное поведение, и на самом деле нам нужно это поведение только тогда, когда мы его декларируем . 
В объявлении типа ничего не говорится о поведении, все дело в данных. 
Именно в функциях и методах происходит поведение. 
Итак, давайте просто ожидаем определенного поведения, когда нам это действительно нужно:
 
```rust,no_run,editable
// Хорошо -------------------------------------------------------------------------------------
 
struct UserService<R> {
    repo: R,
}

// Ожидайте отображения, когда мы выражаем поведение отображения.
impl<R: Display> Display for UserService<R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UserService with repo {}", self.repo)
    }
}

// Ожидайте UserRepo, когда мы выражаем фактическое поведение UserService, который имеет дело с пользователями.
impl<R: UserRepo> UserService<R> {
    fn activate(&self, user: User) {
        // Изменение состояния пользователя в UserRepo ...
    }
}
```

### Структуры данных не дублируют производные границы признаков

```rust,ro_run,editable
// Хорошо -------------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq)]
struct Good<T> { /* ... */ }

// Плохо ---------------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq)]
struct Bad<T: Clone + Debug + PartialEq> { /* ... */ }
```

Исключения

Есть три исключения, когда требуются границы признаков для структур:

- Структура данных относится к ассоциированному типу признака.
- Граница есть `?Sized`.(потому что атоматически компилятор ставит `Sized`)
- В структуре данных есть `Drop` имплант, требующий границ признака. 
 В настоящее время Rust требует, чтобы `Drop` в структуре данных присутствовали все границы признаков в `impl`.


### Уберите не нужные границы

Поднять границы признаков для расширения возможностей использования типа

```rust,ro_run,editable
// Вы должны попытаться максимально поднять границы признаков (особенно в коде библиотеки), поскольку это расширяет возможности использования типа.
// Плохо -------------------------------------------------------------------------------------

#[derive(Clone)]
struct Loader<K, V> {
    state: Arc<Mutex<State<K, V>>>,
}
struct My;

let loader: Loader<My, My> = ..;
let copy = loader.clone(); // compile error as `My` doesn't impl `Clone`
```

Это происходит из-за того, что `#[derive(Clone)]` применяются к `K: Clone` и `V: Clone` в производном коде, несмотря на то, что они вообще не нужны

```rust,ro_run,editable
// Хорошо -------------------------------------------------------------------------------------
// Предоставляя ручную реализацию, мы можем `Loader<My, My>` без проблем клонировать значения типа:

struct Loader<K, V> {
    state: Arc<Mutex<State<K, V>>>,
}
// Ручная реализация используется для исключения применения ненужных границ клонирования.
impl<K, V> Clone for Loader<K, V> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}

let loader: Loader<My, My> = ..;
let copy = loader.clone(); // it compiles now!
```
 
Пример:

```rust,editable
use std::{
    borrow::{Borrow, BorrowMut},
    num::NonZeroU64,
};
fn main() {
    println!("Refactor me!");
}
/// A projected state built from a series of events.
pub trait Aggregate: Default {
    /// A static string representing the type of the aggregate.
    ///
    /// Note: This should effectively be a constant value, and should never change.
    fn aggregate_type() -> &'static str;

    /// Consumes the event, applying its effects to the aggregate.
    fn apply<E>(&mut self, event: E)
    where
        E: AggregateEvent<Self>,
    {
        event.apply_to(self);
    }
}
/// An identifier for an aggregate.
pub trait AggregateId<A>
where
    A: Aggregate,
{
    /// Gets the stringified aggregate identifier.
    fn as_str(&self) -> &str;
}

/// A thing that happened.
pub trait Event {
    /// A static description of the event.
    fn event_type(&self) -> &'static str;
}

/// An event that can be applied to an aggregate.
pub trait AggregateEvent<A: Aggregate>: Event {
    /// Consumes the event, applying its effects to the aggregate.
    fn apply_to(self, aggregate: &mut A);
}

/// Represents an event sequence number, starting at 1
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EventNumber(NonZeroU64);

impl EventNumber {
    /// The minimum [EventNumber].
    #[allow(unsafe_code)]
    pub const MIN_VALUE: EventNumber =
    // One is absolutely non-zero, and this is required for this to be usable in a `const` context.
        EventNumber(unsafe { NonZeroU64::new_unchecked(1) });

    /// Increments the event number to the next value.
    #[inline]
    pub fn incr(&mut self) {
        self.0 = NonZeroU64::new(self.0.get() + 1).unwrap();
    }
}
/// An aggregate version.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Version {
    /// The version of an aggregate that has not had any events applied to it.
    Initial,
    /// The version of the last event applied to the aggregate.
    Number(EventNumber),
}

impl Default for Version {
    #[inline]
    fn default() -> Self {
        Version::Initial
    }
}
impl Version {
    /// Creates a new `Version` from a number.
    ///
    /// The number `0` gets interpreted as being `Version::Initial`, while any other number is interpreted as the
    /// latest event number applied.
    #[inline]
    pub fn new(number: u64) -> Self {
        NonZeroU64::new(number)
            .map(EventNumber)
            .map(Version::Number)
            .unwrap_or(Version::Initial)
    }
    /// Increments the version number to the next in sequence.
    #[inline]
    pub fn incr(&mut self) {
        match *self {
            Version::Initial => *self = Version::Number(EventNumber::MIN_VALUE),
            Version::Number(ref mut en) => en.incr(),
        }
    }
}

/// An aggregate that has been loaded from a source, which keeps track of the version of its last snapshot and the current version of the aggregate.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct HydratedAggregate<A> {
    version: Version,
    snapshot_version: Option<Version>,
    state: A,
}
impl<A> HydratedAggregate<A>
where
    A: Aggregate,
{
    /// The current version of the aggregate.
    pub fn version(&self) -> Version {
        self.version
    }

    /// The version of the snapshot from which the aggregate was loaded.
    pub fn snapshot_version(&self) -> Option<Version> {
        self.snapshot_version
    }

    /// Updates the snapshot version. Generally used to indicate that a snapshot was taken.
    pub fn set_snapshot_version(&mut self, new_snapshot_version: Version) {
        self.snapshot_version = Some(new_snapshot_version);
    }

    /// The actual aggregate.
    pub fn state(&self) -> &A {
        &self.state
    }

    /// Applies a sequence of events to the internal aggregate.
    pub fn apply_events<E: AggregateEvent<A>, I: IntoIterator<Item = E>>(&mut self, events: I) {
        for event in events {
            self.apply(event);
        }
    }

    /// Applies a single event to the aggregate, keeping track of the new aggregate version.
    pub fn apply<E: AggregateEvent<A>>(&mut self, event: E) {
        self.state.apply(event);
        self.version.incr();
    }
}
impl<A> AsRef<A> for HydratedAggregate<A> {
    fn as_ref(&self) -> &A {
        &self.state
    }
}
impl<A> Borrow<A> for HydratedAggregate<A> {
    fn borrow(&self) -> &A {
        &self.state
    }
}
/// An identified, specific instance of a hydrated aggregate.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct Entity<I, A> {
    id: I,
    aggregate: HydratedAggregate<A>,
}
impl<I, A> Entity<I, A>
where
    A: Aggregate,
    I: AggregateId<A>,
{
    /// Creates a new entity from an identifier and an associated hydrated aggregate.
    pub fn new(id: I, aggregate: HydratedAggregate<A>) -> Self {
        Entity { id, aggregate }
    }
    /// The entity's identifier.
    pub fn id(&self) -> &I {
        &self.id
    }
    /// An immutable reference to the underlying aggregate.
    pub fn aggregate(&self) -> &HydratedAggregate<A> {
        &self.aggregate
    }
    /// A mutable reference to the underlying aggregate.
    pub fn aggregate_mut(&mut self) -> &mut HydratedAggregate<A> {
        &mut self.aggregate
    }
}
impl<I, A> From<Entity<I, A>> for HydratedAggregate<A> {
    fn from(entity: Entity<I, A>) -> Self {
        entity.aggregate
    }
}
impl<I, A> AsRef<HydratedAggregate<A>> for Entity<I, A> {
    fn as_ref(&self) -> &HydratedAggregate<A> {
        &self.aggregate
    }
}
impl<I, A> AsMut<HydratedAggregate<A>> for Entity<I, A> {
    fn as_mut(&mut self) -> &mut HydratedAggregate<A> {
        &mut self.aggregate
    }
}
impl<I, A> Borrow<HydratedAggregate<A>> for Entity<I, A> {
    fn borrow(&self) -> &HydratedAggregate<A> {
        &self.aggregate
    }
}
impl<I, A> Borrow<A> for Entity<I, A> {
    fn borrow(&self) -> &A {
        self.aggregate.borrow()
    }
}
impl<I, A> BorrowMut<HydratedAggregate<A>> for Entity<I, A> {
    fn borrow_mut(&mut self) -> &mut HydratedAggregate<A> {
        &mut self.aggregate
    }
}
```


[rust-incubator](https://github.com/instrumentisto/rust-incubator/tree/main/2_idioms/2_3_bound_impl)

[Зацепление связанность](https://en.wikipedia.org/wiki/Coupling_(computer_programming))

---

</details>

## Generic in type out (абстракция аргументов)

Абстрагируйтесь от конкретного типа входных данных строки, что бы увеличить гибкость API, принимая как можно больше различных типов. Типажи в помощь `AsRef, Borrow, Into, Cow`
Обратной стороной этой идиомы является то, что компилятор генерирует больше кода из-за мономорфизации, что потенциально приводит к раздуванию кода. Каноническое решение этой проблемы — исключить внутренний метод, содержащий весь код за вычетом универсальных преобразований, и оставить внешний метод в качестве оболочки.

<details>
<summary>...</summary>


Абстрагируйтесь от конкретного типа входных данных строки, что бы увеличить гибкость API, принимая как можно больше различных типов.
  
Типаж `AsRef` используется в обобщённом коде для преобразования некоторого значения в ссылку на внутренние данные структуры.

```rust,no_run,editable
fn take_a_str(some: impl AsRef<str>) { // тоже самое `fn take_a_str<T:AsRef<str>>(some: T) {...`
    let some = some.as_ref();
    println!("{some}");
}

use core::fmt::Debug;
fn take_a_str_into(some: impl Into<String>+Debug) {
    println!("{:?}",some);
}

fn main() {
    take_a_str("str");
    take_a_str("String".to_string());
    
    // also `&String` is supported:
    let string_ref = "StringRef".to_string();
    take_a_str(&string_ref);

    take_a_str_into("str");
    take_a_str_into("String".to_string());
    let string_ref = "StringRef".to_string();
    take_a_str_into(&string_ref);
}
```

Типаж `Borrow` когда цель абстрагироваться типа владения, нам нужно владеть значением или ссылаться ссылкой, нам это неважно, любой тип `T, &T, &mut T` нам подойдут

```rust,no_run,editable
fn validate(email:&str)->bool{
    true
}
pub fn new<S: Into<String> + AsRef<str>>(email: S) -> Option<String> {
        if  validate(email.as_ref()) {
             Some(email.into())
        } else {
             None
        }
}
fn main() {
  new("email");
  new("email".to_string());
}
```

```rust,no_run,editable
use std::borrow::Borrow;
fn check_borrow<T: Borrow<str>+std::fmt::Display>(s: T) {
    validate(s.borrow());
    print!("{}",s);
}
fn validate(value:&str)->bool{
    true
}
fn main() {
    check_borrow("Hello".to_string());
    check_borrow("Hello");
}
```

Ф-ция возвращает `String` или `&str`  

```rust,no_run,editable
use std::borrow::Cow;
// Если входные данные будут мутироваться то вернем String иначе возвращать входную строку &str
// Т.е. клонирование при мутации Cow, отложите выделение памяти как можно дольше.

// вариант с AsRef т.е. можно String присылать
fn remove_spaces<'a>(input: &'a(impl AsRef<str> + ?Sized)) -> Cow<'a, str> { // <'a> что бы можно было вернуть  Cow<'a...
    let input = input.as_ref();
    if input.contains(' ') {
        input
        .chars()
        .filter(|&x| x != ' ')
        .collect::<std::string::String>()
        .into()
    } else {
        // input.into() // Into<Cow<'a, str>> 
        // или полный синтаксис
        Into::<Cow<'_, str>>::into(input)  
    }
} 
fn main(){
    let s = remove_spaces("Herman"); // Cow::Borrowed  
    let len = s.len(); // impl Deref
    let owned: String = s.into_owned(); // memory is allocated for a new string

    let binding = "Herman Radtke".to_string();
    let s = remove_spaces(&binding); // Cow::Owned 
    let len = s.len(); // impl Deref
    let owned: String = s.into_owned(); // no new memory allocated as we already had a String
}
```

Уменьшение раздутия кода, оптимизация

Обратной стороной этой идиомы является то, что компилятор генерирует больше кода из-за мономорфизации, что потенциально приводит к раздуванию кода.

Статическая диспетчеризация с параметрами типа имеет недостаток: генерируется довольно много кода (для каждого типа), увеличивается размер двоичного файла и потенциально пессимизируется использование кэша выполнения. Однако часто дженерики на самом деле нужны не для скорости, а для эргономики.

Каноническое решение этой проблемы — исключить внутренний метод, содержащий весь код за вычетом универсальных преобразований, и оставить внешний метод в качестве оболочки.

```rust,no_run,editable
#[inline]
pub fn this<I: Into<String>>(i: I) -> usize {
    _this_inner(i.into())
}
fn _this_inner(i: String) -> usize {
    // same code as above without the conversion
}
```

[rust-incubator](https://github.com/instrumentisto/rust-incubator/tree/main/2_idioms/2_4_generic_in_type_out)

---

</details>


## Exhaustivity (проверка на полноту вариантов)

Проверка на полноту вариантов.
Расширяя кодовую базу уже имеющихся перечислений и структур, есть шанс не учесть новые поля в коде, что может привести к ошибке во время выполнения. Для того чтобы отловить эту ошибку на этапе компиляции следует придерживаться следующих рекомендаций. Использовать полный список вариантов перечислений и пользоваться полями созданной струтурой,а не отдельными ее полями.


<details>
<summary>...</summary>
  
Проверка на полноту вариантов.
Расширяя кодовую базу уже имеющихся перечислений и структур, есть шанс не учесть новые поля в коде, что может привести к ошибке во время выполнения. Для того чтобы отловить эту ошибку на этапе компиляции следует придерживаться следующих рекомендаций. Использовать полный список вариантов перечислений и пользоваться полями созданной струтурой,а не отдельными ее полями.

Используйте полный список вариантов перечисления.
Не используйте подстановочный знак `_` в match выражениях. Так как при добавлении нового вариант в перечисление оно не будет корректно учтено в логиге кода.

```rust,no_run,editable
fn grant_permissions(role: &Role) -> Permissions {
    match role {
        Role::Reporter => Permissions::Read,
        Role::Developer => Permissions::Read & Permissions::Edit,
        _ => Permissions::All, // anybody else is administrator 
    }
}
```

Следует заменить на полный список вариантов перечисления и тогда при добавлении нового варианта код не скомпилируется, что даст вам повод исправить код.

```rust,no_run,editable
fn grant_permissions(role: &Role) -> Permissions {
    match role {
        Role::Reporter => Permissions::Read,
        Role::Developer => Permissions::Read & Permissions::Edit,
        Role::Admin => Permissions::All, 
    }
}
```

Для структур так же следует использовать полный список полей с помощью деструктуризации.
В случае добавления нового поля, код не скомпилируется, что даст вам повод для его исправить.

```rust,no_run,editable
struct Address {
    country: Country,
    city: City,
    street: Street,
    zip: Zip,
}

// Плохо
impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.country)?;
        writeln!(f, "{}", self.city)?;
        writeln!(f, "{}", self.street)?;
        write!(f, "{}", self.zip)
    }
}

// Хорошо
impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            country,
            city,
            street,
            zip,
        } = self;
        writeln!(f, "{country}")?;
        writeln!(f, "{city}")?;
        writeln!(f, "{street}")?;
        write!(f, "{zip}")
    }
}
```

[rust-incubator](https://github.com/instrumentisto/rust-incubator/tree/main/2_idioms/2_5_exhaustivity)

---

</details>


## Sealing (запечатывание реализации)

Запечатывание в программировании обычно означает, что некоторые API (в основном общедоступные) не могут быть унаследованы, расширены или реализованы за пределами места их определения.
Основная цель запечатывания признака — это перспективность API. Так как мы уверены в том что только мы используем все имплементации запечатанного трейта то мы можем его менять.
Sealed трейт — общедоступный трейт, который не может быть реализован вне места его определения (модуля или крейта , в зависимости от видимости этого трейта).


<details>
<summary>...</summary>
 
Запечатывание в программировании обычно означает, что некоторые API (в основном общедоступные) не могут быть унаследованы, расширены или реализованы за пределами места их определения.
Основная цель запечатывания признака — это перспективность API. Так как мы уверены в том что только мы используем все имплементации запечатанного трейта то мы можем его менять.

`Sealed` трейт — общедоступный трейт, который не может быть реализован вне места его определения (модуля или крейта, в зависимости от видимости этого трейта).

```rust,no_run,editable
/// This trait is sealed and cannot be implemented for types outside this crate.
pub trait TheTrait: private::Sealed {
    // Zero or more methods that the user is allowed to call.
    fn ...();

    // Zero or more private methods, not allowed for user to call.
    #[doc(hidden)]
    fn ...();
}

// Implement for some types.
impl TheTrait for usize {
    /* ... */
}

mod private {
    pub trait Sealed {}

    // Implement for those same types, but no others.
    impl Sealed for usize {}
}
```

Пустой частный `Sealed` супертрейт не может быть унаследован последующими крейтами, поэтому мы гарантируем, что реализации `Sealed` (и, следовательно `TheTrait`) существуют только в текущем крейте.


Крейт [sealed](https://docs.rs/sealed) предоставляет удобный макрос для запечатывания.

```rust,no_run,editable
use sealed::sealed;

#[sealed]
pub trait TheTrait {}

#[sealed]
impl TheTrait for usize {}
```

### Однако существуют альтернативные способы запечатать признак через [сигнатуру его метода](https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust/#sealing-traits-via-method-signatures) или даже частично запечатать его

```rust,no_run,editable
mod private {
    pub struct Token;
}

pub trait SealedTrait {
    fn method(&self, _: private::Token);
}

pub struct TypeThatImplsSealed;

impl SealedTrait for TypeThatImplsSealed {
    fn method(&self, _: private::Token) {
        // impl here
    }
}
```

И при этом код вне модуля не сможет, ни реализовать признак, ни вызвать метод

```rust,no_run,editable
struct DownstreamType {}

impl upstream::SealedTrait for DownstreamType {
    // ERROR: module `private` is private
    fn method(&self, token: upstream::private::Token) {}
}

fn call_method(value: impl upstream::SealedTrait) {
    // ERROR: module `private` is private
    let token = upstream::private::Token;
    value.method(token);
}
```

### Разрешить вызов только некоторых методов

Это позволяет нам явно решить, какие методы должны быть невызываемыми из последующих крейтов. 

```rust,no_run,editable
mod upstream{
    mod private {
        pub struct Token;
    }

    pub trait SealedTrait {
        fn callable_method(&self);

        fn non_callable_method(&self, _: private::Token);
    }
}

// Ничто не мешает вызывать эти методы из последующих крейтов.

fn call_method(value: impl upstream::SealedTrait) {
    value.callable_method();
}
```

### Частично запечатанные черты

Чтобы сделать `SealedTrait` частично изолированной — нам просто нужно предоставить реализацию по умолчанию для всех методов, которые принимают аргумент, который последующие ящики не могут реализовать.

```rust,no_run,editable
mod private {
    pub struct Token;
}

pub trait PartiallySealedTrait {
    fn callable_method(&self);

    fn non_callable_method(&self, _: private::Token) {
        println!("you can't change this");
    }
}

// Здесь вы, возможно, заметили, что непереопределяемый метод нашей особенности также не может быть вызван последующими крейтами. Однако мы можем разрешить нижестоящим ящикам вызывать его косвенно, если наш ящик предоставит такую ​​функцию

pub fn call_method_indirectly(value: &PartiallySealedTrait) {
    value.non_callable_method(private::Token)
}
```

[rust-incubator](https://github.com/instrumentisto/rust-incubator/tree/main/2_idioms/2_6_sealing)

[Rust API Guidelines: 10. Future proofing: Sealed traits protect against downstream implementations (C-SEALED)](https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed)

[Predrag Gruevski: A definitive guide to sealed traits in Rust](https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust)

[Jack Wrenn: Private Methods on a Public Trait](https://jack.wrenn.fyi/blog/private-trait-methods)

---

</details>


## Конструктор с помощью ф-ции `new` и конструктор по умолчанию `Default`

В Rust нет конструкторов как языковых конструкций. Вместо этого принято использовать ассоциированную функцию new для создания объекта

<details>
<summary>...</summary>

```rust,editable,no_run
/// Time in seconds.
///
/// # Example
///
/// ```
/// let s = Second::new(42);
/// assert_eq!(42, s.value());
/// ```
pub struct Second {
    value: u64
}

impl Second {
    // Constructs a new instance of [`Second`].
    // Note this is an associated function - no self.
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    /// Returns the value in seconds.
    pub fn value(&self) -> u64 {
        self.value
    }
}
```

**Конструктор по умолчанию**

```rust,editable,no_run
/// Time in seconds.
///
/// # Example
///
/// ```
/// let s = Second::default();
/// assert_eq!(0, s.value());
/// ```
// или #[derive(Default)]
pub struct Second {
    value: u64
}

impl Second {
    /// Returns the value in seconds.
    pub fn value(&self) -> u64 {
        self.value
    }
}

impl Default for Second {
    fn default() -> Self {
        Self { value: 0 }
    }
}

```
</details>

## Трейт `Default` в Rust

`Default` — это трейт, который определяет метод `default()`, возвращающий "значение по умолчанию" для типа.
Это полезно, когда мы хотим создать **объект без указания всех значений вручную**.

Rust не умеет абстрагироваться по признаку "у типа есть `new()`", но зато может абстрагироваться по признаку "у типа есть `Default`".

<details>
<summary>...</summary>

Когда полезно:
* Для **генериков** — можно создавать значение типа без знания конкретных полей.
* Для **структур** с множеством полей — не нужно явно инициализировать всё.
* Для **контейнеров** (`Vec`, `String`, `Option`) — у них уже есть свои дефолтные состояния.
* Можно использовать в функциях, например `unwrap_or_default()`.
 
**Пример с авто-derive**

Если все поля структуры реализуют `Default`, можно просто написать:

```rust,editable
#[derive(Default, Debug)]
struct Config {
    // Option — по умолчанию None
    output: Option<String>,
    // Vec — по умолчанию пустой вектор
    search_path: Vec<String>,
    // bool — по умолчанию false
    check: bool,
}

fn main() {
    // Создание с дефолтными значениями
    let mut conf = Config::default();
    println!("{conf:?}");

    // Частичная инициализация
    let conf2 = Config {
        check: true,
        ..Default::default()
    };
    println!("{conf2:?}");
}
```

**Реализация вручную**

Если поля не имеют `Default`, можно реализовать самому:

```rust,editable
struct Point {
    x: i32,
    y: i32,
}
impl Default for Point {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}
fn main() {
    let p = Point::default();
    println!("({}, {})", p.x, p.y);
}
```

**Использование с generics**

`Default` полезен, когда функция не знает конкретный тип:

```rust,editable
fn make_default<T: Default>() -> T {
    T::default()
}
fn main() {
    let v: Vec<i32> = make_default(); // пустой вектор
    let n: i32 = make_default();      // 0
    println!("{v:?}, {n}");
}
```

Плюсы:
* Не нужно вручную инициализировать все поля.
* Можно легко создавать дефолтные значения в generics.
* Удобно для тестов и шаблонов настроек.

Минусы:
* Только один `Default` на тип (нельзя сделать несколько "разных дефолтов").
* Не подходит, если нужна сложная логика инициализации с параметрами.

</details>

## Динамическая диспетчеризация на стеке

Динамическая диспетчеризация на стеке — это способ эффективно и без аллокации, работать с разными типами, реализующими один интерфейс, без необходимости создавать дополнительные структуры.

В отличие от дженериков, где компилятор создаёт отдельный код для каждого типа (статическая диспетчеризация), динамическая диспетчеризация на стеке позволяет писать единый код.

Это снижает размер бинарника и ускоряет компиляцию.

<details>
<summary>...</summary>

Чтобы увеличить время жизни объекта при необходимости, можно использовать отложенную условную инициализацию, как показано ниже:

```rust,editable,no_run
use std::io;
use std::fs;

// Каждая переменная содержит значения только одного типа. 
// В нашем примере stdin имеет тип Stdin, 
// file имеет тип File и readable имеет тип &mut dyn Read
let (mut stdin_read, mut file_read);

let readable: &mut dyn io::Read = if arg == "-" {
    stdin_read = io::stdin();
    &mut stdin_read
} else {
    file_read = fs::File::open(arg)?;
    &mut file_read
};
```

**Преимущества**

Нам не нужно ничего размещать в куче. 
Нам также не нужно инициализировать что-то, что мы не будем использовать позже, и нам не  нужно мономорфизировать весь следующий код для работы с обоими `File` или `Stdin`.

Версия аналогичного кода но с выделением памяти в куче:

```rust,editable,no_run
let readable: Box<dyn io::Read> = if arg == "-" {
    Box::new(io::stdin())
} else {
    Box::new(fs::File::open(arg)?)
};
```
</details>

## Передача переменных в замыкание 

(используйте перепривязку переменных в отдельной области)

По умолчанию замыкания захватывают свое окружение путем заимствования. 
Или вы можете использовать `move-closure` для перемещения всей среды. 
Однако часто требуется переместить в замыкание только некоторые переменные, передать ему копию некоторых данных, передать их по ссылке или выполнить какое-либо другое преобразование.

Для этого используйте перепривязку переменных в отдельной области.

<details>
<summary>...</summary>

```rust,editable,no_run
use std::rc::Rc;

let num1 = Rc::new(1);
let num2 = Rc::new(2);
let num3 = Rc::new(3);

// Хорошо
let closure = {
    // `num1` is moved
    let num2 = num2.clone();  // `num2` is cloned
    let num3 = num3.as_ref();  // `num3` is borrowed
    move || {
        *num1 + *num2 + *num3;
    }
};

// Плохо
let num2_cloned = num2.clone();
let num3_borrowed = num3.as_ref();
let closure = move || {
    *num1 + *num2_cloned + *num3_borrowed;
};

```

**Преимущества**

Скопированные данные группируются вместе с определением замыкания, поэтому их назначение становится более ясным, и они будут немедленно удалены, даже если они не будут использованы замыканием.

Замыкание использует те же имена переменных, что и окружающий код, независимо от того, копируются или перемещаются данные.

</details>

## Временная мутабельность

(заменить мутабельную переменню на не мутабельную путем переопределения переменной)

Часто необходимо подготовить и обработать некоторые данные, но после этого данные только проверяются и никогда не изменяются. Намерение можно сделать явным, переопределив изменяемую переменную как неизменяемую.

Это можно сделать либо путем обработки данных внутри вложенного блока, либо путем переопределения переменной.

<details>
<summary>...</summary>

Скажем, вектор перед использованием необходимо отсортировать.

**Использование вложенного блока:**
```rust,editable,no_run
let data = {
    let mut data = get_vec();
    data.sort();
    data
};

// Here `data` is immutable.
```

**Использование перепривязки переменных:**
```rust,editable,no_run
let mut data = get_vec();
data.sort();
let data = data;

// Here `data` is immutable.
```
</details>

## Возвращать использованный аргумент при ошибке

Если функция потребляет (перемещает) аргумент, верните этот аргумент обратно внутри ошибки.
Что бы не засталять пользователей клонировать эти данные в случае необходимости использовать как-то повторно эти данные.

<details>
<summary>...</summary>

```rust,editable,no_run
pub struct SendError(String);

pub fn send(value: String) -> Result<(), SendError> {
    println!("using {value} in a meaningful way");
    // Simulate non-deterministic fallible action.
    use std::time::SystemTime;
    let period = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    if period.subsec_nanos() % 2 == 1 {
        Ok(())
    } else {
        Err(SendError(value))
    }
}
```
</details>

## Используйте заимствованные (borrow) типы для аргументов

**Коэрция аргументов (Argument Coercion) в Rust**
 
**Коэрция аргументов** — это механизм, благодаря которому компилятор Rust автоматически преобразует переданный тип в ожидаемый тип функции, когда это возможно. Обычно это происходит с помощью `Deref`-коэрции, позволяющей, например, ссылке на `String` (`&String`) автоматически приводиться к `&str`.
 
Например, `&String` может автоматически превратиться в `&str`, `&Vec<T>` в `&[T]`, а `&Box<T>` в `&T`.

Но преобразование в обратную сторону не всегда возможно (например, `&str → &String` не сработает).

Поэтому, когда вы пишете функции, всегда лучше принимать заимствованный тип (`&str` вместо `&String`, `&[T]` вместо `&Vec<T>`, `&T` вместо `&Box<T>`)

<details>
<summary>...</summary>

Зачем это нужно:

* **Повышает гибкость API** — функция сможет принимать больше видов входных данных (литералы строк, срезы, строки в куче и т. д.).
* **Снижает количество уровней косвенных ссылок** — например, `String` уже хранит данные в куче (слой косвенности), а `&String` добавляет ещё один слой. При `&str` этого лишнего слоя нет.
* **Нет лишних аллокаций** — преобразование `String → &str` дешёвое (не требует клонирования/выделения памяти), а `&str → &String` требует выделения памяти.

Итоговое правило:
* Принимайте минимально необходимый тип, который описывает ваши требования.
* Если нужна только возможность чтения строки — берите `&str`, а не `&String`.
* То же самое с `Vec<T>` к (`&[T]`) и `Box<T>` к (`&T`).

**Пример использования аргумента ф-ции как `&str` или `String`**:

```rust,editable
// Если нужна только ссылка (чтение)
// Используйте AsRef<str> — функция будет принимать всё, 
// что можно превратить в &str (String, &String, &str, Arc<str>, и т. д.):
fn take_a_str(some: impl AsRef<str>) { // или fn take_a_str<S: AsRef<str>>(some: S){...
    let some = some.as_ref();
    println!("{some}");
}

// Если нужно владение (String внутри функции)
// Используйте Into<String> — функция примет и String, и &str (второе будет скопировано):
use core::fmt::Debug;
fn take_a_str_into(some: impl Into<String>+Debug){//или fn take_a_str_into<S: Into<String>+Debug>(some: S){..
    println!("{:?}",some);
}
fn main() {
    take_a_str("str");
    take_a_str("String".to_string());
    
    // also `&String` is supported:
    let string_ref = "StringRef".to_string();
    take_a_str(&string_ref);

    take_a_str_into("str");
    take_a_str_into("String".to_string());
    let string_ref = "StringRef".to_string();
    take_a_str_into(&string_ref);
}
```

**Обобщенный вариант для заимствования `&Vec<T>` и `&[T]`**

```rust,editable
fn process<S, T>(data: S)
where
    S: AsRef<[T]>,
    T: std::fmt::Debug,
{
    for item in data.as_ref() {
        println!("{:?}", item);
    }
}
fn main() {
    let v = vec![1, 2, 3];
    let arr = [4, 5, 6];

    process(&v);   // &Vec<i32>
    process(&arr); // &[i32]
}
```

**Обобщенный вариант для владения (чтобы функция принимала и потребляла данные, а не только заимствовала)**

```rust,editable
fn process<S, T>(data: S)
where
    S: Into<Vec<T>>,
    T: std::fmt::Debug,
{
    let owned_vec: Vec<T> = data.into();
    for item in owned_vec {
        println!("{:?}", item);
    }
}
fn main() {
    let v = vec![1, 2, 3];
    let arr = [4, 5, 6];
    let boxed: Box<[i32]> = vec![7, 8, 9].into_boxed_slice();

    process(v);       // Vec<i32> — перемещаем
    process(arr);     // [i32; N] — копирует элементы
    process(boxed);   // Box<[i32]> — без копирования
}
```
</details>

## Объединение строк с помощью `format!`

<details>
<summary>...</summary>

Строки можно создавать: 
- используя методы `push` и `push_str` для изменяемого объекта `String`. Это наиболее эффективно но неудобно.  
  ```rust,editable
  fn main(){
    let mut result = "Hello ".to_owned();
    result.push_str(name);
    result.push('!');
  }
    ```
- или с помощью оператора + у `String`
  ```rust,editable
  fn main(){
    let hello = String::from("Hello");
    let world = String::from("world");
    let rust = " in Rust";

    let sentence = hello + " " + &world + rust;
  }
  ```

- однако часто удобнее использовать [`format!`](https://doc.rust-lang.org/std/macro.format.html)
  ```rust,editable
  fn main(){
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s:String = format!("{}-{s2}-{s4}", s1, s4=s3); // tic-tac-toe  
  } 
  ```
</details>

## Коллекции — это умные указатели

В Rust коллекции, такие как `Vec<T>` или `String`, **владеют** своими данными, а срезы (`&[T]` или `&str`) — это **заимствованные** (borrowed) представления этих данных.

Чтобы удобно работать с коллекциями и их заимствованными версиями, в Rust используется трейт `Deref`. Он позволяет автоматически преобразовывать ссылку на коллекцию в ссылку на срез — это называется **авто-доступом через deref**.

<details>
<summary>...</summary>

Пример с `Vec` и `Deref`:

```rust,editable,no_run
use std::ops::Deref;

struct Vec<T> {
    data: RawVec<T>,
    // ...
}

impl<T> Deref for Vec<T> {
    type Target = [T]; // целевой тип — срез

    fn deref(&self) -> &[T] {
        // возвращаем ссылку на внутренний срез
    }
}
```

* `Vec<T>` — владеет данными
* `&[T]` — это срез (borrowed view) на те же данные

Благодаря `Deref` можно вызывать методы срезов на `&Vec<T>` напрямую, например:

```rust,editable
let v = vec![1, 2, 3];
let slice: &[i32] = &v; // благодаря Deref это работает
println!("Длина: {}", slice.len()); // метод среза вызван через Vec
```

Аналогично для `String` и `&str`:

* `String` — владеет строкой
* `&str` — срез строки (borrowed view)

Реализация `Deref<Target=str>` позволяет использовать методы `&str` напрямую на `&String`.

Зачем это нужно?

* **Удобство API**: большинство методов реализованы для заимствованных типов (`&[T]`, `&str`), и благодаря `Deref` мы можем использовать эти методы и на владеющих типах (`Vec<T>`, `String`).
* **Гибкость**: позволяет работать либо с владением данных, либо с их заимствованием, не переписывая код.
* **Избежание дублирования кода**: методы пишутся один раз для срезов, но работают и для коллекций.

Преимущества:
* Можно писать методы только для заимствованных типов (срезов), и они автоматически доступны для коллекций.
* Клиенты API могут выбирать — владеть данными или просто заимствовать их.

Недостатки:
* При дженериках и ограничениях по трейтам методы, доступные только через `Deref`, могут не учитываться при проверке ограничений.
* Это усложняет некоторую обобщённую программную логику (есть трейты `Borrow` и `AsRef` для решения таких задач).

Итог:

Коллекции в Rust — это как умные указатели, которые владеют данными, а трейт `Deref` позволяет обращаться к этим данным через удобные заимствованные представления. Это одна из мощных особенностей Rust, делающая API гибким и удобным без лишних накладных расходов.

</details>

## Финализация в деструкторах

В Rust **нет** `finally` как в JavaScript, Java или Python — блока кода, который всегда выполняется при выходе из функции (вне зависимости от того, вернулись мы через `return`, `?` или случился `panic!`).

Вместо этого можно использовать **деструкторы** (`Drop`) — код, который выполняется, когда объект выходит из области видимости (**scope**).

<details>
<summary>...</summary>

```rust,editable
fn baz() -> Result<(), ()> {
    // Здесь могла быть логика, но просто вернем ошибку
    Err(())
}
fn bar() -> Result<(), ()> {
    struct Finalizer;

    impl Drop for Finalizer {
        fn drop(&mut self) {
            println!("Функция bar завершена!");
        }
    }
    // Создаем объект, деструктор которого выполнится при выходе из функции
    let _finalizer = Finalizer;

    // Если baz() вернет Err, `?` сделает ранний выход — деструктор всё равно вызовется
    baz()?;

    println!("Завершаем bar() обычным образом");
    Ok(())
}
// Даже если `baz()` вернет ошибку и мы выйдем через `?`, деструктор сработает.
fn main() {
    let _ = bar();
    // output: Функция bar завершена!
}
```

Почему это полезно:

* Позволяет **гарантированно** (почти) выполнить очистку или логирование при **любом** выходе из функции.
* Удобно, если внутри много `return` или используется `?`.
* Можно применять для освобождения ресурсов: закрытия файлов, сокетов, снятия блокировок (mutex).

Ограничения:

* **Не 100% гарантия** — деструктор **не сработает**, если:

  * Программа застряла в бесконечном цикле.
  * Произошел `panic!` **во время** уже выполняющегося `panic!` (double panic).
  * Произошло аварийное завершение процесса.
* Деструктор не должен сам вызывать `panic!` — это приведет к аварийному завершению потока.
* Чтобы деструктор вызвался в конце функции, объект нужно хранить **в переменной**, а не создать и сразу выбросить.

**RAII (Resource Acquisition Is Initialization)**

Этот подход — частный случай RAII-паттерна:
* Ресурсы (файлы, блокировки, соединения) **захватываются** при создании объекта.
* Ресурсы **освобождаются** в `Drop`.

Пример с мьютексом: мьютекс разблокируется в деструкторе `MutexGuard`, даже если произойдет `panic!` в середине блока.

```rust,editable
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(0));
    let data_clone = Arc::clone(&data);
    thread::spawn(move || {
        let mut num = data_clone.lock().unwrap(); // lock захвачен
        *num += 1;
        // lock освободится автоматически при выходе из области видимости
    }).join().unwrap();

    println!("data = {:?}", *data.lock().unwrap());
}
```

**Перехват panic для освобождения ресурсов или записи логов**

В Rust освобождение ресурсов основано на **RAII** (Resource Acquisition Is Initialization): как только объект выходит из области видимости, вызывается его `Drop`.
При `panic!` поведение зависит от профиля компиляции и настройки `panic`:


```toml
Cargo.toml:
[profile.dev]
panic = "unwind"

[profile.release]
panic = 'abort' 
```

* **`panic = "unwind"`** (по умолчанию в `dev`):

  * Происходит **разворачивание стека** (stack unwinding).
  * Rust проходит по каждому фрейму стека в обратном порядке вызова.
  * Для всех живых объектов вызывается их `Drop`.
  * Можно перехватить панику с помощью `std::panic::catch_unwind`.

* **`panic = "abort"`** (по умолчанию в `release` в некоторых проектах):

  * Программа немедленно завершается.
  * **`Drop` не вызывается**.
  * Никакой очистки ресурсов не происходит.

Когда это полезно:
* Закрытие файлов или сетевых соединений.
* Освобождение памяти, выделенной не через Rust (например, через FFI).
* Освобождение мьютексов (чтобы избежать deadlock).
* Откат изменений в системах, где важно оставить корректное состояние.

```rust,editable
use std::fs::File;
use std::io::{self, Write};

struct TempFile {
    path: String,
}
impl Drop for TempFile {
    fn drop(&mut self) {
        println!("Удаляю временный файл: {}", self.path);
        let _ = std::fs::remove_file(&self.path);
    }
}
fn do_work() -> io::Result<()> {
    let _temp = TempFile { path: "temp.txt".into() };
    let mut file = File::create("temp.txt")?;
    writeln!(file, "Важные данные")?;

    // Симуляция ошибки
    panic!("Что-то пошло не так!");

    // _temp удалится даже при панике (только при panic = "unwind")
}
fn main() {
    let result = std::panic::catch_unwind(|| {
        do_work().unwrap();
    });

    if result.is_err() {
        println!("Поймали панику, ресурсы освобождены");
    }
}
```

Как сделать надёжно:

1. **Если ресурс критичен** — используй `catch_unwind` вокруг кода, чтобы гарантировать выполнение `Drop` даже при панике.
2. **В production-сборках**:

   * Если стоит `panic = "abort"`, освобождение ресурсов через `Drop` не сработает.
   * Для действительно критичных операций (закрытие транзакций в БД, сохранение данных) стоит явно вызывать очистку **до** паники.
3. Не паникуй внутри `Drop` — при панике во время `unwind` Rust **немедленно завершит программу**.

Сводка:

| Настройка `panic` | Drop вызывается? | Можно перехватить `panic`? |
| ----------------- | ---------------- | -------------------------- |
| `"unwind"`        | ✅ Да             | ✅ `catch_unwind`           |
| `"abort"`         | ❌ Нет            | ❌                          |

---

</details>

## `mem::{take(_), replace(_)}` для сохранения собственных значений в измененных enum'ов


В Rust мы **не можем просто так "забрать"** значение поля из `&mut`-ссылки на enum,
потому что:

* Компилятор требует, чтобы в поле *всегда* что-то лежало.
* Нельзя оставить "дырку" вместо значения.

<details>
<summary>...</summary>

```rust,no_run,editable
enum MyEnum {
    A { name: String, x: u8 },
    B { name: String },
}

fn bad(e: &mut MyEnum) {
    if let MyEnum::A { name, x: 0 } = e {
        let taken_name = *name; // ❌ ошибка: нельзя "забрать" из &mut
    }
}
```

**Решение: заменить, а потом забрать**

Rust предлагает два стандартных способа:

**`mem::take`**

* Заменяет значение **на его `Default`** (например, `String` → пустая строка).
* Возвращает старое значение.

```rust,no_run,editable
use std::mem;

fn a_to_b(e: &mut MyEnum) {
    if let MyEnum::A { name, x: 0 } = e {
        *e = MyEnum::B {
            name: mem::take(name), // заменили на пустую строку и забрали старую
        };
    }
}
```

Для `String` пустая строка не аллоцирует память — поэтому это быстрый способ.

**`mem::replace`**
* Заменяет значение на **то, что вы явно указали**.
* Не требует `Default`.

```rust,no_run,editable
use std::mem;

fn a_to_b_with_replace(e: &mut MyEnum) {
    if let MyEnum::A { name, x: 0 } = e {
        *e = MyEnum::B {
            name: mem::replace(name, String::from("default_name")),
        };
    }
}
```

**Более сложный пример: перестановка вариантов**

```rust,no_run,editable
use std::mem;

enum MultiEnum {
    A { name: String },
    B { name: String },
    C,
    D,
}
fn swizzle(e: &mut MultiEnum) {
    use MultiEnum::*;
    *e = match e {
        A { name } => B { name: mem::take(name) },
        B { name } => A { name: mem::take(name) },
        C => D,
        D => C,
    };
}
```

Здесь `mem::take` позволяет переставить значения между вариантами **без `.clone()` и аллокаций**.


Когда использовать
* **Изменить enum на месте**, сохранив какое-то поле в новом варианте.
* Избежать лишнего `.clone()` и аллокаций.
* Работать с типами, у которых есть `Default` (для `take`) или вручную указать замену (для `replace`).


| Метод          | Чем заменяет?             | Требует `Default`? |
| -------------- | ------------------------- | ------------------ |
| `mem::take`    | `T::default()`            | ✅ Да               |
| `mem::replace` | Значение, переданное вами | ❌ Нет              |

</details>

 
## Итерация по Option

В Rust `Option<T>` можно рассматривать как **контейнер, в котором либо 0, либо 1 элемент**.
Поскольку `Option` реализует трейты **`IntoIterator`** и **`Iterator` для ссылок**, с ним можно работать так же, как с векторами или другими коллекциями.

<details>
<summary>...</summary>

**`Option` в `.extend()`**

Если у вас есть коллекция, вы можете добавить в неё элемент из `Option` без `if let`:

```rust,no_run,editable
let turing = Some("Turing");
let mut logicians = vec!["Curry", "Kleene", "Markov"];

// Добавим элемент, если он есть
logicians.extend(turing);

// Эквивалентно:
if let Some(inner) = turing {
    logicians.push(inner);
}
```

**`Option` в `.chain()`**

Если хотите добавить элемент в конец существующего итератора:

```rust,no_run,editable
let turing = Some("Turing");
let logicians = vec!["Curry", "Kleene", "Markov"];

// Склеиваем итератор вектора с итератором Option
for name in logicians.iter().chain(turing.iter()) {
    println!("{name} is a logician");
}
```

Если элемент **всегда есть**, лучше использовать `std::iter::once`:

```rust,no_run,editable
for name in logicians.iter().chain(std::iter::once(&"Turing")) {
    println!("{name} is a logician");
}
```

**`for` по `Option`**

Можно итерироваться по `Option` напрямую в цикле `for`:

```rust,no_run,editable
let opt = Some("Rust");

for value in opt {
    println!("Value: {value}");
}

// Эквивалентно:
if let Some(value) = opt {
    println!("Value: {value}");
}
```

Хотя так можно, **`if let` обычно читается лучше**.

**Преобразование `Option` в итератор вручную**
* `option.into_iter()` — забирает значение из `Option`, потребляя её.
* `option.iter()` — даёт итератор по ссылке (`&T`).
* `option.iter_mut()` — даёт итератор по изменяемой ссылке (`&mut T`).


```rust,no_run,editable
let mut opt = Some(10);

for x in opt.iter_mut() {
    *x += 5;
}

assert_eq!(opt, Some(15));
```
 
Дополнительно:
* **`std::iter::once`** — создаёт итератор с ровно одним элементом (удобно, если точно знаете, что он есть).
* **`Iterator::filter_map`** — фильтрует и преобразует элементы в `Option`, убирая `None`.
* **crate `ref_slice`** — умеет превращать `Option<&T>` в слайс длиной 0 или 1.

</details>

## [non_exhaustive] и приватные поля для расширяемости

Существует небольшой набор сценариев, в которых автор библиотеки может захотеть добавить открытые поля в открытую структуру или новые варианты в перечисление, не нарушая при этом обратную совместимость.

Rust предлагает два решения этой проблемы:
- Используется `#[non_exhaustive]` для structs, enums и enumвариантов. 
- Вы можете добавить частное поле к структуре, чтобы предотвратить ее прямое создание или сопоставление с ней

<details>
<summary>...</summary>

Представьте, что ыы написали библиотеку с таким enum:

```rust,no_run,editable
pub enum Shape {
    Circle(f64),
    Square(f64),
}
```

Кто-то использует ваш код и пишет:

```rust,no_run,editable
match shape {
    Shape::Circle(r) => println!("radius {r}"),
    Shape::Square(s) => println!("side {s}"),
}
```

Если завтра вы добавите `Shape::Triangle(f64)`, этот код **сломается** при компиляции (не все варианты покрыты).

Но иногда вам **нужно** иметь возможность добавлять варианты/поля без мажорного релиза.
Вот тут и помогает `#[non_exhaustive]`

1. Для **enum**

```rust,no_run,editable
#[non_exhaustive]
pub enum Shape {
    Circle(f64),
    Square(f64),
}
```

Теперь пользователь **обязан** писать `_` или `..` в `match`, чтобы обработать будущее расширение:

```rust,no_run,editable
match shape {
    Shape::Circle(r) => println!("circle {r}"),
    Shape::Square(s) => println!("square {s}"),
    _ => println!("something new!"),
}
```

Пользователь не сможет исчерпывающе перечислить все варианты — это гарантия, что вы сможете добавить новые.

2. Для **struct**

```rust,no_run,editable
#[non_exhaustive]
pub struct Config {
    pub name: String,
}
```

Теперь нельзя будет создать его напрямую:

```rust,no_run,editable
// ❌ Ошибка — конструктор недоступен
let cfg = Config { name: "hi".into() };
```

Придётся использовать `..`:

```rust,no_run,editable
let Config { name, .. } = cfg;
```

Это гарантирует, что при добавлении новых полей старый код не сломается.

**Альтернатива внутри одного крейта — приватное поле**

Если `#[non_exhaustive]` нужен только **внутри** текущего крейта, можно просто добавить приватное поле:

```rust,no_run,editable
pub struct Config {
    pub name: String,
    _private: (),
}
```

Теперь:
* Пользователь **не может** распаковать все поля без `..`
* Не может сам создать `Config`
* Вы можете безопасно добавлять поля

Когда это уместно

✅ Хорошо:
* API, моделирующие **внешние ресурсы**, которые могут измениться (например, HTTP-ответы или JSON-схемы)
* Когда **важно** не ломать обратную совместимость и не выпускать major-версию

Осторожно:
* Пользователи будут вынуждены писать обработку "неизвестных" случаев, часто с `panic!()` или заглушками
* Это может сделать код менее удобным

Иногда проще просто выпустить **новую major-версию**, чем усложнять API `#[non_exhaustive]`.

---

</details>

## Простая инициализация примера в документации

В доках к методам вам нужно показать рабочий пример,
но для этого нужно **много кода инициализации**, который не относится к сути примера.

<details>
<summary>...</summary>

Например, чтобы вызвать метод `send_request`, вам приходится в каждой доке повторять:

```rust,no_run,editable
struct Connection {
    name: String,
    stream: TcpStream,
}

impl Connection {
    /// Sends a request over the connection.
    ///
    /// # Example
    /// &#96;&#96;&#96;no_run
    /// # // Boilerplate are required to get an example working.
    /// # let stream = TcpStream::connect("127.0.0.1:34254");
    /// # let connection = Connection { name: "foo".to_owned(), stream };
    /// # let request = Request::new("RequestId", RequestType::Get, "payload");
    /// let response = connection.send_request(request);
    /// assert!(response.is_ok());
    /// &#96;&#96;&#96;
    fn send_request(&self, request: Request) -> Result<Status, SendErr> {
        // ...
    }

    /// Oh no, all that boilerplate needs to be repeated here!
    fn check_status(&self) -> Status {
        // ...
    }
}
```

Это:
* Делает пример **длинным**
* Отвлекает читателя от основной идеи метода
* Усложняет сопровождение (меняется инициализация — переписывай везде)


**Решение — обёрточная функция**

Вместо того, чтобы повторять код, выносим всю инициализацию в функцию-хелпер, а в примерах просто показываем вызов метода.

```rust,no_run,editable
/// # Example
/// &#96;&#96;&#96;
/// # fn call_send(connection: Connection, request: Request) {
/// let response = connection.send_request(request);
/// assert!(response.is_ok());
/// # }
/// &#96;&#96;&#96;
fn send_request(&self, request: Request) { /* ... */ }
```

Тут `call_send` — это **не вызываемая** функция, которая скрывает весь подготовительный код.
Rustdoc проверит, что он компилируется, но тестировать его не будет, так как мы его не вызываем.

Плюсы
* Примеры становятся **короче** и чище.
* Нет копипаста — меняем инициализацию только в одном месте.
* Компиляция всё ещё проверяется (`cargo test` без запуска).

Минусы
* Тесты внутри такой функции не выполняются.
* Если нужно именно проверить поведение (assert), надо либо вызывать функцию в тестах, либо использовать другой подход.

**Альтернатива для тестируемых примеров**

Если нужно, чтобы код **выполнялся** при `cargo test`, можно:

* сделать `#[doc(hidden)] pub fn test_connection() -> Connection` внутри библиотеки;
* использовать его в доках как часть публичного API.

Пример:

```rust,no_run,editable
#[doc(hidden)]
pub fn test_connection() -> Connection {
    Connection {
        name: "foo".into(),
        stream: TcpStream::connect("127.0.0.1:34254").unwrap(),
    }
}
```

И в доках:

```rust,no_run,editable
/// &#96;&#96;&#96;
/// let conn = mycrate::test_connection();
/// let status = conn.check_status();
/// assert_eq!(status, Status::Ok);
/// &#96;&#96;&#96;
```
</details>

## [Идиомы FFI](https://rust-unofficial.github.io/patterns/idioms/ffi/intro.html)

 

---

* [Rust idioms rust-incubator](https://github.com/instrumentisto/rust-incubator/tree/main/2_idioms)
* [Rust idioms rust-unofficial](https://rust-unofficial.github.io/patterns/idioms/index.html)
* [Rust idioms rust-unofficial github](https://github.com/Jekahome/RustDesignPatterns/tree/main/src/idioms)
* [Реализация конечного автомата (переходы между состояниями)](https://hoverbear.org/blog/rust-state-machine-pattern/)
* [Typestate шаблон Состояние](https://doc.rust-lang.ru/book/ch17-03-oo-design-patterns.html)

 