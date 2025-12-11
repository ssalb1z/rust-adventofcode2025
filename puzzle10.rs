use std::fs;
use std::env;
use std::process;

fn find_path(target: u32, buttons: &Vec<u32>, depth: usize, pressed: usize, value: u32) -> usize {
    if value == target {
        return pressed;
    } else if depth >= buttons.len() {
        return depth;
    } else {
        let path_with_button = find_path(target, buttons, depth+1, pressed+1, value ^ buttons[depth]);
        let path_without_button = find_path(target, buttons, depth+1, pressed, value);
        return path_with_button.min(path_without_button);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No input argument provided - pass filename and # of connections to add");
        process::exit(1);
    }

    let input_raw = fs::read_to_string(&args[1]).expect("Expected readable file input");
    let input = input_raw.lines();

    let mut total: usize = 0;
    for line in input {
        let words: Vec<&str> = line.split_whitespace().collect();
        let indicators = words[0].strip_prefix("[").unwrap().strip_suffix("]").unwrap();
        let mut target: u32 = 0;
        for (i,c) in indicators.chars().enumerate() {
            if c == '#' {
                target |= 1 << i;
            }
        }
        let mut buttons: Vec<u32> = Vec::new();
        for i in 1..words.len() {
            let w = words[i];
            if w.chars().nth(0).unwrap() != '(' {
                break;
            }
            let trimmed = words[i].strip_prefix("(").unwrap().strip_suffix(")").unwrap();
            let lights_for_button = trimmed.split(',');
            let mut button_lights: u32 = 0;
            for l in lights_for_button {
                let light = l.parse::<u32>().unwrap();
                button_lights |= 1 << light;
            }
            buttons.push(button_lights);
        }
        let path = find_path(target, &buttons, 0, 0, 0);
        println!("shortest path: {path}, target: {target}, buttons: {buttons:?}");
        total += path;
    }

    println!("Final min button presses: {total}");
}
