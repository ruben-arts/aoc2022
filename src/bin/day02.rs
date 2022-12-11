use std::path::Path;
#[derive(PartialEq, Debug, Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}
#[derive(Debug)]
enum GameEnd {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

struct Hand {
    shape: Shape,
    wins_from: Shape,
    looses_from: Shape,
}
impl Hand {
    fn outcome(&self, opponent: &Hand) -> GameEnd {
        if self.shape == opponent.shape {
            GameEnd::Draw
        } else if self.looses_from == opponent.shape {
            GameEnd::Lose
        } else {
            GameEnd::Win
        }
    }
}

/// Part two needs the values to be matched to GameEnd.
fn match_end(end: &str) -> Option<GameEnd> {
    match end {
        "X" => Some(GameEnd::Lose),
        "Y" => Some(GameEnd::Draw),
        "Z" => Some(GameEnd::Win),
        _ => None,
    }
}
fn main() {
    // Read input file
    let day = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
    let input = std::fs::read_to_string(format!("inputs/{day}.txt")).unwrap();

    let rock = Hand {
        shape: Shape::Rock,
        wins_from: Shape::Scissor,
        looses_from: Shape::Paper,
    };
    let paper = Hand {
        shape: Shape::Paper,
        wins_from: Shape::Rock,
        looses_from: Shape::Scissor,
    };
    let scissor = Hand {
        shape: Shape::Scissor,
        wins_from: Shape::Paper,
        looses_from: Shape::Rock,
    };

    let match_hand = |s: &str| match s {
        "A" | "X" => Some(&rock),
        "B" | "Y" => Some(&paper),
        "C" | "Z" => Some(&scissor),
        _ => None,
    };

    let lines = input.lines();
    let mut my_score1 = 0;
    let mut my_score2 = 0;
    for line in lines {
        let (first, second) = line.split_at(1);
        let opponent_hand = match_hand(first).unwrap();

        // Part one
        let my_hand = match_hand(second.trim()).unwrap();
        let outcome = my_hand.outcome(opponent_hand);

        my_score1 += my_hand.shape as usize + outcome as usize;

        // Part two
        let outcome2 = match_end(second.trim()).unwrap();
        let my_hand2_shape = match outcome2 {
            GameEnd::Win => opponent_hand.looses_from,
            GameEnd::Draw => opponent_hand.shape,
            GameEnd::Lose => opponent_hand.wins_from,
        };
        my_score2 += my_hand2_shape as usize + outcome2 as usize;
    }
    println!("Solution 1 = {}", my_score1);
    println!("Solution 2 = {}", my_score2);
}
