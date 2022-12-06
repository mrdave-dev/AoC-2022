use std::fs;

fn get_file_content() -> String {
    return fs::read_to_string("./input.txt")
        .expect("File './input.txt' should have been readable");
}

fn parse_stack_order(input: Vec<&str>) -> (Vec<Vec<char>>, Vec<&str>) {
    let input_iter = input.into_iter();
    let mut input_iter_mut = input_iter.clone();
    // n = find the line break
    let n_opt = input_iter_mut.position(|x| String::from(x).eq(""));
    // assert if no line break
    assert!(n_opt.is_some(), "Expected line break");
    let n = n_opt.unwrap();
    println!("Found line break at index {}", n);
    // find the stacks list size at n-1
    input_iter_mut = input_iter.clone();
    let stacks_label = input_iter_mut.into_iter().as_slice()[n-1];
    println!("Stacks label: {}", stacks_label);
    let stacks_label_vec: Vec<&str> = stacks_label.trim()
        .split_whitespace()
        .collect();
    let stacks_len = stacks_label_vec.len();
    println!("Label vec: {:?}", stacks_label_vec);
    println!("Label len: {}", stacks_len);
    // declare vec
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); stacks_len];

    // backwards to start, add to stacks 
    input_iter_mut = input_iter.clone();
    let lines = Vec::from(
        &input_iter_mut.as_slice()[0..n-1]
    ).into_iter()
    .rev();

    println!("lines: {:#?}", lines);
    
    for (y, line) in lines.enumerate() {
        println!("Current line: {}", line);
        let entries: Vec<String> = line.chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .map(|x| x.into_iter().collect::<String>().trim().to_string())
            .collect();
            
        println!("Entries: {:?}", entries);
        for (x, entry) in entries.into_iter().enumerate() {
            let c = entry.chars().nth(1);
            if c.is_some() {
                println!("c = {}, ({}, {}), col", c.unwrap(), x, y);
                stacks[x].push(c.unwrap());
            }
        }
    }
    
    input_iter_mut = input_iter.clone();
    let instructions = Vec::from(&input_iter_mut.as_slice()[n+1..]);
    return (stacks, instructions);
}

fn apply_instruction(stacks: &mut Vec<Vec<char>>, instruction: &str) {
    // move 5 from 3 to 6
    let instruction_parts: Vec<&str> = instruction.split_whitespace().collect();
    // println!("I: {}", instruction);
    let amount: u32 = instruction_parts[1].parse().unwrap();
    // Minus 1 because the stacks are zero-based
    let from_stack: u32 = instruction_parts[3].parse::<u32>().unwrap() - 1;
    let to_stack: u32 = instruction_parts[5].parse::<u32>().unwrap() - 1;

    // println!("Take {} from stack {} and put it on {}", amount, from_stack, to_stack);
    
    let mut intermediate_stack: Vec<char> = Vec::new();

    for n in 0..amount {
        let removed_value = stacks[from_stack as usize].pop().unwrap();
        intermediate_stack.push(removed_value);
    }

    for n in 0..amount {
        let removed_value = intermediate_stack.pop().unwrap();
        stacks[to_stack as usize].push(removed_value)
    }
}

fn main() {
    let content = get_file_content();
    let lines: Vec<&str> = content.split('\n').collect();
    
    let (mut stacks, instructions) = parse_stack_order(lines);
    println!("Start stack: \n {:#?}", stacks);
    
    for i in instructions {
        apply_instruction(&mut stacks, i);
    }

    println!("End stack: \n {:#?}", stacks);
}
