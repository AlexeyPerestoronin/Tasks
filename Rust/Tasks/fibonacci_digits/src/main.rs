use std;

fn get_user_digit<T>(welcome_message: &str) -> T 
    where T: std::str::FromStr
{
    println!("{welcome_message}");
    let mut buff = String::new();
    std::io::stdin()
        .read_line(&mut buff)
        .expect("Failed to read line!");

    return buff.trim().parse::<T>().expect("Parse error!");
}

fn main() {
    let mut d1 = 0u32;
    let mut d2 = 1u32;

    for _ in 1..get_user_digit::<u32>("Input Fibonacci number:") {
        d2 = d1 + d2;
        d1 = d2 - d1;
    }

    println!("Fibonacci is {d2}")
}
