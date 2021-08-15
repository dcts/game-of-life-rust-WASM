use rand;
use std::{convert::TryInto, thread, time};

const ALIVE: char = '🟡'; // ⚪
const DEAD: char = '⚫';

// 0,1 seconds wait time between iterations
const SLEEP_TIME: u64 = 100;

// GRAPHICS "ENGINE"
pub fn sleep() {
    thread::sleep(time::Duration::from_millis(SLEEP_TIME));
}


fn main() {
    println!("Hello, world!");
    let random_bool: bool = rand::random();
    println!("random bool: {}", random_bool);
    let random_bool: bool = rand::random();
    println!("random bool: {}", random_bool);
    let random_bool: bool = rand::random();
    println!("random bool: {}", random_bool);
    let random_bool: bool = rand::random();
    println!("random bool: {}", random_bool);

    let mut grid = init_grid(10, 0.1);
    println!("grid: {:?}", grid);
    display_grid(&grid);


    // println!("grid[0][0]: {} => {}", grid[0][0], next_state(1, 1, &grid));
    // println!("grid[9][5]: {} => {}", grid[9][5], next_state(1, 1, &grid));
    // println!("grid[1][1]: {} => {}", grid[1][1], next_state(1, 1, &grid));
    // println!("grid[1][2]: {} => {}", grid[1][2], next_state(1, 2, &grid));
    // println!("grid[1][3]: {} => {}", grid[1][3], next_state(1, 3, &grid));
    println!("modulo(-1, 10) => {}", modulo(-1, 10));
    // println!("grid[-1][0] => ", grid[-1][0]);
}

fn init_grid(size: i32, prob: f32) -> Vec<Vec<bool>> {
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
    for col in 0..grid.len() {
        for row in 0..grid.len() {
            if grid[row][col] {
                print!("{}", ALIVE);
            } else {
                print!("{}", DEAD);
            }
        }
        println!("");
    }
}

fn next_generation(grid: &mut Vec<Vec<bool>>) {

}

fn next_state(row: i32, col: i32, grid: &Vec<Vec<bool>>) -> usize {
    let grid_size: i32 = grid.len().try_into().unwrap();
    // let current_state = grid[row][col];
    let mut neighbors: Vec<bool> = vec![];
    let row = modulo(row - 1, grid_size);
    let col = modulo(col, grid_size);
    println!("{},{},{}",row, col, grid[row  as usize][col  as usize]);

    let row = modulo(row - 1, grid_size);
    let col = modulo(col + 1, grid_size);
    println!("{},{},{}",row, col, grid[row  as usize][col  as usize]);

    let row = modulo(row, grid_size);
    let col = modulo(col + 1, grid_size);
    println!("{},{},{}",row, col, grid[row  as usize][col as usize]);

    let row = modulo(row, grid_size);
    let col = modulo(col - 1, grid_size);
    println!("{},{},{}",row, col, grid[row as usize][col as usize]);

    let row = modulo(row + 1, grid_size);
    let col = modulo(col - 1, grid_size);
    println!("{},{},{}",row, col, grid[row as usize][col as usize]);

    let row = modulo(row + 1, grid_size);
    let col = modulo(col + 1, grid_size);
    println!("{},{},{}",row, col, grid[row as usize][col as usize]);

    let row = modulo(row + 1, grid_size);
    let col = modulo(col, grid_size);
    println!("{},{},{}",row, col, grid[row as usize][col as usize]);

    let row = modulo(row - 1, grid_size);
    let col = modulo(col - 1, grid_size);
    println!("{},{},{}",row, col, grid[row as usize][col as usize]);

    let alive_count: usize = neighbors.iter().filter(|&n| *n == true).count();
    alive_count
}

fn modulo(a: i32, b: i32) -> i32 {
    (((a % b) + b) % b)
}
