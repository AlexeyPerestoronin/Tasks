pub fn test() {
    let mut vector = vec![1, 3, 3, 4, 5];
    for element in &vector {
        println!("{element}");
    }

    println!("---");

    for element in &mut vector {
        *element *= *element;
        println!("{}", element);
    }

    println!("---");

    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    for (i, row) in matrix.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            println!("[{i},{j}] = {value}");
        }
    }
}
