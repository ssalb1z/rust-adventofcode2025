use std::fs;
use std::env;
use std::process;

fn get_largest_available(bank: &Vec<char>, start_idx: usize, end_idx: usize) -> (u32, usize) {
    let mut known_max = 0;
    let mut known_max_idx = start_idx;
    for i in start_idx..end_idx {
        let v = bank[i as usize] as u32 - '0' as u32;
        if v > known_max {
            known_max = v;
            known_max_idx = i;
        }
    }
    return (known_max, known_max_idx);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No input argument provided");
        process::exit(1);
    }
    let input_raw = fs::read_to_string(&args[1]).expect("Expected readable file input");
    let lines = input_raw.lines();

    let mut total: u64 = 0;
    for line in lines {
        let bank = line.trim();
        let mut known_max: u64 = 0;
        let bank_a: Vec<char> = bank.chars().collect();
        let mut next_start_idx = 0;
        for i in 0usize..12usize {
            let (max_next_value, max_next_idx) = get_largest_available(&bank_a, next_start_idx, bank_a.len() - (11-i));
            next_start_idx = max_next_idx+1;
            known_max += 10u64.pow(11-i as u32)*(max_next_value as u64);
        }
        total += known_max;
        println!("Bank maximum: {known_max}");
    }
    println!("Final total output joltage: {total}");
}
