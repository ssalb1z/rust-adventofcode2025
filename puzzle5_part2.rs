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
    let mut lines = input_raw.lines();

    let mut fresh_ranges: Vec<(u64, u64)> = Vec::new();

    loop {
        let l = lines.next();
        if l.is_none() {
            break;
        }
        let n = l.unwrap().trim();
        if n.is_empty() {
            break;
        }
        let id_range = n.split_once('-').unwrap();
        let start: u64 = id_range.0.parse().unwrap();
        let end: u64 = id_range.1.parse().unwrap();
        fresh_ranges.push((start, end));
    }

    fresh_ranges.sort();

    let mut upper_bound = 0;
    let mut total = 0;
    for i in &fresh_ranges {
        let new_range_lower = if i.0 < upper_bound { upper_bound } else { i.0 };
        if i.1 < upper_bound {
            continue;
        }
        let new_range_upper = if i.1 < upper_bound { upper_bound } else { i.1 };
        println!("Adding range: {new_range_lower}-{new_range_upper}");
        total += new_range_upper - new_range_lower + 1;
        upper_bound = new_range_upper+1;
    }

    println!("Final total fresh ids: {total}");
}
