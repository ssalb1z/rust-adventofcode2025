use std::fs;
use std::env;
use std::process;
use std::cell::Cell;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No input argument provided - pass filename and # of connections to add");
        process::exit(1);
    }

    let input_raw = fs::read_to_string(&args[1]).expect("Expected readable file input");
    let input = input_raw.lines();
    let mut points: Vec<(i64, i64)> = Vec::new();

    for line in input {
        let ps: Vec<i64> = line.split(',').map(|x| x.trim().parse::<i64>().unwrap()).collect();
        points.push((ps[0], ps[1]));
    }

    let mut max_known_area = 0;
    for i in 0..points.len()-1 {
        for j in i+1..points.len() {
            let c1 = points[i];
            let c2 = points[j];
            let area = ((c1.0 - c2.0).abs() + 1) * ((c1.1 - c2.1).abs() + 1);
            if area > max_known_area {
                max_known_area = area;
            }
        }
    }
    

    println!("Final area: {max_known_area}");
}
