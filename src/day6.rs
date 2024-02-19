pub fn day6(input: &str) {
    // Assuming benign input; could just unwrap. (Heck, could just panic
    // rather than returning None.)
    println!("{}", pos_at_end_of_first_unique(4, input).unwrap_or(0));
    println!("{}", pos_at_end_of_first_unique(14, input).unwrap_or(0));
}

pub fn pos_at_end_of_first_unique(n: usize, input: &str) -> Option<usize> {
    // Brute-forcing this -- start on scratch for each n-sized slice.
    //
    // Could speed this a bit with some KMP-style trickiness, skipping
    // past the first char of the last known duplicate pair, but the savings
    // would be at most in the tens of milliseconds...

    let chars: Vec<char> = input.chars().collect();
    'next_pos: for end_pos in n..input.len() {
        let start_pos = end_pos - n;
        for i in start_pos..(end_pos - 1) {
            for j in (i+1)..end_pos {
                if chars[i] == chars[j] {
                    continue 'next_pos;
                }
            }
        }
        // No duplicate characters from start_pos to end_pos.
        return Some(end_pos);
    }
    return None;
}
