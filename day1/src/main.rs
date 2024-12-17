use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// const IN_FILE: &str = "./test.txt";
const IN_FILE: &str = "./in.txt";

fn main() {
    if let Ok(lines) = read_lines(IN_FILE) {
        // Consumes the iterator, returns an (Optional) String
        let re = Regex::new(r"(?m)^([0-9]+) +([0-9]+)$").unwrap();
        let mut results_1 = vec![];
        let mut results_2 = vec![];

        for line in lines.flatten() {
            for (_, [num_1, num_2]) in re.captures_iter(line.as_str()).map(|c| c.extract()) {
                results_1.push(num_1.parse::<u32>().unwrap());
                results_2.push(num_2.parse::<u32>().unwrap());
            }
        }
        results_1.sort();
        results_2.sort();

        // let mut total = 0;
        // for n in 0..results_1.len() {
        // if results_1[n] > results_2[n] {
        //     total = total + (results_1[n] - results_2[n]);
        // } else if results_1[n] < results_2[n] {
        //     total = total + (results_2[n] - results_1[n]) ;
        // }
        // }
        // println!("Total is {}", total);
        // Total is 1830467

        let mut repeats = HashMap::new();

        for a in results_2 {
            repeats.entry(a).and_modify(|a| *a += 1).or_insert(1);
        }

        let mut total = 0;
        for a in results_1 {
            if repeats.contains_key(&a) {
                total += a * repeats[&a];
            }
        }

        println!("sims total: {}", total);
        // 26674158
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
