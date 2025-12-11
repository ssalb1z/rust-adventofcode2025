// DEPS: good_lp = { version = "1.7", features = ["coin_cbc"] }
use std::fs;
use std::env;
use std::process;
use good_lp::*;

fn solve_with_ilp(buttons: &[Vec<u32>], target: &[u32]) -> Option<usize> {
    // Create variables for button presses (one per button)
    let mut vars = variables!();
    let button_vars: Vec<Variable> = (0..buttons.len())
        .map(|i| vars.add(variable().integer().min(0)))
        .collect();
    
    // Build the problem: Minimize sum of all button presses
    let mut objective = Expression::from(0);
    for &v in &button_vars {
        objective = objective + v;
    }
    
    let mut problem = vars.minimise(objective).using(default_solver);
    
    // Add constraints: for each counter, sum of button effects = target
    for (counter_idx, &target_val) in target.iter().enumerate() {
        let mut constraint_expr = Expression::from(0);
        
        for (button_idx, button) in buttons.iter().enumerate() {
            if button[counter_idx] == 1 {
                constraint_expr = constraint_expr + button_vars[button_idx];
            }
        }
        
        problem = problem.with(constraint!(constraint_expr == target_val as i32));
    }
    
    // Solve
    match problem.solve() {
        Ok(solution) => {
            let presses: Vec<i32> = button_vars.iter()
                .map(|&v| solution.value(v) as i32)
                .collect();
            
            let total: usize = presses.iter().map(|&x| x as usize).sum();
            
            println!("  Solution: {:?}, total: {}", presses, total);
            
            Some(total)
        }
        Err(e) => {
            println!("  Failed to solve: {:?}", e);
            None
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <input_file>", args[0]);
        process::exit(1);
    }

    let input_raw = fs::read_to_string(&args[1]).expect("Expected readable file input");
    let input = input_raw.lines();

    let mut total: usize = 0;
    
    for (line_num, line) in input.enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        
        let words: Vec<&str> = line.split_whitespace().rev().collect();
        
        // Parse joltages
        let js = words[0].strip_prefix("{").unwrap()
            .strip_suffix("}").unwrap()
            .split(',');
        let joltages: Vec<u32> = js.map(|j| j.parse().unwrap()).collect();

        // Parse buttons
        let mut buttons: Vec<Vec<u32>> = Vec::new();
        for i in 1..words.len() {
            let w = words[i];
            if !w.starts_with('(') {
                break;
            }
            let trimmed = w.strip_prefix("(").unwrap()
                .strip_suffix(")").unwrap();
            
            let mut button_jolts = vec![0u32; joltages.len()];
            for l in trimmed.split(',') {
                let jolt: usize = l.parse().unwrap();
                button_jolts[jolt] = 1;
            }
            buttons.push(button_jolts);
        }
        
        println!("Line {}: Solving...", line_num + 1);
        
        match solve_with_ilp(&buttons, &joltages) {
            Some(presses) => {
                println!("Line {}: {} presses\n", line_num + 1, presses);
                total += presses;
            }
            None => {
                println!("Line {}: No solution found!\n", line_num + 1);
            }
        }
    }

    println!("Final min button presses: {total}");
}
