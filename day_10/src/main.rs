use std::{fs, fmt};

fn get_file_content() -> String {
    return fs::read_to_string("./input.txt")
        .expect("File './input.txt' should have been readable");
}

struct Command {
    dir: String,
    amt: i32,
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Command {{ dir: '{}', amt: {} }}", self.dir, self.amt);
    }
}


fn main() {
    let input = get_file_content();
    let commands: Vec<Command> = input
        .split("\n")
        .map(|x| x.split_whitespace().collect())
        .map(|x: Vec<&str>| Command { 
            dir: x[0].to_string(), 
            amt: if x.len() > 1 { x[1].parse::<i32>().unwrap() } else { 0 },
        })
        .collect();
    // println!("commands: {:#?}", commands);

    let signal_strength_markers: Vec<i32> = vec![20, 60, 100, 140, 180, 220];
    let mut signal_image: Vec<String> = vec![];
    let mut signal_strength_sum: i32 = 0;
    let mut x: i32 = 1;
    let mut n_cycle: i32 = 0;
    let mut cmd_queue: Vec<Command> = vec![];
    let mut cmd_iter = commands.into_iter().peekable();
    while cmd_iter.peek().is_some() || cmd_queue.len() > 0 {
        n_cycle += 1;
        println!("Cycle {}", n_cycle);
        

        if (signal_strength_markers.contains(&n_cycle)) {
            // println!("!!! Cycle {}: {}", n_cycle, n_cycle * x);
            signal_strength_sum += n_cycle * x;
        }

        let n_cycle_row = ((n_cycle - 1) % 40);
        
        
        println!("x: {}, n_cycle_row: {}, n_cycle: {}", x, n_cycle_row, n_cycle);
        if x == n_cycle_row - 1 || x == n_cycle_row || x == n_cycle_row + 1 {
            signal_image.push(String::from("#"));
        } else {
            signal_image.push(String::from(" "));
        }

        if n_cycle % 40 == 0 {
            println!("Line break, cycle {}", n_cycle);
            signal_image.push(String::from("\n"));
        }


        // Process step
        if cmd_queue.len() > 0 {
            let cmd = cmd_queue.pop().unwrap();
            // println!("Process: {:?}", cmd);
            if cmd.dir.eq("addx") {
                x += cmd.amt;
            }
        } else if cmd_iter.peek().is_some() {
            // Load step
            let cmd = cmd_iter.next().unwrap();
            // println!("Load: {:?}", cmd);
            if !cmd.dir.eq("noop") {
                cmd_queue.push(cmd);
            }
        }

        // println!("Finishing cycle {}: {}", n_cycle, x);
    }
    println!("Signal strength sum: {}", signal_strength_sum);
    println!("Signal image:\n{}", signal_image.join(""));
}
