use std::fs;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Blueprint {
    name: String,
    identifier: u32,
    ore_robot_cost: u32,
    clay_robot_cost: u32,
    obisidan_robot_cost: (u32, u32),
    geode_robot_cost: (u32, u32),
}

impl Blueprint {
    fn build(name: String, identifier: u32,) -> Blueprint {
        Blueprint { 
            name,
            identifier,
            ore_robot_cost: 0, 
            clay_robot_cost: 0, 
            obisidan_robot_cost: (0, 0), 
            geode_robot_cost: (0, 0) 
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    inventory: Inventory,
    minutes: u32
}

impl State {
    fn init(minutes: u32) -> State {
        State {
            inventory: Inventory::init(),
            minutes: minutes
        }
    }
}

#[derive(Debug, Clone)]
struct Inventory {
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,

    ore_count: u32,
    clay_count: u32,
    obsidian_count: u32,
    geode_count: u32
}

impl Inventory {
    fn init() -> Inventory {
        Inventory { 
            ore_robots: 1,
            clay_robots: 0, 
            obsidian_robots: 0, 
            geode_robots: 0, 
            ore_count: 0, 
            clay_count: 0, 
            obsidian_count: 0, 
            geode_count: 0
        }
    }

    fn remove_ore(&mut self, count: u32) {
        if count > self.ore_count {
            panic!("Ore count cannot be negative")
        }

        self.ore_count -= count;
    }

    fn remove_clay(&mut self, count: u32) {
        if count > self.clay_count {
            panic!("Clay count cannot be negative")
        }

        self.clay_count -= count;
    }

    fn remove_obsidian(&mut self, count: u32) {
        if count > self.obsidian_count {
            panic!("Obsidian count cannot be negative")
        }

        self.obsidian_count -= count;
    }

    fn add_ore_robot(&mut self) {
        self.ore_robots += 1;
    }

    fn add_clay_robot(&mut self) {
        self.clay_robots += 1;
    }

    fn add_obsidian_robot(&mut self) {
        self.obsidian_robots += 1;
    }

    fn add_geode_robot(&mut self) {
        self.geode_robots += 1;
    }

    fn update_inv(&mut self) {
        self.ore_count += self.ore_robots;
        self.clay_count += self.clay_robots;            
        self.obsidian_count += self.obsidian_robots;            
        self.geode_count += self.geode_robots;            
    }

    fn was_able_to_buy_robot_before(&self, robot_cost: u32) -> bool {
        let ore_count = self.ore_count as i32;
        let ore_robots_count = self.ore_robots as i32;
        let robot_cost = robot_cost as i32;

        ore_count - ore_robots_count >= robot_cost
    }

    fn was_able_to_buy_obsidian_robot_before(&self, (ore_cost, clay_cost): (u32, u32)) -> bool {
        let ore_count = self.ore_count as i32;
        let ore_robots_count = self.ore_robots as i32;
        let ore_cost = ore_cost as i32;

        let clay_count = self.clay_count as i32;
        let clay_robots_count = self.clay_robots as i32;
        let clay_robot_cost = clay_cost as i32;

        ore_count - ore_robots_count >= ore_cost && 
        clay_count - clay_robots_count >= clay_robot_cost
    }
}

fn main() {
    let blueprints = parse();

    part1(&blueprints);
    part2(&blueprints);
}

fn part1(blueprints: &HashSet<Blueprint>) {
    let mut collected_geodes = HashSet::new();

    for blueprint in blueprints {
        let state = State::init(24);
        let geodes_count = take_action(state, blueprint);
        
        _ = collected_geodes.insert((blueprint.identifier, geodes_count))
    }

    let mut quality_sum: u32 = 0;
    for (id, geodes) in collected_geodes {
        quality_sum += id * geodes;    
    }

    println!("Quality level sum is {quality_sum}");
}

fn part2(blueprints: &HashSet<Blueprint>) {
    let mut collected_geodes = HashSet::new();

    for blueprint in blueprints {
        let state = State::init(32);
        let geodes_count = take_action(state, blueprint);
        
        _ = collected_geodes.insert((blueprint.identifier, geodes_count))
    }

    let mut geodes_multiply: u32 = 1;
    for (_, geodes) in collected_geodes {
        geodes_multiply *= geodes;    
    }

    println!("Geodes multiplied sum is {geodes_multiply}");
}

fn take_action(mut state: State, blueprint: &Blueprint) -> u32 {
    let mut collected_geodes = HashSet::new();
    let mut can_buy_geode_robot = false;

    if state.minutes == 0 {
        return state.inventory.geode_count;
    }
    state.minutes -= 1;

    if state.inventory.ore_count >= blueprint.geode_robot_cost.0 &&
       state.inventory.obsidian_count >= blueprint.geode_robot_cost.1 {
        can_buy_geode_robot = true;
        let mut new_state = state.clone();

        new_state.inventory.remove_ore(blueprint.geode_robot_cost.0);
        new_state.inventory.remove_obsidian(blueprint.geode_robot_cost.1);
        new_state.inventory.update_inv();
        new_state.inventory.add_geode_robot();

        collected_geodes.insert(take_action(new_state, blueprint));
    }
    if state.inventory.ore_count >= blueprint.ore_robot_cost && 
       !state.inventory.was_able_to_buy_robot_before(blueprint.ore_robot_cost) &&
       !can_buy_geode_robot &&
       state.inventory.ore_count < state.minutes {
        let mut new_state = state.clone();

        new_state.inventory.remove_ore(blueprint.ore_robot_cost);
        new_state.inventory.update_inv();
        new_state.inventory.add_ore_robot();

        collected_geodes.insert(take_action(new_state, blueprint));
    }
    if state.inventory.ore_count >= blueprint.clay_robot_cost && 
       !state.inventory.was_able_to_buy_robot_before(blueprint.clay_robot_cost) &&
       !can_buy_geode_robot &&
       state.inventory.ore_count < state.minutes {
        let mut new_state = state.clone();

        new_state.inventory.remove_ore(blueprint.clay_robot_cost);
        new_state.inventory.update_inv();
        new_state.inventory.add_clay_robot();

        collected_geodes.insert(take_action(new_state, blueprint));
    }
    if state.inventory.ore_count >= blueprint.obisidan_robot_cost.0 &&
       state.inventory.clay_count >= blueprint.obisidan_robot_cost.1 && 
       !state.inventory.was_able_to_buy_obsidian_robot_before(blueprint.obisidan_robot_cost) &&
       !can_buy_geode_robot {
        let mut new_state = state.clone();

        new_state.inventory.remove_ore(blueprint.obisidan_robot_cost.0);
        new_state.inventory.remove_clay(blueprint.obisidan_robot_cost.1);
        new_state.inventory.update_inv();
        new_state.inventory.add_obsidian_robot();

        collected_geodes.insert(take_action(new_state, blueprint));
    }

    if !can_buy_geode_robot {
        state.inventory.update_inv();
        _ = collected_geodes.insert(take_action(state, blueprint));
    }

    return *collected_geodes.iter().max().expect("Geode max count should be present");
}

fn parse() -> HashSet<Blueprint> {
    let input = fs::read_to_string("input.txt").expect("Input file should be present.");

    let mut blueprints: Vec<Blueprint> = Vec::new();
    for i in input.lines() {
        if i.contains("Blueprint") {
            let name = i.split(":").next().unwrap();
            let identifier = i.split(" ").skip(1).next().unwrap()
                .split(":").next().unwrap().parse::<u32>().unwrap();

            let blueprint = Blueprint::build(String::from(name), identifier);
            blueprints.push(blueprint);
        }
        if i.contains("ore robot costs") {
            let ore_cost = i.split("ore robot costs ").skip(1).next().unwrap()
                    .split(" ").next().unwrap().parse::<u32>().unwrap();
            let blueprint = blueprints.last_mut().unwrap();
            blueprint.ore_robot_cost = ore_cost;
        }
        if i.contains("clay robot costs") {
            let ore_cost = i.split("clay robot costs ").skip(1).next().unwrap()
            .split(" ").next().unwrap().parse::<u32>().unwrap();
            let blueprint = blueprints.last_mut().unwrap();
            blueprint.clay_robot_cost = ore_cost;
        }
        if i.contains("obsidian robot costs") {
            let ore_cost = i.split("obsidian robot costs ").skip(1).next().unwrap()
                    .split(" ").next().unwrap().parse::<u32>().unwrap();

            let clay_cost = i.split("and ").skip(1).next().unwrap()
                    .split(" ").next().unwrap().parse::<u32>().unwrap();

            let blueprint = blueprints.last_mut().unwrap();
            blueprint.obisidan_robot_cost = (ore_cost, clay_cost);
        }
        if i.contains("geode robot costs") {
            let geode_split = i.split("geode robot costs ");

            let ore_cost = geode_split.clone().skip(1).next().unwrap()
                    .split(" ").next().unwrap().parse::<u32>().unwrap();

            let obsidian_cost = geode_split.clone().skip(1).next().unwrap().split("and ").skip(1).next().unwrap()
                    .split(" ").next().unwrap().parse::<u32>().unwrap();

            let blueprint = blueprints.last_mut().unwrap();
            blueprint.geode_robot_cost = (ore_cost, obsidian_cost);
        }
    }

    HashSet::from_iter(blueprints)
}

