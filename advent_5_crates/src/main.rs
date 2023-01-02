use std::fs;
use regex::Regex;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

struct CrateStack {
    names: Vec<char>
}

impl CrateStack {
    pub fn display_first(&self) {
        match self.names.last() {
            Some(name) => print!("[{}] ", name),
            None => println!("Empty")
        }
    }

    pub fn pop(&mut self, amount: usize) -> Vec<char> {
        if self.names.len() < amount {
            panic!("Not correct amount");
        }

        let mut popped_crates: Vec<char> = Vec::new();
        for _ in 0..amount {
            popped_crates.push(self.names.pop().unwrap());
        }

        popped_crates.reverse();
        return popped_crates;
    }

    pub fn push(&mut self, crate_names: &mut Vec<char>) {
        self.names.append(crate_names);
    }
}

fn main() -> Result<()> {
    let reader = fs::read_to_string("input.txt")?;

    let mut crate_stacks: Vec<CrateStack> = Vec::new();

    let mut crate_inputs: Vec<&str> = Vec::new();
    let re = Regex::new(r"(\d+)")?;
    let arrangement_re = Regex::new(r"move (\d+) from (\d+) to (\d+)")?;

    for line in reader.lines() {
        if line.contains('[') && line.contains("]") {
            crate_inputs.push(line);
        }

        if line.starts_with(" 1") {
            for cap in re.captures_iter(line) {
                let number = &cap[1];
                let position = line.find(number).unwrap();

                let names: Vec<char> = crate_inputs.iter()
                                 .rev()
                                .map(|inp| inp.chars().nth(position).unwrap())
                                .filter(|inp| inp != &' ')
                                .collect();

                
                let crates = CrateStack {
                    names
                };
                crate_stacks.push(crates);
            }
        }

        if line.starts_with("move") {
            let captures = arrangement_re.captures(line).unwrap();

            let crates_amount = captures[1].parse::<usize>().unwrap();
            let move_from_stack_index = captures[2].parse::<usize>().unwrap() - 1;
            let move_to_stack_index = captures[3].parse::<usize>().unwrap() - 1;

            let from_crate = crate_stacks.get_mut(move_from_stack_index).unwrap();
            let mut popped_crates = from_crate.pop(crates_amount);

            let to_crate = crate_stacks.get_mut(move_to_stack_index).unwrap();
            to_crate.push(&mut popped_crates);
        }
    }

    for stack in crate_stacks {
        stack.display_first();
    }

    return Ok(());
}


/*

--- Day 5: Supply Stacks ---
The expedition can depart as soon as the final supplies have been unloaded from the ships. Supplies are stored in stacks of marked crates, but because the needed supplies are buried under many other crates, the crates need to be rearranged.

The ship has a giant cargo crane capable of moving crates between stacks. To ensure none of the crates get crushed or fall over, the crane operator will rearrange them in a series of carefully-planned steps. After the crates are rearranged, the desired crates will be at the top of each stack.

The Elves don't want to interrupt the crane operator during this delicate procedure, but they forgot to ask her which crate will end up where, and they want to be ready to unload them as soon as possible so they can embark.

They do, however, have a drawing of the starting stacks of crates and the rearrangement procedure (your puzzle input). For example:

    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
In this example, there are three stacks of crates. Stack 1 contains two crates: crate Z is on the bottom, and crate N is on top. Stack 2 contains three crates; from bottom to top, they are crates M, C, and D. Finally, stack 3 contains a single crate, P.

Then, the rearrangement procedure is given. In each step of the procedure, a quantity of crates is moved from one stack to a different stack. In the first step of the above rearrangement procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:

[D]        
[N] [C]    
[Z] [M] [P]
 1   2   3 
In the second step, three crates are moved from stack 1 to stack 3. Crates are moved one at a time, so the first crate to be moved (D) ends up below the second and third crates:

        [Z]
        [N]
    [C] [D]
    [M] [P]
 1   2   3
Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved one at a time, crate C ends up below crate M:

        [Z]
        [N]
[M]     [D]
[C]     [P]
 1   2   3
Finally, one crate is moved from stack 1 to stack 2:

        [Z]
        [N]
        [D]
[C] [M] [P]
 1   2   3
The Elves just need to know which crate will end up on top of each stack; in this example, the top crates are C in stack 1, M in stack 2, and Z in stack 3, so you should combine these together and give the Elves the message CMZ.

After the rearrangement procedure completes, what crate ends up on top of each stack?
*/