use std::fs;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No input argument provided");
        process::exit(1);
    }
    let mut password = 0;
    let mut counter: i32 = 50;

    let input_raw = fs::read_to_string(&args[1]).expect("Expected readable file input");
    let input = input_raw.lines();
    for line in input {
        if line.len() < 1 { break; }
        let value: i32 = (&line[1..]).parse().unwrap();
        let first = line.chars().nth(0);
        if let Some(i) = first {
            if i == 'R' {
                counter += value;
                counter %= 100;
            } else if i == 'L' {
                counter -= value;
                counter %= 100;
                if counter < 0 {
                    counter += 100;
                }
            }
            if counter == 0 {
                password += 1;
            } 
            println!("counter value: {counter}");
        } else {
            panic!("Invalid input line: {line}");
        }
    }
    println!("Final password: {password}");
}
