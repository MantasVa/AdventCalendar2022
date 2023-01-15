use std::fs;

fn main() {
    let numbers = parse();
    part1(numbers.clone());
    part2(numbers);
}

fn part1(numbers_v: Vec<(i64, i64)>) {
    decrypt(numbers_v, 1, 1);
}

fn part2(numbers_v: Vec<(i64, i64)>) {
    decrypt(numbers_v, 811589153, 10);
}

fn decrypt(numbers_v: Vec<(i64, i64)>, key: i64, rounds: u32) {
    let numbers_v: Vec<(i64, i64)> = numbers_v.iter().map(|x| (x.0, x.1 * key)).collect();
    let mut numbers = numbers_v.clone();
    let length = numbers.len() as i64 - 1;

    for _ in 0..rounds {
        for d in &numbers_v {
            let pos = numbers.iter().position(|n| n == d).unwrap() as i64;
            let mut new_pos = (pos + d.1) % length;
    
            if new_pos < 0 {
                new_pos += length;
            }
    
            if new_pos >= length {
                new_pos -= length;
            }
    
            let n = numbers.remove(pos as usize);
            _ = numbers.insert(new_pos as usize, n);          
        }   
    }
    let zero_idx = numbers.iter().position(|x| x.1 == 0).expect("Number should be present");

    let idx_1000th = (zero_idx + 1000) % numbers.len();
    let n1 = numbers[idx_1000th];
    let idx_2000th = (zero_idx + 2000) % numbers.len();
    let n2 = numbers[idx_2000th];
    let idx_3000th = (zero_idx + 3000) % numbers.len();
    let n3 = numbers[idx_3000th];

    println!("Coordinates sum {}", n1.1 + n2.1 + n3.1)
}

fn parse() -> Vec<(i64, i64)> {
    let lines = fs::read_to_string("input.txt").unwrap();
    let numbers = lines.lines()
        .enumerate()
        .map(|n| (n.0 as i64, n.1.parse().unwrap()))
        .collect::<Vec<_>>();

    numbers
}
