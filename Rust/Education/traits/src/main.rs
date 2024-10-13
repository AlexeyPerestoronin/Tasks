use std::os::windows::fs::FileTimesExt;

fn main() {
    Letter {
        address_from: "123 Main St, Anytown, CA 12345".to_string(),
        address_to: "456 Elm St, Othertown, NY 67890".to_string(),
        text: "Access denied!".to_string(),
    }
    .to_string();

    let texts = vec![
        Text::Letter(Letter {
            address_from: "123 Main St, Anytown, CA 12345".to_string(),
            address_to: "456 Elm St, Othertown, NY 67890".to_string(),
            text: "Access denied!".to_string(),
        }),
        Text::Message(Message {
            site: "www.gotham.ru".to_string(),
            nickname_from: "Batman".to_string(),
            nickname_to: "Joker".to_string(),
            text: "I've win you!".to_string(),
        }),
        Text::Diary(Diary {
            owner: "Mikhail Bulgakov, Moscow, 1954".to_string(),
            text: "The Master met Margo when he was walking in the square of...".to_string(),
        }),
    ];

    for text in &texts {
        println!("{}", extract_author(text));
    }
}

trait GetAuthor {
    fn get_author(&self) -> String;
}

fn extract_author<T: GetAuthor>(text: &T) -> String {
    text.get_author()
}

struct Letter {
    address_from: String,
    address_to: String,
    text: String,
}

impl ToString for Letter {
    // this code is ignoring!
    fn to_string(&self) -> String {
        "impl ToString for Letter".to_string()
    }
}

impl Letter {
    // this code is using!
    fn to_string(&self) -> String {
        "impl Letter".to_string()
    }
}


impl GetAuthor for Letter {
    fn get_author(&self) -> String {
        format!("{}!", self.address_from)
    }
}

struct Message {
    site: String,
    nickname_from: String,
    nickname_to: String,
    text: String,
}

impl GetAuthor for Message {
    fn get_author(&self) -> String {
        format!("{}", self.nickname_from)
    }
}

struct Diary {
    owner: String,
    text: String,
}

impl GetAuthor for Diary {
    fn get_author(&self) -> String {
        format!("{}", self.owner)
    }
}

enum Text {
    Letter(Letter),
    Message(Message),
    Diary(Diary),
}

impl GetAuthor for Text {
    fn get_author(&self) -> String {
        match self {
            Text::Letter(x) => x.get_author(),
            Text::Message(x) => x.get_author(),
            Text::Diary(x) => x.get_author(),
        }
    }
}
