use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// use std::collections::HashMap;

// const IN_FILE: &str = "./test.txt";
const IN_FILE: &str = "./in.txt";

fn main() {
    let mut total = 0;

    let mut input = vec![];
    if let Ok(lines) = read_lines(IN_FILE) {
        for line in lines.flatten() {
            input.push(line);
        }
    }

    // Part 1
    let re = Regex::new(r"(?m)(X)").unwrap();

    for y in 0..input.len() {
        // Get the indexs of each X
        // For each, seach in all 8 directions and increment total for each find.
        // for (z, [x]) in re.captures_iter(input[y].as_str()).map(|c| c.extract()) {
        for x in re.find_iter(input[y].as_str()) {
            // println!("X at {},{}", x.start(), y);
            total += is_n(&input, x.start(), y)
                + is_ne(&input, x.start(), y)
                + is_e(&input, x.start(), y)
                + is_se(&input, x.start(), y)
                + is_s(&input, x.start(), y)
                + is_sw(&input, x.start(), y)
                + is_w(&input, x.start(), y)
                + is_nw(&input, x.start(), y);
        }
    }

    // Part 2
    let re = Regex::new(r"(?m)(A)").unwrap();

    for y in 0..input.len() {
        // Get the indexs of each A
        // For each, seach in four directions and increment total for each find of an M & S.
        for x in re.find_iter(input[y].as_str()) {
            // println!("X at {},{}", x.start(), y);
            if is_ne_and_sw(&input, x.start(), y) && is_se_and_nw(&input, x.start(), y) {
                total += 1;
            }
        }
    }

    println!("Total: {}", total);
    // Part 1
    // 2551
    // Part 2
    // 1985
}

fn is_n(input: &Vec<String>, x: usize, y: usize) -> i32 {
    let mut ret: i32 = 0;
    // Check for XMAS reading up.
    // Check bounds then iterate y-1
    if y < 3 {
        println!("N Issue: x:{}, y:{} len:{}", x, y, input[y].len());
        return ret;
    }
    let mut xmas = String::new();
    xmas.push(input[y].chars().nth(x).unwrap_or_default());
    xmas.push(input[y - 1].chars().nth(x).unwrap_or_default());
    xmas.push(input[y - 2].chars().nth(x).unwrap_or_default());
    xmas.push(input[y - 3].chars().nth(x).unwrap_or_default());

    if "XMAS" == xmas {
        ret = 1;
    }
    return ret;
}

fn is_ne(input: &Vec<String>, x: usize, y: usize) -> i32 {
    let mut ret: i32 = 0;
    // Check for XMAS reading up.
    // Check bounds then iterate y-1
    if y < 3 || x + 3 > input[y].len() {
        println!("NE Issue: x:{}, y:{} len:{}", x, y, input[y].len());
        return ret;
    }
    let mut xmas = String::new();
    xmas.push(input[y].chars().nth(x).unwrap_or_default());
    xmas.push(input[y - 1].chars().nth(x + 1).unwrap_or_default());
    xmas.push(input[y - 2].chars().nth(x + 2).unwrap_or_default());
    xmas.push(input[y - 3].chars().nth(x + 3).unwrap_or_default());

    if "XMAS" == xmas {
        ret = 1;
    }
    return ret;
}

fn is_e(input: &Vec<String>, x: usize, y: usize) -> i32 {
    let mut ret: i32 = 0;
    // Check for XMAS reading up.
    // Check bounds then iterate y-1
    if x + 3 > input[y].len() {
        println!("E Issue: x:{}, y:{} len:{}", x, y, input[y].len());

        return ret;
    }
    let mut xmas = String::new();
    xmas.push(input[y].chars().nth(x).unwrap_or_default());
    xmas.push(input[y].chars().nth(x + 1).unwrap_or_default());
    xmas.push(input[y].chars().nth(x + 2).unwrap_or_default());
    xmas.push(input[y].chars().nth(x + 3).unwrap_or_default());

    if "XMAS" == xmas {
        ret = 1;
    }
    return ret;
}

fn is_se(input: &Vec<String>, x: usize, y: usize) -> i32 {
    let mut ret: i32 = 0;
    // Check for XMAS reading up.
    // Check bounds then iterate y-1
    if y + 3 > input.len() - 1 || x + 3 > input[y].len() {
        println!("SE Issue: x:{}, y:{} len:{}", x, y, input[y].len());

        return ret;
    }
    let mut xmas = String::new();
    xmas.push(input[y].chars().nth(x).unwrap_or_default());
    xmas.push(input[y + 1].chars().nth(x + 1).unwrap_or_default());
    xmas.push(input[y + 2].chars().nth(x + 2).unwrap_or_default());
    xmas.push(input[y + 3].chars().nth(x + 3).unwrap_or_default());

    if "XMAS" == xmas {
        ret = 1;
    }
    return ret;
}

fn is_s(input: &Vec<String>, x: usize, y: usize) -> i32 {
    let mut ret: i32 = 0;
    // Check for XMAS reading up.
    // Check bounds then iterate y-1
    if y + 3 > input.len() - 1 {
        println!("S Issue: x:{}, y:{} len:{}", x, y, input[y].len());

        return ret;
    }
    let mut xmas = String::new();
    xmas.push(input[y].chars().nth(x).unwrap_or_default());
    xmas.push(input[y + 1].chars().nth(x).unwrap_or_default());
    xmas.push(input[y + 2].chars().nth(x).unwrap_or_default());
    xmas.push(input[y + 3].chars().nth(x).unwrap_or_default());

    if "XMAS" == xmas {
        ret = 1;
    }
    return ret;
}

fn is_sw(input: &Vec<String>, x: usize, y: usize) -> i32 {
    let mut ret: i32 = 0;
    // Check for XMAS reading up.
    // Check bounds then iterate y-1
    if y + 3 > input.len() - 1 || x < 3 {
        println!("SW Issue: x:{}, y:{} len:{}", x, y, input[y].len());

        return ret;
    }
    let mut xmas = String::new();
    xmas.push(input[y].chars().nth(x).unwrap_or_default());
    xmas.push(input[y + 1].chars().nth(x - 1).unwrap_or_default());
    xmas.push(input[y + 2].chars().nth(x - 2).unwrap_or_default());
    xmas.push(input[y + 3].chars().nth(x - 3).unwrap_or_default());

    if "XMAS" == xmas {
        ret = 1;
    }
    return ret;
}

fn is_w(input: &Vec<String>, x: usize, y: usize) -> i32 {
    let mut ret: i32 = 0;
    // Check for XMAS reading up.
    // Check bounds then iterate y-1
    if x < 3 {
        println!("W Issue: x:{}, y:{} len:{}", x, y, input[y].len());

        return ret;
    }
    let mut xmas = String::new();
    xmas.push(input[y].chars().nth(x).unwrap_or_default());
    xmas.push(input[y].chars().nth(x - 1).unwrap_or_default());
    xmas.push(input[y].chars().nth(x - 2).unwrap_or_default());
    xmas.push(input[y].chars().nth(x - 3).unwrap_or_default());

    if "XMAS" == xmas {
        ret = 1;
    }
    return ret;
}

fn is_nw(input: &Vec<String>, x: usize, y: usize) -> i32 {
    let mut ret: i32 = 0;
    // Check for XMAS reading up.
    // Check bounds then iterate y-1
    if y < 3 || x < 3 {
        println!("NW Issue: x:{}, y:{} len:{}", x, y, input[y].len());
        return ret;
    }
    let mut xmas = String::new();
    xmas.push(input[y].chars().nth(x).unwrap_or_default());
    xmas.push(input[y - 1].chars().nth(x - 1).unwrap_or_default());
    xmas.push(input[y - 2].chars().nth(x - 2).unwrap_or_default());
    xmas.push(input[y - 3].chars().nth(x - 3).unwrap_or_default());

    if "XMAS" == xmas {
        ret = 1;
    }
    return ret;
}

fn is_ne_and_sw(input: &Vec<String>, x: usize, y: usize) -> bool {
    // Check for MAS reading / dir.
    // Check bounds is A is an edge corner, false.
    if y == 0 || y == input.len() - 1 || x == 0 || x == input[y].len() - 1 {
        println!("A on the edge: x:{}, y:{} len:{}", x, y, input[y].len());
        return false;
    }
    let mut mas = String::new();
    mas.push(input[y - 1].chars().nth(x + 1).unwrap_or_default()); // NE
    mas.push(input[y].chars().nth(x).unwrap_or_default()); // A
    mas.push(input[y + 1].chars().nth(x - 1).unwrap_or_default()); // SW

    "MAS" == mas || "SAM" == mas
}

fn is_se_and_nw(input: &Vec<String>, x: usize, y: usize) -> bool {
    // Check for MAS reading \ dir.
    // Check bounds is A is an edge corner, false.
    if y == 0 || y == input.len() - 1 || x == 0 || x == input[y].len() - 1 {
        println!("A on the edge: x:{}, y:{} len:{}", x, y, input[y].len());
        return false;
    }
    let mut mas = String::new();
    mas.push(input[y - 1].chars().nth(x - 1).unwrap_or_default()); // NW
    mas.push(input[y].chars().nth(x).unwrap_or_default()); // A
    mas.push(input[y + 1].chars().nth(x + 1).unwrap_or_default()); // SE

    "MAS" == mas || "SAM" == mas
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
