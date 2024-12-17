use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// const IN_FILE: &str = "./test.txt";
const IN_FILE: &str = "./in.txt";

fn main() {
    let mut equation = Vec::new();

    if let Ok(lines) = read_lines(IN_FILE) {
        for line in lines.flatten() {
            let v: Vec<&str> = line.split(": ").collect();
            let k = v[0].parse::<u64>().unwrap();
            let operands: Vec<u64> = v[1].split(" ").map(|f| f.parse::<u64>().unwrap()).collect();
            // if equation.contains_key(&k) {
            //     println!("Already seen {}", k);
            // }
            equation.push((k, operands));
        }
    }

    let mut total = 0;
    for (k, v) in equation.iter() {
        // Take the first number and add to current vec.
        let mut current = Vec::from([v[0]]);
        for i in 1..v.len() {
            let next = transform2(&current, v[i], *k);
            current = next;
        }
        if current.contains(&k) {
            println!("Solved {}", k);
            total += k;
        }
    }
    println!("Total: {}", total);
    // Part 1
    // 1038838603451 - too high - not using all the numbers.
    // 737128 - too low - found first and broke.
    // 1038838357435 - too low
    // Between 1038838,357435 and 1038838,603451 - diff is 246016 - coincidentally (of course) there is one single row for that number.
    // 360 was already seen but map replaced it. Could just resubmit prev with +360 but wheres the fun...
    // 1038838357795 - after simple change from map to a vector of (int,vec)
    // Part 2
    // 254136560217241 - Simple formatting of the strings as a 3rd operator in the transform2 func.
}

fn transform(current: &Vec<u64>, v: u64, k: u64) -> Vec<u64> {
    // println!("Transforming {:?} to find {k} using {}", current,v);
    let mut next = Vec::new();
    for c in current {
        // Add.
        let mut t = c + v;
        if t <= k {
            next.push(t);
        }

        // Multiply
        t = c * v;
        if t <= k {
            next.push(t);
        }
    }
    next
}

fn transform2(current: &Vec<u64>, v: u64, k: u64) -> Vec<u64> {
    // println!("Transforming {:?} to find {k} using {}", current,v);
    let mut next = Vec::new();
    for c in current {
        // Add.
        let mut t = c + v;
        if t <= k {
            next.push(t);
        }

        // Multiply
        t = c * v;
        if t <= k {
            next.push(t);
        }

        // Append v to c.
        t = format!("{}{}", c, v).parse::<u64>().unwrap();
        if t <= k {
            next.push(t);
        }
    }
    next
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
