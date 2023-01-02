use std::cmp::Ordering;
use std::fs;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Eq, PartialEq, Debug, Clone)]
struct PacketPair {
    left: Data,
    right: Data,
}

impl PacketPair {
    fn new(left: Data, right: Data) -> PacketPair {
        PacketPair {
            left,
            right
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Ord)]
enum Data {
    Number(u32),
    List(Vec<Data>)
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(compare((&self, other)));
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    part1(&input)?;
    part2(&input)?;

    return Ok(());
}

fn part1(input: &str) -> Result<()> {
    let input = input.lines().collect::<Vec<&str>>();

    let mut correct_indices_sum: u32 = 0;
    for (i, c) in input.chunks(3).enumerate() {
        let mut chars_left = c[0].chars().rev().collect::<Vec<char>>();
        chars_left.pop();
        let (left_data, _) = parse(chars_left);

        let mut chars_right = c[1].chars().rev().collect::<Vec<char>>();
        chars_right.pop();
        let (right_data, _) = parse(chars_right);

        let packet_pair = PacketPair::new(left_data, right_data);
        let order = compare((&packet_pair.left, &packet_pair.right));

        if order == Ordering::Less {
            correct_indices_sum += i as u32 + 1;
        }
    }

    println!("Correct indices sum: {}", correct_indices_sum);
    return Ok(());
}

fn part2(input: &str) -> Result<()>  {
    let mut data: Vec<Data> = Vec::new();
    for l in input.lines() {
        if l.starts_with("[") {
            let mut chars = l.chars().rev().collect::<Vec<char>>();
            chars.pop();

            let (line_data, _) = parse(chars);
            data.push(line_data);
        }
    }
    let divider_packet_1 = Data::List(vec![Data::List(vec![Data::Number(2)])]);
    let divider_packet_2 = Data::List(vec![Data::List(vec![Data::Number(6)])]);
    data.push(divider_packet_1.clone());
    data.push(divider_packet_2.clone());

    data.sort();

    for d in &data {
        println!("{:?}", d);
    }

    let index_divider_packet_1 = data.iter().position(|d| d == &divider_packet_1).unwrap() + 1;
    let index_divider_packet_2 = data.iter().position(|d| d == &divider_packet_2).unwrap() + 1;

    println!("Multiplication of two divider packets is: {}", index_divider_packet_1 * index_divider_packet_2);

    return Ok(());
}

fn compare(packet_pair: (&Data, &Data)) -> Ordering {
    match packet_pair {
        (Data::List(ll), Data::List(lr)) => {
            let mut idx = 0;
            loop {
                if ll.len() <= idx || lr.len() <= idx {
                    if ll.len() < lr.len() {
                        return Ordering::Less;
                    } else if ll.len() == lr.len() {
                        return Ordering::Equal;
                    } else {
                        return Ordering::Greater;
                    }
                }

                match (&ll[idx], &lr[idx]) {
                    (Data::Number(l), Data::Number(r)) => {
                        if l < r {
                            return Ordering::Less
                        } else if  l > r {
                            return Ordering::Greater
                        }
                    },
                    (Data::List(_), Data::Number(r)) => {
                        let ordering = compare((&ll[idx], &Data::List(vec![Data::Number(*r)])));
                        if ordering != Ordering::Equal {
                            return ordering;
                        }
                    }
                    (Data::Number(l), Data::List(_)) => {
                        let ordering = compare((&Data::List(vec![Data::Number(*l)]), &lr[idx]));
                        if ordering != Ordering::Equal {
                            return ordering;
                        }
                    }
                    (Data::List(_), Data::List(_)) => {
                        let ordering = compare((&ll[idx], &lr[idx]));
                        if ordering != Ordering::Equal {
                            return ordering;
                        }
                    }
                };
                idx += 1;
            }
        },
        _ => panic!("Not expected input")
    };
}

fn parse(mut chars: Vec<char>) -> (Data, Vec<char>) {
    let mut data_packet: Vec<Data> = Vec::new();
    let mut number: i32 = -1;

    while let Some(c) = chars.pop() {
        let digit_value = c as u32;

        match digit_value {
            // 1..=9
            48..=57 => {
                let parsed_number = c.to_string().parse::<i32>().unwrap();
                if number == -1 {
                    number = parsed_number as i32;
                } else {
                    number = (number * 10) + parsed_number;
                }
            },
            // [
            91 => {
                let (data, charss) = parse(chars);
                data_packet.push(data);
                chars = charss;
            },
            // ,
            44 => if number > -1 {
                data_packet.push(Data::Number(number as u32));
                number = -1;
            },
            // ]
            93 => {
                if number > -1 {
                    data_packet.push(Data::Number(number as u32));
                }
                break;
            },
            _ => panic!("Not expected input {}", c)
        }
    }

    return (Data::List(data_packet), chars)
}