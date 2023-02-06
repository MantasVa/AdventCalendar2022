use std::fs;
use std::collections::HashMap;
use regex::Regex;
use Command::*;
use Direction::*;
use Block::*;

#[derive(Debug, PartialEq)]
enum Block {
    Empty,
    Wall
}

#[derive(Debug)]
enum Command {
    Move(u32),
    Turn(bool)
}

struct State {
    direction: Direction,
    pos: (i32, i32),
    map: HashMap<(i32, i32), Block>
}

impl State {
    fn init(map: HashMap<(i32, i32), Block>) -> State {
        let start_pos = State::get_start_pos(&map);

        State {
            direction: Right,
            pos: start_pos,
            map
        }
    }

    fn get_start_pos(map: &HashMap<(i32, i32), Block>) -> (i32, i32) {
        map.iter().filter(|&((_, y), _)| *y == 1)
            .min_by(|&l, &r| l.0.0.cmp(&r.0.0)).unwrap().0.clone()
    }

    fn get_next_pos(&self) -> Option<(i32, i32)> {
        let new_pos = self.lookup_next_pos();

        if let Some(b) = self.map.get(&new_pos) {
            if *b != Wall {
                return Some(new_pos);
            } else {
                return None;
            }
        }

        match self.direction {
            Right => {
                let row_start = 
                    self.map.iter().filter(|((_, y), _)| *y == self.pos.1)
                    .min_by(|l, r| l.0.0.cmp(&r.0.0)).unwrap();

                return match row_start.1 {
                    Wall => None,
                    _ => Some(row_start.0.clone())
                }
            },
            Left => {
                let row_end = 
                    self.map.iter().filter(|((_, y), _)| *y == self.pos.1)
                    .max_by(|l, r| l.0.0.cmp(&r.0.0)).unwrap();

                return match row_end.1 {
                    Wall => None,
                    _ => Some(row_end.0.clone())
                }
            },
            Down => {
                let col_start = 
                    self.map.iter().filter(|((x, _), _)| *x == self.pos.0)
                    .min_by(|l, r| l.0.1.cmp(&r.0.1)).unwrap();

                return match col_start.1 {
                    Wall => None,
                    _ => Some(col_start.0.clone())
                }
            },
            Up => {
                let col_end = 
                    self.map.iter().filter(|((x, _), _)| *x == self.pos.0)
                    .max_by(|l, r| l.0.1.cmp(&r.0.1)).unwrap();

                return match col_end.1 {
                    Wall => None,
                    _ => Some(col_end.0.clone())
                }
            },
        }
    }

    fn move_to(&mut self, new_pos: (i32, i32)) {
        self.pos = new_pos;
    }

    fn lookup_next_pos(&self) -> (i32, i32) {
        match self.direction {
            Right => (self.pos.0 + 1, self.pos.1),
            Left => (self.pos.0 - 1, self.pos.1),
            Down => (self.pos.0, self.pos.1 + 1),
            Up => (self.pos.0, self.pos.1 - 1),
        }
    }

    fn turn(&mut self, to_right: bool) {
        match self.direction {
            Right if to_right => self.direction = Down,
            Right if !to_right => self.direction= Up,
            Down if to_right => self.direction = Left,
            Down if !to_right => self.direction= Right,
            Left if to_right => self.direction = Up,
            Left if !to_right => self.direction= Down,
            Up if to_right => self.direction = Right,
            Up if !to_right => self.direction= Left,
            _ => panic!("Not implemented turn {:?}, {}", self.direction, to_right)
        }
    } 
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Right = 0,
    Down = 1, 
    Left = 2,
    Up = 3
} 

fn main() {
    let (coordinates, directions) = parse();

    part1(coordinates, &directions)
}

fn part1(coordinates: HashMap<(i32, i32), Block>, directions: &Vec<Command>) {
    let mut state = State::init(coordinates);
    for d in directions {
        match d {
            Move(steps) => {
                for _ in 0..*steps {
                    if let Some(coord) = state.get_next_pos() {
                        state.move_to(coord);

                    } else { break; }
                }
            },
            Turn(to_right) => state.turn(*to_right)
        }
    }

    println!("Final row {} column {} facing {}", state.pos.1, state.pos.0, state.direction.clone() as i32);
    println!("Password is {}", 1000 * state.pos.1 + 4 * state.pos.0 + state.direction as i32);
}

fn parse() -> (HashMap<(i32, i32), Block>, Vec<Command>) {
    let input = fs::read_to_string("input.txt").expect("Input file should be present");
    
    let mut coordinates = HashMap::new();
    let mut directions = Vec::new();
    let regex = Regex::new(r"(\d+|R|L)").unwrap();

    for (y, line) in input.lines().enumerate() {
        if line.contains("R") {
            for i in regex.find_iter(line) {
                let direction = &line[i.start()..i.end()];
                
                if let Ok(c) = direction.parse::<u32>() {
                    directions.push(Command::Move(c));
                } else {
                    if direction == "L" {
                        directions.push(Command::Turn(false));
                    } else if direction == "R" {
                        directions.push(Command::Turn(true));
                    }
                }

            }
        } else if !line.is_empty() {
            let row_coordinates = line.chars().enumerate()
            .filter_map(|(x, b)| 
                match b {
                    ' ' => None,
                    '.' => Some(((1 + x as i32, 1 + y as i32), Block::Empty)),
                    '#' => Some(((1 + x as i32, 1 + y as i32), Block::Wall)),
                    _ => panic!("Not expected type")
                })
            .collect::<HashMap<(i32, i32), Block>>();

            coordinates.extend(row_coordinates);
        }
    }

    (coordinates, directions)
}