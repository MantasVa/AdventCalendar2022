use std::fs;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Job {
    Number(u64),
    Arithmetic(String, Operation, String)
}

#[derive(Debug, Clone)]
enum Operation {
    Plus,
    Minus,
    Multiply,
    Divide,
}

fn main() {
    let jobs = parse();

    part1(&jobs);
    part2(&jobs);
}

fn part1(jobs: &HashMap<String, Job>) {
    let answer = find_answers("root".to_string(), jobs);
    println!("Part 1 {answer}");
}

fn part2(jobs: &HashMap<String, Job>) {
    let root = "root".to_string();
    let path = find_human("root".to_string(), jobs).unwrap();
    let path = path.iter().rev().collect::<Vec<_>>();

    let (left, right) = match jobs.get(&root).unwrap() {
        Job::Number(_) => panic!("root monkey has no data"),
        Job::Arithmetic(left, _, right) => (left, right),
    };

    let correct_val = if left == path[1] {
        find_answers(right.to_string(), jobs)
    } else {
        find_answers(left.to_string(), jobs)
    };
    
    println!("Part 2 {}", find_adjustment(&path, 1, jobs, correct_val));
}

fn find_adjustment(
    path: &Vec<&String>,
    index: usize,
    tree: &HashMap<String, Job>,
    cv: u64,
) -> u64 {
    match tree.get(path[index]).unwrap() {
        Job::Number(_) => cv,
        Job::Arithmetic(l, op, r) => {
            let left = find_answers(l.to_string(), tree);
            let right = find_answers(r.to_string(), tree);
            let new_cv = if l == path[index + 1] {
                match op {
                    Operation::Plus => cv - right,
                    Operation::Minus => cv + right,
                    Operation::Multiply => cv / right,
                    Operation::Divide => cv * right,
                }
            } else {
                match op {
                    Operation::Plus => cv - left,
                    Operation::Minus => left - cv,
                    Operation::Multiply => cv / left,
                    Operation::Divide => left / cv,
                }
            };
            find_adjustment(path, index + 1, tree, new_cv)
        }
    }
}

fn find_answers(loc: String, jobs: &HashMap<String, Job>) -> u64 {
    let mut answers = HashMap::new();

    let number_jobs = jobs.iter().filter_map(|x| match x {
        (_, Job::Arithmetic(_, _, _)) => None,
        (name, Job::Number(x)) => Some((name, *x))
    });

    for (s, n) in number_jobs {
        answers.insert(s.to_string(), n);
    }

    let arithmetic_jobs = jobs.iter().filter_map(|x| match x {
        (_, Job::Number(_)) => None,
        (name, job) => Some((name.to_string(), job.clone()))
    }).collect::<Vec<(String, Job)>>();

    loop {
        for (name, job) in &arithmetic_jobs {
            match job {
                Job::Arithmetic(n1, op, n2) => {
                    if answers.contains_key(n1) && answers.contains_key(n2) {
                        let numb1 = answers[n1];
                        let numb2 = answers[n2];
                        match op {
                            Operation::Plus => answers.insert(name.to_string(), numb1 + numb2).map(|_| ()),
                            Operation::Minus => answers.insert(name.to_string(), numb1 - numb2).map(|_| ()),
                            Operation::Multiply => answers.insert(name.to_string(), numb1 * numb2).map(|_| ()),
                            Operation::Divide => answers.insert(name.to_string(), numb1 / numb2).map(|_| ()),
                        };
                    }

                    continue;
                },
                _ => panic!("Expect filtered jobs")
            }
        }

        if let Some(val) = answers.get(&loc) {
            return *val;
        }
    }
}

fn find_human(loc: String, jobs: &HashMap<String, Job>) -> Option<Vec<String>> {
    if loc == "humn" {
        return Some(vec![loc])
    }

    if let Some(Job::Arithmetic(l, _, r)) = jobs.get(&loc) {
        if let Some(mut vec) = find_human(l.to_string(), jobs) {
            vec.push(loc);
            return Some(vec)
        }
        if let Some(mut vec) = find_human(r.to_string(), jobs) {
            vec.push(loc);
            return Some(vec)
        }
    }

    None
}


fn parse() -> HashMap<String, Job> {
    let input = fs::read_to_string("input.txt").expect("Input should be present");
    let mut jobs = HashMap::new();

    for i in input.lines() {
        let mut split = i.split(": ");
        let name = split.next().unwrap();

        let job_str = split.next().unwrap();
        if job_str.parse::<u32>().is_ok() {
            let job = Job::Number(job_str.parse::<u64>().unwrap());
            jobs.insert(name.to_string(), job);
        } else {
            let mut job_split = job_str.split(" ");
            
            let name_1 = job_split.next().unwrap();
            
            let op = match job_split.next().unwrap() {
                "+" => Operation::Plus,
                "-" => Operation::Minus,
                "*" => Operation::Multiply,
                "/" => Operation::Divide,
                _ => panic!("Expect arithmetic op")
            };
            let name_2 = job_split.next().unwrap();

            let job = Job::Arithmetic(name_1.to_string(), op, name_2.to_string());
            jobs.insert(name.to_string(), job);
        }
    }

    jobs
}
