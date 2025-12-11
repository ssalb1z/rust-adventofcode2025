use std::fs;
use std::env;
use std::process;
use std::collections::HashMap;
use std::collections::HashSet;

fn dfs(graph: &HashMap<String, Vec<String>>, seen: &HashSet<String>, k: &str) -> u32 {
    if k == "out" {
        return 1;
    } else if seen.contains(k) {
        return 0;
    } else {
        let nodes = graph.get(k).unwrap();
        let mut total = 0;
        let mut path: HashSet<String> = HashSet::new();
        for s in seen {
            path.insert(s.to_owned());
        }
        path.insert(k.to_owned());
        for n in nodes {
            if !path.contains(n) {
                total += dfs(graph, &path, n);
            }
        }
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

    println!("wtf {graph:?}");
    // dfs to find all paths from you -> out.
    let seen: HashSet<String> = HashSet::new();
    let paths = dfs(&graph, &seen, "you");

    println!("Final paths: {paths}");
}
