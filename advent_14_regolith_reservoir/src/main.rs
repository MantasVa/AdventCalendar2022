use std::fs;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum ElementType {
    Rock,
    Sand
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let grid = simulate_sand_fall(false);

    let sand_elements_count = grid.iter().filter(|&e| e.1 == &ElementType::Sand).map(|e| e.1).collect::<Vec<&ElementType>>().len();
    println!("Sand elements at rest count: {}", sand_elements_count);
}

fn part2() {
    let grid = simulate_sand_fall(true);

    let sand_elements_count = grid.iter().filter(|&e| e.1 == &ElementType::Sand).map(|e| e.1).collect::<Vec<&ElementType>>().len();
    println!("Sand elements until cave is filled count: {}", sand_elements_count);
}

fn simulate_sand_fall(has_floor: bool) -> HashMap<(i32, i32), ElementType> {
    let (mut grid, lowest_y_point) = parse();
    let floor_y_coordinate = lowest_y_point + 2;
    const SAND_FALL_COORDINATES: (i32, i32) = (500, 0);

    while !grid.contains_key(&SAND_FALL_COORDINATES) {
        let mut fall_sand_coordinates = SAND_FALL_COORDINATES;

        while let Some(p) = drop(&grid, fall_sand_coordinates) {
            fall_sand_coordinates = p;

            if !has_floor && p.1 > lowest_y_point {
                return grid;
            } else if has_floor && p.1 + 1 == floor_y_coordinate {
                break;
            }
        }

        grid.insert(fall_sand_coordinates, ElementType::Sand);
    }

    return grid;
}

fn drop(grid: &HashMap<(i32, i32), ElementType>, curr_pos: (i32, i32)) -> Option<(i32, i32)> {
    let possible_paths: [(i32, i32); 3] = [(0, 1), (-1, 1), (1, 1)];

    for p in possible_paths {
        let possible_position = (curr_pos.0 + p.0, curr_pos.1 + p.1);
        if !grid.contains_key(&possible_position) {
            return Some(possible_position);
        }
    }

    return None;
}

fn parse() -> (HashMap<(i32, i32), ElementType>, i32) {
    let input = fs::read_to_string("input.txt").expect("Input file should be present at root");

    let mut grid: HashMap<(i32, i32), ElementType> = HashMap::new();
    let mut grid_lowest_y_point: i32 = 0;

    for line in input.lines() {
        let splits = line.split(" -> ").collect::<Vec<&str>>();

        let mut iter = splits.iter().peekable();
        while let Some(path) = iter.next() {
            let next_path = iter.peek();

            if next_path.is_some() {
                let next_path = next_path.unwrap();

                let split = path.split(",").collect::<Vec<&str>>();
                let coordinates_1 = (split[0].parse::<i32>().expect("Should contain number"), split[1].parse::<i32>().expect("Should contain number"));

                let split = next_path.split(",").collect::<Vec<&str>>();
                let coordinates_2 = (split[0].parse::<i32>().expect("Should contain number"), split[1].parse::<i32>().expect("Should contain number"));

                let rock_coordinates = match (coordinates_1, coordinates_2) {
                    ((x_1, y_1), (x_2, y_2)) if x_1 == x_2 && y_1 > y_2 => (y_2..=y_1).collect::<Vec<i32>>().iter().map(|y| (x_1, *y)).collect::<Vec<(i32, i32)>>(),
                    ((x_1, y_1), (x_2, y_2)) if x_1 == x_2 && y_1 < y_2 => (y_1..=y_2).collect::<Vec<i32>>().iter().map(|y| (x_1, *y)).collect::<Vec<(i32, i32)>>(),
                    ((x_1, y_1), (x_2, y_2)) if x_1 > x_2 && y_1 == y_2 => (x_2..=x_1).collect::<Vec<i32>>().iter().map(|x| (*x, y_1)).collect::<Vec<(i32, i32)>>(),
                    ((x_1, y_1), (x_2, y_2)) if x_1 < x_2 && y_1 == y_2 => (x_1..=x_2).collect::<Vec<i32>>().iter().map(|x| (*x, y_1)).collect::<Vec<(i32, i32)>>(),
                    x => panic!("Expecting range, and straight line, found: {:?}", x) 
                };

                for coordinates in rock_coordinates {
                    grid.insert(coordinates, ElementType::Rock);

                    if coordinates.1 > grid_lowest_y_point {
                        grid_lowest_y_point = coordinates.1;
                    }
                }
            }
        }
    }

    return (grid, grid_lowest_y_point);
}