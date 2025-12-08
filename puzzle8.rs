use std::fs;
use std::env;
use std::process;
use std::cell::Cell;
use std::collections::HashSet;


#[derive(Debug)]
struct DisjointSet {
    parents: Vec<Cell<usize>>,
    ranks: Vec<i64>,
    counts: Vec<i64>,
}

fn find_root(d: &mut DisjointSet, c: usize) -> usize {
    let mut parent = d.parents[c].get();
    if c == parent {
        return c;
    }
    loop {
        let gp = d.parents[parent].get();
        if gp == parent {
            return gp;
        }
        // path compression
        d.parents[c].set(gp);
        parent = gp;
    }
}

fn merge_nodes(d: &mut DisjointSet, a: usize, b: usize) -> bool {
    let ar = find_root(d, a);
    let br = find_root(d, b);

    if ar == br {
        return false;
    }

    let b_rank: i64 = d.ranks[br];
    let a_rank: &mut i64 = &mut d.ranks[ar];

    if *a_rank < b_rank {
      d.parents[br].set(ar);
      d.counts[ar] += d.counts[br];
    } else {
        if *a_rank == b_rank  {
            *a_rank += 1;
        }
        d.parents[ar].set(br);
        d.counts[br] += d.counts[ar];
    }

    return true;
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("No input argument provided - pass filename and # of connections to add");
        process::exit(1);
    }

    let input_raw = fs::read_to_string(&args[1]).expect("Expected readable file input");
    let connections: usize = args[2].parse().unwrap();
    let input = input_raw.lines();
    let mut boxes: Vec<(i64, i64, i64)> = Vec::new();

    for line in input {
        let bs: Vec<i64> = line.split(',').map(|x| x.trim().parse::<i64>().unwrap()).collect();
        boxes.push((bs[0], bs[1], bs[2]));
    }
    let mut dists: Vec<(usize, usize, i64)> = Vec::with_capacity(boxes.len() * boxes.len());
    for i in 0..boxes.len()-1 {
        for j in i+1..boxes.len() {
            let b = boxes[i];
            let b2 = boxes[j];
            let dx = b.0 - b2.0;
            let dy = b.1 - b2.1;
            let dz = b.2 - b2.2;
            let dist = dx*dx + dy*dy + dz*dz;
            dists.push((i, j, dist));
        }
    }

    dists.sort_by_key(|k| k.2);

    let mut d: DisjointSet = DisjointSet {
        parents: (0..boxes.len()).map(Cell::new).collect(),
        ranks: vec![0; boxes.len()],
        counts: vec![1; boxes.len()],
    };

    for i in 0..connections {
        let dist = dists[i];
        merge_nodes(&mut d, dist.0, dist.1);
    }
    

    let mut largest: i64 = 0;
    let mut largest2: i64 = 0;
    let mut largest3: i64 = 0;
    let mut seen: HashSet<usize> = HashSet::new();
    for i in 0..d.parents.len() {
        let root = find_root(&mut d, i);
        if seen.contains(&root) {
            continue;
        }
        seen.insert(root);
        let count = d.counts[root];
        if count > largest {
            largest3 = largest2;
            largest2 = largest;
            largest = count;
        } else if count > largest2 {
            largest3 = largest2;
            largest2 = count;
        } else if count > largest3 {
            largest3 = count;
        }
    }

    println!("Final circuit size: {}", largest * largest2 * largest3);
}
