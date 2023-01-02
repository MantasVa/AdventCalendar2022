use std::fs;
use std::collections::HashMap;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

const START_SYMBOL: char = 'S';
const END_SYMBOL: char = 'E';

struct Map {
    map: Vec<Vec<u8>>,
    rows_count: usize,
    cols_count: usize,
    start: Coordinates,
    end: Coordinates,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinates {
    x: usize,
    y: usize,
}

fn main() -> Result<()> {
    part1()?;
    part2()?;

    return Ok(());
}

fn part1() -> Result<()> {
    let map = get_map()?;

    let mut shortest_paths: HashMap<Coordinates, u32> = HashMap::new();
    shortest_paths.insert(map.start, 0);

    let mut to_visit = get_neighbours(&map, map.start);
    while let Some(loc) = to_visit.pop() {
        let curr_height = map.map[loc.y][loc.x];

        let neighbour_points = get_neighbours(&map, loc);
        let valid_neighbours = neighbour_points
        .iter().filter(|c| map.map[c.y][c.x] + 1 >= curr_height)
        .map(|c| *c)
        .collect::<Vec<Coordinates>>();

        let new_path_dist = valid_neighbours.iter().filter_map(|c| shortest_paths.get(c)).min();
        if new_path_dist.is_none() {
            continue;
        }
        let new_path_dist = new_path_dist.unwrap() + 1;

        let old_path_dist = shortest_paths.entry(loc).or_insert(u32::MAX);
        if *old_path_dist > new_path_dist {
            shortest_paths.insert(loc, new_path_dist);
            to_visit.extend(neighbour_points);
        }
    }

    let path = shortest_paths.get(&map.end).expect("Shortest path is not found");
    println!("Shortest path {:?}", path);

    return Ok(());
}

fn part2() -> Result<()> {
    let map = get_map()?;

    let mut shortest_paths: HashMap<Coordinates, u32> = HashMap::new();
    shortest_paths.insert(map.end, 0);

    let mut to_visit = get_neighbours(&map, map.end);
    while let Some(loc) = to_visit.pop() {
        let curr_height = map.map[loc.y][loc.x];

        let neighbour_points = get_neighbours(&map, loc);
        let valid_neighbours = neighbour_points
        .iter().filter(|c| map.map[c.y][c.x] - 1 <= curr_height)
        .map(|c| *c)
        .collect::<Vec<Coordinates>>();

        let new_path_dist = valid_neighbours.iter().filter_map(|c| shortest_paths.get(c)).min();
        if new_path_dist.is_none() {
            continue;
        }
        let new_path_dist = new_path_dist.unwrap() + 1;

        let old_path_dist = shortest_paths.entry(loc).or_insert(u32::MAX);
        if *old_path_dist > new_path_dist {
            shortest_paths.insert(loc, new_path_dist);
            to_visit.extend(neighbour_points);
        }
    }

    let mut shortest_start_trail: u32 = u32::MAX;
    for (y, height) in map.map.iter().enumerate() {
        for (x, height) in height.iter().enumerate() {
            if *height == 'a' as u8 {
                let path_length = shortest_paths.get(&Coordinates { x, y}).unwrap_or(&u32::MAX);

                if shortest_start_trail > *path_length {
                    shortest_start_trail = *path_length;
                }
            }
        }
    }
    println!("Shortest path from the bottom is: {}", shortest_start_trail);

    return Ok(());
}

fn get_neighbours(map: &Map, target: Coordinates) -> Vec<Coordinates> {
    const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let neighbours = DIR
        .iter()
        .map(|c| (target.x as i32 + c.0, target.y as i32 + c.1))
        .filter(|c| {
            c.0 >= 0 && c.1 >= 0 && c.0 < map.cols_count as i32 && c.1 < map.rows_count as i32
        })
        .map(|t| Coordinates {
            x: t.0 as usize,
            y: t.1 as usize,
        })
        .collect::<Vec<Coordinates>>();

    return neighbours;
}

fn get_map() -> Result<Map> {
    let input = fs::read_to_string("input.txt")?;

    let mut map: Vec<Vec<u8>> = Vec::new();
    let mut start: Option<Coordinates> = None;
    let mut end: Option<Coordinates> = None;

    for (i, line) in input.lines().enumerate() {
        let mut heights = line.chars().map(|c| c as u8).collect::<Vec<u8>>();

        if let Some(x) = heights.iter().position(|x| *x == START_SYMBOL as u8) {
            heights[x] = 'a' as u8;
            start = Some(Coordinates { x: x, y: i });
        }

        if let Some(x) = heights.iter().position(|x| *x == END_SYMBOL as u8) {
            heights[x] = 'z' as u8;
            end = Some(Coordinates { x: x, y: i });
        }

        map.push(heights);
    }

    let cols_count: usize = map.first().expect("Map should be created").len();
    let rows_count = map.len();

    Ok(Map {
        map,
        rows_count,
        cols_count,
        start: start.expect("Start should be found in map"),
        end: end.expect("End should be found in map"),
    })
}
