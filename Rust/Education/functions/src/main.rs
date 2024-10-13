fn main() {
    println!("main");

    function_1();
    function_2(32);
    function_3();
    let res = function_4();
    println!("function_4: {res:?}");
    let res = function_5();
    println!("function_5: {res:?}");
    let res = function_6();
    println!("function_6: {res:?}");
}

fn function_1() {
    println!("function_1");
}

fn function_2(x: i32) {
    println!("function_2: {x}");
}

fn function_3() {
    let y = {
        let x = 44;
        x * 2
    };

    println!("function_3: {y}");
}

fn function_4() -> (u32, char) {
    (11, '🤔')
}

fn function_5() -> (u32, char) {
    return (12, '😎')
}

/// # Function №6
/// ***
/// Проверка работоспособности md-комментариев.
fn function_6() -> (u32, char) {
    return (13, '😄');
}

// fn function_3(x: &i32) {
//     x = x + 1;
//     println!("function_2: {x}.");
// }
