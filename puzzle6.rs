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
    let mut lines = input_raw.lines();

    let mut items: Vec<Vec<u64>> = Vec::new();
    let mut operators: Vec<char> = Vec::new();

    loop {
        let l = lines.next();
        if l.is_none() {
            break;
        }
        let n = l.unwrap().trim();
        if n.is_empty() {
            break;
        }
        let xs: Vec<&str> = n.split_whitespace().collect();
        let fc: char = n.chars().nth(0).unwrap();
        if fc == '*' || fc == '+' {
            for i in xs {
                let c: char = i.chars().nth(0).unwrap();
                operators.push(c);
            }
        } else {
            let mut row: Vec<u64> = Vec::with_capacity(xs.len());
            for i in xs {
                let v: u64 = i.parse().unwrap();
                row.push(v);
            }
            items.push(row);
        }
    }

    let mut totals: Vec<u64> = Vec::with_capacity(items[0].len());
    for i in &operators {
        if *i == '+' {
            totals.push(0);
        } else {
            totals.push(1);
        }
    }
    for r in &items {
        for (j, c)  in r.iter().enumerate() {
            if operators[j] == '+' {
                totals[j] += c;
            } else {
                totals[j] *= c;
            }
        }
    }

    let total: u64 = totals.into_iter().sum();

    println!("Final total fresh ids: {total}");
}
