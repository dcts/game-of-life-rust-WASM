use rand;

const ALIVE: char = '⚫';
const DEAD: char = '⚪';

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
