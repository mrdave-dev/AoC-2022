use std::{fs, collections::HashSet};

fn get_file_content() -> String {
    return fs::read_to_string("./input.txt")
        .expect("File './input.txt' should have been readable");
}

fn has_all_unique_chars(sub_str: &str) -> bool {
    println!("Considering: {}", sub_str);
    let mut characters_in_str: HashSet<char> = HashSet::new();
    for c in sub_str.chars() {
        if characters_in_str.contains(&c) {
            return false
        } else {
            characters_in_str.insert(c);
        }
    }
    return true;
}

fn main() {
    let input = get_file_content();
    println!("Input: {}", input);
    
    for n in 0..(input.len()-3) {
        println!("n = {}", n);
        match has_all_unique_chars(&input.as_str()[n..n+14]) {
            true => {
                println!("Start found: {}", n+14);
                break;
            },
            false => {
                continue
            },
        };
    }
    
}
