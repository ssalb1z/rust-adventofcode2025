use std::fs;
use std::env;
use std::process;
use std::cmp::Ordering;

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

    let mut total: u64 = 0;
    loop {
        let l = lines.next();
        if l.is_none() {
            break;
        }
        let n = l.unwrap().trim();
        let v: u64 = n.parse().unwrap();
        let mut linear_scan_match = false;
        for i in &fresh_ranges {
            if v >= i.0 && v <= i.1 {
                linear_scan_match = true;
                break;
            }
        }
    }

    println!("Final total fresh ids: {total}");
}
