use std::ops::RangeInclusive;

pub fn day4(input: &str) {
    let mut range_pairs = Vec::<(RangeInclusive<u32>, RangeInclusive<u32>)>::new();
    for line in input.lines() {
        // Assuming benign input -- and even so, there's gotta be a parser
        // library someplace that makes less of a hash of this.
        let mut toks = line.trim().split(|chr| chr == '-' || chr == ',');
        let r1 = toks.next().unwrap().parse().unwrap()..=
            toks.next().unwrap().parse().unwrap();
        let r2 = toks.next().unwrap().parse().unwrap()..=
            toks.next().unwrap().parse().unwrap();
        range_pairs.push((r1, r2));
    }

    println!("{}", range_pairs.iter().filter(|p| chk_engulf(p)).count());
    println!("{}", range_pairs.iter().filter(|p| chk_overlap(p)).count());
}

fn chk_engulf(pair: &(RangeInclusive<u32>, RangeInclusive<u32>)) -> bool {
    (pair.0.start() <= pair.1.start() && pair.1.end() <= pair.0.end()) ||
        (pair.1.start() <= pair.0.start() && pair.0.end() <= pair.1.end())
}

fn chk_overlap(pair: &(RangeInclusive<u32>, RangeInclusive<u32>)) -> bool {
    pair.0.contains(pair.1.start()) || pair.0.contains(pair.1.end()) ||
        pair.1.contains(pair.0.start()) || pair.1.contains(pair.0.end())
}
