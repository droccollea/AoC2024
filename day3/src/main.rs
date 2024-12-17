use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// const IN_FILE: &str = "./test.txt";
const IN_FILE: &str = "./in.txt";

fn main() {
    let mut total = 0;

    if let Ok(lines) = read_lines(IN_FILE) {
        // Consumes the iterator, returns an (Optional) String
        let re = Regex::new(r"(?m)(mul\([0-9]{1,3},[0-9]{1,3}\))").unwrap();
        let mut input = vec![];

        // Join one big string.
        let mut all_in = String::new();

        for line in lines.flatten() {
            all_in.push_str(&line);
        }

        // Find the don't() - do() and remove anything in between.
        let mut start;
        let mut end;
        loop {
            // seek next don't()
            // if no dont, were done
            let s = all_in.find("don't()");
            if s == None {
                break;
            } else {
                start = s.unwrap();
            }
            // seek next do()
            // if no do, end is all_in.len()-1 and were done
            // Skip spurious dos before dont with .{start}

            let e = all_in.find("do()");
            if e == None {
                end = all_in.len() - 1;
            } else {
                // add chars for "o()" string (hope its not out of bounds.)
                end = e.unwrap() + 3;
            }

            // If do before the dont, Fudge it.
            if start > end {
                all_in.replace_range(end - 3..end, "");
            } else {
                // Discard the dont section.
                all_in.replace_range(start..end, "");
            }
        }

        for (q, [_]) in re.captures_iter(all_in.as_str()).map(|c| c.extract()) {
            input.push(map_ints(q));
        }

        for (a, b) in input {
            total += a * b;
        }

        println!("Total: {}", total);
        // 2999776 - too low
        // 56275602
    }
}

fn map_ints(line: &str) -> (u32, u32) {
    let re = Regex::new(r"(?m)mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    for (_, [num_1, num_2]) in re.captures_iter(line).map(|c| c.extract()) {
        x = num_1.parse::<u32>().unwrap();
        y = num_2.parse::<u32>().unwrap();
    }
    return (x, y);
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
