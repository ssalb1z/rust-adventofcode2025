use std::fs;
use std::env;
use std::process;

type Point = (i64, i64);
type Edge = (Point, Point);

fn on_line(e: Edge, p: Point) -> bool {
    let x = p.0;
    let y = p.1;
    let x1 = e.0.0;
    let x2 = e.1.0;
    let y1 = e.0.1;
    let y2 = e.1.1;
    let det = ((y - y1) * (x2-x1)) - (x-x1)*(y2-y1);
    let max_x = x1.max(x2);
    let max_y = y1.max(y2);
    let min_x = x1.min(x2);
    let min_y = y1.min(y2);
    return det == 0 && x <= max_x && x >= min_x && y <= max_y && y >= min_y;
}


fn is_inside(edges: &Vec<Edge>, p: Point) -> bool {
    for e in edges {
        if on_line(*e, p) {
            return true;
        }
    }

    // Ray casting - only count vertical edges
    let mut intersections = 0;
    for e in edges {
        let (x1, y1) = e.0;
        let (x2, y2) = e.1;

        // Only consider vertical edges
        if x1 == x2 {
            let x = x1;
            let (min_y, max_y) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            if x > p.0 && p.1 > min_y && p.1 <= max_y {
                intersections += 1;
            }
        }
    }
    return intersections % 2 == 1
}

// Check if any polygon edge passes through the interior of the rectangle
fn edge_crosses_rect_interior(edges: &Vec<Edge>, min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> bool {
    for e in edges {
        let (x1, y1) = e.0;
        let (x2, y2) = e.1;

        if x1 == x2 {
            // Vertical edge at x = x1
            // Only bad if x is strictly inside rectangle's x-range
            if x1 > min_x && x1 < max_x {
                let (ey_min, ey_max) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
                // And y-ranges overlap
                if ey_min < max_y && ey_max > min_y {
                    return true;
                }
            }
        } else {
            // Horizontal edge at y = y1
            // Only bad if y is strictly inside rectangle's y-range
            if y1 > min_y && y1 < max_y {
                let (ex_min, ex_max) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
                // And x-ranges overlap
                if ex_min < max_x && ex_max > min_x {
                    return true;
                }
            }
        }
    }
    return false;
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No input argument provided - pass filename and # of connections to add");
        process::exit(1);
    }

    let input_raw = fs::read_to_string(&args[1]).expect("Expected readable file input");
    let input = input_raw.lines();
    let mut points: Vec<Point> = Vec::new();
    let mut edges: Vec<Edge> = Vec::new();
    let mut last_added_point: (i64, i64) = (-1, -1);

    for line in input {
        let ps: Vec<i64> = line.split(',').map(|x| x.trim().parse::<i64>().unwrap()).collect();
        points.push((ps[0], ps[1]));
        if last_added_point.0 > 0 {
            edges.push(((ps[0], ps[1]), last_added_point));
        }
        last_added_point = (ps[0], ps[1]);
    }
    edges.push((last_added_point, points[0]));

    let mut max_known_area = 0;
    for i in 0..points.len()-1 {
        for j in i+1..points.len() {
            let c1 = points[i];
            let c2 = points[j];
            let opp_corner1 = (c1.0, c2.1);
            let opp_corner2 = (c2.0, c1.1);
            let area = ((c1.0 - c2.0).abs() + 1) * ((c1.1 - c2.1).abs() + 1);
            if area <= max_known_area {
                continue;
            }

            let inside1 = is_inside(&edges, opp_corner1);
            let inside2 = is_inside(&edges, opp_corner2);

            if !inside1 || !inside2 {
                continue;
            }

            let min_x = c1.0.min(c2.0);
            let max_x = c1.0.max(c2.0);
            let min_y = c1.1.min(c2.1);
            let max_y = c1.1.max(c2.1);

            if edge_crosses_rect_interior(&edges, min_x, max_x, min_y, max_y) {
                continue;
            }
            println!("{inside1}");
            println!("{inside2}");
            println!("{c1:?}, {c2:?}, {opp_corner1:?}, {opp_corner2:?} -> {area}");
            max_known_area = area;
        }
    }
    

    println!("Final area: {max_known_area}");
}
