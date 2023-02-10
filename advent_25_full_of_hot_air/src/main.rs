use std::fs;

struct State {
    nums: Vec<String>
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let state = State { nums: input.lines().map(|x| x.to_string()).collect::<Vec<String>>() };

    let sum: i64 = state.nums.iter().map(|x| to_i64(&x)).sum();

    println!("{}", to_snafu(sum));
}

fn to_i64(s: &str) -> i64 {
    let mut result = 0;
    for c in s.chars() {
        match c {
            '0'..='2' => result = (result * 5) + (c as u8 - b'0') as i64,
            '-' => result = result * 5 - 1,
            '=' => result = result * 5 - 2,
            _ => panic!("Bad input")
        }
    }

    result
}

fn to_snafu(mut num: i64) -> String {
    let mut result = String::new();

    loop {
        let n = num % 5;
        match n {
            0..=2 => {
                result.push((n as u8 + b'0') as char);
                num /= 5;
            }
            3 => {
                result.push('=');
                num = (num + 2) / 5;
            },
            4 => {
                result.push('-');
                num = (num + 1) / 5;
            },
            _ => panic!("Bad input")
        }

        if num == 0 {
            break;
        }
    }

    result.chars().rev().collect()
}
