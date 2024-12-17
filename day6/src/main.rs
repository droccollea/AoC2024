use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// const IN_FILE: &str = "./test.txt";
const IN_FILE: &str = "./in.txt";

fn main() {
    // let mut total = 0;
    let mut input = vec![];
    let mut blocks = vec![];
    let dirs = Vec::from(['N', 'E', 'S', 'W']);
    let mut g_map = HashMap::new();
    let mut start: (i32, i32) = (0, 0);
    let mut obs = vec![];

    if let Ok(lines) = read_lines(IN_FILE) {
        let mut y = 0;
        for line in lines.flatten() {
            for x in line.match_indices("#") {
                blocks.push((x.0 as i32, y));
            }
            for x in line.match_indices("^") {
                start = (x.0 as i32, y);
            }
            input.push(line);
            y += 1;
        }
    }

    let max_y: i32 = input.len() as i32;
    let max_x: i32 = input[0].len() as i32;
    let mut ob;
    for i in 0..max_y {
        for j in 0..max_x {
            ob = (j, i);
            // println!("New ob at: {:?}", ob);
            // Start facing N. Add to the guard map.
            let mut facing = 0;
            g_map.clear();
            g_map.insert(start, Vec::from([dirs[facing]]));

            // Advance through the map until we arrive back at a point been before facing the same dir.
            let mut cur = start;
            // println!("Starting at {:?}", start);
            loop {
                let nxt = get_next(cur, dirs[facing]);
                // println!("Trying {:?}", nxt);

                // if pos seen before with dir break and record this obstacle.
                if g_map
                    .get(&nxt)
                    .unwrap_or(&vec!['x'])
                    .contains(&dirs[facing])
                {
                    println!("Looping at {:?}, ob {:?}", nxt, ob);
                    obs.push(ob);
                    break;
                }

                if blocks.contains(&nxt)
                    || match nxt {
                        (x, y) => x == j && y == i,
                    }
                {
                    // Rotate direction 90 degrees.
                    facing = (facing + 1) % 4;
                    // println!("Blocked at {:?} rotated to face {}", nxt, dirs[facing]);
                    continue;
                }

                // if invalid - out of bounds or blocked, rotate and try again.
                if match nxt {
                    (_x, -1) => true,
                    (-1, _y) => true,
                    (x, y) => x >= max_x || y >= max_y,
                } {
                    // println!("Leaving map at {:?} to {:?}", cur, nxt);
                    break;
                }

                // else set pos and dir as current
                cur = nxt;
                if g_map.contains_key(&cur) {
                    g_map.entry(cur).and_modify(|f| f.push(dirs[facing]));
                } else {
                    g_map.insert(cur, Vec::from([dirs[facing]]));
                }
            }
        }
    }

    println!("Total: {}", obs.len());
    // 1304 - brute forced part 2.
}

fn get_next(cur: (i32, i32), dir: char) -> (i32, i32) {
    match dir {
        'N' => (cur.0, cur.1 - 1),
        'E' => (cur.0 + 1, cur.1),
        'S' => (cur.0, cur.1 + 1),
        'W' => (cur.0 - 1, cur.1),
        _ => (0, 0),
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
