use std::fs;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No input argument provided");
        process::exit(1);
    }
    let input_raw = fs::read_to_string(&args[1]).expect("Expected readable file input");
    let lines = input_raw.lines();

    let grid: Vec<Vec<char>> = lines.map(|x| { let trimmed = x.trim(); trimmed.chars().collect() }).collect();
    let nrows = grid[0].len();
    let ncols = grid.len();

    let mut total: u32 = 0;
    let rows: &Vec<Vec<char>> = &grid;

    for (i, row) in rows.into_iter().enumerate() {
        for (j, col) in row.into_iter().enumerate() {
            let mut adj = 0;
            if *col != '@' {
                continue;
            }
            for ar in -1isize..=1 {
                for ac in -1isize..=1 {
                    let nx = ar + i as isize;
                    let ny = ac + j as isize;
                    if nx >= 0 && ny >= 0 && (nx as usize) < nrows && (ny as usize) < ncols {
                        if grid[nx as usize][ny as usize] == '@' {
                            adj += 1;
                            println!("testing: {i}, {j} - adj += {nx}, {ny}");
                        }
                    }
                }
            }
            if adj < 5 {
                total += 1;
                println!("adding: {i}, {j}"); 
            }
        }
    }
    println!("Final total output rolls: {total}");
}
