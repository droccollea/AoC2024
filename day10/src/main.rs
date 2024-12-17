use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// const IN_FILE: &str = "./test.txt";
const IN_FILE: &str = "./in.txt";

fn main() {

    let mut height_map: HashMap<usize, Vec<(usize,usize)>> = HashMap::new();

    if let Ok(lines) = read_lines(IN_FILE) {
        let mut y: usize = 0;
        for line in lines.flatten() {
            for x in 0..line.len() {
                let xy = line.chars().nth(x).unwrap().to_digit(10).unwrap() as usize;
                if height_map.contains_key(&xy) {
                    height_map.entry(xy).and_modify(|f| f.push((x,y)));
                } else {
                    height_map.insert(xy,Vec::from([(x,y)]));
                }
            }
            y += 1;
        }
    }
    
    // Part 1, find all trail heads.
    // 1. Map of vec coords of all levels.
    //  fn to get paths from a to b where coords are +/-1 on X or Y (not both)
    //  Iterate levels 0..8 get the list of 0's as inital vec, then loop the results
    // final loop reaches are paths to 9, count these.
    
    let mut paths;
    let mut nexts;
    let mut total = 0;

    for th in height_map.get(&0).unwrap() {
        paths = vec![th.clone()];
        for h in 0..9 {
            let next_heights = height_map.get(&(h+1)).unwrap();
            nexts = paths_up_from(&paths, next_heights);
            paths = nexts;
        }
        total += paths.len();
    }
    println!("Total: {total}");
    // Part 1 - 694 - correct.

}

fn paths_up_from(paths: &Vec<(usize, usize)>, steps_up: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut nexts: Vec<(usize,usize)> = Vec::new();

    for (x,y) in paths {
        if steps_up.contains(&(x+1,*y)) && !nexts.contains(&(x+1,*y)) {
            nexts.push((x+1,y.clone()));
        }
        if *x>0 && steps_up.contains(&(x-1,*y)) && !nexts.contains(&(x-1,*y)) {
            nexts.push((x-1,y.clone()));
        }
        if steps_up.contains(&(*x,y+1)) && !nexts.contains(&(*x,y+1)) {
            nexts.push((x.clone(),y+1));
        }
        if *y>0 && steps_up.contains(&(*x,y-1)) && !nexts.contains(&(*x,y-1)) {
            nexts.push((x.clone(),*y-1));
        }
    }
    nexts

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
