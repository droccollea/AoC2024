// use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// use std::collections::HashMap;

// const IN_FILE: &str = "./test.txt";
const IN_FILE: &str = "./in.txt";

fn main() {
    let mut total = 0;
    let mut pages = vec![];
    let mut before: HashMap<String, Vec<String>> = HashMap::new();

    let mut bad_pages = vec![];

    if let Ok(lines) = read_lines(IN_FILE) {
        for line in lines.flatten() {
            // Rules.
            // If line contains | its a rule.
            if line.contains('|') {
                let r: Vec<String> = line.split('|').map(str::to_string).collect();
                before.entry(r[0].clone()).or_insert(Vec::new());
                let v: &mut Vec<String> = before.get_mut(&r[0]).unwrap();
                v.push(r[1].clone());
            } else if line.contains(',') {
                // Pages
                let r: Vec<String> = line.split(',').map(str::to_string).collect();
                pages.push(r);
            }
        }
    }

    for page in pages {
        let mut good: bool = true;
        for i in 0..page.len() {
            for j in 0..page.len() {
                if i == j {
                    continue;
                } else if j < i {
                    // j after i. So is the an i rule broken?
                    if before.contains_key(&page[i]) && before[&page[i]].contains(&page[j]) {
                        good = false;
                    }
                } else {
                    // j before i
                    if before.contains_key(&page[j]) && before[&page[j]].contains(&page[i]) {
                        good = false;
                    }
                }
            }
            if !good {
                bad_pages.push(page.clone());
                break;
            }
        }
    }

    for mut page in bad_pages {
        for i in 0..page.len() {
            for j in 0..page.len() {
                if i == j {
                    continue;
                } else if j < i {
                    // j after i. So is the an i rule broken?
                    if before.contains_key(&page[i]) && before[&page[i]].contains(&page[j]) {
                        let temp = page[i].clone();
                        page[i] = page[j].clone();
                        page[j] = temp;
                    }
                } else {
                    // j before i
                    if before.contains_key(&page[j]) && before[&page[j]].contains(&page[i]) {
                        let temp = page[i].clone();
                        page[i] = page[j].clone();
                        page[j] = temp;
                    }
                }
            }
        }
        // let x: i32 = page.len()+1/2;
        total += page[(&page.len() - 1) / 2].parse::<i32>().unwrap();
    }

    println!("Total: {}", total);
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
