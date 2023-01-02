use std::fs;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

const DRAWN: char = '#';
const EMPTY: char = '.';
const GRID_SIZE: usize = 240;

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let mut cycle: i32 = 0;
    let mut x: i32 = 1;
    let mut signal_strength_sum: i32 = 0;
    let mut crt_image: Vec<char> = vec![' '; GRID_SIZE];
    for line in input.lines() {
        cycle += 1;

        signal_strength_sum = get_signal_strength (cycle, x, signal_strength_sum);
        crt_image = draw_pixel(cycle, x, crt_image);
        let command: Vec<&str> = line.split(" ").collect();

        match command[0] {
            "addx" => {
                cycle += 1;
                signal_strength_sum = get_signal_strength (cycle, x, signal_strength_sum);
                crt_image = draw_pixel(cycle, x, crt_image);

                let add_v: i32 = command[1].parse::<i32>()?;
                x += add_v;
            }
            "noop" => (),
            _ => println!("Not defined CPU command {}", command[0])
        }
    }

    println!("Total cycles run: {}", cycle);
    println!("X value: {}", x);
    println!("Signal strength sum: {}", signal_strength_sum);
    print_image(&crt_image);

    return Ok(());
}

fn draw_pixel(cycle: i32, x: i32, mut crt_image: Vec<char>) -> Vec<char> {
    let drawing_index = match cycle {
        _ if cycle > 40 && cycle <= 80 => cycle - 40 - 1,
        _ if cycle > 80 && cycle <= 120 => cycle - 80 - 1,
        _ if cycle > 120 && cycle <= 160 => cycle - 120 - 1,
        _ if cycle > 160 && cycle <= 200 => cycle - 160 - 1,
        _ if cycle > 200 => cycle - 200 - 1,
        _ => cycle - 1
    } ;

    let index: usize  = cycle as usize - 1;
    if x - 1 == drawing_index || x == drawing_index || x + 1 == drawing_index {
        crt_image[index] = DRAWN;
    }
    else {
        crt_image[index] = EMPTY;
    }

    return crt_image;
}

fn get_signal_strength (cycle: i32, x: i32, signal_strength_sum: i32) -> i32 {
    let signal_strength = match cycle {
        20 => cycle * x,
        60 => cycle * x,
        100 => cycle * x,
        140 => cycle * x,
        180 => cycle * x,
        220 => cycle * x,
        _ => 0
    };

    return signal_strength_sum + signal_strength;
}

fn print_image (crt_image: &Vec<char>) {
    let chunks = crt_image.chunks(40);

    for chunk in chunks {
        let s: String = chunk.into_iter().collect();
        println!("{}", s);
    }
}