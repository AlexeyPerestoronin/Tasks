#[path = "user_input.rs"]
mod user_input;

use rand::Rng;
use std::{collections::BTreeSet, error::Error};

pub type VecI32 = Vec<i32>;

pub fn obtain_massive() -> Result<VecI32, Box<dyn Error>> {
    match user_input::choose_from(
        "Which massive do you want?",
        &vec!["random generated;", "keyboard input;"],
    )? {
        0 => Ok(random_massive_generation(
            user_input::direct::<usize>("Which size of massive? → ")?,
            user_input::direct::<i32>("Which min value of a element? → ")?,
            user_input::direct::<i32>("Which max value of a element? → ")?,
        )),
        1 => Ok(user_massive_generation()),
        _ => Err("Error: unavailable choose!".into()),
    }
}

pub fn detect_max(massive: &VecI32) -> Option<i32> {
    Some(massive.iter().max()?.to_owned())
}

pub fn detect_min(massive: &VecI32) -> Option<i32> {
    Some(massive.iter().min()?.to_owned())
}

pub fn detect_median(massive: &VecI32) -> Option<i32> {
    Some(massive.get(massive.len() / 2)?.to_owned())
}

fn random_massive_generation(len: usize, min: i32, max: i32) -> VecI32 {
    let mut random = rand::thread_rng();
    (0..len)
        .map(|_| random.gen_range(min..=max))
        .collect::<VecI32>()
}

fn user_massive_generation() -> VecI32 {
    (0..user_input::direct::<usize>("Which size of massive? → ")
        .expect("Size of massive wasn't determined!"))
        .map(|i| {
            user_input::direct::<i32>(format!("Which value for {i} element? → ").as_str())
                .expect(format!("The {i} element wasn't determined!").as_str())
        })
        .collect::<VecI32>()
}
