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

    let result: usize = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(find_overlapping_item_in_group)
        .map(char_to_priority)
        .sum();

    println!("Solution part 2 : {}", result);
}

fn find_overlapping_item_in_group<'a>(group: &'a [&'a str]) -> char {
    let first_sack = group[0];
    for c in first_sack.chars() {
        if group[1].contains(c) && group[2].contains(c) {
            return c;
        }
    }
    unreachable!()
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
    use crate::{char_to_priority, find_overlapping_item_in_group};

    #[test]
    fn test_char_to_priority() {
        assert_eq!(char_to_priority('L'), 38);
        assert_eq!(char_to_priority('a'), 1);
        assert_eq!(char_to_priority('p'), 16);
        assert_eq!(char_to_priority('P'), 42);
        assert_eq!(char_to_priority('v'), 22);
        assert_eq!(char_to_priority('s'), 19);
    }

    #[test]
    fn test_find_overlapping_item_in_group() {
        assert_eq!(
            find_overlapping_item_in_group(&[
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg"
            ]),
            'r'
        );
        assert_eq!(
            find_overlapping_item_in_group(&[
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw"
            ]),
            'Z'
        );
    }
}
