
# Database Patterns
Шаблоны баз данных, сохранять и извлекать данные из баз данных и устанавливать соответствие между объектами базы данных и приложения.

- Active Record
- Identity Map
- Data Mapper
- Repository
- Unit of Work (UOW)
- Lazy Load
 
---

## Active Record	

`TODO: добавить больше информации, пример, пояснение`

Схема Active Record — это подход к доступу к данным в базе данных. Таблица базы данных или представление обёрнуты в классы. 
Таким образом, объектный экземпляр привязан к единственной строке в таблице. После создания объекта новая строка будет добавляться к таблице на сохранение. 
Любой загруженный объект получает свою информацию от базы данных. Когда объект обновлён, соответствующая строка в таблице также будет обновлена. 
Класс обёртки реализует методы средства доступа или свойства для каждого столбца в таблице или представлении.
Нарушает принцип единственной ответственности (SRP) или нет ?

---

## Identity Map	

(`Identity Map => Data Mapper => Repository => Unit of Work`)

`TODO: добавить больше информации, пример, пояснение`

Обеспечивает однократную загрузку объекта, сохраняя данные об объекте в карте соответствия. 
При обращении к объектам, ищет их в карте соответсвия.
Паттерн `Identity Map` (Карта присутствия / Карта соответствия) хранит записи о всех объектах, которые были считаны из БД за время выполнения одного действия. 
Когда происходит обращение к объекту, проверяется карта соответствия (присутствия), чтобы узнать, загружен ли объект.
В простом случае с изоморфной схемой у вас будет одна карта для каждой таблицы базы данных. 
Когда вы загружаете объект из базы данных, вы сначала проверяете карту. Если в нем есть объект, соответствующий тому, который вы загружаете, вы его возвращаете. Если нет, вы обращаетесь к базе данных и помещаете объекты на карту для дальнейшего использования по мере их загрузки.

Намерение

Гарантирует, что каждый объект загружается только один раз, сохраняя каждый загруженный объект на карте. Ищет объекты, используя карту при обращении к ним.
Карта идентичности хранит запись обо всех объектах, которые были прочитаны из базы данных в ходе одной бизнес-транзакции. Всякий раз, когда вам нужен объект, вы сначала проверяете Карту идентичности, чтобы узнать, есть ли он у вас уже.

Identity Map — это, по сути, способ кэширования данных из какого-то медленного хранилища, например, диска или базы данных. Вместо того, чтобы каждый раз получать новое значение, вам дается ссылка на одно и то же значение где-то в кеше.

[Identity Map martinfowler.com](https://martinfowler.com/eaaCatalog/identityMap.html)

[Identity Map  www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2018/04/identity-map-pattern.html)

---

## Data Mapper

(`Identity Map => Data Mapper => Repository => Unit of Work`)

`TODO: добавить больше информации, пример, пояснение`

`Data Mapper` Преобразователь Данных — это паттерн, который выступает в роли посредника для двунаправленной передачи данных между постоянным хранилищем данных (часто, реляционной базы данных) и представления данных в памяти (слой домена, то что уже загружено и используется для логической обработки). 
Цель паттерна в том, чтобы держать представление данных в памяти и постоянное хранилище данных независимыми друг от друга и от самого преобразователя данных. 
Слой состоит из одного или более mapper-а (или объектов доступа к данным), отвечающих за передачу данных. 
Реализации mapper-ов различаются по назначению. 
Общие mapper-ы могут обрабатывать всевозоможные типы сущностей доменов, а выделенные mapper-ы будет обрабатывать один или несколько конкретных типов.

Предоставляет вам объекты, которые выглядят точно так же, как записи в вашей структуре записей, но доступны с помощью обычных механизмов вашего языка программирования. Все детали доступа к источникам данных скрыты за этим интерфейсом.

В отличии от ActiveRecord где полностью отображается структура реаляционной базы на свойства классов.
В связке с паттреном Repository который представляет слой над DataMaper получаем независимую друг от друга систему общения бизнес-логики с данными.

Персистентность (т.е. минимизация подверженности изменениям ) архитектурного слоя поддерживает Repository за счет того что он берет на себя все изменения/новые требования бизнес-правил и в итоге обьекты DataMapers остаются неизменны.

[Data Mapper](https://martinfowler.com/eaaCatalog/dataMapper.html)

[Data Mapper designpatternsphp.readthedocs.io](https://designpatternsphp.readthedocs.io/ru/latest/Structural/DataMapper/README.html)

[Data Mapper www.sitepoint.com](https://www.sitepoint.com/integrating-the-data-mappers/)

[Row Data Gateway](http://design-pattern.ru/patterns/row-data-gateway.html)

[Table Data Gateway](http://design-pattern.ru/patterns/table-data-gateway.html)

---

## Repository

(`Identity Map => Data Mapper => Repository => Unit of Work`)

`Pattern Repository` - это абстракция бизнес модели над данными из которых она состоит. Реализация репозитория инкапсулирует доступ к данным.
Бизнес логика работает с абстракным репозиторием, поэтому бизнес логика остается персистентной 
т.е. постоянной и не подвергается изменениям если источник данных будет изменятся.

Единственный репозиторий не нуждается в `Unit Of Work` так как нет бизнес транзакции нуждающейся в контроле сохраняемого состояния не связанных сущностей!
Когда в системе появляются связи сущностей (репозитории), тогда есть запрос на поддержания консистентности их состояния в хранилище данных.

`Pattern Repository` дает унифицированный набор общих операций над сущностью в БД, это то, что на самом деле является моделью - набор методов, реализующих бизнес-логику приложения.
Внутри `Repository` может быть использован `Data Mapper`. Назначение `Data Mapper` именно в маппинге сущности из БД на доменную сущность (или модель).

`Pattern Repository` - это классы или компоненты, которые инкапсулируют логику, необходимую для доступа к источникам данных. 
Они централизуют общие функции доступа к данным, обеспечивая лучшую ремонтопригодность и отделяя инфраструктуру или технологии, используемые для доступа к базам данных, от уровня модели предметной области.

`Pattern Repository` — это абстракция постоянного хранилища. Он скрывает скучные детали доступа к данным, делая вид, что все наши данные находятся в памяти.

По сути, он обеспечивает абстракцию данных, так что ваше приложение может работать с простой абстракцией, интерфейс которой приближается к коллекции. Добавление, удаление, обновление и выбор элементов из этой коллекции осуществляется с помощью ряда простых методов без необходимости решать проблемы с базой данных, такие как соединения, команды, курсоры или устройства чтения. Использование этого шаблона может помочь добиться слабой связи и не учитывать постоянство объектов домена. 

"оградить ваше приложение от изменений в хранилище данных и облегчить автоматизированное модульное тестирование"

Благодаря абстракции от реального хранилища данных, мы можем тестировать приложение (просто подставляете другую реализации репозитория но интерфейс прежний) не опасаясь повреденить реальные данные.
Также если приложение работает с различными источниками данные то репозиторий будет выступать их HUB'ом централизованно управлять. Вы сможете реализовать и централизовать стратегию кэширования (...,`Identity Map`) для источника данных.
Вы сможете улучшить удобство сопровождения и читаемость кода, отделив бизнес-логику от логики доступа к данным или службам.
Также за счет использования строго типизированных бизнес-сущностей (`Data Mapper`), а не примитивов, вы сможете выявлять проблемы во время компиляции, а не во время выполнения.
 
Repository - коллекция хранения объектов инкапсулирует слой определения от слоя использования. Репозиторий позволяет абстрагироваться от конкретных подключений к источникам данных, с которыми работает программа, и является промежуточным звеном между классами, непосредственно взаимодействующими с данными, и остальной программой.

Персистентность (не подверженность изменению) архитектурного слоя Repository поддерживает за счет того, что он берет на себя все изменения/новые требования бизнес-правил и в итоге, обьекты `DataMapers` остаются неизменны.

Есть простые обьекты предметной области/бизнес-логики
Есть mappers обьекты они занимаются сопоставлением/картограф/отображением обьектов предметной области на базу данных и обратно поиск в базе и восзосдание обьекта предметной области, таким образом обьекты предметной области не зависят/игнорируют от базы данных.
Но использовать mappers обьекта в слое бизнес-логики это загрязнение логики издежками инфраструктуры.
А что если бизнес-правила станут еще более извилистей и потребуют более детализированных запросов характерных для бизне-правил?
Тогда придется расширять обьект mappers т.е. впихнуть бизнес правила в обьект отображения базы данных!
Репозиторий является еще одним слоем абстракции над mappers обьектом и берет на себя все изменения/условия бизнес-правил для реализации их требоаний,
являясь единой точкой входа для логики приложения к данным.
Репозиторий эффективно обменивает бизнес-терминологию с клиентским кодом (так называемый универсальный язык, придуманный Эриком Эвансом в его книге «Дизайн, управляемый доменом» )

`Pattern Repository` — это паттерн проектирования, который предоставляет абстрактный интерфейс для работы с коллекцией объектов. 
Он предоставляет механизм для сохранения, извлечения и поиска объектов внутри хранилища (например, базы данных), а также инкапсулирует логику доступа к данным.

###### Основные элементы паттерна Repository:
**Интерфейс репозитория (Repository Interface):** Определяет операции, которые могут быть выполнены над объектами в хранилище. Обычно это включает в себя методы для создания, чтения, обновления и удаления объектов (CRUD).

**Конкретная реализация репозитория (Concrete Repository):** Предоставляет конкретную реализацию интерфейса репозитория и реализует методы для работы с данными в конкретном хранилище (например, в базе данных).

**Объекты-сущности (Entity Objects):** Представляют объекты, которые сохраняются и извлекаются из хранилища. Эти объекты могут представлять бизнес-сущности или данные.

**Контекст хранилища (Storage Context):** Определяет, где и как будут храниться объекты. Это может быть база данных, файловая система или другой источник данных.

Преимущества использования паттерна Repository включают:

**Изоляция кода доступа к данным:** Репозиторий абстрагирует код доступа к данным, что позволяет легко изменять и заменять источники данных без изменения остальной части кода.

**Централизованное управление доступом к данным:** Все операции с данными выполняются через единый интерфейс репозитория, что упрощает управление и поддержку кода.

**Улучшение тестируемости:** Изоляция доступа к данным делает код более тестируемым, так как можно легко создавать заглушки (mocks) или имитации для репозиториев в тестах.

Пример:
```rust
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(dead_code)]

/*
Репозиторий - это абстракция бизнес модели над даееыми из которых она состоит. Реализация репозитория инкапсулирует доступ к данным.
Бизнес логика работает с абстракным репозиторием, поэтому бизнес логика остается персистентной
т.е. постоянной и не подвергается изменениям если источник данных будет изменятся.

Шаблон репозитория можно реализовать следующими способами:

    1.Один репозиторий на объект (не универсальный).
    Этот тип реализации предполагает использование одного класса репозитория для каждого объекта.
    Например, если у вас есть две сущности «Заказ» и «Клиент», каждая сущность будет иметь свой собственный репозиторий.

    2.Общий репозиторий. Общий репозиторий — это тот, который можно использовать для всех объектов,
    другими словами, его можно использовать либо для Заказа, либо для Клиента, либо для любого другого объекта.
*/
use std::convert::TryInto;
type ShortResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

trait UserRepository {
    fn create(&mut self, name: &str, money: u64) -> ShortResult<Id>;
    fn read(&mut self, id: Id) -> ShortResult<()>;
    fn update(&mut self, id: Id, user: UserDTO) -> ShortResult<()>;
    fn delete(&mut self, id: Id) -> ShortResult<()>;
}

use domain::*;
pub mod domain {
    use std::convert::TryFrom;
    use std::fmt;
    use std::str::FromStr;
    pub struct Id(i64);
    impl Id {
        pub fn new(id: i64) -> Self {
            Id(id)
        }
    }
    impl TryFrom<i64> for Id {
        type Error = &'static str;

        fn try_from(value: i64) -> Result<Self, Self::Error> {
            if value == 0 {
                Err("Id only accepts value than zero!")
            } else {
                Ok(Id(value))
            }
        }
    }
    impl From<Id> for i64 {
        fn from(a: Id) -> Self {
            a.0
        }
    }
    impl PartialEq<usize> for Id {
        fn eq(&self, other: &usize) -> bool {
            self.0 as usize == *other
        }
    }
    impl fmt::Display for Id {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl FromStr for Id {
        type Err = std::num::ParseIntError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let n = s.parse::<i64>()?;
            Ok(Id(n))
        }
    }

    pub struct User {
        pub id: Id,
        pub name: String,
        pub money: u64,
    }
    impl User {
        pub fn new(name: &str, money: u64) -> Self {
            Self {
                id: Id(0),
                name: name.to_owned(),
                money,
            }
        }
    }
    impl fmt::Display for User {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "(User)\nid: {} \nname: {} \nmoney:{}",
                self.id, self.name, self.money
            )
        }
    }
    impl From<User> for String {
        fn from(user: User) -> Self {
            format!("{}|{}|{}\n", user.id, user.name, user.money)
        }
    }
    impl TryFrom<&str> for User {
        type Error = &'static str;
        fn try_from(s: &str) -> Result<Self, Self::Error> {
            let v: Vec<&str> = s.split("|").collect();
            if v.len() == 3 {
                if let Ok(id) = v[0].parse::<i64>() {
                    if let Ok(user_id) = Id::try_from(id) {
                        if let Ok(money) = v[2].parse::<u64>() {
                            let mut user = User::new(v[1], money);
                            user.id = user_id;
                            return Ok(user);
                        }
                    }
                }
            }
            Err("invalid data")
        }
    }
}

pub struct UserDTO {
    pub name: Option<String>,
    pub money: Option<u64>,
}

use db_file::FileUserRepository;
pub mod db_file {
    use super::domain::{Id, User};
    use super::ShortResult;
    use super::UserDTO;
    use super::UserRepository;
    use std::convert::{TryFrom, TryInto};
    use std::fs::{File, OpenOptions};
    use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
    use std::path::{Path, PathBuf};

    pub struct FileUserRepository {
        dir: PathBuf,
        users: PathBuf,
        last_id: i64,
    }
    impl FileUserRepository {
        pub fn new(dir: &(impl AsRef<Path> + ?Sized)) -> std::io::Result<Self> {
            let path: &Path = dir.as_ref();
            let mut last_id = 0i64;
            if !path.exists() {
                std::fs::create_dir(path)?;
                std::fs::write(path.join("users"), b"0\n")?;
            } else if !path.join("users").exists() {
                std::fs::write(path.join("users"), b"0\n")?;
            } else {
                let f = File::open(path.join("users"))?;
                let lines = BufReader::new(f).lines();
                if let Some(Ok(res)) = lines.take(1).next() {
                    if let Ok(id) = res.parse::<i64>() {
                        last_id = id;
                    } else {
                        std::fs::write(path.join("users"), b"0\n")?;
                    }
                } else {
                    std::fs::write(path.join("users"), b"0\n")?;
                }
            }
            Ok(Self {
                dir: path.to_path_buf(),
                users: path.join("users"),
                last_id,
            })
        }
    }
    impl UserRepository for FileUserRepository {
        fn create(&mut self, name: &str, money: u64) -> ShortResult<Id> {
            let mut user = User::new(name, money);
            self.last_id = self.last_id + 1;
            user.id = self.last_id.try_into()?;
            let mut f = OpenOptions::new()
                .write(true)
                .append(true)
                .open(self.users.as_path())?;

            let s: String = user.into();
            let data = s.as_bytes();

            f.write(data)?;
            f.sync_all()?;

            let mut f = OpenOptions::new().write(true).open(self.users.as_path())?;
            f.seek(SeekFrom::Start(0))?;
            f.write(format!("{}\n", self.last_id).as_bytes())?;
            f.seek(SeekFrom::End(1))?;

            Ok(self.last_id.try_into()?)
        }
        fn read(&mut self, id: Id) -> ShortResult<()> {
            let find_user_id: usize = Into::<i64>::into(id) as usize;
            let f = File::open(self.users.as_path())?;
            let lines = BufReader::new(f).lines();

            for line in lines.skip(1) {
                let line = line?;
                if let Ok(ref user) = User::try_from(line.as_str()) {
                    if user.id == find_user_id {
                        println!("{}", user);
                    }
                }
            }
            Ok(())
        }
        fn update(&mut self, id: Id, user_dto: UserDTO) -> ShortResult<()> {
            let find_user_id: usize = Into::<i64>::into(id) as usize;
            let f = File::open(self.users.as_path())?;
            let lines = BufReader::new(f).lines();
            {
                let mut buffer = "".to_string();
                for line in lines.map(|mut s| {
                    if let Ok(ref mut l) = s {
                        l.push('\n');
                    }
                    s
                }) {
                    let line = line?;
                    if let Ok(mut user) = User::try_from(line.as_str()) {
                        if user.id == find_user_id {
                            if let Some(ref name) = user_dto.name {
                                user.name = name.clone();
                            }
                            if let Some(money) = user_dto.money {
                                user.money = money;
                            }

                            let s: String = user.into();
                            buffer.push_str(s.as_str());
                        } else {
                            buffer.push_str(line.as_str());
                        }
                    } else {
                        buffer.push_str(line.as_str());
                    }
                }
                std::fs::write(self.users.as_path(), buffer.as_bytes())?;
            }

            Ok(())
        }
        fn delete(&mut self, id: Id) -> ShortResult<()> {
            let find_user_id: usize = i64::from(id) as usize;
            let f = File::open(self.users.as_path())?;

            let lines = BufReader::new(f).lines();
            {
                let mut buffer = "".to_string();
                for line in lines.map(|mut s| {
                    if let Ok(ref mut l) = s {
                        l.push('\n');
                    }
                    s
                }) {
                    let line = line?;
                    if let Ok(user) = User::try_from(line.as_str()) {
                        if user.id != find_user_id {
                            buffer.push_str(line.as_str());
                        }
                    } else {
                        buffer.push_str(line.as_str());
                    }
                }
                std::fs::write(self.users.as_path(), buffer.as_bytes())?;
            }
            Ok(())
        }
    }
}

use db_sqlite::SqliteUserRepository;
pub mod db_sqlite {
    use super::domain::{Id, User};
    use super::ShortResult;
    use super::UserDTO;
    use super::UserRepository;
    use rusqlite::{params, Connection, Result};
    use std::convert::TryInto;

    pub struct SqliteUserRepository {
        conn: Connection,
    }
    impl SqliteUserRepository {
        pub fn new() -> Result<Self> {
            let conn = Connection::open_in_memory()?;
            conn.execute(
                "CREATE TABLE IF NOT EXISTS users (
                    id    INTEGER PRIMARY KEY,
                    name  TEXT NOT NULL,
                    money INTEGER
                )",
                (), // empty list of parameters.
            )?;
            Ok(Self { conn })
        }
    }
    impl UserRepository for SqliteUserRepository {
        fn create(&mut self, name: &str, money: u64) -> ShortResult<Id> {
            self.conn.execute(
                "INSERT INTO users (name, money) VALUES (?1, ?2)",
                (&name, &money),
            )?;
            let id = self.conn.last_insert_rowid();
            Ok(id.try_into()?)
        }
        fn read(&mut self, id: Id) -> ShortResult<()> {
            let mut stmt = self
                .conn
                .prepare("SELECT id, name, money FROM users WHERE id = ?1")
                .unwrap();
            let user = stmt.query_row([&i64::from(id)], |row| {
                Ok(User {
                    id: Id::new(row.get(0)?),
                    name: row.get(1)?,
                    money: row.get(2)?,
                })
            })?;
            println!("{}", user);
            Ok(())
        }
        fn update(&mut self, id: Id, user: UserDTO) -> ShortResult<()> {
            let id = i64::from(id);
            if let (Some(name), Some(money)) = (user.name.clone(), user.money) {
                let _ = self.conn.execute(
                    "UPDATE users SET name = ?,money - ? WHERE id=?",
                    params![&name, &money, &id],
                )?;
            } else if let Some(name) = user.name {
                let _ = self
                    .conn
                    .execute("UPDATE users SET name = ? WHERE id=?", params![&name, &id])?;
            } else if let Some(money) = user.money {
                let _ = self.conn.execute(
                    "UPDATE users SET money = ? WHERE id=?",
                    params![&money, &id],
                )?;
            }
            Ok(())
        }
        fn delete(&mut self, id: Id) -> ShortResult<()> {
            let _ = self
                .conn
                .execute("DELETE FROM users WHERE id = ?", [&i64::from(id)])?;
            Ok(())
        }
    }
}

use db_test::TestUserRepositiry;
pub mod db_test {
    use super::domain::{Id, User};
    use super::ShortResult;
    use super::UserDTO;
    use super::UserRepository;
    use std::collections::HashMap;
    use std::convert::TryInto;

    pub struct TestUserRepositiry {
        users: HashMap<i64, (String, u64)>, // user_id,(name,money)
        last_id: i64,
    }
    impl TestUserRepositiry {
        pub fn new() -> Self {
            Self {
                users: HashMap::new(),
                last_id: 0,
            }
        }
    }
    impl UserRepository for TestUserRepositiry {
        fn create(&mut self, name: &str, money: u64) -> ShortResult<Id> {
            self.last_id = self.last_id + 1;
            self.users.insert(self.last_id, (name.to_owned(), money));
            Ok(self.last_id.try_into()?)
        }
        fn read(&mut self, id: Id) -> ShortResult<()> {
            if let Some(val) = self.users.get(&i64::from(id)) {
                println!("{}", User::new(&val.0, val.1));
            }
            Ok(())
        }
        fn update(&mut self, id: Id, user: UserDTO) -> ShortResult<()> {
            let id = i64::from(id);
            if let (Some(name), Some(money)) = (user.name.clone(), user.money) {
                if let Some(ref mut val) = self.users.get_mut(&id) {
                    val.0 = name;
                    val.1 = money;
                }
            } else if let Some(name) = user.name {
                if let Some(ref mut val) = self.users.get_mut(&id) {
                    val.0 = name;
                }
            } else if let Some(money) = user.money {
                if let Some(ref mut val) = self.users.get_mut(&id) {
                    val.1 = money;
                }
            }
            Ok(())
        }
        fn delete(&mut self, id: Id) -> ShortResult<()> {
            let id = i64::from(id);
            if self.users.contains_key(&id) {
                self.users.remove(&i64::from(id)).unwrap();
            }
            Ok(())
        }
    }
}

// Единственный репозиторий не нуждается в `Unit Of Work` так как нет бизнес транзакции нуждающейся в контроле сохраняемого состояния не связанных сущностей!
// Когда в системе появляются связи сущностей (репозитории), тогда есть запрос на поддержания консистентности их состояния в хранилище данных.

trait Controller: UserRepository {}
struct ControllerImpl<R> {
    pub rep: R, // TODO: вот тут бизнес логика не зависит от источника данных, все что нужно для ее работы это методы UserRepository
}
impl<R: UserRepository> ControllerImpl<R> {
    pub fn new(rep: R) -> Self {
        Self { rep }
    }
}
impl<R: UserRepository> UserRepository for ControllerImpl<R> {
    fn create(&mut self, name: &str, money: u64) -> ShortResult<Id> {
        self.rep.create(name, money)
    }
    fn read(&mut self, id: Id) -> ShortResult<()> {
        self.rep.read(id)
    }
    fn update(&mut self, id: Id, user: UserDTO) -> ShortResult<()> {
        self.rep.update(id, user)
    }
    fn delete(&mut self, id: Id) -> ShortResult<()> {
        self.rep.delete(id)
    }
}
impl<R: UserRepository> Controller for ControllerImpl<R> {}

fn main() {
    /*
        let mut type_repository = String::new();
        println!("Enter type of repository (file or sqlite or test): ");
        let _ = std::io::stdin().read_line(&mut type_repository).unwrap();

        let mut controller:Box<dyn Controller>;
        if type_repository == "file" {
            controller = Box::new(ControllerImpl::new(FileUserRepository::new("design_patterns/unit-of-works-2/examples/db_file").unwrap()));
        }else if type_repository == "sqlite" {
            controller = Box::new(ControllerImpl::new(SqliteUserRepository::new().unwrap()));
        }else{
            controller = Box::new(ControllerImpl::new(TestUserRepositiry::new()));
        }

        let _ = controller.create("Jeka", 200).unwrap();
        let _ = controller.create("Alex", 300).unwrap();
        let _ = controller.create("Zorro", 100).unwrap();
        let _ = controller.delete(1.try_into().unwrap()).unwrap();
        let _ = controller.update(2.try_into().unwrap(), UserDTO{name:Some("Hola".into()),money: None}).unwrap();
        let _ = controller.read(2.try_into().unwrap()).unwrap();
    */
    // ------------------------------------------------------------------------------------------------------

    let mut controller =
        ControllerImpl::new(FileUserRepository::new("design_patterns/repository/db").unwrap());
    // or
    let mut controller = ControllerImpl::new(SqliteUserRepository::new().unwrap());
    // or
    let mut controller = ControllerImpl::new(TestUserRepositiry::new());

    let _ = controller.create("Jeka", 200).unwrap();
    let _ = controller.create("Alex", 300).unwrap();
    let _ = controller.create("Zorro", 100).unwrap();
    let _ = controller.delete(1.try_into().unwrap()).unwrap();
    let _ = controller
        .update(
            2.try_into().unwrap(),
            UserDTO {
                name: Some("Hola".into()),
                money: None,
            },
        )
        .unwrap();
    let _ = controller.read(2.try_into().unwrap()).unwrap();
}
```



* [Repository habr.com](https://habr.com/ru/post/248505/)
* [Repository metanit.com](https://metanit.com/sharp/articles/mvc/11.php)
* [Repository learn.microsoft.com](https://learn.microsoft.com/en-us/previous-versions/msp-n-p/ff649690(v=pandp.10)?redirectedfrom=MSDN)

---

## Unit of Work (UOW)

(`Identity Map => Data Mapper => Repository => Unit of Work`)

Этот шаблон принадлежит каталогу объектно-реляционных поведенческих шаблонов, а этот каталог принадлежит шаблонам архитектуры корпоративных приложений.

`Unit of Work` может выполняет сразу две важные задачи:

1. Уменьшить количество запросов по сети к хранилищу данных, за счет хранение состояния обьектов в памяти. 
   После завершения всех операций обновления, отправляет или нет текущее состояние обьекта в используемое хранилище (MYSQL, POSTGRES, ...)

2. Обеспечивает целосность бизнес транзакции, за счет обьединения всех операций над обьектами участниками в одну общую транзакцию.
   Транзакция либо выполнится для всех, либо произойдет откат в стабильное состояние (по аналогии с ACID - Атомарность и Согласованность).
   Механизм отката обеспечивается для баз данных поддерживающих транзакции.
   Для баз данных не поддержиющей транзакцию, придется хранить и сбрасывать состояние обьекта находящегося в памяти. 

Реализация `Unit of Work` и `Pattern Repository` может помочь оградить ваше приложение от изменений в хранилище данных и облегчить автоматизированное модульное тестирование или разработку через тестирование (TDD).

**Намерение**

Гарантирует, что единица транзакции, охватывающая несколько связанных репозиториев, либо завершится для всех объектов, либо полностью завершится неудачно, обеспечив согласованность базы данных.

Когда бизнес-транзакция завершается, все эти обновления отправляются как одна большая единица работы, которая сохраняется в базе данных за один раз, чтобы свести к минимуму количество обращений к базе данных.

`Unit of Work` использует одну транзакцию или одну единицу работы для нескольких операций вставки, обновления и удаления. Эти операции либо завершаются успешно, либо проваливаются как единое целое. Другими словами, все операции будут зафиксированы как одна транзакция или отменены как единое целое.

**Объяснение**

Шаблон проектирования «Единица работы» выполняет две важные задачи: во-первых, он поддерживает обновления в памяти, а во-вторых, отправляет эти обновления в памяти как одну транзакцию в базу данных.
Итак, для достижения вышеуказанных целей необходимо пройти два этапа:
- Он хранит в памяти списки бизнес-объектов, которые были изменены (вставлены, обновлены или удалены) во время транзакции.
- После завершения транзакции все эти обновления отправляются как одна большая единица работы, которая физически сохраняется в базе данных за один раз.

Ключевой момент в `Unit of Work` заключается в том, что, когда приходит время принятия обязательств, `Unit of Work` решает, что делать. Он открывает транзакцию, выполняет проверку параллелизма и записывает изменения в базу данных. Программисты приложений никогда явно не вызывают методы для обновления базы данных. Таким образом, им не придется отслеживать, что изменилось, или беспокоиться о том, как ссылочная целостность влияет на порядок, в котором им нужно что-то делать.

`Unit of Work` простое хранилище объектов в памяти, которое отслеживает, какие объекты домена должны быть запланированы для вставки, обновления и удаления

Работает вместе с `Repository`
Множество обращений к базе можно уменьшить умной стратегией кеширования.
Обеспечивает целостность данных (атомарную синхронизацию изменений т.е. один момент времени обьект сохраняет единственный метод), выполняется в транзакции.
Для этого нужна реализация `Identity Map` обеспичавающая одну ссылку на обьект по всей системе, тогда все изменения будут только для одного обьекта.
Задача `Identity Map` - сохранение карты созданных объектов, взятых из хранилища с тем чтобы гарантировать что одна единица информации из хранилища представлена ровно одним экземпляром объекта данных в приложении. Это позволяет избежать конфликтов изменений т.к. не допускает ситуации когда два объекта, представляющих один и тот же элемент данных в хранилище, изменены по-разному. Информация из `Identity Map` используется в методе commit() паттерна `Unit of Work` для вычисления разницы между исходными данными и накопленными изменениями.

Пример: в Doctrine, сущность после изменения не сразу обновляется в базе данных, она отслеживается и если вернется на прежнее состояние до вызова метода `flush`
 т.е. явного сохранения в базу, то в базу запроса не будет, а если она изменила свое состояние, то пойдет запрос базу.

Поскольку для вычисления разницы (и, соответственно, определения того что и каким образом должно быть изменено в хранилище) необходимо знать какие данные и как именно хранятся в объектах - как правило необходима также реализация паттерна `Metadata Mapping`, описывающего связь между содержимым хранилища (к примеру таблицами и столбцами базы данных) и классами / свойствами объектов.

Обслуживает набор объектов, изменяемых в бизнес-транзакции (бизнес-действии) и управляет записью изменений и разрешением проблем конкуренции данных.

Когда необходимо писать и читать из БД, важно следить за тем, что вы изменили и если не изменили - не записывать данные в БД. Также необходимо вставлять данные о новых объектах и удалять данные о старых.

**Проблема:**

Можно записывать в БД каждое изменение объекта, но это приведёт к большому количеству мелких запросов к БД, что закончится замедлением работы приложения. Более того, это требует держать открытую транзакцию всё время работы приложения, что непрактично, если приложение обрабатывает несколько запросов одновременно. Ситуация ещё хуже, если необходимо следить за чтением из и БД, чтобы избежать неконсистентного чтения.

Реализация паттерна `Unit of Work` следит за всеми действиями приложения, которые могут изменить БД в рамках одного бизнес-действия. Когда бизнес-действие завершается, `Unit of Work` выявляет все изменения и вносит их в БД.

Также, если данные в хранилище не являются независимыми (к примеру связи между таблицами в базе данных) - может потребоваться реализации ряда паттернов, отвечающих за сохранение информации о связях между данными (это паттерны раздела `Object-Relational Structural Patterns`:  Identity Field, Foreign Key Mapping, Association Table Mapping, Dependent Mapping, Embedded Value, Serialized LOB, Single Table Inheritance, Class Table Inheritance, 
Concrete Table Inheritance, Inheritance Mappers).

Используйте шаблон `Unit of Work`, когда:
 - Оптимизировать время, затрачиваемое на транзакции базы данных.
 - Отправка изменений в базу данных как единица работы, обеспечивающая атомарность транзакции.
 - Чтобы уменьшить количество обращений к базе данных.

Пример:
```rust
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;

use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Transaction;
use std::convert::TryFrom;

/*
`Unit of Work` может выполняет сразу две важные задачи:

1. Уменьшить количество запросов по сети к хранилищу данных, за счет хранение состояния обьектов в памяти.
   После завершения всех операций обновления, отправляет или нет текущее состояние обьекта в используемое хранилище (MYSQL, POSTGRES, ...)

2. Обеспечивает целосность бизнес транзакции, за счет обьединения всех операций над обьектами участниками в одну общую транзакцию.
   Транзакция либо выполнится для всех, либо произойдет откат в стабильное состояние (по аналогии с ACID - Атомарность и Согласованность).
   Механизм отката обеспечивается для баз данных поддерживающих транзакции.
   Для баз данных не поддержиющей транзакцию, придется хранить и сбрасывать состояние обьекта находящегося в памяти.

Repository - выполняет операции бизнес логики с presentation обьектами. Получает и отдает обьекты у Mapper
Mapper - отвечает за преобразование DB представление обьекта в presentation представление обьекта
Identity - отвечает за загрузку в кеш уникальных обьектов (т.е. для этого хранить весь обьект он не обязан и выполнять CREATE/UPDATE/DELETE тоже не должен, только SELECT если до этого не загрул и ответить простым bool)
UnitOfWork - 1.собирает связанные операции с обьектами в одну транзакцию и 2.посылает один запрос на сущность в базу
*/

// Реализация без поддержки состояния, реализация целосности бизнес транзакции и абстракция источника данных.
// p.s. Асинронный источник данных меняет интерфейс реализации полностью.
type ShortResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
trait UnitOfWork<'a, T> {
    fn unit_of_work<'b: 'a, U>(&'b mut self, unit: U) -> ShortResult<()>
    where
        U: FnOnce(&mut T) -> ShortResult<()>;
}

use domain_entities::{Id, Order, OrderDTO, User, UserDTO};
pub mod domain_entities {
    use super::TryFrom;

    #[derive(Debug)]
    pub struct User {
        pub id: Id,
        pub name: String,
        pub money: u64,
    }
    pub struct UserDTO<'a> {
        pub name: &'a str,
        pub money: u64,
    }

    #[derive(Debug)]
    pub struct Order {
        pub id: Id,
        pub user_id: Id,
        pub product: String,
        pub total_cost: u64,
    }
    pub struct OrderDTO<'a> {
        pub user_id: Id,
        pub product: &'a str,
        pub total_cost: u64,
    }

    #[derive(Debug)]
    pub struct Id(i64);
    impl Id {
        pub fn new(id: i64) -> Self {
            Id(id)
        }
    }
    impl TryFrom<i64> for Id {
        type Error = &'static str;

        fn try_from(value: i64) -> Result<Self, Self::Error> {
            if value == 0 {
                Err("Id only accepts value than zero!")
            } else {
                Ok(Id(value))
            }
        }
    }
    impl From<Id> for i64 {
        fn from(a: Id) -> Self {
            a.0
        }
    }
    impl PartialEq<usize> for Id {
        fn eq(&self, other: &usize) -> bool {
            self.0 as usize == *other
        }
    }
}

use uow::UnitOfWorkWithoutState;
pub mod uow {
    use super::db_layer::{self, ConcreteDb, DbContext};
    use super::{ShortResult, UnitOfWork};
    pub struct UnitOfWorkWithoutState {
        db: ConcreteDb,
        pool: Option<r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>>,
    }
    impl UnitOfWorkWithoutState {
        pub fn new(db: ConcreteDb) -> Self {
            Self { db, pool: None }
        }
    }
    impl<'a> UnitOfWork<'a, db_layer::ImplTransaction<'a>> for UnitOfWorkWithoutState {
        // TODO: задача - обернуть все вызовы c общим соединением в транзакцию
        fn unit_of_work<'b: 'a, U>(&'b mut self, unit: U) -> ShortResult<()>
        where
            U: FnOnce(&mut db_layer::ImplTransaction<'a>) -> ShortResult<()>,
        {
            self.pool = Some(self.db.get_pool()?);
            if let Some(ref mut pool) = &mut self.pool {
                let mut tx/* : dyn DbContext*/ = db_layer::ImplTransaction::new(pool.transaction()?);
                unit(&mut tx)?;
                tx.commit();
            }
            Ok(())
        }
    }
}

use test_uow::{TestImplTransaction, TestUnitOfWorkWithoutState};
pub mod test_uow {
    use super::db_layer::{self, ConcreteDb, DbContext};
    use super::domain_entities::{Id, OrderDTO, User};
    use super::{ShortResult, UnitOfWork};
    use std::convert::TryFrom;
    use std::marker::PhantomData;

    pub struct TestUnitOfWorkWithoutState {
        db: Vec<u8>,
    }
    impl TestUnitOfWorkWithoutState {
        pub fn new() -> Self {
            Self { db: vec![] }
        }
    }
    impl<'a> UnitOfWork<'a, TestImplTransaction> for TestUnitOfWorkWithoutState {
        // TODO: задача - обернуть все вызовы c общим соединением в транзакцию
        fn unit_of_work<'b: 'a, U>(&'b mut self, unit: U) -> ShortResult<()>
        where
            U: FnOnce(&mut TestImplTransaction) -> ShortResult<()>,
        {
            let mut tx/* : dyn DbContext*/ = TestImplTransaction::new();
            unit(&mut tx)?;
            tx.commit();

            Ok(())
        }
    }

    pub struct TestImplTransaction {}

    impl TestImplTransaction {
        pub fn new() -> Self {
            Self {}
        }
    }
    impl DbContext for TestImplTransaction {
        fn commit(self) {
            println!("TEST COMMIT");
        }
        fn rollback(self) {}
        fn get_user(&self, user_id: i64) -> ShortResult<User> {
            Ok(User {
                id: Id::try_from(user_id).unwrap(),
                name: "AK".into(),
                money: u64::MAX,
            })
        }
        fn update_user_money(&self, user_id: i64, total_cost: u64) -> ShortResult<()> {
            Ok(())
        }
        fn new_order(&self, order: OrderDTO) -> ShortResult<()> {
            Ok(())
        }
    }
}

use db_layer::{
    ConcreteDb, DbContext, ImplTransaction, SqliteOrderRepository, SqliteUserRepository,
};
pub mod db_layer {
    use r2d2_sqlite::SqliteConnectionManager;
    use rusqlite::params;

    use super::domain_entities::{Id, Order, OrderDTO, User};
    use super::{ShortResult, TryFrom};

    pub struct ConcreteDb {
        conn: r2d2::Pool<SqliteConnectionManager>,
    }
    impl ConcreteDb {
        pub fn new(conn: r2d2::Pool<SqliteConnectionManager>) -> Self {
            Self { conn }
        }
        pub fn get_pool(
            &self,
        ) -> Result<r2d2::PooledConnection<SqliteConnectionManager>, r2d2::Error> {
            self.conn.clone().get()
        }
    }
    pub trait DbContext {
        fn commit(self);
        fn rollback(self);

        fn get_user(&self, user_id: i64) -> ShortResult<User>;
        fn update_user_money(&self, user_id: i64, total_cost: u64) -> ShortResult<()>;
        fn new_order(&self, order: OrderDTO) -> ShortResult<()>;
    }

    // Все запросы к базе от всех репозиториев
    pub struct ImplTransaction<'a> {
        tx: rusqlite::Transaction<'a>,
    }
    impl<'a> ImplTransaction<'a> {
        pub fn new(tx: rusqlite::Transaction<'a>) -> Self {
            Self { tx }
        }
    }
    impl<'a> DbContext for ImplTransaction<'a> {
        fn commit(self) {
            // TODO: return Result
            let _ = self.tx.commit();
        }
        fn rollback(self) {
            // TODO: return Result
            let _ = self.tx.rollback();
        }
        fn get_user(&self, user_id: i64) -> ShortResult<User> {
            let mut stmt = self
                .tx
                .prepare("SELECT id, name, money FROM users WHERE id = ?1")
                .unwrap();
            let user = stmt.query_row([&user_id], |row| {
                Ok(User {
                    id: Id::try_from(row.get::<_, i64>(0)?).unwrap(),
                    name: row.get::<_, String>(1)?,
                    money: row.get::<_, u64>(2)?,
                })
            })?;
            Ok(user)
        }
        fn update_user_money(&self, user_id: i64, total_cost: u64) -> ShortResult<()> {
            let mut user = self.get_user(user_id)?;
            if let Some(balance) = user.money.checked_sub(total_cost) {
                user.money = balance;
            } else {
                return Err("Insufficient funds in the account".into());
            }
            let _ = self.tx.execute(
                "UPDATE users SET name = ?1, money = ?2 WHERE id=?3",
                params![&user.name, &user.money, &i64::from(user.id)],
            )?;
            Ok(())
        }
        fn new_order(&self, order: OrderDTO) -> ShortResult<()> {
            self.tx.execute(
                "INSERT INTO orders (user_id, product, total_cost) VALUES (?1, ?2, ?3)",
                (i64::from(order.user_id), order.product, order.total_cost),
            )?;
            Ok(())
        }
    }

    // Конкретные репозитории
    pub struct SqliteUserRepository<'a, T> {
        db_ctx: &'a T,
    }
    impl<'a, 'b, T: DbContext> SqliteUserRepository<'a, T> {
        pub fn new(db_ctx: &'a T) -> Self {
            Self { db_ctx }
        }
        pub fn get_user(&self, user_id: i64) -> ShortResult<User> {
            self.db_ctx.get_user(user_id)
        }
        pub fn update_user_money(&self, user_id: i64, total_cost: u64) -> ShortResult<()> {
            self.db_ctx.update_user_money(user_id, total_cost)?;
            Ok(())
        }
    }

    pub struct SqliteOrderRepository<'a, T> {
        db_ctx: &'a T,
    }
    impl<'a, 'b, T: DbContext> SqliteOrderRepository<'a, T> {
        pub fn new(db_ctx: &'a T) -> Self {
            Self { db_ctx }
        }
        pub fn new_order(&self, order: OrderDTO) -> ShortResult<()> {
            self.db_ctx.new_order(order)
        }
    }
}

fn init_schema(
    pool: r2d2::Pool<SqliteConnectionManager>,
    user_id: i64,
    name: &str,
    money: u64,
) -> ShortResult<()> {
    use rusqlite::params;
    let mut conn = pool.get()?;
    let tx: Transaction = conn.transaction()?;
    tx.execute(
        "CREATE TABLE IF NOT EXISTS orders (
            id    INTEGER PRIMARY KEY,
            user_id    INTEGER NOT NULL,
            product    TEXT NOT NULL,
            total_cost    INTEGER NOT NULL
        )",
        params![],
    )?;

    tx.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id    INTEGER PRIMARY KEY,
            name     TEXT NOT NULL,
            money    INTEGER NOT NULL
        )",
        params![],
    )?;

    let user = User {
        id: Id::try_from(user_id)?,
        name: name.to_owned(),
        money,
    };
    println!("-------------------------------");
    println!("Before orders: {:?}", user);
    tx.execute(
        "INSERT OR IGNORE INTO users (id, name, money) VALUES (?1, ?2, ?3)",
        (&i64::from(user.id), user.name, user.money),
    )?;
    tx.commit()?;
    Ok(())
}

fn domain_controller_new_order<'a, U, T>(
    uow: &'a mut U,
    user_id: i64,
    product: &str,
    total_cost: u64,
) -> ShortResult<()>
where
    U: UnitOfWork<'a, T>,
    T: db_layer::DbContext,
{
    let product = product.to_owned();
    // TODO: формирование единицы работы в виде замыкания FnOnce, которая должна быть по аналогии с ACID - Атомарна и Согласованна
    let _ = uow.unit_of_work(move |db_ctx: &mut T| -> ShortResult<()> {
        let rep_user: db_layer::SqliteUserRepository<'_, T> =
            db_layer::SqliteUserRepository::new(db_ctx);
        rep_user.update_user_money(user_id, total_cost)?;

        let user = rep_user.get_user(user_id)?;

        let order = domain_entities::OrderDTO {
            user_id: user.id,
            product: product.as_str(),
            total_cost,
        };
        let rep_order = db_layer::SqliteOrderRepository::new(db_ctx);
        rep_order.new_order(order)?;
        Ok(())
    })?;
    Ok(())
}

// cargo run --bin unit-of-works
// cargo run --bin unit-of-works --features "test"
fn main() {
    let pool: r2d2::Pool<SqliteConnectionManager> = {
        let manager =
            SqliteConnectionManager::file("design_patterns/unit-of-works/db/uow_without_state.db");
        r2d2::Pool::new(manager).unwrap()
    };

    // ------------------------------------------------ PREPARE --------------------------------------------------
    let user_id: i64 = 1;
    {
        let name: &str = "Alex";
        let money: u64 = 100;
        init_schema(pool.clone(), user_id, name, money).unwrap();
    }

    // ------------------------------------------------ UOW ------------------------------------------------------
    let mut uow: UnitOfWorkWithoutState =
        UnitOfWorkWithoutState::new(ConcreteDb::new(pool.clone()));

    #[cfg(feature = "test")]
    let mut uow: TestUnitOfWorkWithoutState = TestUnitOfWorkWithoutState::new();

    print!("new order - (77$) ");
    if let Err(e) = domain_controller_new_order(&mut uow, user_id, "TV Toshiba", 77) {
        println!("ERROR: {:?}", e);
    } else {
        println!("SUCCESS");
    }
    print!("new order - (3$) ");
    if let Err(e) = domain_controller_new_order(&mut uow, user_id, "Bread", 3) {
        println!("ERROR: {:?}", e);
    } else {
        println!("SUCCESS");
    }
    print!("new order - (24$) ");
    if let Err(e) = domain_controller_new_order(&mut uow, user_id, "Notepad iPad", 24) {
        println!("ERROR: {:?}", e);
    } else {
        println!("SUCCESS");
    }

    // ------------------------------------------------ SHOW -----------------------------------------------------
    show(pool, user_id);
}

fn show(pool: r2d2::Pool<SqliteConnectionManager>, user_id: i64) {
    let conn = pool.get().unwrap();

    let mut stmt = conn
        .prepare("SELECT id, user_id, product, total_cost FROM orders WHERE user_id = ?1")
        .unwrap();
    let orders_rows = stmt
        .query_map([&user_id], |row| {
            Ok(Order {
                id: Id::try_from(row.get::<_, i64>(0)?).unwrap(),
                user_id: Id::try_from(row.get::<_, i64>(1)?).unwrap(),
                product: row.get::<_, String>(2)?,
                total_cost: row.get::<_, u64>(3)?,
            })
        })
        .unwrap();
    let orders: Vec<Order> = orders_rows.into_iter().map(|o| o.unwrap()).collect();
    println!("Orders:");
    for order in orders {
        println!("{:?}", order);
    }

    let mut stmt = conn
        .prepare("SELECT id, name, money FROM users WHERE id = ?1")
        .unwrap();
    let user = stmt.query_row([&user_id], |row| {
        Ok(User {
            id: Id::try_from(row.get::<_, i64>(0)?).unwrap(),
            name: row.get::<_, String>(1)?,
            money: row.get::<_, u64>(2)?,
        })
    });
    println!("After orders: {:?}", user.unwrap());
}
```

Еще пример UOW with state:
```rust
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
/*
  Суть паттерна Unit Of Work, я считаю, тут отображена полностью.
  Бизнес транзакция выполняется целостно и поддежка состояния уменьшает количество запросов к базе данных.

  p.s. напутано с обязаностями доступа к базе данных и жесткая зависимость от sqlite, т.е. необходим `Pattern Repository`.
*/
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;

use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use rusqlite::Transaction;

type ShortResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

trait UnitOfWork {
    fn register_new_user(&mut self, entity: UserEntityDTO);
    fn register_new_order(&mut self, entity: OrderEntityDTO);
    fn register_dirty_user(&mut self, id: i64, entity: UserEntityDTO);
    fn register_removed_user(&mut self, id: i64);
    fn commit(&mut self);
    fn clear(&mut self);
}

pub struct NewIdentityEntities {
    user_collection: Vec<UserEntityDTO>,
    order_collection: Vec<OrderEntityDTO>,
}
impl NewIdentityEntities {
    pub fn new() -> Self {
        Self {
            user_collection: vec![],
            order_collection: vec![],
        }
    }
    fn register_new_user(&mut self, entity: UserEntityDTO) {
        self.user_collection.push(entity);
    }
    fn register_new_order(&mut self, entity: OrderEntityDTO) {
        self.order_collection.push(entity);
    }
    pub fn get_users(&self) -> &Vec<UserEntityDTO> {
        &self.user_collection
    }
    pub fn get_orders(&self) -> &Vec<OrderEntityDTO> {
        &self.order_collection
    }
    pub fn clear(&mut self) {
        self.user_collection.clear();
        self.order_collection.clear();
    }
}

use uow::UnitOfWorkWithState;
pub mod uow {
    use super::db_layer;
    use super::domain_entities::{OrderEntity, OrderEntityDTO, UserEntity, UserEntityDTO};
    use super::{NewIdentityEntities, UnitOfWork};

    pub struct UnitOfWorkWithState {
        new_entities: NewIdentityEntities, // для новых сущностей
        db: db_layer::ConcreteDb,
        identity: db_layer::IdentityMap, // хранитель единственного состояния обьекта в системе
    }

    impl UnitOfWorkWithState {
        pub fn new(db: db_layer::ConcreteDb) -> Self {
            let identity = db_layer::IdentityMap::new(db.get_pool().unwrap());
            Self {
                new_entities: NewIdentityEntities::new(),
                db,
                identity,
            }
        }
        pub fn get_user_by_email(&mut self, email: &str) -> Option<UserEntity> {
            self.identity.get_user_by_email(email)
        }
        pub fn get_user(&mut self, id: i64) -> Option<UserEntity> {
            self.identity.get_user_by_id(id)
        }
    }

    impl UnitOfWork for UnitOfWorkWithState {
        fn register_new_user(&mut self, entity: UserEntityDTO) {
            self.new_entities.register_new_user(entity);
        }
        fn register_new_order(&mut self, entity: OrderEntityDTO) {
            self.new_entities.register_new_order(entity);
        }
        fn register_dirty_user(&mut self, id: i64, entity: UserEntityDTO) {
            self.identity.update_user(id, entity);
        }

        fn register_removed_user(&mut self, id: i64) {
            self.identity.removed_user(id);
        }

        fn commit(&mut self) {
            // инициируем подключение к базе данных
            // общее подключение для всех операций в одной транзакции
            if let Ok(mut binding) = self.db.get_pool() {
                let tx = binding.transaction().unwrap();

                // TODO: очередность операций лежит на вас
                // TODO: если обьект по итогу не изменился, зачем его в базе обновлять? (хранить его первоначальный хеш?)
                println!("Committing new entities:");
                let new_user_entities = self.new_entities.get_users();
                if new_user_entities.len() > 0 {
                    self.db.add_users(new_user_entities, &tx);
                }
                let new_order_entities = self.new_entities.get_orders();
                if new_order_entities.len() > 0 {
                    self.db.add_orders(new_order_entities, &tx);
                }

                println!("Committing dirty/removed entities:");
                self.identity.fixed_change_users(&tx);
                self.identity.fixed_change_orders(&tx);

                // Очистка списков изменений после коммита
                self.new_entities.clear();
                self.identity.clear();

                let _ = tx.commit().unwrap();
            } else {
                unimplemented!()
            }
        }

        fn clear(&mut self) {
            println!("clear");
            // Очистка списков
            self.new_entities.clear();
            self.identity.clear();
        }
    }
}

pub mod db_layer {
    use super::domain_entities::{OrderEntity, OrderEntityDTO, UserEntity, UserEntityDTO};
    use rusqlite::params;
    use std::collections::HashMap;

    pub enum Status {
        LOADED,
        DIRTY,
        REMOVED,
    }

    pub struct IdentityMap {
        conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
        order_identity: HashMap<i64, (OrderEntity, Status)>,
        user_identity: HashMap<i64, (UserEntity, Status)>,
    }
    impl IdentityMap {
        pub fn new(conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>) -> Self {
            Self {
                conn,
                order_identity: HashMap::new(),
                user_identity: HashMap::new(),
            }
        }
        pub fn get_user_by_email(&mut self, email: &str) -> Option<UserEntity> {
            if let Some(user) = self
                .user_identity
                .values()
                .into_iter()
                .find(|(user, status)| user.get_email() == email)
            {
                return Some(user.0.clone());
            }
            if self.load_user_by_email(email) {
                return self.get_user_by_email(email);
            }
            None
        }
        pub fn get_user_by_id(&mut self, id: i64) -> Option<UserEntity> {
            if self.user_identity.contains_key(&id) {
                if let Some(user) = &self.user_identity.get(&id) {
                    return Some(user.0.clone());
                }
            }
            if self.load_user(id) {
                return self.get_user_by_id(id);
            }
            None
        }
        pub fn update_user(&mut self, id: i64, user: UserEntityDTO) -> bool {
            if !self.user_identity.contains_key(&id) {
                self.load_user(id);
            }
            if self.user_identity.contains_key(&id) {
                let new_user = UserEntity::new(id, user.name, user.email, user.money);
                self.user_identity.insert(id, (new_user, Status::DIRTY));
                return true;
            }
            false
        }
        pub fn removed_user(&mut self, id: i64) {
            if self.user_identity.contains_key(&id) {
                if let Some((user, status)) = self.user_identity.get_mut(&id) {
                    *status = Status::REMOVED;
                }
            }
        }
        pub fn fixed_change_users(&mut self, tx: &rusqlite::Transaction) {
            for (key, (user, status)) in &self.user_identity {
                match *status {
                    Status::DIRTY => {
                        tx.execute(
                            "UPDATE users SET name = ?1, email = ?2, money = ?3 WHERE id=?4",
                            params![
                                user.get_name(),
                                user.get_email(),
                                user.get_money(),
                                user.get_id()
                            ],
                        )
                        .unwrap();
                    }
                    Status::REMOVED => {
                        tx.execute("DELETE FROM users WHERE id=?1", params![user.get_id()])
                            .unwrap();
                    }
                    Status::LOADED => {}
                }
            }
        }
        pub fn fixed_change_orders(&mut self, tx: &rusqlite::Transaction) {
            // unimplemented!()
        }
        fn load_user(&mut self, id: i64) -> bool {
            let mut stmt = self
                .conn
                .prepare("SELECT id, name, email, money FROM users WHERE id = ?1")
                .unwrap();
            let res = stmt.query_row([&id], |row| {
                Ok(UserEntity::new(
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, u64>(3)?,
                ))
            });
            if let Ok(user) = res {
                self.user_identity.insert(id, (user, Status::LOADED));
                return true;
            }
            false
        }
        fn load_user_by_email(&mut self, email: &str) -> bool {
            let mut stmt = self
                .conn
                .prepare("SELECT id, name, email, money FROM users WHERE email = ?1")
                .unwrap();
            let res = stmt.query_row([&email], |row| {
                Ok(UserEntity::new(
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, u64>(3)?,
                ))
            });
            if let Ok(user) = res {
                self.user_identity
                    .insert(user.get_id(), (user, Status::LOADED));
                return true;
            }
            false
        }
        pub fn clear(&mut self) {
            self.order_identity.clear();
            self.user_identity.clear();
        }
    }

    pub struct ConcreteDb {
        conn: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>,
    }
    impl ConcreteDb {
        pub fn new(conn: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>) -> Self {
            Self { conn }
        }
        pub fn get_pool(
            &self,
        ) -> Result<r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>, r2d2::Error>
        {
            self.conn.clone().get()
        }
        pub fn add_users(&self, users: &Vec<UserEntityDTO>, tx: &rusqlite::Transaction) {
            for user in users {
                tx.execute(
                    "INSERT INTO users (name, email, money) VALUES (?1, ?2, ?3)",
                    (&user.name, &user.email, &user.money),
                )
                .unwrap();
            }
            // p.s. BATCH INSERT ?
        }
        pub fn add_orders(&self, orders: &Vec<OrderEntityDTO>, tx: &rusqlite::Transaction) {
            for order in orders {
                tx.execute(
                    "INSERT INTO orders (user_id, product, total_cost) VALUES (?1, ?2, ?3)",
                    (&order.user_id, &order.product, &order.total_cost),
                )
                .unwrap();
            }
            // p.s. BATCH INSERT ?
        }
    }
}

use domain_entities::{OrderEntity, OrderEntityDTO, UserEntity, UserEntityDTO};
pub mod domain_entities {
    use std::fmt::Debug;

    #[derive(Debug, Clone)]
    pub struct UserEntity {
        id: i64,
        name: String,
        email: String,
        money: u64,
    }
    impl UserEntity {
        pub fn new(id: i64, name: String, email: String, money: u64) -> Self {
            Self {
                id,
                name,
                email,
                money,
            }
        }
        pub fn get_id(&self) -> i64 {
            self.id
        }
        pub fn get_name(&self) -> &str {
            &self.name
        }
        pub fn get_email(&self) -> &str {
            &self.email
        }
        pub fn get_money(&self) -> u64 {
            self.money
        }
    }

    pub struct UserEntityDTO {
        pub name: String,
        pub email: String,
        pub money: u64,
    }
    impl UserEntityDTO {
        pub fn new(name: &str, email: &str, money: u64) -> Self {
            Self {
                name: name.to_owned(),
                email: email.to_owned(),
                money,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct OrderEntity {
        id: i64,
        user_id: i64,
        product: String,
        total_cost: u64,
    }
    impl OrderEntity {
        pub fn new(id: i64, user_id: i64, product: String, total_cost: u64) -> Self {
            Self {
                id,
                user_id,
                product,
                total_cost,
            }
        }
    }

    pub struct OrderEntityDTO {
        pub user_id: i64,
        pub product: String,
        pub total_cost: u64,
    }
    impl OrderEntityDTO {
        pub fn new(user_id: i64, product: &str, total_cost: u64) -> Self {
            Self {
                user_id,
                product: product.to_owned(),
                total_cost,
            }
        }
    }
}

fn init_schema(pool: r2d2::Pool<SqliteConnectionManager>) -> ShortResult<()> {
    use rusqlite::params;
    let mut conn = pool.get()?;
    let tx: Transaction = conn.transaction()?;
    tx.execute(
        "CREATE TABLE IF NOT EXISTS orders (
            id    INTEGER PRIMARY KEY,
            user_id    INTEGER NOT NULL,
            product    TEXT NOT NULL,
            total_cost    INTEGER NOT NULL
        )",
        params![],
    )?;

    tx.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id    INTEGER PRIMARY KEY,
            name     TEXT NOT NULL,
            email    TEXT NOT NULL,
            money    INTEGER NOT NULL,
            UNIQUE(email)
        )",
        params![],
    )?;

    tx.commit()?;
    Ok(())
}

// cargo run --example uow-with-state
fn main() -> ShortResult<()> {
    // ------------------------------------------------ PREPARE --------------------------------------------------
    let pool: r2d2::Pool<SqliteConnectionManager> = {
        let manager =
            SqliteConnectionManager::file("design_patterns/unit-of-works/db/uow_with_state.db");
        r2d2::Pool::new(manager).unwrap()
    };

    let _ = init_schema(pool.clone());

    let mut uow = UnitOfWorkWithState::new(db_layer::ConcreteDb::new(pool.clone()));

    let user_id: i64 = {
        let (name, email_unique, money) = ("Alex", "Alex@email.com", 100);
        let entity = UserEntityDTO::new(name, email_unique, money);
        uow.register_new_user(entity);
        uow.commit();
        let new_user = uow.get_user_by_email(email_unique).unwrap(); // TODO: get_user не для UOW

        println!("-------------------------------");
        println!("Before orders: {:?}", new_user);
        new_user.get_id()
    };

    // ------------------------------------------------ UOW ------------------------------------------------------
    // Сценарий: Новый заказ (этот сценарий не показывает преимущество Unit Of Work с поддержкой состояния,
    //    т.е. количество запросов в базу не сокращает, но под транзакцией все же есть оптимизация в базе данных)
    // Если взять за транзакцию больше операций, то выгода от состояния появялется.

    // -- START TRANSACTION --
    for (product, total_cost) in [
        ("TV Toshiba", 77u64),
        ("Bread", 3u64),
        ("Notepad iPad", 19u64),
    ] {
        // Изменение существующей сущности User
        let current_user: UserEntity = uow.get_user(user_id).unwrap();
        if let Some(res) = current_user.get_money().checked_sub(total_cost) {
            uow.register_dirty_user(
                current_user.get_id(),
                UserEntityDTO::new(current_user.get_name(), current_user.get_email(), res),
            );
        } else {
            uow.clear();
            return Err("Insufficient funds in the account".into());
        }

        // Создание новой сущности Order
        uow.register_new_order(OrderEntityDTO::new(
            current_user.get_id(),
            product,
            total_cost,
        ));
    }
    uow.commit();
    // -- STOP TRANSACTION --

    // ------------------------------------------------ SHOW -----------------------------------------------------
    show(pool, user_id)
}

fn show(pool: r2d2::Pool<SqliteConnectionManager>, user_id: i64) -> ShortResult<()> {
    let conn = pool.get().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, user_id, product, total_cost FROM orders WHERE user_id = ?1")
        .unwrap();
    let orders_rows = stmt
        .query_map([&user_id], |row| {
            Ok(OrderEntity::new(
                row.get::<_, i64>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, u64>(3)?,
            ))
        })
        .unwrap();
    let orders: Vec<OrderEntity> = orders_rows.into_iter().map(|o| o.unwrap()).collect();
    println!("Orders:");
    for order in orders {
        println!("{:?}", order);
    }

    let mut stmt = conn
        .prepare("SELECT id, name, email, money FROM users WHERE id = ?1")
        .unwrap();
    let user = stmt.query_row([&user_id], |row| {
        Ok(UserEntity::new(
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, u64>(3)?,
        ))
    });
    println!("After orders: {:?}", user.unwrap());

    Ok(())
}
```




[Unit of Work www.sitepoint.com](https://www.sitepoint.com/implementing-a-unit-of-work/)

[Unit of Work gist.github.com/voronkovich](https://gist.github.com/voronkovich/d35cdcdf6eb09e986ab9b16f91a5b2e8)

[Unit of Work design-pattern.ru](http://design-pattern.ru/patterns/unit-of-work.html)

[Unit of Work www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2018/04/unit-of-work.html)

[Unit of Work learn.microsoft.com](https://learn.microsoft.com/en-us/previous-versions/msp-n-p/ff649690(v=pandp.10)?redirectedfrom=MSDN)

---

## Lazy Load 

`TODO: добавить больше информации, пример, пояснение`

Загрузка данных по мере необходимости. Объект, не содержит данных, но знает, где их взять.
При первом обращении или при свободном ресурсе, происходит загрузка, последующие обращения используют тот же обьект не загружая обьект из источника.

**Существует четыре основных варианта ленивой загрузки.**

| Подход              | Когда грузим                     | Как устроено                                 |
| ------------------- | -------------------------------- | -------------------------------------------- |
| Lazy Initialization | При первом доступе к **полю**    | Маркер `None` / `null`                       |
| Virtual Proxy       | При первом **методе интерфейса** | Прокси с тем же API, создаёт реальный объект |
| Value Holder        | При первом `getValue()`          | Обёртка с лямбдой-загрузчиком                |
| Ghost               | При первом методе                | Пустой объект, сразу грузит **все** данные   |

<details>
<summary>...</summary>

1. Lazy Initialization (Ленивая Инициализация) использует специальный макер (обычно null), чтобы пометить поле, как не загруженное. При каждом обращении к полю проверяется значение маркера и, если значение поля не загружено - оно загружается.

```rust
struct Data {
    value: Option<String>,
}

impl Data {
    fn new() -> Self {
        Self { value: None }
    }

    fn get_value(&mut self) -> &str {
        if self.value.is_none() {
            println!("Загружаем данные...");
            self.value = Some("Hello from DB".to_string());
        }
        self.value.as_ref().unwrap()
    }
}

fn main() {
    let mut data = Data::new();
    println!("Первый доступ: {}", data.get_value());
    println!("Второй доступ: {}", data.get_value()); // Уже без загрузки
}

```

2. Virtual Proxy (Виртуальный Прокси) - объект с таким же интерфейсом, как и настоящий объект. При первом обращении к методу объекта, виртуальный прокси загружает настоящий объект и перенаправляет выполнение.  Прокси реализует тот же интерфейс (Service), но создаёт настоящий объект только при первом вызове.

```rust
trait Service {
    fn process(&self);
}

struct RealService {
    payload: String,
}

impl RealService {
    fn new() -> Self {
        println!("Загружаем реальный объект...");
        Self { payload: "Real Data".into() }
    }
}

impl Service for RealService {
    fn process(&self) {
        println!("Обработка: {}", self.payload);
    }
}

struct VirtualProxy {
    real: Option<RealService>,
}

impl VirtualProxy {
    fn new() -> Self {
        Self { real: None }
    }
}

impl Service for VirtualProxy {
    fn process(&self) {
        if self.real.is_none() {
            // ❌ Ошибка: self не mut — значит надо менять структуру
            // решение: сделать RefCell для внутренней мутации
            println!("Придётся использовать RefCell для ленивой инициализации");
        }
    }
}

fn main() {
    // Чтобы Virtual Proxy работал без лишнего мута, используем RefCell
    use std::cell::RefCell;

    struct VirtualProxyCell {
        real: RefCell<Option<RealService>>,
    }

    impl VirtualProxyCell {
        fn new() -> Self {
            Self { real: RefCell::new(None) }
        }
    }

    impl Service for VirtualProxyCell {
        fn process(&self) {
            if self.real.borrow().is_none() {
                let mut r = self.real.borrow_mut();
                println!("Создаём реальный объект...");
                *r = Some(RealService::new());
            }
            self.real.borrow().as_ref().unwrap().process();
        }
    }

    let proxy = VirtualProxyCell::new();
    proxy.process(); // Создание и вызов
    proxy.process(); // Уже без создания
}

```

3. Value Holder (Контейнер значения) - объект с методом getValue. Клиент вызывает метод getValue, чтобы получить реальный объект. getValue вызывает загрузку. Хранится замыкание, которое знает, как получить данные, и само значение.

```rust
struct ValueHolder<T, F>
where
    F: Fn() -> T,
{
    value: Option<T>,
    loader: F,
}

impl<T, F> ValueHolder<T, F>
where
    F: Fn() -> T,
{
    fn new(loader: F) -> Self {
        Self { value: None, loader }
    }

    fn get_value(&mut self) -> &T {
        if self.value.is_none() {
            println!("Загружаем через ValueHolder...");
            self.value = Some((self.loader)());
        }
        self.value.as_ref().unwrap()
    }
}

fn main() {
    let mut holder = ValueHolder::new(|| {
        println!("Функция загрузки...");
        "Data from Loader".to_string()
    });

    println!("{}", holder.get_value()); // первая загрузка
    println!("{}", holder.get_value()); // уже кэшировано
}

```

4. Ghost (Призрак) - объект без каких-либо данных. При первом обращении к его методу, призрак загружает все данные сразу. Объект пустой при создании, но при первом использовании грузит всю необходимую информацию.

```rust
struct Ghost {
    loaded: bool,
    data: Option<String>,
}

impl Ghost {
    fn new() -> Self {
        Self { loaded: false, data: None }
    }

    fn process(&mut self) {
        if !self.loaded {
            println!("Ghost загружает ВСЕ данные сразу...");
            self.data = Some("Full dataset".into());
            self.loaded = true;
        }
        println!("Работаем с: {}", self.data.as_ref().unwrap());
    }
}

fn main() {
    let mut ghost = Ghost::new();
    ghost.process(); // Загружает сразу всё
    ghost.process(); // Уже использует готовые данные
}

```
</details>

P.S. В Rust'е итераторы ленивы, также `std::borrow::Cow` обладает свойствами бережного обращения к русурсам, и smart pointer `Rc/Arc`

[Lazy Load](http://design-pattern.ru/patterns/lazy-load.html)

---
 

## [Паттерны Объектно-Реляционного структурирования](http://design-pattern.ru/patterns/)
`TODO: добавить больше информации, пример, пояснение`

- Identity Field (Поле первичного ключа)
- Foreign Key Mapping (Разметка внешних ключей)
- Association Table Mapping (Разметка таблиц связей)
- Dependent Mapping (Управление распределением подчинённых сущностей)
- Embedded Value (Объединённое свойство)
- Serialized LOB (Сериализованный LOB)
- Single Table Inheritance (Наследование с единой таблицей)
- Class Table Inheritance (Наследование с таблицами классов)
- Concrete Table Inheritance (Наследование с таблицами конечных классов)
- Inherritance Mappers (Наследуемые распределители)