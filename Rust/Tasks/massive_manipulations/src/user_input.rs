use std::{error::Error, io};

pub fn choose_from(
    welcome_message: &str,
    available_choose: &Vec<&str>,
) -> Result<usize, Box<dyn Error>> {
    println!("{welcome_message}");
    for (i, choose) in available_choose.iter().enumerate() {
        println!("{i}) {choose}");
    }
    println!("Your choose is ");

    let mut buf = String::new();
    for _ in 0..10 {
        buf.clear();
        io::stdin().read_line(&mut buf)?;
        let choose = buf.trim().parse::<usize>()?;
        if choose < available_choose.len() {
            return Ok(choose);
        }

        println!(" [wrong input] ");
    }

    Err("Error: too much attempts!".into())
}

pub fn direct<R>(welcome_message: &str) -> Result<R, Box<dyn Error>>
where
    R: std::str::FromStr,
{
    println!("{welcome_message}");
    let mut buf = String::new();
    for _ in 0..10 {
        io::stdin().read_line(&mut buf)?;
        if let Ok(x) = buf.trim().parse::<R>() {
            return Ok(x);
        }

        println!(" [wrong input] ");
    }

    Err("Error: too much attempts!".into())
}
