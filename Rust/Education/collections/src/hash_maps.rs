use std::collections::HashMap;

fn char_counter(string: &str) -> HashMap<char, i32> {
    let mut letters = HashMap::new();

    for ch in string.chars() {
        letters.entry(ch).and_modify(|counter| { *counter += 1; }).or_insert(1);
    }

    return letters;
}

pub fn test() {
    let mut hash_map = HashMap::<i32, &str>::new();
    hash_map.insert(0, "Zero");
    hash_map.insert(1, "One");
    hash_map.insert(2, "Two");
    hash_map.insert(3, "Free");
    hash_map.insert(4, "Four");
    hash_map.insert(5, "Five");
    hash_map.insert(6, "Six");
    hash_map.insert(7, "Seven");
    hash_map.insert(8, "Eight");
    hash_map.insert(9, "Nine");
    for (key, value) in &hash_map {
        println!("{key} = {value}");
        for (char, quantity) in char_counter(&value) {
            println!("{char} occurs {quantity} times");
        }
    }
}