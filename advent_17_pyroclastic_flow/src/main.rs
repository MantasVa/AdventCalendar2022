use std::{fs, vec};

struct Chamber {
    fallen_rocks: u32,
    highest_point: u32,
    heights: Vec<u32>,
    jets: Vec<char>,
    jet_index: usize
}

impl Chamber {
    const WIDTH: u32 = 7;
    const MAX_ROCK_COUNT: u32 = 2022;

    fn new(jets: Vec<char>) -> Chamber {
        Chamber {
            fallen_rocks: 0,
            highest_point: 0,
            heights: vec![0; 7],
            jets,
            jet_index: 0
        }
    }

    fn get_jet(&mut self) -> char {
        if self.jet_index >= self.jets.len() {
            self.jet_index = 0;
        }

        let index = self.jet_index;
        self.jet_index += 1;
        self.jets[index]
    } 
}

struct Rock {
    rock_type: RockType,
    coordinates: (u32, u32)
}

impl Rock {
    const ROCK_TYPES_COUNT: u32 = 5;

    fn new(index: u32, y_point: u32) -> Rock {
        let mut index = index;

        if index >= Rock::ROCK_TYPES_COUNT {
            index = 0;
        }

        let rock_type = match index {
            x if x == RockType::Horizontal as u32 => RockType::Horizontal,
            x if x == RockType::Cross as u32 => RockType::Cross,
            x if x == RockType::LShape as u32 => RockType::LShape,
            x if x == RockType::Vertical as u32 => RockType::Vertical,
            x if x == RockType::Square as u32 => RockType::Square,
            _ => panic!("Not covered rock type")
        };

        Rock {
            rock_type,
            coordinates: (2, y_point)
        }
    }

    fn fall(&mut self) {
        self.coordinates = (self.coordinates.0, self.coordinates.1 - 1);
    }

    fn move_left(&mut self) {
        self.coordinates = (self.coordinates.0 - 1, self.coordinates.1);
    }

    fn move_right(&mut self) {
        self.coordinates = (self.coordinates.0 + 1, self.coordinates.1);
    }

    fn get_rock_width(&self) -> u32 {
        match self.rock_type {
            RockType::Horizontal => 4,
            RockType::Cross | RockType::LShape => 3,
            RockType::Vertical => 1,
            RockType::Square => 2
        }
    }

    fn get_coordinates(&self) -> Vec<(u32, u32)> {
        let rock_type_coord = match self.rock_type {
            RockType::Horizontal => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            RockType::Cross => vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
            RockType::LShape => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            RockType::Vertical => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            RockType::Square => vec![(0, 0), (0, 1), (1, 0), (1, 1)]
        };

        rock_type_coord.iter()
                       .map(|(x, y)| 
                       (self.coordinates.0 + *x, self.coordinates.1 + *y))
                       .collect::<Vec<(u32, u32)>>()
    }
}

enum RockType {
    Horizontal,
    Cross,
    LShape,
    Vertical,
    Square
}

fn main() {
    part1();
}

fn part1() {
    let input = fs::read_to_string("input.txt").expect("Should contain input file");

    let mut chamber = Chamber::new(input.chars().collect::<Vec<char>>());
    let mut rock_index = 0;
    while chamber.fallen_rocks != Chamber::MAX_ROCK_COUNT {
        if rock_index >= Rock::ROCK_TYPES_COUNT {
            rock_index = 0;
        }

        let mut rock = Rock::new(rock_index, chamber.highest_point + 4);
        let mut is_move = false;
        loop {
            if is_move {
                if rock.coordinates.1 - 1 > chamber.highest_point {
                    rock.fall();
                } else {
                    let rock_coordinates = rock.get_coordinates();
                    let mut can_move_down = true;
                    for cord in &rock_coordinates {
                        if chamber.heights[cord.0 as usize] >= cord.1 - 1 {
                            can_move_down = false;
                            break;
                        }
                    }

                    if can_move_down {
                        rock.fall();
                    }
                    else {
                        let mut new_highest_y = chamber.highest_point;
                        for cord in rock_coordinates {
                            chamber.heights[cord.0 as usize] = cord.1;

                            if cord.1 > new_highest_y {
                                new_highest_y = cord.1;
                            }
                        }
                        chamber.fallen_rocks += 1;
                        chamber.highest_point = new_highest_y;
                        break;
                    }
                }
            } else {
                let jet = chamber.get_jet();

                match jet {
                    '<' => {
                        if rock.coordinates.0 > 0 {
                            let mut rock_coordinates = rock.get_coordinates();
                            rock_coordinates = rock_coordinates.iter().map(|c| (c.0 - 1, c.1)).collect::<Vec<(u32, u32)>>();

                            let mut can_move = true;
                            for cord in rock_coordinates {
                                let y = chamber.heights[cord.0 as usize];
                                if (cord.0, y) == cord {
                                    can_move = false;
                                    break;
                                }
                            }

                            if can_move {
                                rock.move_left();
                            }
                        }
                    },
                    '>' => {
                        let rock_width = rock.get_rock_width();
                        if rock.coordinates.0 + rock_width < Chamber::WIDTH {

                            let mut rock_coordinates = rock.get_coordinates();
                            rock_coordinates = rock_coordinates.iter().map(|c| (c.0 + 1, c.1)).collect::<Vec<(u32, u32)>>();

                            let mut can_move = true;
                            for cord in rock_coordinates {
                                let y = chamber.heights[cord.0 as usize];

                                if (cord.0, y) == cord {
                                    can_move = false;
                                    break;
                                }
                            }

                            if can_move {
                                rock.move_right();
                            }
                        }
                    },
                    other => panic!("Jet movement is invalid {}", other)
                }
            }
            is_move = !is_move;
        }
        
        rock_index += 1;
    }

    println!("Highest reached Y is {} after rocks {}", chamber.highest_point, chamber.fallen_rocks);
}
