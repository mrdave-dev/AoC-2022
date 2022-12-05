use std::{fs, collections::HashMap};
use core::fmt;

fn get_file_content() -> String {
    return fs::read_to_string("./input.txt")
        .expect("File './input.txt' should have been readable");
}

struct Bounds {
    lower: u32,
    upper: u32,
}

impl fmt::Debug for Bounds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "[{}, {}]", self.lower, self.upper);
    }
}

fn get_bounds(input: &str) -> (Bounds, Bounds) {
    let items: Vec<&str> = input.split(",").collect();
    assert!(items.len() == 2, "Expected item to be of length 2 after split");
    
    let first_item_ints: Vec<u32> = items[0].split("-")
        .map(|n| n.parse().unwrap())
        .collect();
    assert!(first_item_ints.len() == 2, "Expected first item to be of length 2");
    let first_item_bounds = Bounds { lower: first_item_ints[0], upper: first_item_ints[1] };
    
    let second_item_ints = items[1].split("-")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u32>>();
    assert!(second_item_ints.len() == 2, "Expected second item to be of length 2");
    let second_item_bounds = Bounds { lower: second_item_ints[0], upper: second_item_ints[1] };
    
    let first_item_size = first_item_bounds.upper - first_item_bounds.lower;
    let second_item_size = second_item_bounds.upper - second_item_bounds.lower;
    
    if (first_item_size < second_item_size) {
        return (first_item_bounds, second_item_bounds);
    } else {
        return (second_item_bounds, first_item_bounds);
    }
}


fn main() {
    let content = get_file_content();
    let lines: Vec<&str> = content.split('\n').collect();

    let mut total_subsets: u32 = 0;
    let mut total_overlapping_pairs: u32 = 0;
    let mut item_index_map: HashMap<u32, u32> = HashMap::new();
    // find the item with the lesser lower bound
    // if the second item's lower bound is gte the first item's lower bound and gte the first item's upper bound
    // and the second item's upper bound is lte the first item's upper bound and gte the first item's lower bound
    // it's fully contained
    for line in lines {
        let (first_item_bounds, second_item_bounds) = get_bounds(line);
        
        if 
            first_item_bounds.lower >= second_item_bounds.lower && 
            first_item_bounds.upper <= second_item_bounds.upper 
        {
            println!("Subset found: {:?}, {:?}", first_item_bounds, second_item_bounds);
            total_subsets += 1;
        }
        
        if (
            first_item_bounds.lower >= second_item_bounds.lower && 
            first_item_bounds.lower <= second_item_bounds.upper
        ) || (
            first_item_bounds.upper >= second_item_bounds.lower &&
            first_item_bounds.upper <= second_item_bounds.upper
        ) {
            println!("Overlap found: {:?}, {:?}", first_item_bounds, second_item_bounds);
            total_overlapping_pairs += 1;
        }
        

    }

    println!("Total subsets: {}", total_subsets);
    println!("Total overlaps: {}", total_overlapping_pairs);
    
}
