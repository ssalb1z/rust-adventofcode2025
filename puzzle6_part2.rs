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
    let mut lines: Vec<&str> = input_raw.lines().collect();
    let stride_length = lines.len() - 1;
    let t_rows = lines[0].len();
    let mut transposed_lines: Vec<Vec<char>> = Vec::with_capacity(t_rows);
    for i in 0..t_rows {
        let nv: Vec<char> = vec![' '; stride_length];
        transposed_lines.push(nv);
    }
    for i in 0..stride_length {
        let line = lines[i];
        for (j, c) in line.chars().enumerate() {
            transposed_lines[j][i] = c;
        }
    }
    let operators: Vec<char> = lines[stride_length].split_whitespace().map(|x| x.chars().nth(0).unwrap()).collect();
    let mut operator_idx = 0;
    let mut totals: Vec<u64> = operators.iter().map(|x| if *x == '+' { 0 } else { 1 }).collect();
    for i in transposed_lines {
        let transposed_row: String = i.into_iter().collect();
        let trim = transposed_row.trim();
        if trim.is_empty() {
            operator_idx += 1;
            continue;
        }
        let val: u64 = trim.parse().unwrap();
        let op = operators[operator_idx];
        if op == '+' {
            totals[operator_idx] += val;
        } else {
            totals[operator_idx] *= val;
        }
    }
    let total: u64 = totals.into_iter().sum();
    println!("total: {total}");
}
