use std::collections::HashSet;
use std::fs;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinates {
    x: i32,
    y: i32,
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let mut head = Coordinates { x: 0, y: 0 };
    let mut tail = Coordinates { x: 0, y: 0 };
    let mut visited_nodes: HashSet<Coordinates> = HashSet::new();

    for command in input.lines() {
        let command = command.split(" ").collect::<Vec<&str>>();
        let direction = command[0];
        let steps_count = command[1].parse::<usize>()?;

        (head, tail, visited_nodes) = match direction {
            "U" => move_rope(head, tail, visited_nodes, Direction::Up, steps_count)?,
            "R" => move_rope(head, tail, visited_nodes, Direction::Right, steps_count)?,
            "D" => move_rope(head, tail, visited_nodes, Direction::Down, steps_count)?,
            "L" => move_rope(head, tail, visited_nodes, Direction::Left, steps_count)?,
            _ => panic!("Not expected input"),
        };
    }
    visited_nodes.insert(tail);

    let visited = visited_nodes.len();
    println!("Visited nodes count: {}", visited);

    // --- Part Two ---

    const KNOTS_COUNT: usize = 10;
    let mut knots = vec![Coordinates {x: 0, y: 0}; KNOTS_COUNT];
    let mut visited_nodes: HashSet<Coordinates> = HashSet::new();

    for command in input.lines() {
        let command = command.split(" ").collect::<Vec<&str>>();
        let direction = command[0];
        let steps_count = command[1].parse::<usize>()?;

        (knots, visited_nodes) = match direction {
            "U" => move_rope_multiple(knots, KNOTS_COUNT, visited_nodes, Direction::Up, steps_count)?,
            "R" => move_rope_multiple(knots, KNOTS_COUNT, visited_nodes, Direction::Right, steps_count)?,
            "D" => move_rope_multiple(knots, KNOTS_COUNT, visited_nodes, Direction::Down, steps_count)?,
            "L" => move_rope_multiple(knots, KNOTS_COUNT, visited_nodes, Direction::Left, steps_count)?,
            _ => panic!("Not expected input"),
        };
    }
    visited_nodes.insert(knots[KNOTS_COUNT - 1]);

    let visited = visited_nodes.len();
    println!("Visited nodes count: {}", visited);
    return Ok(());
}

fn move_rope(
    mut head: Coordinates,
    mut tail: Coordinates,
    mut visited: HashSet<Coordinates>,
    direction: Direction,
    steps_count: usize,
) -> Result<(Coordinates, Coordinates, HashSet<Coordinates>)> {
    for _ in 0..steps_count {
        head = match direction {
            Direction::Up => Coordinates { x: head.x, y: head.y + 1, },
            Direction::Right => Coordinates { x: head.x + 1, y: head.y, },
            Direction::Down => Coordinates { x: head.x, y: head.y - 1, },
            Direction::Left => Coordinates { x: head.x - 1, y: head.y },
        };

        if tail.x == head.x && tail.y + 2 == head.y {
            _ = visited.insert(tail);
            tail.y += 1;
        } else if tail.x == head.x && tail.y - 2 == head.y  {
            _ = visited.insert(tail);
            tail.y -= 1;
        } else if tail.x + 2 == head.x && tail.y == head.y {
            _ = visited.insert(tail);
            tail.x += 1;
        } else if tail.x - 2 == head.x && tail.y == head.y {
            _ = visited.insert(tail);
            tail.x -= 1;
        } else if !is_head_tail_touching(&head, &tail) &&
                  !is_head_in_same_line(&head, &tail)
        {
            let new_tail = match direction {
                Direction::Up if head.x == tail.x + 1 && head.y == tail.y + 2 => Coordinates { x: tail.x + 1, y: tail.y + 1 },
                Direction::Up if head.x == tail.x - 1 && head.y == tail.y + 2 => Coordinates { x: tail.x - 1, y: tail.y + 1 },
                Direction::Right if head.y == tail.y + 1 && head.x == tail.x + 2 => Coordinates { x: tail.x + 1, y: tail.y + 1 },
                Direction::Right  if head.y == tail.y - 1 && head.x == tail.x + 2 => Coordinates { x: tail.x + 1, y: tail.y - 1 },
                Direction::Down if head.x == tail.x + 1 && head.y == tail.y - 2 => Coordinates { x: tail.x + 1, y: tail.y - 1 },
                Direction::Down if head.x == tail.x - 1 && head.y == tail.y - 2 => Coordinates { x: tail.x - 1, y: tail.y - 1 },
                Direction::Left if head.y == tail.y + 1 && head.x == tail.x - 2 => Coordinates { x: tail.x - 1, y: tail.y + 1 },
                Direction::Left if head.y == tail.y - 1  && head.x == tail.x - 2 => Coordinates { x: tail.x - 1, y: tail.y - 1 },
                _ => tail
            };

            if new_tail != tail {
                _ = visited.insert(tail);
                tail = new_tail;
            }
        }

    }

    return Ok((head, tail, visited));
}

fn move_rope_multiple(
    mut knots: Vec<Coordinates>,
    knots_count: usize,
    mut visited: HashSet<Coordinates>,
    direction: Direction,
    steps_count: usize,
) -> Result<(Vec<Coordinates>, HashSet<Coordinates>)> {
        for _ in 0..steps_count {
            knots[0] = match direction {
                Direction::Up => Coordinates { x: knots[0].x, y: knots[0].y + 1, },
                Direction::Right => Coordinates { x: knots[0].x + 1, y: knots[0].y, },
                Direction::Down => Coordinates { x: knots[0].x, y: knots[0].y - 1, },
                Direction::Left => Coordinates { x: knots[0].x - 1, y: knots[0].y },
            };

            for i in 0..knots_count - 1 {
                let mut i_tail_visited: HashSet<Coordinates> = HashSet::new();

                if knots[i + 1].x == knots[i].x && knots[i + 1].y + 2 == knots[i].y {
                    _ = i_tail_visited.insert(knots[i + 1]);
                    knots[i + 1].y += 1;
                } else if knots[i + 1].x == knots[i].x && knots[i + 1].y - 2 == knots[i].y  {
                    _ = i_tail_visited.insert(knots[i + 1]);
                    knots[i + 1].y -= 1;
                } else if knots[i + 1].x + 2 == knots[i].x && knots[i + 1].y == knots[i].y {
                    _ = i_tail_visited.insert(knots[i + 1]);
                    knots[i + 1].x += 1;
                } else if knots[i + 1].x - 2 == knots[i].x && knots[i + 1].y == knots[i].y {
                    _ = i_tail_visited.insert(knots[i + 1]);
                    knots[i + 1].x -= 1;
                } else if !is_head_tail_touching(&knots[i], &knots[i + 1]) &&
                        !is_head_in_same_line(&knots[i], &knots[i + 1])
                {
                    let new_tail = match direction {
                        _ if knots[i].x == knots[i + 1].x + 1 && knots[i].y == knots[i + 1].y + 2 => Coordinates { x: knots[i + 1].x + 1, y: knots[i + 1].y + 1 },
                        _ if knots[i].x == knots[i + 1].x - 1 && knots[i].y == knots[i + 1].y + 2 => Coordinates { x: knots[i + 1].x - 1, y: knots[i + 1].y + 1 },
                        _ if knots[i].x == knots[i + 1].x + 2 && knots[i].y == knots[i + 1].y + 1 => Coordinates { x: knots[i + 1].x + 1, y: knots[i + 1].y + 1 },
                        _ if knots[i].x == knots[i + 1].x - 2 && knots[i].y == knots[i + 1].y + 1 => Coordinates { x: knots[i + 1].x - 1, y: knots[i + 1].y + 1 },
                        _ if knots[i].x == knots[i + 1].x + 2 && knots[i].y == knots[i + 1].y + 2 => Coordinates { x: knots[i + 1].x + 1, y: knots[i + 1].y + 1 },
                        _ if knots[i].x == knots[i + 1].x - 2 && knots[i].y == knots[i + 1].y + 2 => Coordinates { x: knots[i + 1].x - 1, y: knots[i + 1].y + 1 },

                        _ if knots[i].y == knots[i + 1].y + 1 && knots[i].x == knots[i + 1].x + 2 => Coordinates { x: knots[i + 1].x + 1, y: knots[i + 1].y + 1 },
                        _  if knots[i].y == knots[i + 1].y - 1 && knots[i].x == knots[i + 1].x + 2 => Coordinates { x: knots[i + 1].x + 1, y: knots[i + 1].y - 1 },
                        _  if knots[i].y == knots[i + 1].y + 2 && knots[i].x == knots[i + 1].x + 1 => Coordinates { x: knots[i + 1].x + 1, y: knots[i + 1].y + 1 },
                        _  if knots[i].y == knots[i + 1].y - 2 && knots[i].x == knots[i + 1].x + 1 => Coordinates { x: knots[i + 1].x + 1, y: knots[i + 1].y - 1 },
                        _  if knots[i].y == knots[i + 1].y + 2 && knots[i].x == knots[i + 1].x + 2 => Coordinates { x: knots[i + 1].x + 1, y: knots[i + 1].y + 1 },
                        _  if knots[i].y == knots[i + 1].y - 2 && knots[i].x == knots[i + 1].x + 2 => Coordinates { x: knots[i + 1].x + 1, y: knots[i + 1].y - 1 },

                        _ if knots[i].x == knots[i + 1].x + 1 && knots[i].y == knots[i + 1].y - 2 => Coordinates { x: knots[i + 1].x + 1, y: knots[i + 1].y - 1 },
                        _ if knots[i].x == knots[i + 1].x - 1 && knots[i].y == knots[i + 1].y - 2 => Coordinates { x: knots[i + 1].x - 1, y: knots[i + 1].y - 1 },
                        _ if knots[i].x == knots[i + 1].x + 2 && knots[i].y == knots[i + 1].y - 1 => Coordinates { x: knots[i + 1].x + 1, y: knots[i + 1].y - 1 },
                        _ if knots[i].x == knots[i + 1].x - 2 && knots[i].y == knots[i + 1].y - 1 => Coordinates { x: knots[i + 1].x - 1, y: knots[i + 1].y - 1 },
                        _ if knots[i].x == knots[i + 1].x + 2 && knots[i].y == knots[i + 1].y - 2 => Coordinates { x: knots[i + 1].x + 1, y: knots[i + 1].y - 1 },
                        _ if knots[i].x == knots[i + 1].x - 2 && knots[i].y == knots[i + 1].y - 2 => Coordinates { x: knots[i + 1].x - 1, y: knots[i + 1].y - 1 },

                        _ if knots[i].y == knots[i + 1].y + 1 && knots[i].x == knots[i + 1].x - 2 => Coordinates { x: knots[i + 1].x - 1, y: knots[i + 1].y + 1 },
                        _ if knots[i].y == knots[i + 1].y - 1 && knots[i].x == knots[i + 1].x - 2 => Coordinates { x: knots[i + 1].x - 1, y: knots[i + 1].y - 1 },
                        _ if knots[i].y == knots[i + 1].y + 2 && knots[i].x == knots[i + 1].x - 1 => Coordinates { x: knots[i + 1].x - 1, y: knots[i + 1].y + 1 },
                        _ if knots[i].y == knots[i + 1].y - 2 && knots[i].x == knots[i + 1].x - 1 => Coordinates { x: knots[i + 1].x - 1, y: knots[i + 1].y - 1 },
                        _ if knots[i].y == knots[i + 1].y + 2 && knots[i].x == knots[i + 1].x - 2 => Coordinates { x: knots[i + 1].x - 1, y: knots[i + 1].y + 1 },
                        _ if knots[i].y == knots[i + 1].y - 2 && knots[i].x == knots[i + 1].x - 2 => Coordinates { x: knots[i + 1].x - 1, y: knots[i + 1].y - 1 },

                        _ => knots[i + 1]
                    };

                    if new_tail != knots[i + 1] {
                        _ = i_tail_visited.insert(knots[i + 1]);
                        knots[i + 1] = new_tail;
                    }
                }
                
                if i == knots_count - 2 {
                    println!("Tail position: {:?}", knots[9]);
                    visited.extend(i_tail_visited);
                }
        }
    }
    return Ok((knots, visited));
}

fn is_head_tail_touching(head: &Coordinates, tail: &Coordinates) -> bool {
    tail.x == head.x && tail.y + 1 == head.y ||
    tail.x == head.x && tail.y - 1 == head.y ||
    tail.x + 1 == head.x && tail.y == head.y ||
    tail.x - 1 == head.x && tail.y == head.y 
}

fn is_head_in_same_line(head: &Coordinates, tail: &Coordinates) -> bool {
    head.x == tail.x || head.y == tail.y
}

enum Direction {
    Up,
    Right,
    Down,
    Left
}
