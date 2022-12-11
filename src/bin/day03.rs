use std::path::Path;

fn main() {
    // Read input file
    let day = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
    let input = std::fs::read_to_string(format!("inputs/{day}.txt")).unwrap();

    // Find common char in both compartments
    let result: usize = input
        .lines()
        .map(find_overlapping_item)
        .map(char_to_priority)
        .sum();

    println!("Solution part 1 : {}", result);
}

fn find_overlapping_item(line: &str) -> char {
    let (left, right) = line.split_at(line.len() / 2);
    for c in left.chars() {
        if right.contains(c) {
            return c;
        }
    }
    unreachable!()
}

fn char_to_priority(c: char) -> usize {
    match c {
        'a'..='z' => (c as usize) - ('a' as usize) + 1,
        'A'..='Z' => (c as usize) - ('A' as usize) + 27,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod test {
    use crate::char_to_priority;

    #[test]
    fn test_char_to_priority() {
        assert_eq!(char_to_priority('L'), 38);
        assert_eq!(char_to_priority('a'), 1);
        assert_eq!(char_to_priority('p'), 16);
        assert_eq!(char_to_priority('P'), 42);
        assert_eq!(char_to_priority('v'), 22);
        assert_eq!(char_to_priority('s'), 19);
    }
}
