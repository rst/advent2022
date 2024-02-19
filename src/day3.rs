pub fn day3(input: &str) {
    // Part 1 -- total score of sole duplicate in first and 2nd half of a line.
    let mut total_part1 = 0;
    for line in input.lines() {
        // Split the string in halves, avoiding utf8 length oddities --
        // could avoid this since we know all the characters are ASCII,
        // but that *might* be bad style.
        
        let line: Vec<char> = line.trim().chars().collect();
        let half_size = line.len() / 2;
        let first_half = &line[0..half_size];
        let second_half = &line[half_size..];
        for chr in first_half {
            if second_half.iter().any(|x| x == chr) {
                total_part1 += score(*chr);
                break;
            }
        }
    }
    println!("{}", total_part1);

    // Part 2 -- total score of sole triplicate in groups of three lines.
    let mut lines = input.lines();
    let mut total_part2 = 0;
    loop {
        let line1 = match lines.next() {
            Some(x) => x,
            None => break
        };
        let line2 = lines.next().unwrap_or_else(|| panic!("truncated?"));
        let line3 = lines.next().unwrap_or_else(|| panic!("truncated?"));

        for chr in line1.chars() {
            if line2.chars().any(|x| x == chr) &&
               line3.chars().any(|x| x == chr)
            {
                total_part2 += score(chr);
                break;
            }
        }
    }
    println!("{}", total_part2);
}

fn score(chr: char) -> u32 {
    match chr {
        'a'..='z' => (chr as u32) - ('a' as u32) + 1,
        'A'..='Z' => (chr as u32) - ('A' as u32) + 27,
        _ => panic!("Unexpected char {}", chr)
    }
}
