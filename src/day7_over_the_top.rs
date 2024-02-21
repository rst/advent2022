use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;
use std::cell::Cell;

// Version of day 7 which avoids some hash lookups by storing references
// to the innards of the hash table.  This is the sort of speed bum which
// ordinarily makes sense only in the innermost loops of performance-critical
// code -- but I'm not trying to make sense; I'm trying to drag as much of
// the library into this code as I can.

pub fn day7(input: &str) {
    // Size of contents of each directory.
    let mut dir_size = HashMap::<String, Rc<Cell<usize>>>::new();

    // Set of files encountered -- we elsewhere assume no dups, so we
    // don't have to worry about double-counting, but at least check.
    // (Not strictly necessary; instructions don't discuss how to
    // handle this, so we can infer it doesn't happen.  But...
    // HashSet! messier Regex!)
    let mut dup_catcher = HashSet::<String>::new();

    // Current directory stack, as full paths.  (Don't bother with the
    // root, as "cd /" is at the start of every input.)
    let mut dir_stack = Vec::<String>::new();

    // And an equivalent stack of refs to sizes of those directories,
    // so we can update those without hash lookups.  Negligible
    // speedup at substantial extra complexity, but ... Rc<Cell<...>>!
    let mut dir_sz_stack = Vec::<Rc<Cell<usize>>>::new();

    // Regexes -- don't recompile for every line.
    let cdup_rx = Regex::new(r"\$ cd \.\.").unwrap();
    let cd_rx = Regex::new(r"\$ cd ([A-Za-z.]+)").unwrap();
    let cdroot_rx = Regex::new(r"\$ cd /").unwrap();
    let ls_rx = Regex::new(r"^\$ ls").unwrap();
    let dir_rx = Regex::new(r"^dir ").unwrap();
    let file_rx = Regex::new(r"^([0-9]+) ([A-Za-z.]+)$").unwrap();

    for line in input.lines() {
        if ls_rx.is_match(line) || dir_rx.is_match(line) {
            // Redundant lines; can safely ignore
        }
        else if cdroot_rx.is_match(line) {
            dir_stack.clear();
            dir_sz_stack.clear();
            dir_stack.push(String::from("/"));
            dir_sz_stack.push(dir_size.entry(String::from("/"))
                              .or_insert(Rc::new(Cell::new(0)))
                              .clone());
        }
        else if cdup_rx.is_match(line) {
            if dir_stack.len() > 1 {
                dir_stack.pop();
                dir_sz_stack.pop();
            }
        }
        else if let Some(caps) = cd_rx.captures(line) {
            let (_, [subdir]) = caps.extract();
            let full_dir = format!("{}/{}", dir_stack.last().unwrap(), subdir);
            dir_stack.push(full_dir.clone());
            dir_sz_stack.push(dir_size.entry(full_dir)
                              .or_insert(Rc::new(Cell::new(0)))
                              .clone());
        }
        else if let Some(caps) = file_rx.captures(line) {
            let (_, [file_sz_str, filename]) = caps.extract();
            let file_sz: usize = file_sz_str.parse().unwrap(); // all digits!
            let full_path = format!("{}/{}",
                                    dir_stack.last().unwrap(),
                                    filename);

            // Check assumption that each file is seen only once.
            if dup_catcher.contains(&full_path) {
                panic!("duplicate full path {}", full_path);
            }
            else {
                dup_catcher.insert(full_path);
            }

            // Add this file's size to total size of its directory, and
            // *every* parent.  We've cached pointers to the values, so
            // look, Ma... no hash lookups!
            for sz_cell_ref in &dir_sz_stack {
                sz_cell_ref.set(file_sz + sz_cell_ref.get());
            }
        }
        else {
            panic!("Unrecognized line: {}", line);
        }
    }

    // Part 1 -- total size of all small directories.  Per instructions,
    // it's OK if *this* double-counts files (tiny dir and still-small parent).
    println!("{}", dir_size.values()
                           .map(|ref_rc_cell| ref_rc_cell.get())
                           .filter(|s| *s <= 100_000)
                           .sum::<usize>());

    // Part 2 -- size of smallest directory big *enough* that nuking it would
    // permit adding a large file.
    let bytes_avail = 70_000_000 - dir_size.get("/").unwrap().get();
    let space_req = 30_000_000 - bytes_avail;
    println!("{}", dir_size.values()
                           .map(|ref_rc_cell| ref_rc_cell.get())
                           .filter(|s| *s >= space_req)
                           .min().unwrap());
}
