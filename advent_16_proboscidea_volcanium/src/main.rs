use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;
use std::fmt;
use std::hash::Hash;

#[derive(Debug, Clone)]
struct State {
    curr_loc: Name,
    minutes_left: u32,
    opened_valves: HashSet<Name>,
    helper: bool
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.curr_loc == other.curr_loc &&
        self.minutes_left == other.minutes_left &&
        self.opened_valves == other.opened_valves &&
        self.helper == other.helper
    }
}

impl Eq for State {}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.curr_loc.hash(state);
        self.minutes_left.hash(state);
        self.helper.hash(state);
        let vec = self.opened_valves.iter().collect::<Vec<&Name>>();
        for v in vec {
            v.hash(state);
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Valve {
    name: Name,
    flow_rate: u32,
    neighbors: Vec<String>
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Name([char; 2]);

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [a, b] = self.0;
        write!(f, "{}{}", a as char, b as char)
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Name {
    fn parse(i: &str) -> Option<Name> {
        if i.len() == 2 {
            let mut chars = i.chars();
            Some(Name([chars.next().unwrap(), chars.next().unwrap()]))
        } else {
            None
        }
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let valves = parse();
    let mut valve_shortcuts: HashMap<Name, HashMap<Name, u32>> = HashMap::new();
    for v in valves.keys() {
        valve_shortcuts.insert(v.clone(), shortcuts(v, &valves));
    }

    let state = State {
        curr_loc: Name::parse("AA").unwrap(),
        minutes_left: 30,
        opened_valves: HashSet::new(),
        helper: false
    };

    let max_flow = Search { seen: HashMap::new() }.dfs(&state, &valves, &valve_shortcuts);
    println!("Pressure released during 30 minutes: {}", max_flow);
}

fn part2() {
    let valves = parse();
    let mut valve_shortcuts: HashMap<Name, HashMap<Name, u32>> = HashMap::new();
    for v in valves.keys() {
        valve_shortcuts.insert(v.clone(), shortcuts(v, &valves));
    }

    let state = State {
        curr_loc: Name::parse("AA").unwrap(),
        minutes_left: 26,
        opened_valves: HashSet::new(),
        helper: true
    };

    let max_flow = Search { seen: HashMap::new() }.dfs(&state, &valves, &valve_shortcuts);
    println!("Pressure released during 30 minutes with helper: {}", max_flow);
}

#[derive(Debug)]
struct Search {
    seen: HashMap<State, u32>
}

impl Search {
    fn dfs(
        &mut self,
        state: &State,
        valves: &HashMap<Name, Valve>,
        shortcuts: &HashMap<Name, HashMap<Name, u32>>
    ) -> u32 {
        if let Some(answer) = self.seen.get(state) {
            return *answer;
        }

        let mut max_flow = if state.helper {
            self.dfs(&State {
                curr_loc: Name::parse("AA").unwrap(),
                minutes_left: 26,
                opened_valves: state.opened_valves.clone(),
                helper: false
            },
            valves, 
            shortcuts
        )
        } else {
            0
        };

        if !state.opened_valves.contains(&state.curr_loc) && state.minutes_left > 0 {
            let mut opened_valves = state.opened_valves.clone();
            opened_valves.insert(state.curr_loc.clone());
            let flow = valves.get(&state.curr_loc).unwrap().flow_rate * (state.minutes_left - 1);

            max_flow = max_flow.max(
                self.dfs(&State {
                    curr_loc: state.curr_loc.clone(),
                    minutes_left: state.minutes_left - 1,
                    opened_valves,
                    helper: state.helper
                },
                valves, 
                shortcuts
            ) + flow)
        }

        let map = shortcuts.get(&state.curr_loc).unwrap();
        for (dest, cost) in map  {
            if *cost < state.minutes_left {
                max_flow = max_flow.max(
                    self.dfs(&State {
                        curr_loc: *dest,
                        minutes_left: state.minutes_left - *cost,
                        opened_valves: state.opened_valves.clone(),
                        helper: state.helper
                    },
                    valves,
                    shortcuts
                ));
            }
        }

        self.seen.insert(state.clone(), max_flow);
        max_flow
    }
}

fn shortcuts(start: &Name, valves: &HashMap<Name, Valve>) -> HashMap<Name, u32> {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    let mut paths = HashMap::new();

    seen.insert(start.clone());
    queue.push_back((start, 0));

    while let Some((node, dist)) = queue.pop_front() {
        let v = valves.get(node).unwrap();

        for path in &v.neighbors {
            let name = Name::parse(path).unwrap();
            if !seen.insert(name) {
                continue;
            }
            let valve = valves.get(&name).unwrap();
            if valve.flow_rate > 0 && &valve.name != start {
                paths.insert(valve.name.clone(), dist + 1);
            }

            queue.push_back((&valve.name, dist + 1));
        }
    }

    paths
}

fn parse() -> HashMap<Name, Valve> {
    let input = fs::read_to_string("input.txt").expect("Should contain input file");

    let mut valves: HashMap<Name, Valve> = HashMap::new();
    for line in input.lines() {
        let valve_name = &line[6..=7];
        let flow_rate = line.split("=").collect::<Vec<&str>>()[1].split(";").collect::<Vec<&str>>()[0].parse::<u32>().expect("Input should contain flow rate");

        let mut neighbors: Vec<String> = Vec::new();
        let neighbors_input = line.split("to valve").collect::<Vec<&str>>()[1].split(",").collect::<Vec<&str>>();
        for neighbor in neighbors_input {
            let neighbor = neighbor.replace("s", "");
            neighbors.push(String::from(neighbor.trim()));
        }

        let name = Name::parse(valve_name).unwrap();
        _ = valves.insert(name, Valve {name, flow_rate, neighbors})
    }

    valves
}