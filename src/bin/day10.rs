enum Op {
    Noop,
    AddX(isize),
}

impl Op {
    fn duration(&self) -> usize {
        match self {
            Op::Noop => 1,
            Op::AddX(_) => 2,
        }
    }

    fn execute(&self, x: isize) -> isize {
        match self {
            Op::Noop => x,
            Op::AddX(i) => x + *i,
        }
    }
}

fn main() {
    // Read input file
    let day = std::path::Path::new(file!())
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();
    let input = std::fs::read_to_string(format!("inputs/{day}.txt")).unwrap();

    // Parse the instructions
    let ops = input
        .lines()
        .map(|line| {
            if line.starts_with("noop") {
                Op::Noop
            } else if let Some(x) = line.strip_prefix("addx ") {
                Op::AddX(x.parse().unwrap())
            } else {
                unreachable!()
            }
        })
        .collect::<Vec<_>>();

    let mut pixels = Vec::new();
    let mut signal_strength = 0;
    let mut cycle = 0;
    let mut register: isize = 1;
    for op in ops {
        for _ in 0..op.duration() {
            let x = (cycle % 40) as isize;
            cycle += 1;
            pixels.push((register - 1) <= x && (register + 1) >= x);
            if (cycle + 20) % 40 == 0 {
                signal_strength += cycle * register;
            }
        }
        register = op.execute(register);
    }
    println!("Solution 1: {signal_strength}");

    for y in 0..6 {
        for x in 0..40 {
            if pixels[y * 40 + x] {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!();
    }
}
