use rand;
use std::{convert::TryInto, thread::{self}, time};

const ALIVE: char = '🟡';
const DEAD: char = '⚫';

// 0,1 seconds wait time between iterations
const SLEEP_TIME: u64 = 50;

// GRAPHICS "ENGINE"
pub fn sleep() {
    thread::sleep(time::Duration::from_millis(SLEEP_TIME));
}


fn main() {
    let mut grid = init_grid(20);
    loop {
        display_grid(&grid);
        next_generation(&mut grid);
        sleep();
    }
}

fn init_grid(size: i32) -> Vec<Vec<bool>> {
    let mut grid = vec![];
    for _ in 0..size {
        let mut row_vec: Vec<bool> = vec![];
        for _ in 0..size {
            let random_bool: bool = rand::random();
            if random_bool {
                row_vec.push(true);
            } else {
                row_vec.push(false);
            }
        }
        grid.push(row_vec);
    }
    grid
}

fn display_grid(grid: &Vec<Vec<bool>>) {
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    for row in 0..grid.len() {
        for col in 0..grid.len() {
            if grid[col][row] {
                print!("{}", ALIVE);
            } else {
                print!("{}", DEAD);
            }
        }
        println!("");
    }
}

fn next_generation(grid: &mut Vec<Vec<bool>>) {
    let mut new_grid = grid.clone();
    for col in 0..grid.len() {
        for row in 0..grid.len() {
            new_grid[col][row] = next_state(col as i32,row as i32,grid);
        }
    }
    *grid = new_grid;
}

fn next_state(col: i32, row: i32, grid: &Vec<Vec<bool>>) -> bool {
    let grid_size: i32 = grid.len().try_into().unwrap();
    let current_state = grid[col as usize][row as usize];
    // let mut neighbors: Vec<bool> = vec![];
    let mut alive_count: usize = 0;

    let c = modulo(col - 1, grid_size);
    let r = modulo(row - 1, grid_size);
    // println!("{},{},{}", c, r, grid[c as usize][r as usize]);
    if grid[c  as usize][r  as usize] {
        alive_count += 1;
    }

    let c = modulo(col, grid_size);
    let r = modulo(row - 1, grid_size);
    // println!("{},{},{}", c, r, grid[c as usize][r as usize]);
    if grid[c  as usize][r  as usize] {
        alive_count += 1;
    }

    let c = modulo(col + 1, grid_size);
    let r = modulo(row - 1, grid_size);
    // println!("{},{},{}", c, r, grid[c as usize][r as usize]);
    if grid[c  as usize][r  as usize] {
        alive_count += 1;
    }

    let c = modulo(col - 1, grid_size);
    let r = modulo(row, grid_size);
    // println!("{},{},{}", c, r, grid[c as usize][r as usize]);
    if grid[c  as usize][r  as usize] {
        alive_count += 1;
    }

    let c = modulo(col + 1, grid_size);
    let r = modulo(row, grid_size);
    // println!("{},{},{}", c, r, grid[c as usize][r as usize]);
    if grid[c  as usize][r  as usize] {
        alive_count += 1;
    }

    let c = modulo(col - 1, grid_size);
    let r = modulo(row + 1, grid_size);
    // println!("{},{},{}", c, r, grid[c as usize][r as usize]);
    if grid[c  as usize][r  as usize] {
        alive_count += 1;
    }

    let c = modulo(col, grid_size);
    let r = modulo(row + 1, grid_size);
    // println!("{},{},{}", c, r, grid[c as usize][r as usize]);
    if grid[c  as usize][r  as usize] {
        alive_count += 1;
    }

    let c = modulo(col + 1, grid_size);
    let r = modulo(row + 1, grid_size);
    // println!("{},{},{}", c, r, grid[c as usize][r as usize]);
    if grid[c  as usize][r  as usize] {
        alive_count += 1;
    }

    // let alive_count: usize = neighbors.iter().filter(|&n| *n == true).count();
    if current_state == true && (alive_count == 2 || alive_count == 3) {
        true
    } else if current_state == false && alive_count == 3 {
        true
    } else {
        false
    }
}

fn modulo(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}
