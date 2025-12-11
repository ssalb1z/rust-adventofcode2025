use std::fs;
use std::env;
use std::process;
// Gave up on optimizing my own branch-and-bound and just used a proper ILP library, which I likely
// should have just done from the start...

// Optimized: inline small frequently-called functions
#[inline(always)]
fn all_leq(v1: &[u32], v2: &[u32]) -> bool {
    v1.iter().zip(v2).all(|(a, b)| a <= b)
}

#[inline(always)]
fn add_to_vec_with_mult(mult: u32, v1: &Vec<u32>, out: &mut Vec<u32>) {
    for i in 0..v1.len() {
        out[i] += mult * v1[i];
    }
}

#[inline(always)]
fn sub_from_vec_with_mult(mult: u32, v1: &Vec<u32>, out: &mut Vec<u32>) {
    for i in 0..v1.len() {
        out[i] -= mult * v1[i];
    }
}

fn eq(v1: &Vec<u32>, v2: &Vec<u32>) -> bool {
    if v1.len() != v2.len() {
        return false;
    }
    for i in 0..v1.len() {
        if v1[i] != v2[i] {
            return false;
        }
    }
    return true;
}

fn max_possible_presses(target: &Vec<u32>, partial_jolts: &Vec<u32>, buttons: &Vec<Vec<u32>>, button: usize) -> u32 {
    let mut max_presses_possible = u32::MAX;
    for i in 0..target.len() {
        // button will be zero or one if it increments the toggle, so skip all cases where the
        // button toggle is zero.
        let button_val_for_position = buttons[button][i];
        if button_val_for_position != 0 {
            let max_for_position = target[i] - partial_jolts[i];
            if max_for_position < max_presses_possible {
                max_presses_possible = max_for_position;
            }
        }
    }
    return max_presses_possible;
}

// Optimized: better heuristic ordering - process buttons that affect fewer counters first
fn optimize_button_order(buttons: &[Vec<u32>]) -> Vec<usize> {
    let mut order: Vec<(usize, usize)> = buttons.iter()
        .enumerate()
        .map(|(i, btn)| (i, btn.iter().filter(|&&v| v != 0).count()))
        .collect();

    // Sort by number of affected counters (ascending)
    order.sort_by_key(|(_, count)| *count);
    order.iter().map(|(idx, _)| *idx).collect()
}


fn ilp(buttons: &Vec<Vec<u32>>, target: &Vec<u32>, partial: &mut Vec<u32>, partial_jolts: &mut Vec<u32>, depth: usize, best: &mut usize, button_order: &[usize], lower_bound: usize) {
    if lower_bound >= *best {
        // Don't explore this path further, we've already done more button presses than the current known
        // best path.
        return;
    }

    if depth == partial.len() {
        if eq(&partial_jolts, target) {
            if lower_bound < *best {
                *best = lower_bound;
            }
        }
        return;
    }
    let button_idx = button_order[depth];
    let max_possible_presses_for_current_button = max_possible_presses(target, &partial_jolts, buttons, button_idx);
    let button = &buttons[button_idx];
    for i in (0..=max_possible_presses_for_current_button).rev() {
        partial[button_idx] = i;
        add_to_vec_with_mult(i, button, partial_jolts);
        if all_leq(&partial_jolts, target) {
            ilp(buttons, target, partial, partial_jolts, depth+1, best, button_order, lower_bound + i as usize);
        }
        sub_from_vec_with_mult(i, button, partial_jolts);
    }
    partial[button_idx] = 0;
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
        let words: Vec<&str> = line.split_whitespace().rev().collect();
        let js = words[0].strip_prefix("{").unwrap().strip_suffix("}").unwrap().split(',');
        let mut joltages: Vec<u32> = Vec::new();
        for j in js {
            joltages.push(j.parse::<u32>().unwrap());
        }
        
        let mut buttons: Vec<Vec<u32>> = Vec::new();
        for i in 1..words.len() {
            let w = words[i];
            if w.chars().nth(0).unwrap() != '(' {
                break;
            }
            let trimmed = words[i].strip_prefix("(").unwrap().strip_suffix(")").unwrap();
            let lights_for_button = trimmed.split(',');
            let mut button_jolts: Vec<u32> = vec![0; joltages.len()];
            for l in lights_for_button {
                let jolt = l.parse::<usize>().unwrap();
                button_jolts[jolt] = 1;
            }
            buttons.push(button_jolts);
        }
        let mut partial: Vec<u32> = vec![0; buttons.len()];
        let mut partial_jolts: Vec<u32> = vec![0; joltages.len()];
        let mut best: usize = usize::MAX;
        let button_order = optimize_button_order(&buttons);
        ilp(&buttons, &joltages, &mut partial, &mut partial_jolts, 0, &mut best, &button_order, 0);
        println!("shortest path: {best}, target: {joltages:?}, solution: {partial:?}");
        total += best;
    }

    println!("Final min button presses: {total}");
}
