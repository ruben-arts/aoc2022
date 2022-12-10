use std::path::Path;
#[derive(PartialEq, Debug)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}
#[derive(Debug)]
enum GameEnd {
    Win = 6,
    Lose = 0,
    Tie = 3,
}

/// Match hand from input with Hand enum
fn match_hand(hand: &str) -> Option<Shape> {
    return match hand {
        "A" | "X" => Some(Shape::Rock),
        "B" | "Y" => Some(Shape::Paper),
        "C" | "Z" => Some(Shape::Scissor),
        _ => None,
    };
}
/// Part two needs the values to be matched to GameEnd.
fn match_end(end: &str) -> Option<GameEnd>{
    return match end {
        "X" => Some(GameEnd::Lose),
        "Y" => Some(GameEnd::Tie),
        "Z" => Some(GameEnd::Win),
        _ => None,
    };
}
fn main() {
    // Read input file
    let day = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
    let input = std::fs::read_to_string(format!("inputs/{day}.txt")).unwrap();

    let mut lines = input.lines();
    let mut my_score1 = 0;
    let mut my_score2 = 0;
    while let Some(line) = lines.next() {
        let (first, second) = line.split_at(1);
        let oponent_hand = match_hand(first).unwrap();
        let my_hand = match_hand(second.trim()).unwrap();
        let outcome = match oponent_hand {
            Shape::Rock if my_hand == Shape::Paper => GameEnd::Win,
            Shape::Rock if my_hand == Shape::Scissor => GameEnd::Lose,
            Shape::Paper if my_hand == Shape::Rock => GameEnd::Lose,
            Shape::Paper if my_hand == Shape::Scissor => GameEnd::Win,
            Shape::Scissor if my_hand == Shape::Paper => GameEnd::Lose,
            Shape::Scissor if my_hand == Shape::Rock => GameEnd::Win,
            _ => GameEnd::Tie,
        };

        my_score1 += my_hand as usize + outcome as usize;

        // Part two
        let outcome2 = match_end(second.trim()).unwrap();
        my_score2 += oponent_hand as usize + outcome2 as usize;
        println!("{my_score2}");


    }
    println!("Solution 1 = {}", my_score1);
    println!("Solution 2 = {}", my_score2);

}
