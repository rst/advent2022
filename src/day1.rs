pub fn day1(input: &str) {
    let mut totals = Vec::<i32>::new();
    let mut cur_total = 0;

    for line in input.lines() {
        let line = line.trim();
        if line == "" {
            totals.push(cur_total);
            cur_total = 0;
        }
        else {
            cur_total += line.parse::<i32>()
                .unwrap_or_else(|_| panic!("{line} not a number"));
        }
    }

    if cur_total > 0 {
        // Pick up the group ended by EOF!
        totals.push(cur_total);
    }

    totals.sort_by_key(|x| -x);
    println!("{}", totals[0]);
    println!("{}", totals[0] + totals[1] + totals[2]);
}
