use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;
use std::ops::RangeInclusive;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Sensor(i64, i64);

impl Sensor {
    fn coordinates(&self) -> (i64, i64)  {
        (self.0, self.1)
    }
}

#[derive(Debug,)]
struct Beacon(i64, i64);

impl Beacon {
    fn coordinates(&self) -> (i64, i64)  {
        (self.0, self.1)
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let beacons_map = parse();
    let fill_row_y = 2000000;
    let row_set = get_row_spots(&beacons_map, fill_row_y);

    let same_row_sensors: HashSet<i64> = HashSet::from_iter(beacons_map.iter().filter(|&m| m.1.1 == fill_row_y).map(|i| i.1.0));
    let row_sensors_len = same_row_sensors.len();
    println!("Chars len at index 2000000: {}", row_set.len() - row_sensors_len);
}

fn part2() {
    let beacons_map = parse();
    let max_coordinate: i64 = 20;

    for row in 0..=max_coordinate {
        let mut row_data = vec![0..=max_coordinate];

        for (sensor, beacon) in beacons_map.iter() {
            let radius = get_distance(sensor.coordinates(), beacon.coordinates());
            let top: i64 = 0.max(sensor.1 - radius);
            let bottom: i64 = max_coordinate.min(sensor.1 + radius);

            if top > row || bottom < row {
                continue;
            }

            let dist = (sensor.1 - row).abs();
            let min_x = 0.max(sensor.0 - (radius - dist));
            let max_x = max_coordinate.min(sensor.0 + (radius - dist));

            let mut new_range: Vec<RangeInclusive<i64>> = Vec::new();
            for r in &row_data {
                let start = *r.start();
                if start > max_x {
                    new_range.push(r.clone());
                    continue;
                }

                let end = *r.end();
                if end < min_x {
                    new_range.push(r.clone());
                    continue;
                }
 
                if start < min_x {
                    new_range.push(start..=min_x - 1);
                }

                if end > max_x {
                    new_range.push(max_x + 1..=end);
                }

            }
            row_data = new_range; 
        }

        if !row_data.is_empty() {
            let x = *row_data[0].start();
            println!("{:?}", row_data);
            println!("Frequency is: {}", x * 4000000 + row);
            break;
        }
    }
}

fn get_row_spots(beacons_map: &HashMap<Sensor, Beacon>, y: i64) -> HashSet<i64> {
    let mut row_set: HashSet<i64> = HashSet::new();
    for sb in beacons_map {
        let (sensor, beacon) = sb;
        let radius = get_distance(sensor.coordinates(), beacon.coordinates());
        let distance_to_row_y = (sensor.1 - y).abs();

        if distance_to_row_y > radius {
            continue;
        }

        let remainder = radius - distance_to_row_y;

        let left_x = sensor.0 - remainder;
        let right_x = sensor.0 + remainder;

        for x in left_x..=right_x {
            row_set.insert(x);
        }
    }

    return row_set;
}

fn get_distance(from: (i64, i64), to: (i64, i64)) -> i64 {
    (from.0 - to.0).abs() + (from.1 - to.1).abs()
}

fn parse() -> HashMap<Sensor, Beacon> {
    let input = fs::read_to_string("input.txt").expect("Input file should be present.");

    let mut beacons_map: HashMap<Sensor, Beacon> = HashMap::new();
    for line in input.lines() {
        let splits = line.split(":").collect::<Vec<&str>>();

        let sensor_coordinates = 
            (splits[0].split("x=").collect::<Vec<&str>>()[1].split(",").collect::<Vec<&str>>()[0].parse::<i64>().expect("Should contain x coordinate"), 
             splits[0].split("y=").collect::<Vec<&str>>()[1].parse::<i64>().expect("Should contain y coordinate"));
        let sensor = Sensor(sensor_coordinates.0, sensor_coordinates.1);

        let beacon_coordinates = 
             (splits[1].split("x=").collect::<Vec<&str>>()[1].split(",").collect::<Vec<&str>>()[0].parse::<i64>().expect("Should contain x coordinate"), 
              splits[1].split("y=").collect::<Vec<&str>>()[1].parse::<i64>().expect("Should contain y coordinate"));
        let beacon = Beacon(beacon_coordinates.0, beacon_coordinates.1);

        beacons_map.insert(sensor, beacon);
    }

    return beacons_map;
}
