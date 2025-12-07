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

    let mut manifold: Vec<Vec<i64>> = Vec::new();
    for line in lines {
        let mut row: Vec<i64> = Vec::with_capacity(line.len());
        for c in line.chars() {
            row.push(if c == 'S' { 1 } else if c == '^' { -1 } else { 0 });
        }
        manifold.push(row);
    }

    let row_len = manifold[0].len();
    let mut timelines = 1;
    for i in 0..manifold.len() {
        for j in 0..row_len  {
            let c = manifold[i][j];
            if i+1 < manifold.len() {
                if c >= 1 {
                    let nc = &mut manifold[i+1][j];
                    if *nc == -1 {
                        timelines += c;
                        if j-1 > 0 {
                            manifold[i+1][j-1] += c;
                        }
                        if j+1 < row_len {
                            manifold[i+1][j+1] += c;
                        }
                    } else {
                        *nc += c;
                    }
                }
            }
        }
    }


    println!("Final total timelines: {timelines}");
}
