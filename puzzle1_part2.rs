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
        let full_loops = value / 100;
        let mut increment = value % 100;
        let starting_value = counter;
        let starting_sign = if counter != 0 { counter / counter.abs() } else { 1 };
        if let Some(i) = first {
            if i == 'L' {
                increment *= -1;
            }
        } else {
            panic!("Invalid input line: {line}");
        }
        counter += increment;
        let norm = counter % 100;
        if counter > 100 && norm != 0 {
          password += 1;
        }
        counter %= 100;
        let final_sign = if counter != 0 { counter / counter.abs() } else { 1 };
        if counter < 0 {
            counter += 100;
        }
        password += full_loops;
        if starting_sign != final_sign && counter != 0 && starting_value != 0 {
            password += 1;
        }
        if counter == 0 {
            password += 1;
        } 
        println!("counter value: {counter}");
        println!("password value: {password}");
    }
    println!("Final password: {password}");
}
