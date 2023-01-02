use std::fs;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let grid = create_tree_grid()?;
    let mut visible_count: i32 = 0;

    let last_row_index = grid.len() - 1;
    let last_col_index = grid[0].len() - 1;
    for (x, row) in grid.iter().enumerate() {
        for (y, value) in row.iter().enumerate() {
                if x == 0 || x == last_row_index || y == 0 || y == last_col_index {
                    visible_count += 1;
                } else {
                    let mut is_visible: bool = false;

                    let mut row_index = 0;
                    while row_index <= x {
                        let value_front = grid[row_index][y];

                        if row_index == x {
                            is_visible = true;
                            println!("is visible value {}, position: {}:{}", value, x, y)
                        } else if value_front >= *value {
                            break;
                        }

                        row_index += 1;
                    }

                    row_index = last_row_index;
                    while row_index >= x {
                        let value_bottom = grid[row_index][y];

                        if row_index == x {
                            is_visible = true;
                            println!("is visible value {}, position: {}:{}", value, x, y)
                        } else if value_bottom >= *value {
                            break;
                        }

                        row_index -= 1;
                    }
                    
                    let mut column_index = 0;
                    while column_index <= y {
                        let value_left = grid[x][column_index];

                        if column_index == y {
                            is_visible = true;
                            println!("is visible value {}, position: {}:{}", value, x, y)
                        } else if value_left >= *value {
                            break;
                        }

                        column_index += 1;
                    }

                    column_index = last_col_index;
                    while column_index >= y {
                        let value_right = grid[x][column_index];

                        if column_index == y {
                            is_visible = true;
                            println!("is visible value {}, position: {}:{}", value, x, y)
                        } else if value_right >= *value {
                            break;
                        }

                        column_index -= 1;
                    }

                    if is_visible {
                        visible_count += 1;
                    }
                }
        }
    }
    
    println!("Visible trees count: {}", visible_count);

    //--- Part Two ---

    let mut highest_scenic_score = 0;

    for (x, row) in grid.iter().enumerate() {
        for (y, value) in row.iter().enumerate() {

            let mut top_scenic_count = 0;
            if x > 0 {
                let mut row_index = x - 1;
                while row_index >= 0 {
                    let value_front = grid[row_index][y];

                    if value_front < *value {
                        top_scenic_count += 1;
                    } else if value_front >= * value {
                        top_scenic_count += 1;
                        break;
                    }
    
                    if row_index == 0 {
                        break;
                    }
                    row_index -= 1;
                }
            }

            let mut bottom_scenic_count = 0;
            if x < last_row_index {
                let mut row_index = x + 1;
                while row_index <= last_row_index {
                    let value_bottom = grid[row_index][y];

                    if value_bottom < *value {
                        bottom_scenic_count += 1;
                    } else if value_bottom >= * value {
                        bottom_scenic_count += 1;
                        break;
                    }
    
                    row_index += 1;
                }
            }

            let mut left_scenic_count = 0;
            if y > 0 {
                let mut col_index = y - 1;
                while col_index >= 0 {
                    let value_left = grid[x][col_index];

                    if value_left < *value {
                        left_scenic_count += 1;
                    } else if value_left >= * value {
                        left_scenic_count += 1;
                        break;
                    }
                    
                    if col_index == 0 {
                        break;
                    }
                    col_index -= 1;
                }
            }

            let mut right_scenic_count = 0;
            if y < last_col_index {
                let mut col_index = y + 1;
                while col_index <= last_col_index {
                    let value_right = grid[x][col_index];

                    if value_right < *value {
                        right_scenic_count += 1;
                    } else if value_right >= * value {
                        right_scenic_count += 1;
                        break;
                    }
    
                    col_index += 1;
                }
            }

            let spot_scenic_count = top_scenic_count * bottom_scenic_count * left_scenic_count * right_scenic_count;

            if spot_scenic_count > highest_scenic_score {
                highest_scenic_score = spot_scenic_count;
            }
        }
    }

    println!("Highest found scenic score is: {}", highest_scenic_score);

    return Ok(());
}


fn create_tree_grid() -> Result<Vec<Vec<u32>>> {
    let input = fs::read_to_string("input.txt")?;

    let mut grid:Vec<Vec<u32>> = Vec::new();
    for line in input.lines() {

        let mut grid_value: Vec<u32> = Vec::new();
        for char in line.chars() {
            let value = char.to_digit(10).expect("Char is not a number!");
            grid_value.push(value);
        }
        grid.push(grid_value);
    }

    return Ok(grid);
}


/*
--- Day 8: Treetop Tree House ---
The expedition comes across a peculiar patch of tall trees all planted carefully in a grid. The Elves explain that a previous expedition planted these trees as a reforestation effort. Now, they're curious if this would be a good location for a tree house.

First, determine whether there is enough tree cover here to keep a tree house hidden. To do this, you need to count the number of trees that are visible from outside the grid when looking directly along a row or column.

The Elves have already launched a quadcopter to generate a map with the height of each tree (your puzzle input). For example:

30373
25512
65332
33549
35390
Each tree is represented as a single digit whose value is its height, where 0 is the shortest and 9 is the tallest.

A tree is visible if all of the other trees between it and an edge of the grid are shorter than it. Only consider trees in the same row or column; that is, only look up, down, left, or right from any given tree.

All of the trees around the edge of the grid are visible - since they are already on the edge, there are no trees to block the view. In this example, that only leaves the interior nine trees to consider:

The top-left 5 is visible from the left and top. (It isn't visible from the right or bottom since other trees of height 5 are in the way.)
The top-middle 5 is visible from the top and right.
The top-right 1 is not visible from any direction; for it to be visible, there would need to only be trees of height 0 between it and an edge.
The left-middle 5 is visible, but only from the right.
The center 3 is not visible from any direction; for it to be visible, there would need to be only trees of at most height 2 between it and an edge.
The right-middle 3 is visible from the right.
In the bottom row, the middle 5 is visible, but the 3 and 4 are not.
With 16 trees visible on the edge and another 5 visible in the interior, a total of 21 trees are visible in this arrangement.

Consider your map; how many trees are visible from outside the grid?
*/