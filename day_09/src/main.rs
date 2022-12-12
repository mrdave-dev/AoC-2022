use std::{fs, collections::{HashSet, HashMap}};
use core::fmt;
use std::f64;

fn get_file_content() -> String {
    return fs::read_to_string("./input.txt")
        .expect("File './input.txt' should have been readable");
}

struct Command {
    dir: String,
    amt: u32,
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Command {{ dir: '{}', amt: {} }}", self.dir, self.amt);
    }
}

fn get_next_tail_position(position_head: &(i32, i32), position_tail: &(i32, i32)) -> (i32, i32) {
    // get the slope and distance
    // if it's diagonal move rows
    // https://doc.rust-lang.org/std/primitive.f64.html#method.atan2
    // https://math.stackexchange.com/questions/707673/find-angle-in-degrees-from-one-point-to-another-in-2d-space

    // -pi/4 radians (45 deg clockwise)
    // let x1 = f64::from(position_head.0);
    // let y1 = f64::from(position_head.1);

    // // 3pi/4 radians (135 deg counter-clockwise)
    // let x2 = f64::from(position_tail.0);
    // let y2 = f64::from(position_tail.1);

    // let abs_difference_1 = (y1.atan2(x1) - (-std::f64::consts::FRAC_PI_4)).abs();
    // let abs_difference_2 = (y2.atan2(x2) - (3.0 * std::f64::consts::FRAC_PI_4)).abs();


    let x_diff = position_head.0 - position_tail.0;
    let y_diff = position_head.1 - position_tail.1;
    // let z = f64::from(y).atan2(x);
    println!("From {:?} to {:?}: {:?}", position_head, position_tail, (x_diff, y_diff));
    // diagonal moves right
    if x_diff > 1 && y_diff > 1 {
        println!("Head is diagonal right and up");
        return (position_tail.0 + 1, position_tail.1 + 1);
    } else if x_diff > 1 && y_diff < -1 {
        println!("Head is diagonal right and down");
        return (position_tail.0 + 1, position_tail.1 - 1);
    
    // diagonal moves left
    } else if x_diff < -1 && y_diff > 1 {
        println!("Head is diagonal left and up");
        return (position_tail.0 - 1, position_tail.1 + 1);
    } else if x_diff < -1 && y_diff < -1 {
        println!("Head is diagonal left and down");
        return (position_tail.0 - 1, position_tail.1 - 1);
    
    // x_diff == 1
    } else if x_diff == 1 && y_diff > 1 {
        println!("Head is diagonal right and up, y major");
        return (position_tail.0 + 1, position_tail.1 + 1);
    } else if x_diff == 1 && y_diff < -1 {
        println!("Head is diagonal right and down, y major");
        return (position_tail.0 + 1, position_tail.1 - 1);
    
    // x_diff == -1
    } else if x_diff == -1 && y_diff > 1 {
        println!("Head is diagonal left and up, y major");
        return (position_tail.0 - 1, position_tail.1 + 1);
    } else if x_diff == -1 && y_diff < -1 {
        println!("Head is diagonal left and down, y major");
        return (position_tail.0 - 1, position_tail.1 - 1);

    // y_diff == 1
    } else if x_diff > 1 && y_diff == 1 {
        println!("Head is diagonal right and up, x major");
        return (position_tail.0 + 1, position_tail.1 + 1);
    } else if x_diff < -1 && y_diff == 1 {
        println!("Head is diagonal left and up, x major");
        return (position_tail.0 - 1, position_tail.1 + 1);
    
    // y_diff == -1
    } else if x_diff > 1 && y_diff == -1 {
        println!("Head is diagonal right and down, x major");
        return (position_tail.0 + 1, position_tail.1 - 1);
    } else if x_diff < -1 && y_diff == -1 {
        println!("Head is diagonal left and down, x major");
        return (position_tail.0 - 1, position_tail.1 - 1);
        
    } else if x_diff > 1 {
        println!("Head has moved pos horizontally");
        return (position_tail.0 + 1, position_tail.1);
    } else if x_diff < -1 {
        println!("Head has moved neg horizontally");
        return (position_tail.0 - 1, position_tail.1);
    } else if y_diff > 1 {
        println!("Head has moved pos vertically");
        return (position_tail.0, position_tail.1 + 1);
    } else if y_diff < -1 {
        println!("Head has moved neg vertically");
        return (position_tail.0, position_tail.1 - 1);
    } else {
        println!("Head is close enough; no action");
        return position_tail.clone();
    }
}

fn main() {
    let input = get_file_content();
    let commands: Vec<Command> = input
        .split("\n")
        .map(|x| x.split_whitespace().collect())
        .map(|x: Vec<&str>| Command { dir: x[0].to_string(), amt: x[1].parse::<u32>().unwrap() })
        .collect();
    println!("commands: {:#?}", commands);

    // coords are (x, y)
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut current_position_head = (0, 0);
    let mut current_position_tail = (0, 0);
    visited.insert((0, 0));

    // part 2
    let mut rope_parts: Vec<(i32, i32)> = vec![(0,0); 10];
    let mut visited_part_2: HashSet<(i32, i32)> = HashSet::new();
    visited_part_2.insert((0, 0));

    for cmd in commands {
        println!("CMD: {} {}", cmd.dir, cmd.amt);
        for _ in 0..cmd.amt {
            match cmd.dir.as_str() {
                // Add to x
                "R" => {
                    current_position_head = (current_position_head.0 + 1, current_position_head.1);
                    // rope_parts[0] = current_position_head.clone();
                },
                // Add to y
                "U" => {
                    current_position_head = (current_position_head.0, current_position_head.1 + 1);
                    // rope_parts[0] = current_position_head.clone();
                },
                // Sub from x
                "L" => {
                    current_position_head = (current_position_head.0 - 1, current_position_head.1);
                    // rope_parts[0] = current_position_head.clone();
                },
                // Sub from y
                "D" => {
                    current_position_head = (current_position_head.0, current_position_head.1 - 1);
                    // rope_parts[0] = current_position_head.clone();
                },
                _ => {
                    panic!("Expected known direction");
                }
            }

            // current_position_tail = get_next_tail_position(&current_position_head, &current_position_tail);
            // println!("Loop: tail now at {:?}", current_position_tail);

            if !visited.contains(&current_position_tail) {
                // println!("New location: {:?}", current_position_tail);
                visited.insert(current_position_tail.clone());
            }
            
            // part 2
            rope_parts[0] = current_position_head;
            let mut prev_tail: (i32, i32) = rope_parts[0];
            for (n_tail, tail) in rope_parts.iter_mut().enumerate() {
                // if n_tail == 0 {
                //     *tail = current_position_head.clone();
                // } else {
                    *tail = get_next_tail_position(&prev_tail, &tail);
                    prev_tail = *tail;
                // }
                // println!("Tail {}: {:?}", n_tail, tail);
            }
            println!("Rope parts loop:\n{:?}\n", rope_parts);

            if !visited_part_2.contains(&rope_parts[9]) {
                println!("New tail visit: {:?}", rope_parts[9]);
                visited_part_2.insert(rope_parts[9].clone());
            }
            // println!("\n");
        } // end: for _ in 0..cmd.amt
        // if cmd.amt == 8 && cmd.dir.eq("U") { break }

        // println!("Head now at {:?}", current_position_head);
        // println!("Tail now at {:?}\n", current_position_tail);
        // println!("Tail visited:\n{:?}", visited);
        // println!("Total visited: {}", visited.len());
        
        // Part 2: 
        // We need to keep track of 0..=9. Create a vector with each entry's starting point
        // As the head moves, iterate through each point, comparing it to the last
        // Set each point's new position with the get_next_tail_position fn
        // As you iterate through each vector item, check for newly visited
        println!("Rope parts after moves:\n  {:?}", rope_parts);

        // 2713 too high
        println!("Visited part 2: {}", visited_part_2.len());
    }
}
