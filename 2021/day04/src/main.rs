use std::fmt::{Display, Formatter};

const BINGO_WIDTH: usize = 5;
const BINGO_HEIGHT: usize = 5;

#[derive(Debug)]
enum BingoWin {
    Row(usize),
    Column(usize),
}

#[derive(Clone, Debug)]
struct BingoCard {
    numbers: [[u32; BINGO_WIDTH]; BINGO_HEIGHT],
    marked: [[bool; BINGO_WIDTH]; BINGO_HEIGHT],
}

impl Display for BingoCard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for (row, row_marks) in self.numbers.iter().zip(self.marked.iter()) {
            for (&number, &marked) in row.iter().zip(row_marks.iter()) {
                if marked {
                    write!(f, "[{:>2}]", number)?;
                } else {
                    write!(f, " {:>2} ", number)?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl BingoCard {
    fn from_lines(lines: &Vec<&str>) -> Self {
        let mut numbers = [[0; BINGO_WIDTH]; BINGO_HEIGHT];

        for (row, line) in lines.iter().enumerate() {
            let cols = line.trim().split_whitespace();
            for (col, number) in cols.enumerate() {
                numbers[row][col] = number.parse().expect("Can't parse integer");
            }
        }

        Self {
            numbers,
            marked: [[false; BINGO_WIDTH]; BINGO_HEIGHT],
        }
    }

    fn check_win(&self) -> Option<BingoWin> {
        if let Some(row) = self.check_win_rows() {
            Some(BingoWin::Row(row))
        } else if let Some(col) = self.check_win_cols() {
            Some(BingoWin::Column(col))
        } else {
            None
        }
    }

    fn check_win_rows(&self) -> Option<usize> {
        for (y, row) in self.marked.iter().enumerate() {
            if row.iter().all(|&marked| marked) {
                return Some(y);
            }
        }

        None
    }

    fn check_win_cols(&self) -> Option<usize> {
        for x in 0..BINGO_WIDTH {
            if self.marked.iter().all(|row| row[x]) {
                return Some(x);
            }
        }

        None
    }

    fn mark(&mut self, drawn_number: u32) {
        for (row, row_marks) in self.numbers.iter().zip(self.marked.iter_mut()) {
            for (&number, marked) in row.iter().zip(row_marks.iter_mut()) {
                if number == drawn_number {
                    *marked = true;
                }
            }
        }
    }

    fn numbers_with_mark(&self, mark: bool) -> Vec<u32> {
        let mut numbers = vec![];

        for (row, row_marks) in self.numbers.iter().zip(self.marked.iter()) {
            for (&number, &marked) in row.iter().zip(row_marks.iter()) {
                if marked == mark {
                    numbers.push(number);
                }
            }
        }

        numbers
    }
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<BingoCard>) {
    let mut lines = input.lines();

    let drawn_numbers = lines
        .next()
        .expect("No drawn numbers in input")
        .split(",")
        .map(|n| n.parse().expect("Can't parse drawn number"))
        .collect();

    lines.next(); // skip empty line

    let mut cards: Vec<BingoCard> = vec![];
    let mut card_lines: Vec<&str> = vec![];
    let mut i = 0;

    // this is reaaally ugly...
    loop {
        if let Some(line) = lines.next() {
            if !line.is_empty() {
                card_lines.push(line);
                i += 1;
            }

            if i >= BINGO_HEIGHT {
                let card = BingoCard::from_lines(&card_lines);
                cards.push(card);

                card_lines.clear();
                i = 0;
            }
        } else {
            break;
        }
    }

    (drawn_numbers, cards)
}

fn play(drawn_numbers: &Vec<u32>, cards: &mut Vec<BingoCard>) -> Option<(u32, BingoCard)> {
    for &number in drawn_numbers {
        for card in cards.iter_mut() {
            card.mark(number);

            let win = card.check_win();
            if win.is_some() {
                // clone the card because the borrow checker doesn't like me :(
                return Some((number, card.clone()));
            }
        }
    }

    None
}

fn play_but_let_the_giant_squid_win(
    drawn_numbers: &Vec<u32>,
    cards: &mut Vec<BingoCard>,
) -> Option<(u32, BingoCard)> {
    let mut last_win: Option<(u32, BingoCard)> = None;
    let mut boards_that_won: Vec<bool> = vec![false; cards.len()];

    for &number in drawn_numbers {
        for (has_won, card) in boards_that_won.iter_mut().zip(cards.iter_mut()) {
            card.mark(number);

            let win = card.check_win();
            if win.is_some() && !*has_won {
                *has_won = true;
                // clone the card because the borrow checker doesn't like me :(
                last_win = Some((number, card.clone()));
            }
        }

        if boards_that_won.iter().all(|&w| w) {
            break;
        }
    }

    last_win
}

fn calculate_score(winning_number: u32, winning_card: &BingoCard) -> u32 {
    let unmarked_sum = winning_card.numbers_with_mark(false).iter().sum::<u32>();
    let score = winning_number * unmarked_sum;

    score
}

fn main() {
    let input = include_str!("../input.txt");
    let (drawn_numbers, mut cards) = parse_input(input);

    let (winning_number, winning_card) = play(&drawn_numbers, &mut cards).expect("No winning card");
    println!("Winning card with number {}:", winning_number);
    println!("{}", winning_card);

    let score = calculate_score(winning_number, &winning_card);

    println!("Score: {}", score);

    println!("Okay, now let's let the squid win a couple times...");

    let (winning_number, winning_card) =
        play_but_let_the_giant_squid_win(&drawn_numbers, &mut cards).expect("No winning card");
    println!("Last winning card with number {}:", winning_number);
    println!("{}", winning_card);

    let score = calculate_score(winning_number, &winning_card);

    println!("Score: {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        let (drawn_numbers, mut cards) = parse_input(input);

        // part1
        let (winning_number, winning_card) =
            play(&drawn_numbers, &mut cards).expect("No winning card");

        println!("winning card:");
        println!("{}", winning_card);

        let unmarked_sum = winning_card.numbers_with_mark(false).iter().sum::<u32>();
        let score = calculate_score(winning_number, &winning_card);

        assert_eq!(winning_number, 24);
        assert_eq!(unmarked_sum, 188);
        assert_eq!(score, 4512);

        // part2
        let (winning_number, winning_card) =
            play_but_let_the_giant_squid_win(&drawn_numbers, &mut cards).expect("No winning card");

        println!("winning card:");
        println!("{}", winning_card);

        let unmarked_sum = winning_card.numbers_with_mark(false).iter().sum::<u32>();
        let score = calculate_score(winning_number, &winning_card);

        assert_eq!(winning_number, 13);
        assert_eq!(unmarked_sum, 148);
        assert_eq!(score, 1924);
    }
}
