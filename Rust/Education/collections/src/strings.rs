pub fn test() {
    let greetings = vec![
        String::from("السلام عليكم"),
        String::from("Dobrý den"),
        String::from("Hello"),
        String::from("שלום"),
        String::from("नमस्ते"),
        String::from("こんにちは"),
        String::from("안녕하세요"),
        String::from("你好"),
        String::from("Olá"),
        String::from("Здравствуйте"),
        String::from("Hola"),
    ];

    for greeting in &greetings {
        println!("---");
        print!("Chars = ");
        for (i, symbol) in greeting.chars().enumerate() {
            print!("[{i},{symbol}]");
        }
        println!();
        print!("Bytes = ");
        for (i, byte) in greeting.bytes().enumerate() {
            print!("[{i},{byte}]");
        }
        println!();
    }
}
