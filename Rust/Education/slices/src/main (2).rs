// using std;

fn main() {
    let message = String::from("Hello, World!");
    for (i, ch) in message.chars().enumerate() {
        println!("message[{i}] = {ch}");
    }
    let slice = &message[0..5];
    println!("slice[0..5] = {slice}");
    
    let slice = &message[..5];
    println!("slice[..5] = {slice}");
    
    let slice = &message[5..];
    println!("slice[5..] = {slice}");
    
    let slice = &message[..];
    println!("slice[..] = {slice}");
}
