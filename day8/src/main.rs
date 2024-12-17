use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// const IN_FILE: &str = "./test.txt";
const IN_FILE: &str = "./in.txt";

fn main() {
    // let mut total = 0;
    // let mut ants: Vec<(i32,i32)> = Vec::new();
    let mut map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut uniqs: HashSet<(i32, i32)> = HashSet::new();

    let mut max_x = 0;
    let mut max_y = 0;

    if let Ok(lines) = read_lines(IN_FILE) {
        let mut y = 0;
        for line in lines.flatten() {
            for x in 0..line.len() {
                let xy = line.chars().nth(x).unwrap();
                if '.' == xy {
                    continue;
                }

                let xi32 = x.try_into().unwrap();
                if map.contains_key(&xy) {
                    map.entry(xy).and_modify(|f| f.push((xi32, y)));
                } else {
                    map.insert(xy, Vec::from([(xi32, y)]));
                }
                uniqs.insert((xi32, y));
            }
            y += 1;
        }
        max_y = y - 1;
        max_x = y - 1;
    }

    // Iterate the key values.
    // For each group, check for a pair then check if the pair has an antinode either side.
    // Assuming an antenna/antinode serves many groups.
    for (_key, nodes) in map.iter() {
        for i in 0..nodes.len() {
            for j in i + 1..nodes.len() {
                // get x and y diff between nodes[i] and nodes[j]
                // check it there is an antenna either side
                // if so add the coords to uniqs - to get unique locations.

                let (mut ix, mut iy) = nodes[i];
                let (mut jx, mut jy) = nodes[j];
                let dx: i32 = ix - jx; // X
                let dy: i32 = iy - jy; // Y

                // Before i
                // Part 2 loop until map edge.
                let mut cand;
                loop {
                    cand = (&ix + &dx, &iy + &dy);
                    if match cand {
                        (x, y) => x < 0 || x > max_x || y < 0 || y > max_y,
                    } {
                        break;
                    }
                    uniqs.insert(cand);
                    (ix, iy) = cand;
                }
                //  After j
                loop {
                    cand = (&jx - &dx, &jy - &dy);
                    if match cand {
                        (x, y) => x < 0 || x > max_x || y < 0 || y > max_y,
                    } {
                        break;
                    }
                    uniqs.insert(cand);
                    (jx, jy) = cand;
                }
            }
        }
    }

    println!("Total: {}", uniqs.len());
    // Part 1
    // 261  - correct
    // Part 2
    // Looped until map edges. Also added all known initially locations too.
    // 898 - correct.
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
