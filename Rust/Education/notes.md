# Условные выражения
## if with let
```rs
let condition = ...;
let number = if condition { 5 } else { 8 };
```

# Структуры
Объявление типа структуры:
```rs
// case-1
struct INN(u8, u8, u8, u8);

// case-2
struct User {
    first_name: String,
    last_name: String,
    age: u8,
    country: String,
    city: String,
    inn: INN,
}
```

Создание переменной:
```rs
let user1 = User {
    first_name: String::from_str("Jo").expect("fist name isn't valid"),
    last_name: String::from_str("Black").expect("last name isn't valid"),
    age: 33,
    city: String::from_str("New York").expect("city isn't valid"),
    country: String::from_str("USA").expect("country isn't valid"),
    inn: INN(1, 2, 3, 4),
};

let user2 = User {
    first_name: String::from("Suzann"),
    last_name: String::from("Greenwood"),
    ..user1
};
```

Объявление методов:
```rs
impl User {
    // static method
    fn dbg_build(user: User) -> User {
        println!("User::dbg_build");
        User { ..dbg!(user) }
    }

    // immutable method
    fn get_country_code(&self) -> u32 {
        println!("User::get_country_code");
        if self.city.eq("USA") {
            0u32
        } else {
            1u32
        }
    }

    // ownership method
    fn clear(self) {
        println!("User::clear");
    }
}
```

# Перечисления
Вариант №1:
```rs
#[derive(Debug)]
enum Gender {
    Men,
    Women,
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    gender: Gender,
}

#[derive(Debug)]
struct Country {
    name: String,
    square: u32,
}

#[derive(Debug)]
enum Data {
    Person(Person),
    Country(Country),
}

#[derive(Debug)]
struct Item {
    id: u32,
    data: Data,
}
```

Вариант №2:
```rs
#[derive(Debug)]
enum Gender {
    Men,
    Women,
}

#[derive(Debug)]
enum Data {
    None,
    Country {
        name: String,
        square: u32,
    },
    Person {
        name: String,
        surname: String,
        age: u8,
        gender: Gender,
    },
    OOO {
        name: String,
        inn: (u8, u8, u8, u8),
    },
}

#[derive(Debug)]
struct Item {
    id: u32,
    data: Data,
}
```

# Сопоставления с образцом match
```rs
impl Item {
    fn to_string(&self) -> String {
        format!(
            "custom string representation is:\nid = {}\n{}",
            self.id,
            // we have to use '&' cause the self is passed with '&'
            match &self.data {
                Data::None => "None".to_string(),
                Data::Country { .. } => "Country".to_string(),
                Data::Person {
                    name: _, // escaping of unused members
                    surname: _,
                    age: _,
                    gender,
                } => {
                    println!("Data::Person");
                    format!(
                        "{}-Person",
                        match gender {
                            Gender::Men => "Men",
                            Gender::Women => "Women",
                        }
                    )
                }
                Data::OOO {
                    name,
                    inn: (a, b, c, d),
                } => {
                    format!("«{}» OOO-{}:{}:{}:{}", name, a, b, c, d)
                }
            }
        )
    }
}
```