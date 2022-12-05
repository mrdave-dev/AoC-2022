use std::{fs, collections::{HashMap, binary_heap::PeekMut, HashSet}, option::Iter};
use core::fmt;

/**
 * 1. For each line:
 *   1. Split the string in half
 *   2. Find the common element in the two halves
 *   3. Add to the score
 * 
 * Lowercase item types a through z have priorities 1 through 26.
 * Uppercase item types A through Z have priorities 27 through 52.
 */

fn get_file_content() -> String {
    return fs::read_to_string("./input.txt")
        .expect("File './input.txt' should have been readable");
}

fn slice_string(input: &str) -> (&str, &str) {
    if (input.len() % 2 == 1) {
        panic!("Expected length to be even, was {}", input.len());
    }
    let half_length = ((input.len() as f64)/2 as f64).floor() as usize;
    return (&input[0..half_length], &input[half_length..input.len()])
}

/// Rust: a = 97, A = 65
/// AoC : a = 1, A = 27
fn get_letter_value(input: &char) -> u32 {
    match input.cmp(&'a') {
        // Uppercase
        std::cmp::Ordering::Less => {
            (*input as u32 - ('A' as u32)) + 27
        },
        std::cmp::Ordering::Equal => {
            1
        },
        // Lowercase
        std::cmp::Ordering::Greater => {
            (*input as u32 - ('a' as u32)) + 1
        },
    }
}

fn main() {
    println!("Hello, world!");

    let file_content = get_file_content();
    let lines: Vec<&str> = file_content.split("\n").collect();
    let mut total_score = 0;
    // for line in lines {
    //     let (first_half, second_half) = slice_string(line);
    //     println!("{} | {}", first_half, second_half);

    //     let mut first_half_occurrences: HashMap<char, i32> = HashMap::new();
    //     let mut second_half_occurrences: HashMap<char, i32> = HashMap::new();
    //     let mut common_occurrences: HashMap<char, i32> = HashMap::new();
    //     for c in first_half.chars() {
    //         *first_half_occurrences.entry(c).or_insert(0) += 1;
    //     }

    //     for c in second_half.chars() {
    //         if (first_half_occurrences.get(&c).is_some()) {
    //             let first_half_count = first_half_occurrences.get(&c).or(Some(&1)).unwrap();
    //             *common_occurrences.entry(c).or_insert(*first_half_count) += 1;
    //         }
    //         *second_half_occurrences.entry(c).or_insert(0) += 1;
    //     }
        
    //     println!("{:?}", common_occurrences.iter().map(|(k, v)| format!("({}, {})", k, v)));
    //     common_occurrences.iter().for_each(|(k, v)| {
    //         println!(" Letter {} value = {}", k, get_letter_value(k));
    //         total_score += get_letter_value(k);
    //     })
    // }

    // let mut iter = lines.iter();

    // let iter = lines.iter().peekable();
    // while let Some((x, y, z)) = iter.take(3).collect() {
    // let mut x = &iter.take(3);

    let loop_times = ((lines.len() as f32) / 3.0).ceil();

    // while ()
    // while let Some(x) = iter.next() {
    for n in 0..loop_times as usize {
        let start_index = (n*3); 
        let first_elm = lines[start_index]; 
        let second_elm = lines[start_index+1];
        let third_elm = lines[start_index+2];
        println!("{}, {}, {}, {}", start_index, first_elm, second_elm, third_elm);

        
        let mut occurrences: HashMap<char, u32> = HashMap::new();
        for elm in [first_elm, second_elm, third_elm] {
            let mut characters_in_string: HashSet<char> = HashSet::new();
            for c in elm.chars() {
                characters_in_string.insert(c);
            }
            for c in characters_in_string {
                *occurrences.entry(c).or_insert(0) += 1;
            }
        }

        occurrences.retain(|_k, &mut v| v > 2);
        let x: u32 = occurrences.iter().map(|(c, _)| get_letter_value(c)).sum();
        println!("x = {}", x);
        total_score += x;
    }

    println!("Total score: {}", total_score);
    println!("{}, {}", loop_times, lines.len());
    
}
