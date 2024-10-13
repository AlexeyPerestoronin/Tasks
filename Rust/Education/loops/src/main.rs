use rand::Rng;

fn main() {
    let mut counter: u32 = rand::thread_rng().gen_range(1..=10);
    println!("counter = {counter}");

    let name = 'loop1: loop {
        if counter == 0 {
            break "loop №1"
        };

        counter -= 1;

        loop {
            if counter == 0 {
                break 'loop1 "loop №2"
            };

            counter -= 1;
            break;
        };
    };

    println!("{name} was decrementing first!");

    let massive = 1..5;

    for number in massive {
        println!("{number}");
    };
}
