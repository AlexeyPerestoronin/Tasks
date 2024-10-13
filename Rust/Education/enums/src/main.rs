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

fn main() {
    let item = Item {
        id: 0,
        data: Data::None,
    };
    println!("{item:?}\n{}", item.to_string());

    let item = Item {
        id: 0,
        data: Data::Country {
            name: "Russia".to_string(),
            square: 17_125_200,
        },
    };
    println!("{item:?}\n{}", item.to_string());

    let item = Item {
        id: 0,
        data: Data::Person {
            name: "Vladimir".to_string(),
            surname: "Putin".to_string(),
            age: 69,
            gender: Gender::Men,
        },
    };
    println!("{item:?}\n{}", item.to_string());

    let item = Item {
        id: 0,
        data: Data::OOO {
            name: "Cows and Bowls".to_string(),
            inn: (1, 2, 3, 4),
        },
    };
    println!("{item:?}\n{}", item.to_string());
}
