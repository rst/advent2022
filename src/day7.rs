use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn day7(input: &str) {
    // Size of contents of each directory.
    let mut dir_size = HashMap::<String, usize>::new();

    // Set of files encountered -- we elsewhere assume no dups, so we
    // don't have to worry about double-counting, but at least check.
    let mut dup_catcher = HashSet::<String>::new();

    // Current directory stack, as full paths.  (Don't bother with the
    // root, as "cd /" is at the start of every input.)
    let mut dir_stack = Vec::<String>::new();

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
            dir_stack.push(String::from("/"));
        }
        else if cdup_rx.is_match(line) {
            if dir_stack.len() > 1 {
                dir_stack.pop();
            }
        }
        else if let Some(caps) = cd_rx.captures(line) {
            let (_, [subdir]) = caps.extract();
            dir_stack.push(format!("{}/{}", dir_stack.last().unwrap(), subdir));
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
            // *every* parent.  (Some extra hash lookups here, rather than
            // rolling up sums later, but it saves *me* time... could keep
            // a stack of references to size cells if it mattered much.)
            for dir in &dir_stack {
                let cur_sz = dir_size.entry(dir.clone()).or_insert(0);
                *cur_sz += file_sz;
            }
        }
        else {
            panic!("Unrecognized line: {}", line);
        }
    }

    // Part 1 -- total size of all small directories.  Per instructions,
    // it's OK if *this* double-counts files (tiny dir and still-small parent).
    println!("{}", dir_size.values().filter(|s| **s <= 100_000).sum::<usize>());

    // Part 2 -- size of smallest directory big *enough* that nuking it would
    // permit adding a large file.
    let bytes_avail = 70_000_000 - dir_size.get("/").unwrap();
    let space_req = 30_000_000 - bytes_avail;
    println!("{}", dir_size.values().filter(|s| **s >= space_req).min().unwrap());
}
