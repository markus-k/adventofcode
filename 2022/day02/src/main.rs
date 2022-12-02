use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt");
    let score = calculate_score_part1(input);
    let score2 = calculate_score_part2(input);

    println!("Our score part1: {score}");
    println!("Our score part2: {score2}");
}

#[derive(Clone, PartialEq, Eq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn hand_score(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn play(&self, other: &Hand) -> Outcome {
        if self == other {
            Outcome::Draw
        } else {
            match (self, other) {
                (Hand::Rock, Hand::Scissors)
                | (Hand::Paper, Hand::Rock)
                | (Hand::Scissors, Hand::Paper) => Outcome::Win,
                _ => Outcome::Lose,
            }
        }
    }

    fn play_score(&self, other: &Hand) -> u32 {
        self.play(other).score() + self.hand_score()
    }

    fn for_outcome(outcome: Outcome, against: &Hand) -> Self {
        match outcome {
            Outcome::Lose => match against {
                Hand::Rock => Hand::Scissors,
                Hand::Paper => Hand::Rock,
                Hand::Scissors => Hand::Paper,
            },
            Outcome::Draw => against.clone(),
            Outcome::Win => match against {
                Hand::Rock => Hand::Paper,
                Hand::Paper => Hand::Scissors,
                Hand::Scissors => Hand::Rock,
            },
        }
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissors,
            _ => Err(())?,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => Err(())?,
        })
    }
}

fn calculate_score_part1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| line.split_once(' '))
        .filter_map(|(theirs, ours)| {
            Some((theirs.parse::<Hand>().ok()?, ours.parse::<Hand>().ok()?))
        })
        .map(|(theirs, ours)| ours.play_score(&theirs))
        .sum::<u32>()
}

fn calculate_score_part2(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| line.split_once(' '))
        .filter_map(|(theirs, outcome)| {
            Some((
                theirs.parse::<Hand>().ok()?,
                outcome.parse::<Outcome>().ok()?,
            ))
        })
        .map(|(theirs, outcome)| Hand::for_outcome(outcome, &theirs).play_score(&theirs))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let input = "A Y
B X
C Z";
        assert_eq!(calculate_score_part1(input), 15);
        assert_eq!(calculate_score_part2(input), 12);
    }
}
