use std::fs;
use std::collections::{HashSet, HashMap, VecDeque};

struct State {
    surface_area: u32
}

impl State {
    const NEIGHBOR_XYZ: [(i32, i32, i32); 6] = 
                         [(1, 0, 0), (-1, 0, 0),
                          (0, 1, 0), (0, -1, 0),
                          (0, 0, 1), (0, 0, -1)];

    fn new() -> State {
        State {
            surface_area: 0
        }
    }

    fn add(&mut self) {
        self.surface_area += 1;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coordinates(i32, i32, i32);

fn main() {
    let cubes = parse();

    part1(&cubes);
    part2(&cubes);
}

fn part1(cubes: &HashSet<Coordinates>) {
    let mut state = State::new();
    for c in cubes {

        if !cubes.contains(&Coordinates(c.0 - 1, c.1, c.2)) {
            state.add();
        }
        if !cubes.contains(&Coordinates(c.0 + 1, c.1, c.2)) {
            state.add();
        }
        if !cubes.contains(&Coordinates(c.0, c.1 - 1, c.2)) {
            state.add();
        }
        if !cubes.contains(&Coordinates(c.0, c.1 + 1, c.2)) {
            state.add();
        }
        if !cubes.contains(&Coordinates(c.0, c.1, c.2 - 1)) {
            state.add();
        }
        if !cubes.contains(&Coordinates(c.0, c.1, c.2 + 1)) {
            state.add();
        }
    }

    println!("Surface area is {}", state.surface_area);
}

fn part2(cubes: &HashSet<Coordinates>) {
    let mut outer_cube = HashMap::new();
    let mut xrange = (i32::MAX, i32::MIN);
    let mut yrange = (i32::MAX, i32::MIN);
    let mut zrange = (i32::MAX, i32::MIN);

    for c in cubes {
        xrange.0 = xrange.0.min(c.0);
        xrange.1 = xrange.1.max(c.0);
        yrange.0 = yrange.0.min(c.1);
        yrange.1 = yrange.1.max(c.1);
        zrange.0 = zrange.0.min(c.2);
        zrange.1 = zrange.1.max(c.2);
        outer_cube.insert(c, 6);
    }

    xrange = (xrange.0 - 1, xrange.1 + 1);
    yrange = (yrange.0 - 1, yrange.1 + 1);
    zrange = (zrange.0 - 1, zrange.1 + 1);

    let mut found = HashMap::new();
    let mut seen = HashSet::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_back(Coordinates(xrange.0, yrange.0, zrange.0));

    let mut count = 0;
    while let Some(pos) = to_visit.pop_front() {
        if !seen.insert(pos.clone()) {
            continue;
        }

        for d in State::NEIGHBOR_XYZ {
            let next = Coordinates(pos.0 + d.0, pos.1 + d.1, pos.2 + d.2);

            if next.0 < xrange.0 ||
               next.0 > xrange.1 ||
               next.1 < yrange.0 ||
               next.1 > yrange.1 ||
               next.2 < zrange.0 ||
               next.2 > zrange.1 
            {
                continue;
            }

            if let Some(surface) = outer_cube.get(&next) {
                found.insert(next, *surface);
                count += 1;
            } else {
                to_visit.push_back(next);
            }
        }
    }

    println!("Outer area is {}", count);
}

fn parse() -> HashSet<Coordinates> {
    let input = fs::read_to_string("input.txt").expect("Input file should be present");

    let mut cubes: HashSet<Coordinates> = HashSet::new();
    for line in input.lines() {
        let sides = line.split(",")
                        .map(|s| s.parse::<i32>()
                        .expect("Should be a number"))
                        .collect::<Vec<i32>>();

        let cube_coordinates = Coordinates(sides[0], sides[1], sides[2]);
        cubes.insert(cube_coordinates);
    }

    cubes
}