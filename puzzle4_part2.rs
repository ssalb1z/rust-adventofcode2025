use std::fs;
use std::env;
use std::process;
use std::cell::RefCell;

fn debug_grid(grid: &Vec<Vec<RefCell<char>>>) {
    for r in grid {
        for c in r {
            let v = c.borrow();
            print!("{v}");
        }
        println!("");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No input argument provided");
        process::exit(1);
    }
    let input_raw = fs::read_to_string(&args[1]).expect("Expected readable file input");
    let lines = input_raw.lines();

    let grid: Vec<Vec<RefCell<char>>> = lines.map(|x| { let trimmed = x.trim(); trimmed.chars().map(|y| RefCell::new(y)).collect() }).collect();
    let nrows = grid[0].len();
    let ncols = grid.len();

    let mut total: u32 = 0;
    let mut removed = 1;

    while removed > 0 {
        removed = 0;
        for (i, row) in grid.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                {
                    let col_val = col.borrow();
                    if *col_val != '@' {
                        continue;
                    }
                }
                let mut adj = 0;
                for ar in -1isize..=1 {
                    for ac in -1isize..=1 {
                        let nx = ar + i as isize;
                        let ny = ac + j as isize;
                        if nx >= 0 && ny >= 0 && (nx as usize) < nrows && (ny as usize) < ncols {
                            let v = grid[nx as usize][ny as usize].borrow();
                            if *v == '@' {
                                adj += 1;
                            }
                        }
                    }
                }
                if adj < 5 {
                    total += 1;
                    let mut col_ref = col.borrow_mut();
                    *col_ref = '.';
                    removed += 1;
                }
            }
        }
    }
    println!("Final total output rolls: {total}");
}
