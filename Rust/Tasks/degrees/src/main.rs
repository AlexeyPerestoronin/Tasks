use std;

fn get_user_digit(welcome_message: &str) -> u32 {
    println!("{welcome_message}");
    let mut buff = String::new();
    std::io::stdin()
        .read_line(&mut buff)
        .expect("Failed to read line!");

    return buff.trim().parse::<u32>().expect("Parse error!");
}

fn main() {
    let celsius_degree = get_user_digit("Input degrees by Celsius:");
    let fahrenheit_degree = (celsius_degree * 9 / 5) + 32;

    println!("Fahrenheit degrees is {fahrenheit_degree}");
}
