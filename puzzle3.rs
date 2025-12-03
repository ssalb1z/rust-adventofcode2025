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

    let mut total: u32 = 0;
    for line in lines {
        let bank = line.trim();
        let mut known_max = 0;
        let bank_a: Vec<char> = bank.chars().collect();
        for i in 0..bank_a.len()-1 {
          let n = bank_a[i] as u32 - '0' as u32;
          for j in i+1..bank_a.len() {
              let tot = n*10 + (bank_a[j] as u32 - '0' as u32);
              if tot > known_max {
                  known_max = tot;
              }
          }
        }
        total += known_max;
        println!("Bank maximum: {known_max}");
    }
    println!("Final total output joltage: {total}");
}
