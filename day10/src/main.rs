fn main() {
    let input = include_str!("../input.txt");

    let score = syntax_checker_score(input);

    println!("Syntax checker score: {}", score);
}

#[derive(Debug, PartialEq)]
enum BracketType {
    Round,
    Square,
    Curly,
    Angle,
}

impl BracketType {
    fn value(&self) -> u64 {
        match self {
            BracketType::Round => 3,
            BracketType::Square => 57,
            BracketType::Curly => 1197,
            BracketType::Angle => 25137,
        }
    }
}

#[derive(Debug)]
enum Symbol {
    Opening(BracketType),
    Closing(BracketType),
}

impl From<char> for Symbol {
    fn from(c: char) -> Self {
        match c {
            '(' => Symbol::Opening(BracketType::Round),
            '[' => Symbol::Opening(BracketType::Square),
            '{' => Symbol::Opening(BracketType::Curly),
            '<' => Symbol::Opening(BracketType::Angle),
            ')' => Symbol::Closing(BracketType::Round),
            ']' => Symbol::Closing(BracketType::Square),
            '}' => Symbol::Closing(BracketType::Curly),
            '>' => Symbol::Closing(BracketType::Angle),
            _ => panic!("Invalid symbol."),
        }
    }
}

enum ParserError {
    IllegalClosingBracket(BracketType),
    None,
}

fn parse_line(line: &str) -> ParserError {
    let line = line.trim();
    let mut brackets: Vec<BracketType> = vec![];

    for c in line.chars() {
        let symbol = Symbol::from(c);

        match symbol {
            Symbol::Opening(bracket) => {
                brackets.push(bracket);
            }
            Symbol::Closing(bracket) => {
                let expected = brackets.pop().unwrap();
                if bracket != expected {
                    println!(
                        "Illegal closing character: {:?}, expected: {:?}",
                        bracket, expected
                    );

                    return ParserError::IllegalClosingBracket(bracket);
                }
            }
        };
    }

    return ParserError::None;
}

fn syntax_checker_score(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let err = parse_line(line);
            match err {
                ParserError::IllegalClosingBracket(bracket) => bracket.value(),
                ParserError::None => 0,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example_input() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        let score = syntax_checker_score(input);
        assert_eq!(score, 26397);
    }
}
