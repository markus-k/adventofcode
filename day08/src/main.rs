fn main() {
    let input = include_str!("../input.txt");

    let outputs = parse_input(input);
    let count = count_simple_digits(&outputs);

    println!("Digits 1, 4, 7 and 8 in input. {}", count);
}

fn decode_digits(patterns: Vec<&str>, outputs: Vec<&str>) -> Vec<u8> {
    outputs
        .iter()
        .map(|output_pattern| match output_pattern.trim().len() {
            2 => 1,
            3 => 7,
            4 => 4,
            7 => 8,
            _ => 99,
        })
        .collect()
}

fn parse_line(line: &str) -> Vec<u8> {
    let (patterns, outputs) = line.split_once("|").unwrap();
    let patterns = patterns.split_whitespace();
    let outputs = outputs.split_whitespace();

    decode_digits(patterns.collect(), outputs.collect())
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line: &str| parse_line(line)).collect()
}

fn count_simple_digits(outputs: &Vec<Vec<u8>>) -> usize {
    outputs
        .iter()
        .flat_map(|output| {
            output
                .iter()
                .filter(|&&digit| digit == 1 || digit == 4 || digit == 7 || digit == 8)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        let outputs = parse_input(input);
        assert_eq!(outputs[0], vec![8, 99, 99, 4]);

        let count = count_simple_digits(&outputs);
        assert_eq!(count, 26);
    }
}
