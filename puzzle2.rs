use std::fs;
use std::env;
use std::process;
use std::cmp;

fn check_repeated(x: u64) -> bool {
  let mag = x.ilog10();
  if mag % 2 == 0 {
      return false;
  }
  let half = (mag+1)/2;
  let mult = 10u64.pow(half);
  let upper = x / mult;
  let lower = x % mult;
  return upper == lower;
}

fn get_upper_half(x: u64) -> u64 {
  let mag = x.ilog10();
  let nx = if mag % 2 == 0 { x/10 } else { x };
  let half = (mag+1)/2;
  let mult = 10u64.pow(half);
  return nx / mult;
}

fn sum_repeated_range(lower_half: u64, upper_half: u64, lower_bound: u64, upper_bound: u64) -> u64 {
    let mut acc = 0;
    let mag = lower_half.ilog10()+1;
    println!("sum_repeated_range: lower half: {lower_half}, upper_half: {upper_half}, mag: {mag}");
    for i in lower_half..=upper_half {
        let rep = i + (i*10u64.pow(mag));
        println!("srr-loop: lower bound: {lower_bound}, upper_bound: {upper_bound}, rep: {rep}");
        if rep < lower_bound {
            continue;
        }
        if rep > upper_bound {
            break;
        }
        println!("Adding repeated range: {rep}");
        acc += rep;
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
    for line in input_ranges {
        let product_range = line.trim().split_once('-').unwrap();
        let start = product_range.0;
        let end = product_range.1;
        let start_val: u64 = start.parse().unwrap();
        let end_val: u64 = end.parse().unwrap();
        let mut current_val: u64 = start_val;
        // step through each order of magnitude, base 10 and detect all the repeated sequences in
        // the valid range for that order of magnitude
        while current_val <= end_val {
            let mag = current_val.ilog10();
            let half = (mag+1)/2;
            if mag % 2 == 0 {
              current_val = 10u64.pow(mag+1);
              continue;
            }
            let upper_half_lower = get_upper_half(current_val);
            let upper_half_upper = get_upper_half(cmp::min(end_val, 10u64.pow(mag+1)-1));
            println!("Debug: mag - {mag}, upper_half_lower: {upper_half_lower}, upper_half_upper: {upper_half_upper}, current_val: {current_val}");
            let sum = sum_repeated_range(upper_half_lower, upper_half_upper, current_val, end_val);
            total += sum;
            current_val = 10u64.pow(mag+2);
        }
    }
    println!("Final invalid ID sum: {total}");
}
