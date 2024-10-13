use rand::{self, Rng};

fn main() {
    let number1 = rand::thread_rng().gen_range(1..=10);
    println!("the number1 is {number1}");

    let number2 = rand::thread_rng().gen_range(1..=10);
    println!("the number2 is {number2}");

    let diff = if number1 > number2 {
        println!("if: {number1} > {number2}");
        number1 - number2
    } else if number1 < number2 {
        println!("else if: {number1} < {number2}");
        number2 - number1
    } else {
        println!("else: {number1} = {number2}");
        0i32
    };

    println!("diff is {diff}");
}
