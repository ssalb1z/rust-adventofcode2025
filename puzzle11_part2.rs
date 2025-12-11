use std::fs;
use std::env;
use std::process;
use std::collections::HashMap;

fn dfs<'a>(graph: &'a HashMap<String, Vec<String>>, paths: &mut HashMap<&'a str, u32>, k: &'a str, target: &str) -> u32 {
    if k == target {
        return 1;
    } else if let Some(v) = paths.get(k) {
        return *v;
    } else if k == "out" {
        return 0;
    } else {
        let nodes = graph.get(k).unwrap();
        let mut total = 0;
        for n in nodes {
            total += dfs(graph, paths, n, target);
        }
        paths.insert(k, total);
        return total;
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
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in input {
        let words: Vec<&str> = line.split_whitespace().collect();
        let key = words[0].strip_suffix(":").unwrap();
        let mut nodes: Vec<String> = Vec::new();
        for i in 1..words.len() {
            nodes.push(words[i].to_owned());
        }
        graph.insert(key.to_owned(), nodes);
    }

    // dfs to find all paths for different possible node connections of interest
    let checks: Vec<(&str, &str)> = vec![
        ("svr", "dac"),
        ("svr", "fft"),
        ("dac", "fft"),
        ("fft", "dac"),
        ("fft", "out"),
        ("dac", "out"),
    ];

    for i in checks {
        let mut seen_paths: HashMap<&str, u32> = HashMap::new();
        let paths = dfs(&graph, &mut seen_paths, i.0, i.1);
        println!("{}->{} = {}", i.0, i.1, paths);
    }
}
