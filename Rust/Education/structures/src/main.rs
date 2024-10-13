use std::str::FromStr;

#[derive(PartialEq)]
#[derive(Debug)]
enum Country {
    Russia,
    USA
}

#[derive(Debug)]
struct INN(u8, u8, u8, u8);

#[derive(Debug)]
struct User {
    first_name: String,
    last_name: String,
    age: u8,
    country: Country,
    city: String,
    inn: INN,
}

impl User {
    // static method
    fn dbg_build(user: User) -> User {
        dbg!("User::dbg_build");
        User { ..dbg!(user) }
    }

    // immutable method
    fn get_country_code(&self) -> u32 {
        dbg!("User::get_country_code");
        if self.country == Country::Russia {
            1u32
        } else if self.country == Country::USA {
            2u32
        } else {
            0u32
        }
    }

    // ownership method
    fn clear(self) {
        dbg!("User::clear");
    }
}

fn main() {
    let user1 = User::dbg_build(User {
        first_name: "Jo".to_string(),
        last_name: "Black".to_string(),
        age: 33,
        city: "New Y".to_string(),
        country: Country::USA,
        inn: INN(1, 2, 3, 4),
    });
    println!("{user1:?}");

    println!("country code is {}", user1.get_country_code());

    user1.clear();
    // compile error
    // println!("{user1:?}");
}
