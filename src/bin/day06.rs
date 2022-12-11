use std::path::Path;

fn main() {
    // Read input file
    let day = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
    let input = std::fs::read_to_string(format!("inputs/{day}.txt")).unwrap();

    let result = find_marker_index(&input, 4);
    println!("Solution 1: {result}");

    let result = find_marker_index(&input, 14);
    println!("Solution 2: {result}");
}

fn find_marker_index(input: &str, window_size: usize) -> usize {
    'outer: for i in 0..input.len() - window_size - 1 {
        let window = &input[i..i + window_size];
        for (idx, c) in window.chars().enumerate() {
            if window[idx + 1..].contains(c) {
                continue 'outer;
            }
        }
        return i + window_size;
    }

    unreachable!();
}

#[cfg(test)]
mod test {
    use super::find_marker_index;

    #[test]
    fn test_find_marker_index() {
        assert_eq!(find_marker_index("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(find_marker_index("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(
            find_marker_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            10
        );
        assert_eq!(find_marker_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn test_find_marker_index_14() {
        assert_eq!(find_marker_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(find_marker_index("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(find_marker_index("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(
            find_marker_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
            29
        );
        assert_eq!(
            find_marker_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14),
            26
        );
    }
}
