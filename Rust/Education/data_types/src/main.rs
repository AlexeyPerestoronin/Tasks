fn main() {
    let number = "153".parse::<u32>().expect("not a number!");
    println!("The number is {number}");
    
    let number : i32 = "-88".parse().expect("not a number!");
    println!("The number is {number}");

    let number : i64 = -1_321_000;
    println!("The number is {number}");

    let number : u8 = 0b1111_1111;
    println!("The number is {number}");

    let number : f32 = "3.14".parse().expect("not a number!");
    println!("The number is {number}");
    
    // compilation error: type conversion error
    // let number : i32 = 3.14;
    // println!("The number is {number}");
    
    let ch : char = '🥰';
    println!("The char is {ch}");

    let tup: (char, u32, f64) = ('🌟', 55, 1.47);
    println!("The tup is {tup:?}");
    println!("The tup.2 is {}", tup.2);

    let massive : [i32;5] = [1,2,3,4,5];
    println!("The massive is {massive:?}");
    println!("The massive[2] is {}", massive[2]);
    
    let massive  = [1;2];
    println!("The massive is {massive:?}");
    
    let massive  = [2;1];
    println!("The massive is {massive:?}");

    let massive  = [2;22];
    println!("The massive is {massive:?}");
}
