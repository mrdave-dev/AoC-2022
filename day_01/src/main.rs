use core::fmt;
/**
 * 1. Read the file
 * 2. Create a list
 * 3. Iterate over each line
 *   a. If the entry is a value, parse the number, add to entries, add to sum property
 *   b. If entry is empty, make new list item
 */

use std::fs;

struct ElfEntry {
    entries: Vec<u32>,
    sum: u32,
}

impl fmt::Debug for ElfEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        return write!(f, "\n{}: [{}]", self.sum, self.entries.iter().map(|x| x.to_string() + ", ").collect::<String>());
    }
}

fn main() {
    let contents = fs::read_to_string("./input.txt")
        .expect("File './input.txt' should have been readable");

    let mut entries: Vec<&str> = contents.split('\n')
        .collect();

    let mut elf_entries: Vec<ElfEntry> = Vec::new();
    elf_entries.push( ElfEntry { entries: Vec::new(), sum: 0 });

    // println!("{:#?}", entries);
    let mut maxSum = 0;

    for e in &mut entries {
        let l = elf_entries.len()-1;
        if e.eq_ignore_ascii_case("") {
            elf_entries.push(ElfEntry { entries: Vec::new(), sum: 0 });
            if elf_entries[l].sum > maxSum {
                maxSum = elf_entries[l].sum;
            }
        } else {
            let n = e.parse::<u32>().unwrap();
            
            elf_entries[l].sum += n;
            elf_entries[l].entries.push(n);
        }
    }

    println!("{:?}", elf_entries);
    println!("Max sum: {}", maxSum);

    elf_entries.sort_by(|a,b| b.sum.cmp(&a.sum));
    println!("{:#?}\n{:#?}\n{:#?}\n",  elf_entries[0], elf_entries[1], elf_entries[2]);
    println!("Top 3 sum: {}", elf_entries[0].sum + elf_entries[1].sum + elf_entries[2].sum);
}

