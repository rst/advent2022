// Day 2: Rock-paper-scissors "strategy" books.
// 
// An explicit enum for "rock, paper, scissors" would make it obvious to
// the compiler that the match in part 2 _is_ exhaustive, but math on these
// values is a convenience, particularly given how "game_score" is specified.

pub fn day2(input: &str) {
    let mut game_specs = Vec::<(i8, i8)>::new();

    for line in input.lines() {
        let mut strs = line.split_ascii_whitespace();
        let elf_val = match strs.next() {
            Some("A") => 1,
            Some("B") => 2,
            Some("C") => 3,
            _ => panic!("'{line}' malformatted")
        };
        let my_val = match strs.next() {
            Some("X") => 1,
            Some("Y") => 2,
            Some("Z") => 3,
            _ => panic!("'{line}' malformatted")
        };
        game_specs.push((elf_val, my_val));
    }

    // Part 1 -- "my_val" dictates plays.
    let mut score = 0;
    for (elf_val, my_val) in &game_specs {
        score += game_score(*elf_val, *my_val);
    }
    println!("{}", score);

    // Part 2 -- "my_val" dictates outcomes.
    score = 0;
    for (elf_val, my_val) in &game_specs {
        let my_play = match my_val {
            1 => if *elf_val == 1 {3} else {*elf_val - 1}, // force loss
            2 => *elf_val,                                 // force draw
            3 => if *elf_val == 3 {1} else {*elf_val + 1}, // force win
            _ => panic!("impossible my_val {}", my_val)
        };
        score += game_score(*elf_val, my_play);
    }
    println!("{}", score);
}

fn game_score(elf_play: i8, my_play: i8) -> i32 {
    let val = if my_play == elf_play {
        my_play + 3             // draw
    }
    else if my_play == elf_play + 1 || (my_play == 1 && elf_play == 3) {
        my_play + 6             // win
    }
    else {
        my_play                 // loss
    };
    return val.into();
}
