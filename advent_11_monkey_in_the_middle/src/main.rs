use std::fs;
use regex::Regex;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    decision_test: Option<DecisionTest>,
    inspection_count: usize
}

#[derive(Debug)]
enum Operation {
    Multiply(i64),
    Add(i64),
    MultiplyItself,
    AddItself,
    None
}

#[derive(Debug)]
struct DecisionTest {
    divisible_by: Option<i64>,
    throw_to_if_true: Option<usize>,
    throw_to_if_false: Option<usize>
}

struct PendingItem {
    to: usize,
    item_worry_level: i64,
}

fn main() -> Result<()> {
    simluate_rounds(true, 20)?;
    simluate_rounds(false, 10000)?;

    return Ok(());
}

fn simluate_rounds(is_part1: bool, round_count: usize) -> Result<()> {
    let mut monkeys = fill_monkeys_vec()?;

    let mut pending_items: Vec<PendingItem> = Vec::new();

    let mod_values: i64 = monkeys.iter().map(|m| m.decision_test.as_ref().unwrap().divisible_by.unwrap()).product();
    for _ in 0..round_count {
        for (monkey_index, monkey) in monkeys.iter_mut().enumerate() {
            let items_to_add = pending_items.iter().filter(|i| i.to == monkey_index);
            for item in items_to_add {
                monkey.items.push(item.item_worry_level);
            }
            for i in (0..pending_items.len()).rev() {
                if pending_items[i].to == monkey_index {
                    pending_items.remove(i);
                }
            }

            if monkey.items.len() == 0 {
                continue;
            }

            for (_, item_worry_level) in monkey.items.iter().enumerate() {
                let mut worry_level = *item_worry_level; 

                worry_level = match monkey.operation {
                    Operation::MultiplyItself => worry_level * worry_level,
                    Operation::AddItself => worry_level + worry_level,
                    Operation::Multiply(v) => v * worry_level,
                    Operation::Add(v) => v + worry_level,
                    _ => worry_level
                };

                if is_part1 {
                    worry_level = worry_level / 3;
                } else {
                    worry_level =  worry_level % mod_values;
                }

                let decision_test = monkey.decision_test.as_ref().expect("Monkey should have decision test");
                let throw_to = match decision_test {
                    DecisionTest {divisible_by: Some(d), 
                                  throw_to_if_true: Some(t), 
                                  throw_to_if_false: Some(f)} => {
                                    if worry_level % *d == 0 {
                                        *t
                                    }
                                    else {
                                        *f
                                    }
                                  }
                    x => panic!("Decision should have all values: divisible by: {:?}, true case: {:?}, false case: {:?}", x.divisible_by, x.throw_to_if_true, x.throw_to_if_false)
                };

                pending_items.push(PendingItem {to: throw_to, item_worry_level: worry_level});
            }

            monkey.inspection_count += monkey.items.len();
            for i in (0..monkey.items.len()).rev() {
                monkey.items.remove(i);
            }
        }
    }

    println!("After 20 rounds:");
    println!("Inspection counts: ");
    for (i, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {} inspection counts: {}", i, monkey.inspection_count);
    }

    monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));
    println!("Most active monkeys scores: {} and {}", monkeys[0].inspection_count, monkeys[1].inspection_count);
    println!("Multiplied value: {}", monkeys[0].inspection_count * monkeys[1].inspection_count);

    return Ok(());
}

fn fill_monkeys_vec() -> Result<Vec<Monkey>> {
    let input = fs::read_to_string("input.txt")?;

    let mut monkeys: Vec<Monkey> = Vec::new();
    let number_re = Regex::new(r"(\d+)")?;
    let operation_re = Regex::new(r"(\*|\+) (\d+|old)")?;
    let true_re = Regex::new(r"true:.*(\d+)")?;
    let false_re = Regex::new(r"false:.*(\d+)")?;

    for line in input.lines() {

        match line {
            _ if line.starts_with("Monkey") => monkeys.push(
                Monkey {
                    items: Vec::new(), 
                    operation: Operation::None, 
                    decision_test: None,
                    inspection_count: 0
            }),
            _ if line.trim().starts_with("Starting items:") => {
                for item in number_re.captures_iter(line) {
                    let item_id = item[1].parse::<i64>()?;
                    
                    let last_index = monkeys.len() - 1;
                    monkeys[last_index].items.push(item_id);
                }
            }
            _ if line.trim().starts_with("Operation:") => {
                let captures = operation_re.captures(line).unwrap();

                let number_value = &captures[2];
                let operation = match &captures[1] {
                    "*" if number_value == "old" => Operation::MultiplyItself,
                    "+" if number_value == "old" => Operation::AddItself,
                    "*" => Operation::Multiply(number_value.parse::<i64>()?),
                    "+" => Operation::Add(number_value.parse::<i64>()?),
                    _ => Operation::None
                };
                let last_index = monkeys.len() - 1;
                monkeys[last_index].operation = operation;
            }
            _ if line.trim().starts_with("Test:") => {
                let captures = number_re.captures(line).unwrap();
                let number = captures[1].parse::<i64>()?;

                let last_index = monkeys.len() - 1;
                monkeys[last_index].decision_test = match &monkeys[last_index].decision_test {
                    Some(t) => Some(DecisionTest { 
                        divisible_by: Some(number), 
                        throw_to_if_true: t.throw_to_if_true, 
                        throw_to_if_false: t.throw_to_if_false 
                    }),
                    None => Some(DecisionTest { 
                        divisible_by: Some(number), 
                        throw_to_if_true: None, 
                        throw_to_if_false: None 
                    }),
                }
            }
            _ if line.trim().starts_with("If true:") => {
                let captures = true_re.captures(line).unwrap();
                let throw_to = captures[1].parse::<usize>()?;

                let last_index = monkeys.len() - 1;
                monkeys[last_index].decision_test = match &monkeys[last_index].decision_test {
                    Some(t) => Some(DecisionTest { 
                        divisible_by: t.divisible_by, 
                        throw_to_if_true: Some(throw_to), 
                        throw_to_if_false: t.throw_to_if_false 
                    }),
                    None => Some(DecisionTest { 
                        divisible_by: None, 
                        throw_to_if_true: Some(throw_to), 
                        throw_to_if_false: None 
                    }),
                }
            }
            _ if line.trim().starts_with("If false:") => {
                let captures = false_re.captures(line).unwrap();
                let throw_to = captures[1].parse::<usize>()?;

                let last_index = monkeys.len() - 1;
                monkeys[last_index].decision_test = match &monkeys[last_index].decision_test {
                    Some(t) => Some(DecisionTest { 
                        divisible_by: t.divisible_by, 
                        throw_to_if_true: t.throw_to_if_true, 
                        throw_to_if_false: Some(throw_to)
                    }),
                    None => Some(DecisionTest { 
                        divisible_by: None, 
                        throw_to_if_true: None, 
                        throw_to_if_false: Some(throw_to) 
                    }),
                }
            }
            _ => ()
        }
    }

    return Ok(monkeys);
}