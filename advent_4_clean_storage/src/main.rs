use std::fs;
use regex::Regex;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let reader = fs::read_to_string("input.txt")?;

    let regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)")?;
    let mut matches: u32 = 0;

    for line in reader.lines() {
        let cleaning_pair = regex.captures(line).unwrap();
        let first_range: Vec<u32> = (cleaning_pair[1].parse::<u32>().unwrap()..=cleaning_pair[2].parse::<u32>().unwrap()).collect();
        let second_range: Vec<u32> = (cleaning_pair[3].parse::<u32>().unwrap()..=cleaning_pair[4].parse::<u32>().unwrap()).collect();

        let first_length = first_range.len();
        let second_length = second_range.len();

        if first_length > second_length {
            let match_count: Vec<&u32> = first_range.iter().filter(|num| second_range.contains(num)).collect();
            
            if match_count.len() > 0 {
                matches += 1;
            }
        }
        else {
            let match_count: Vec<&u32> = second_range.iter().filter(|num| first_range.contains(num)).collect();
            if match_count.len() > 0 {
                matches += 1;
            }
        }
    }

    println!("{}", matches);
    return Ok(());
}
