use std::ops::RangeInclusive;
use std::path::Path;

fn main() {
    // Read input file
    let day = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
    let input = std::fs::read_to_string(format!("inputs/{day}.txt")).unwrap();

    let result = input
        .lines()
        .map(parse_pairs)
        .filter(|(a, b)| completely_contains(a, b))
        .count();

    println!("Solution part 1 : {result}");

    let result = input
        .lines()
        .map(parse_pairs)
        .filter(|(a, b)| contains(a, b))
        .count();

    println!("Solution part 2 : {result}");
}

fn parse_range(line: &str) -> RangeInclusive<usize> {
    let (left, right) = line.split_once('-').unwrap();
    left.parse().unwrap()..=right.parse().unwrap()
}

fn parse_pairs(line: &str) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    let (left, right) = line.split_once(',').unwrap();
    (parse_range(left), parse_range(right))
}

fn completely_contains(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool {
    a.start() >= b.start() && a.end() <= b.end() || b.start() >= a.start() && b.end() <= a.end()
}

fn contains(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool {
    a.start() <= b.end() && b.start() <= a.end()
}

#[cfg(test)]
mod test {
    use crate::{completely_contains, contains, parse_pairs, parse_range};
    #[test]
    fn text_parse_range() {
        assert_eq!(parse_range("2-4"), 2..=4);
        assert_eq!(parse_range("2-3"), 2..=3);
        assert_eq!(parse_range("5-7"), 5..=7);
    }

    #[test]
    fn text_parse_pairs() {
        assert_eq!(parse_pairs("2-4,6-8"), (2..=4, 6..=8));
        assert_eq!(parse_pairs("2-3,4-5"), (2..=3, 4..=5));
        assert_eq!(parse_pairs("5-7,7-9"), (5..=7, 7..=9));
        assert_eq!(parse_pairs("6-6,4-6"), (6..=6, 4..=6));
        assert_eq!(parse_pairs("2-8,3-7"), (2..=8, 3..=7));
    }

    #[test]
    fn text_completely_contains() {
        assert_eq!(completely_contains(&(2..=4), &(6..=8)), false);
        assert_eq!(completely_contains(&(2..=3), &(4..=5)), false);
        assert_eq!(completely_contains(&(5..=7), &(7..=9)), false);
        assert_eq!(completely_contains(&(6..=6), &(4..=6)), true);
        assert_eq!(completely_contains(&(2..=8), &(3..=7)), true);
    }
    #[test]
    fn text_contains() {
        assert_eq!(contains(&(2..=4), &(6..=8)), false);
        assert_eq!(contains(&(2..=8), &(3..=7)), true);
        assert_eq!(contains(&(6..=6), &(4..=6)), true);
        assert_eq!(contains(&(2..=6), &(4..=8)), true);
    }
}
