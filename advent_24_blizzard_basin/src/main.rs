use std::fs;
use std::collections::HashMap;

struct State {
    user: (i32, i32),
    exit: (i32, i32),
    last_row: u32,
    last_col: u32,
    map: HashMap<(i32, i32), Type>
}

impl State {
    fn new(user: (i32, i32), exit: (i32, i32), last_row: u32,
           last_col: u32, map: HashMap<(i32, i32), Type>) -> State {
        State {user, exit, last_row, last_col, map}
    }

    fn update_blizzard_pos(&mut self) {
        for ((x, y), t) in &self.map {
            match *t {
                Type::Blizzard(Direction::North) => (),
                Type::Blizzard(Direction::East) => (),
                Type::Blizzard(Direction::South) => (),
                Type::Blizzard(Direction::West) => (),
                _ => ()
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Type {
    Wall,
    Ground,
    Blizzard(Direction),
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East
}

fn main() {
    let state = parse();
    print_map(&state);
}

fn part1() {
    println!("Unresolved")
}

fn print_map(state: &State) {
    let max_width = state.map.iter().max_by(|a, b| a.0.0.cmp(&b.0.0)).unwrap().0.0;
    let max_height = state.map.iter().max_by(|a, b| a.0.1.cmp(&b.0.1)).unwrap().0.1;

    for y in 0..=max_height {
        for x in 0..=max_width {
            match state.map.get(&(x, y)).unwrap() {
                Type::Ground if (x, y) == state.user => print!("E"),
                Type::Ground if (x, y) == state.exit => print!("X"),
                Type::Ground => print!("."),
                Type::Wall => print!("#"),
                Type::Blizzard(Direction::North) => print!("^"),
                Type::Blizzard(Direction::West) => print!("<"),
                Type::Blizzard(Direction::South) => print!("v"),
                Type::Blizzard(Direction::East) => print!(">"),
            }
        }
        println!();
    }
}

fn parse() -> State {
    let input = fs::read_to_string("input.txt").expect("Input file should be present");
    
    let mut map = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        let tiles = line
            .chars()
            .enumerate()
            .filter_map(|(x, b)| {
                match b {
                    '.' => Some(((x as i32, y as i32), Type::Ground)),
                    '#' => Some(((x as i32, y as i32), Type::Wall)),
                    '^' => Some(((x as i32, y as i32), Type::Blizzard(Direction::North))),
                    '>' => Some(((x as i32, y as i32), Type::Blizzard(Direction::East))),
                    'v' => Some(((x as i32, y as i32), Type::Blizzard(Direction::South))),
                    '<' => Some(((x as i32, y as i32), Type::Blizzard(Direction::West))),
                    _ => panic!("Not expected type"),
                }})
            .collect::<HashMap<(i32, i32), Type>>();

        map.extend(tiles);
    }

    let user_entry = &map.iter().filter(|&((_, y), t)| y == &0 && t == &Type::Ground).next().unwrap();
    let max_height = &map.iter().max_by(|a, b| a.0.1.cmp(&b.0.1)).unwrap().0.1;
    let exit_entry = &map.iter().filter(|&((_, y), t)| y == max_height && t == &Type::Ground).next().unwrap();

    let max_width = &map.iter().max_by(|a, b| a.0.0.cmp(&b.0.0)).unwrap().0.0;
    let max_height = &map.iter().max_by(|a, b| a.0.1.cmp(&b.0.1)).unwrap().0.1;

    State::new(user_entry.0.clone(), exit_entry.0.clone(), *max_height as u32, *max_width as u32, map)
}