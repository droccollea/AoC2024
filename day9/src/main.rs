use std::{collections::HashSet, fs};

// const IN_FILE: &str = "./test.txt";
const IN_FILE: &str = "./in.txt";

fn main() {
    // Read the signle line file into a vector.
    // iterate by char parsed to i64
    // Create a new vector as the defragged state.
    // Part 1 - append to defragged for the even file ids and backfill spaces one by one
    // repeat until the new logical end has been reached.
    // Part 2 - contiguous files. Add a set of moved file ids.
    // Before writing, check if the file has already been moved, if so, zero the file id and advance.
    // For spaces, loop back checking for a file id that fits that hasnt already been moved.

    let disk: String = fs::read_to_string(IN_FILE).unwrap();
    let mut defragged: Vec<_> = Vec::new();
    let mut initial: Vec<_> = Vec::new();
    let mut id_moved: HashSet<_> = HashSet::new();
    let mut files_by_size = Vec::new();

    // let mut tail_ix = disk.len()-1;
    // let mut tail_id= tail_ix/2;
    // let mut tail_len = disk.chars().nth(tail_ix).unwrap().to_digit(10).unwrap();

    // Build initial disk state.
    for (i, c) in disk.char_indices() {
        let l = c.to_digit(10).unwrap();
        // Even = a file id
        if i % 2 == 0 {
            let id = i / 2;
            initial.push((id, l));

            // Record file by size for easy retrieval later.
            files_by_size.push((id, l));
        } else {
            initial.push((0, l));
        }
    }

    // Defrag.
    for i in 0..initial.len() {
        // File id, add if not yet moved or zero it.
        if i % 2 == 0 {
            if id_moved.contains(&initial[i].0) {
                defragged.push((0, initial[i].1));
            } else {
                defragged.push(initial[i]);
                id_moved.insert(initial[i].0);
            }
        } else {
            let mut l = initial[i].1;
            loop {
                let r = l - find_by_size(&files_by_size, l, &mut id_moved, &mut defragged);
                if r == 0 {
                    break;
                }
                l = r;
            }
        }
    }
    // Count it up.
    let mut total: i64 = 0;
    let mut i: i64 = 0;

    for (id, len) in defragged {
        for _l in 0..len {
            total += i * id as i64;
            i += 1;
        }
    }
    println!("Total: {total}");
    // Part 1 - 6399153661894 - correct.
    // Part 2 - 6421724645083 - correct.
}

fn find_by_size(
    files_by_size: &Vec<(usize, u32)>,
    l: u32,
    id_moved: &mut HashSet<usize>,
    defragged: &mut Vec<(usize, u32)>,
) -> u32 {
    let mut id: usize = 0;
    let mut size = l;
    // Find an unmoved matching file size.
    for (f, s) in files_by_size {
        if s <= &l && !id_moved.contains(&f) {
            id = f.clone();
            size = s.clone();
        }
    }
    defragged.push((id, size));
    id_moved.insert(id);

    return size;
}
