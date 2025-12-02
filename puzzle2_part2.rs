use std::fs;
use std::env;
use std::process;
use std::cmp;
use std::collections::HashSet;

fn get_upper_segment(x: u64, l: u32) -> u64 {
  let digits = x.ilog10()+1;
  // assume the segment length divides the digits evenly 
  let mult = 10u64.pow(digits - l);
  return x / mult;
}


fn sum_repeated_range(lower_segment: u64, upper_segment: u64, digits: u32, seg_len: u32, lower_bound: u64, upper_bound: u64, seen_values: &mut HashSet<u64>) -> u64 {
    let mut acc = 0;
    println!("srr-debug: lower_segment: {lower_segment}, upper_segment: {upper_segment}, digits: {digits}, seg_len: {seg_len}, lower_bound: {lower_bound}, upper_bound: {upper_bound}");
    for i in lower_segment..=upper_segment {
        let reps = digits/seg_len;
        let mut val = 0;
        for j in 0..reps {
            val += i*10u64.pow(j*seg_len);
        }
        println!("evaluating: {val}");
        if val < lower_bound {
            continue;
        }
        if val > upper_bound {
            break;
        }
        if seen_values.contains(&val) {
            continue;
        }
        println!("Found invalid ID: {val}");
        seen_values.insert(val);
        acc += val;
    }
    return acc;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No input argument provided");
        process::exit(1);
    }
    let input_raw = fs::read_to_string(&args[1]).expect("Expected readable file input");
    let input_ranges: Vec<&str> = input_raw.split(',').collect();

    let mut total: u64 = 0;
    let mut seen_values: HashSet<u64> = HashSet::new();
    for line in input_ranges {
        let product_range = line.trim().split_once('-').unwrap();
        let start = product_range.0;
        let end = product_range.1;
        let start_val: u64 = start.parse().unwrap();
        let end_val: u64 = end.parse().unwrap();
        let mut current_val: u64 = start_val;
        while current_val <= end_val {
            let digits = current_val.ilog10()+1;
            let max_segment_len = digits/2;
            for segment_length in 1..=max_segment_len {
                if digits % segment_length != 0 {
                    continue;
                }
                let upper_segment_lower = get_upper_segment(current_val, segment_length);
                let upper_segment_upper = get_upper_segment(cmp::min(end_val, 10u64.pow(digits)-1), segment_length);
                let sum = sum_repeated_range(
                    upper_segment_lower, upper_segment_upper, digits, segment_length, current_val, end_val, &mut seen_values);
                total += sum;
            }
            current_val = 10u64.pow(digits);
        }
    }
    println!("Final invalid ID sum: {total}");
}
