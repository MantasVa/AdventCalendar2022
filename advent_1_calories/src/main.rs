use std::fs;
     
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let input = fs::read_to_string("./input.txt")?;
    
    let mut calories: Vec<u32> = Vec::new();
    for line in input.lines() {
        if line.is_empty() || calories.len() == 0 {
            calories.push(0);
        }
        
        if !line.is_empty() {
            if let Ok(calorie_count) = line.parse::<u32>() {
                //Certain to unwrap, because in line 12 item is added if list is empty
                let calories_bag = calories.last_mut().unwrap();
                *calories_bag = *calories_bag + calorie_count;
            } else {
                println!("Bad input, not number {}", line);
                continue;
            }

        }
    }

    calories.sort();
    calories.reverse();
    let calories_top_3 = calories[..3].to_vec();
    let sum: u32 = calories_top_3.iter().sum();
    println!("{}", sum);
    return Ok(());
}
