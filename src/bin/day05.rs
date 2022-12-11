use lazy_regex::regex_captures;
use std::path::Path;

fn main() {
    // Read input file
    let day = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
    let input = std::fs::read_to_string(format!("inputs/{day}.txt")).unwrap();

    let (stacks_s, instructions_s) = input.split_once("\n\n").unwrap();
    let instructions: Vec<Instruction> = instructions_s.lines().map(parse_instruction).collect();
    let mut state = parse_stacks(stacks_s);

    for instruction in instructions {
        state = perform_instruction(state, instruction);
    }

    let result = state
        .into_iter()
        .filter_map(|v| v.last().copied())
        .collect::<String>();

    println!("Solution part 1 : {result}");
}

#[derive(PartialEq, Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn perform_instruction(mut state: Vec<Vec<char>>, instruction: Instruction) -> Vec<Vec<char>> {
    for _i in 0..instruction.count {
        let value = state[instruction.from].pop().unwrap();
        state[instruction.to].push(value);
    }

    state
}
fn parse_instruction(line: &str) -> Instruction {
    let (_, count, from, to) = regex_captures!(r#"move (\d*) from (\d*) to (\d*)$"#, line).unwrap();
    // Minus one as we use the index of a vector which starts at 0 instead of 1.
    Instruction {
        count: count.parse::<usize>().unwrap(),
        from: from.parse::<usize>().unwrap() - 1,
        to: to.parse::<usize>().unwrap() - 1,
    }
}

fn parse_stacks(stack_lines: &str) -> Vec<Vec<char>> {
    let r = lazy_regex::regex!(r"(?:\s{3,3}|\[([A-Z])\])\s?");
    let mut result = Vec::new();
    for line in stack_lines.lines().rev().skip(1) {
        for (idx, stack) in r.captures_iter(line).enumerate() {
            if idx >= result.len() {
                result.resize_with(idx + 1, Vec::new);
            }
            if let Some(Some(m)) = stack.iter().nth(1) {
                result[idx].push(m.as_str().chars().next().unwrap());
            }
        }
    }
    result
}

#[cfg(test)]
mod test {
    use crate::{parse_instruction, parse_stacks, Instruction};

    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            parse_instruction("move 1 from 2 to 1"),
            Instruction {
                count: 1,
                from: 1,
                to: 0
            }
        );
        assert_eq!(
            parse_instruction("move 3 from 1 to 3"),
            Instruction {
                count: 3,
                from: 0,
                to: 2
            }
        );
        assert_eq!(
            parse_instruction("move 2 from 2 to 1"),
            Instruction {
                count: 2,
                from: 1,
                to: 0
            }
        );
        assert_eq!(
            parse_instruction("move 1 from 1 to 2"),
            Instruction {
                count: 1,
                from: 0,
                to: 1
            }
        );
    }

    #[test]
    fn test_parse_stacks() {
        let stacks = parse_stacks("    [D]\n[N] [C]\n[Z] [M] [P]\n 1   2   3");
        assert_eq!(stacks, vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]);
    }
}
