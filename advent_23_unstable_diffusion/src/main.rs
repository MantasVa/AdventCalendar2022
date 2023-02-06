use std::collections::{HashSet, HashMap};
use std::fs;

#[derive(Debug)]
struct State {
    rows_count: u32,
    cols_count: u32,
    elfs: HashSet<(i32, i32)>,
    directions: Vec<Direction>,
    rounds: u32,
}

impl State {
    fn init(rows_count: u32, cols_count: u32, elfs: HashSet<(i32, i32)>) -> State {
        use Direction::*;

        State {
            rows_count,
            cols_count,
            elfs,
            directions: vec![North, South, West, East],
            rounds: 10
        }
    }

    fn is_alone(&self, elf_coords: &(i32, i32)) -> bool {
        for d in &self.directions {
                let adjecent =  d.get_adjacent_coords_by_direction(elf_coords);
 
            if self.elfs.contains(&adjecent.0) ||
               self.elfs.contains(&adjecent.1) || 
               self.elfs.contains(&adjecent.2) {
                 return false;
            }
         }

         return true;
    }

    fn get_proposed_coordinates(&self, elf_coords: &(i32, i32)) -> Option<(i32, i32)> {
        for d in &self.directions {
           let adjecent =  d.get_adjacent_coords_by_direction(elf_coords);

           if self.is_in_bounds(adjecent.1) && !self.elfs.contains(&adjecent.0) && 
              !self.elfs.contains(&adjecent.1) && !self.elfs.contains(&adjecent.2) {
                return Some(adjecent.1);
           }
        }

        None
    }

    fn is_in_bounds(&self, coords: (i32, i32)) -> bool {
        coords.0 >= 0 && coords.1 >= 0 && coords.0 < self.cols_count as i32 && coords.1 < self.rows_count as i32
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East
}

impl Direction {
    fn get_adjacent_coords_by_direction(&self, coords: &(i32, i32)) -> ((i32, i32), (i32, i32), (i32, i32)) {
        use Direction::*;

        match self {
            North => ((coords.0 - 1, coords.1 - 1), (coords.0, coords.1 - 1), (coords.0 + 1, coords.1 - 1)),
            South => ((coords.0 - 1, coords.1 + 1), (coords.0, coords.1 + 1), (coords.0 + 1, coords.1 + 1)),
            West => ((coords.0 - 1, coords.1 - 1), (coords.0 - 1, coords.1), (coords.0 - 1, coords.1 + 1)),
            East => ((coords.0 + 1, coords.1 - 1), (coords.0 + 1, coords.1), (coords.0 + 1, coords.1 + 1)),
        }
    }
}

fn main() {
    let state = parse();
    part1(state);
}

fn part1(mut state: State) {
    for _ in 0..state.rounds {
        let mut proposals = HashMap::new();

        for coords in &state.elfs {
            if state.is_alone(coords) {
                continue;
            }

            if let Some(proposed) = state.get_proposed_coordinates(coords) {
                if proposals.contains_key(&proposed) {
                    _ = proposals.remove(&proposed);
                } else {
                    proposals.insert(proposed, coords.clone());
                }
            }
        }

        for (prop, original_coords) in proposals {
            _ = state.elfs.remove(&original_coords);
            state.elfs.insert(prop.clone());
        }

        let direction = state.directions.remove(0);
        state.directions.push(direction);
    }

    for y in 0..state.rows_count {
        for x in 0..state.cols_count {
            if state.elfs.contains(&(x as i32, y as i32)) {
                print!("#");
            } else {
                print!(".");
            }
        } 
        println!();
    }
    calculate_empty_spaces(state)
}

fn calculate_empty_spaces(state: State) {
    let mut rectangle_rows = (0, state.rows_count - 1);
    let mut rectangle_cols = (0, state.cols_count - 1);

    let mut idx = 0;
    loop {
        let row_elfs = state.elfs.iter().any(|x| x.1 == idx);

        if !row_elfs {
            rectangle_rows.0 += 1;
            idx += 1;
        } else {
            break;
        }
    }

    idx = state.rows_count as i32 - 1;
    loop {
        let row_elfs = state.elfs.iter().any(|x| x.1 == idx);

        if !row_elfs {
            rectangle_rows.1 -= 1;
            idx -= 1;
        } else {
            break;
        }
    }

    idx = 0;
    loop {
        let row_elfs = state.elfs.iter().any(|x| x.0 == idx);

        if !row_elfs {
            rectangle_cols.0 += 1;
            idx += 1;
        } else {
            break;
        }
    }

    idx = state.cols_count as i32 - 1;
    loop {
        let row_elfs = state.elfs.iter().any(|x| x.0 == idx);

        if !row_elfs {
            rectangle_cols.1 -= 1;
            idx -= 1;
        } else {
            break;
        }
    }

    let area = (rectangle_rows.1 - rectangle_rows.0 + 1) * (rectangle_cols.1 - rectangle_cols.0 + 1);

    println!("Empty spaces in area: {}", area - state.elfs.len() as u32);
}

fn parse() -> State {
    let input = fs::read_to_string("input.txt").expect("Input file should be present");

    let mut elf_coords = HashSet::new();
    let rows_count = input.lines().count();
    let mut cols_count = 0;
    for (y, line) in input.lines().enumerate() {
        let row_coordinates = line
            .chars()
            .enumerate()
            .filter_map(|(x, b)| {
                match b {
                    ' ' | '.' => None,
                    '#' => Some((x as i32, y as i32)),
                    _ => panic!("Not expected type"),
                }})
            .collect::<HashSet<(i32, i32)>>();

        cols_count = line.len();
        elf_coords.extend(row_coordinates);
    }

    State::init(rows_count as u32, cols_count as u32, elf_coords)
}
