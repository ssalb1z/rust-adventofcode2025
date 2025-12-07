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

    let mut manifold: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let mut row: Vec<char> = Vec::with_capacity(line.len());
        for c in line.chars() {
            row.push(c);
        }
        manifold.push(row);
    }

    let row_len = manifold[0].len();
    let mut splits = 0;
    for i in 0..manifold.len() {
        for j in 0..row_len  {
            let c = manifold[i][j];
            if i+1 < manifold.len() {
                if c == 'S' || c == '|' {
                    let nc = &mut manifold[i+1][j];
                    if *nc == '^' {
                        if j-1 > 0 {
                            manifold[i+1][j-1] = '|';
                        }
                        if j+1 < row_len {
                            manifold[i+1][j+1] = '|';
                        }
                        splits += 1;
                    } else {
                        *nc = '|';
                    }
                }
            }
        }
    }


    println!("Final total splits: {splits}");
}
