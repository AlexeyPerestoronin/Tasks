// затенение переменных → obscures of variables
fn obscures() {
    let x = 5;
    let x = x + 1;
    println!("scope = main→obscures: x={x}");
    {
        let x = x * 2;
        println!("scope = main→obscures→(): x={x}");
    }
}

fn mutable_1() {
    let spaces = "    ";
    println!("scope = main→mutable_1: spaces={spaces}");

    let spaces = spaces.len();
    println!("scope = main→mutable_1: spaces={spaces}");
}

fn mutable_2() {
    let mut spaces = "    ";
    println!("scope = main→mutable_1: spaces={spaces}");

    // compilation error: types is not equal
    // spaces = spaces.len();
    println!("scope = main→mutable_1: spaces={spaces}");
}

fn main() {
    obscures();
    mutable_1();
    mutable_2();
}
