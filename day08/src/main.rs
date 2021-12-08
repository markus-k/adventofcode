fn main() {
    let input = include_str!("../input.txt");

    let outputs = parse_input(input);
    let count = count_simple_digits(&outputs);

    println!("Digits 1, 4, 7 and 8 in input. {}", count);

    let digits = make_digits(&outputs);
    let sum = digits.iter().sum::<u64>();
    println!("Sum of all numbers: {}", sum);
}

fn segment_to_index(segment: char) -> usize {
    match segment {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        _ => panic!("Invalid segment"),
    }
}

fn index_to_segment(index: usize) -> char {
    match index {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        _ => panic!("Invalid index"),
    }
}

fn guess_number_from_pattern_length(len: usize) -> Option<u8> {
    match len {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        _ => None,
    }
}

fn decode_digits(patterns: &Vec<&str>, outputs: &Vec<&str>) -> Vec<u8> {
    let segment_map: [Vec<char>; 10] = [
        // 0
        vec!['a', 'b', 'c', 'e', 'f', 'g'],
        // 1
        vec!['c', 'f'],
        // 2
        vec!['a', 'c', 'd', 'e', 'g'],
        // 3
        vec!['a', 'c', 'd', 'f', 'g'],
        // 4
        vec!['b', 'c', 'd', 'f'],
        // 5
        vec!['a', 'b', 'd', 'f', 'g'],
        // 6
        vec!['a', 'b', 'd', 'e', 'f', 'g'],
        // 7
        vec!['a', 'c', 'f'],
        // 8
        vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'],
        // 9
        vec!['a', 'b', 'c', 'd', 'f', 'g'],
    ];

    /*
     * for pattern in patterns {
     *   if guess = guess_number(pattern) {
     *     segs = SEGMENT_MAP[guess]
     *
     *     candidates.enumerate().filter(i not in segs.map(segment_to_index)).
     *   }
     * }
     */
    // map of every actual segment, which segment it could be
    // at the start we have no clue at all, so every actual segment could be
    // any segment from the input
    let mut candidates: Vec<Vec<char>> = (0..7)
        .map(|_| vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'])
        .collect();

    //println!("Building map...");
    for pattern in patterns.iter() {
        let pattern = pattern.trim();

        // check every pattern if it's one we can determine from it's length
        // then remove the corresponding actual segments from all the ones
        // that should be off
        //

        //print!("Pattern: {}", pattern);

        if let Some(num) = guess_number_from_pattern_length(pattern.len()) {
            //print!(" (guessed: {})", num);
            let actual_segments = &segment_map[num as usize];

            for &segment in actual_segments {
                candidates[segment_to_index(segment)]
                    .retain(|s| pattern.chars().find(|x| x == s).is_some());
            }
            for i in 0..7 {
                if !actual_segments
                    .iter()
                    .map(|&segment| segment_to_index(segment))
                    .collect::<Vec<usize>>()
                    .contains(&i)
                {
                    candidates[i].retain(|s| !pattern.chars().find(|x| x == s).is_some());
                }
            }
        } else {
            //print!(" (no idea)");
        }

        //println!("");
        //println!("candidates: {:?}", candidates);
    }

    //println!("Guessing:");
    outputs
        .iter()
        .map(|pattern| {
            let pattern = pattern.trim();

            //print!("Pattern: {}, ", pattern);

            if let Some(num) = guess_number_from_pattern_length(pattern.len()) {
                //print!("easy, it's a {}", num);

                num
            } else {
                // remap actual segments
                let mut possible_remaps: Vec<Vec<char>> = Vec::new();

                for c in pattern.chars() {
                    // find all possible actual segments
                    let possible_segments = candidates
                        .iter()
                        .enumerate()
                        .filter(|(_, candidate)| candidate.contains(&c))
                        .map(|(i, _)| index_to_segment(i))
                        .collect::<Vec<char>>();
                    possible_remaps.push(possible_segments);
                }

                //println!("possible remaps: {:?}, ", possible_remaps);

                let possible_numbers = segment_map
                    .iter()
                    .enumerate()
                    .filter(|(_, segments)| {
                        //println!("");
                        //println!("segment: {} {:?}", i, segments);

                        if segments.len() != possible_remaps.len() {
                            return false;
                        }

                        let mut pr_copy = possible_remaps.clone();
                        segments.iter().all(|s| {
                            //println!("all {} in {:?}", s, possible_remaps);

                            if let Some(i) = pr_copy.iter().position(|pr| {
                                //println!("   find {} in {:?}", s, pr);
                                pr.iter().find(|&ps| ps == s).is_some()
                            }) {
                                pr_copy.remove(i);
                                true
                            } else {
                                false
                            }
                        })
                    })
                    .map(|(i, _)| i as u8)
                    .collect::<Vec<u8>>();
                //println!("possible numbers: {:?}", possible_numbers);

                possible_numbers[0]
            }
        })
        .collect()
}

fn parse_line(line: &str) -> (Vec<&str>, Vec<&str>) {
    let (patterns, outputs) = line.split_once("|").unwrap();
    let patterns = patterns.split_whitespace();
    let outputs = outputs.split_whitespace();

    (patterns.collect(), outputs.collect())
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let lines: Vec<(Vec<&str>, Vec<&str>)> =
        input.lines().map(|line: &str| parse_line(line)).collect();

    lines
        .iter()
        .map(|(patterns, outputs)| decode_digits(patterns, outputs))
        .collect()
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

fn make_digits(outputs: &Vec<Vec<u8>>) -> Vec<u64> {
    outputs.iter().map(|digits| digits_to_int(digits)).collect()
}

fn digits_to_int(digits: &Vec<u8>) -> u64 {
    digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &d)| d as u64 * 10_u64.pow(i as u32))
        .sum()
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

        let input2 =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

        let outputs = parse_input(input2);
        assert_eq!(outputs[0], vec![5, 3, 5, 3]);

        let outputs = parse_input(input);
        assert_eq!(outputs[0], vec![8, 3, 9, 4]);

        let count = count_simple_digits(&outputs);
        assert_eq!(count, 26);

        let digits = make_digits(&outputs);
        assert_eq!(digits[0], 8394);
        assert_eq!(digits.iter().sum::<u64>(), 61229);
    }

    #[test]
    fn test_digits_to_int() {
        let input = vec![1, 2, 3, 4];
        assert_eq!(digits_to_int(&input), 1234);

        let input = vec![7, 1, 9, 3, 5];
        assert_eq!(digits_to_int(&input), 71935);
    }
}
