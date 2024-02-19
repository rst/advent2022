use std::collections::VecDeque;
use std::fmt;

struct MoveCommand {
    number: usize,
    from: usize,
    to: usize
}

impl MoveCommand {
    fn new(number: usize, from: usize, to: usize) -> MoveCommand {
        MoveCommand {
            number,
            from,
            to,
        }
    }

    // These don't really need to be generic here -- we only ever use 'em
    // with Vec::<VecDeque::<char>>.  But I'm trying to learn this #$@#%
    // language...

    fn step_pt1<A: Copy>(self: &MoveCommand,
                         towers: &mut Vec::<VecDeque::<A>>)
    {
        // Pop elements off one tower, push onto the other --
        // reversing the order of the elements moved.
        //
        // Problem conditions are that underflow "can't happen",
        // so we don't try to handle it.
        for _ in 0..self.number {
            let item = towers[self.from].pop_front().expect("no underflow");
            towers[self.to].push_front(item);
        }
    }

    fn step_pt2<A: Copy>(self: &MoveCommand,
                         towers: &mut Vec::<VecDeque::<A>>)
    {
        // This time, we're supposed to preserve the order of elements moved.
        // Well, we can do that by reversing twice...
        let mut tmp = Vec::<A>::new();
        for _ in 0..self.number {
            // Underflows here indicate buggy input.
            tmp.push(towers[self.from].pop_front().expect("no underflow"));
        }
        for item in tmp.iter().rev() {
            towers[self.to].push_front(*item);
        }
    }
}

// Left over from debugging...
impl fmt::Display for MoveCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "move #{} from #{} to #{}", self.number, self.from, self.to)
    }
}

// Run a bunch of MoveCommands, return labels of boxes on top.
fn execute<A: Copy>(stepper: fn(&MoveCommand, &mut Vec::<VecDeque::<A>>) -> (),
                    program: &Vec<MoveCommand>,
                    at_start: &Vec::<VecDeque::<A>>)
    -> Vec::<A>
{
    let mut state = at_start.clone();
    for step in program {
        stepper(step, &mut state);
    }
    let mut result = Vec::<A>::new();
    for deque in state {
        // Again, underflows here indicate buggy input.
        result.push(*deque.front().expect("no underflow"));
    }
    return result;
}

pub fn day5(input: &str) {
    // Parse initial state of towers.
    let mut towers = Vec::<VecDeque::<char>>::new();
    let mut lines = input.lines();

    // Why doesn't a "for line in lines" work here?!
    loop {
        let line = match lines.next() {
            Some(line) => line,
            None => break
        };
        if line == "" { break }; // end tower specs
        let chars: Vec<char> = line.chars().collect(); // make indexable
        for pos in 0.. {
            let chr_pos = 1 + 4 * pos;
            if chr_pos >= chars.len() {
                break
            };
            let chr = chars[chr_pos];
            if chr != ' ' {
                while towers.len() <= pos {
                    towers.push(VecDeque::<char>::new());
                }
                towers[pos].push_back(chr);
            }
        }
    }

    // Parse instructions.
    let mut program = Vec::<MoveCommand>::new();

    loop {
        let line = match lines.next() {
            Some(line) => line.trim(),
            None => break     // EOF expected here
        };
        let mut tokens = line.trim().split_ascii_whitespace();
        tokens.next();          // "move"
        let number: usize = tokens.next().unwrap().parse().unwrap();
        tokens.next();          // "from"
        let from: usize = tokens.next().unwrap().parse().unwrap();
        tokens.next();          // "to"
        let to: usize = tokens.next().unwrap().parse().unwrap();
        program.push(MoveCommand::new(number, from - 1, to - 1));
    }

    // Run the simulations.
    let s: String = execute(MoveCommand::step_pt1, &program, &towers)
        .into_iter().collect();
    println!("{}", s);
    let s: String = execute(MoveCommand::step_pt2, &program, &towers)
        .into_iter().collect();
    println!("{}", s);
}
