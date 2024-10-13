mod manipulation;
mod user_input;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut massive = manipulation::obtain_massive().expect("Error during obtaining massive!");
    println!("The massive is {massive:?}");

    loop {
        match user_input::choose_from(
            "What need to do with the massive?",
            &vec![
                "print",
                "get max value;",
                "get min value;",
                "get median value",
                "sort",
                "exit!",
            ],
        )? {
            0 => println!("The massive is: {massive:?}"),
            1 => match manipulation::detect_max(&massive) {
                None => println!("Massive hasn't maximum value!"),
                Some(x) => println!("Max value is {x}"),
            },
            2 => match manipulation::detect_min(&massive) {
                None => println!("Massive hasn't minimum value!"),
                Some(x) => println!("Min value is {x}"),
            },
            3 => match manipulation::detect_median(&massive) {
                None => println!("Massive hasn't median value!"),
                Some(x) => println!("Median value is {x}"),
            },
            4 => {
                massive.sort();
                println!("The sort massive is: {massive:?}");
            },
            _ => break,
        };
    }

    Ok(())
}
