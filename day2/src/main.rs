use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// const IN_FILE: &str = "./test.txt";
const IN_FILE: &str = "./in.txt";

fn main() {
    let mut total = 0;
    let mut input: Vec<Vec<i32>> = Vec::new();

    // Build vector of lines of parsed numbers.
    if let Ok(lines) = read_lines(IN_FILE) {
        // Get the space separated nums to a vector. Add this to the main vector.
        for line in lines.flatten() {
            // let nums = line.split(' ').map(|x| x.parse::<u32>()).collect();
            input.push(line.split(' ').map(|x| x.parse::<i32>().unwrap()).collect());
        }
    }

    // Iterate vector and for each line inc safe if
    // number are only up or down
    // numbers are 1-3 apart (not equal)
    for levels in input {
        let len = levels.len();
        if is_safe(levels.clone()) {
            // println!("Levels is safe");
            total += 1;
        }
        // second chance. Iterate original and skip one by one until safe or end of vec.
        else {
            for i in 0..len {
                let mut second: Vec<i32> = Vec::new();
                for (j, _v) in levels.iter().enumerate() {
                    if i == j {
                        continue;
                    } else {
                        second.push(levels[j]);
                    }
                }
                if is_safe(second) {
                    // println!("Second is safe");
                    total += 1;
                    break;
                }
            }
        }
    }

    println!("Total safe: {}", total);
}

fn is_safe(levels: Vec<i32>) -> bool {
    let asc = levels[0] < levels[1];
    let len = levels.len();

    for i in 0..len - 1 {
        // if inc, needs to be always up and by 1-3 only.
        // else needs to be always down and by 1-3 only.
        if asc && !(levels[i + 1] > levels[i] && levels[i + 1] <= levels[i] + 3) {
            // println!("asc unsafe {} {}", levels[i], levels[i+1]);
            return false;
        } else if !asc && !(levels[i + 1] < levels[i] && levels[i + 1] >= levels[i] - 3) {
            // println!("dsc unsafe {} {}", levels[i], levels[i+1]);
            return false;
        }
    }
    return true;
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
