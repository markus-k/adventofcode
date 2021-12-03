fn main() {
    let input = include_str!("../input.txt");
    let report = DiagnosticsReport::from(input);

    println!("Power consumption: {}", report.power_consumption());
    println!("Life support rating: {}", report.life_support_rating());
}

fn bools_to_int(bools: &Vec<bool>) -> u64 {
    let value = bools.iter().fold(0, |acc, &b| acc * 2 + b as u64);

    value
}

struct DiagnosticsReport {
    width: usize,
    parsed: Vec<Vec<bool>>,
}

impl From<&str> for DiagnosticsReport {
    fn from(input: &str) -> Self {
        let width = input.lines().next().expect("No first line").len();
        let parsed = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '1' => true,
                        '0' => false,
                        _ => panic!("Illegal character in input."),
                    })
                    .collect()
            })
            .collect();

        Self {
            width: width,
            parsed: parsed,
        }
    }
}

impl DiagnosticsReport {
    fn with_new_data(&self, parsed: Vec<Vec<bool>>) -> Self {
        Self {
            width: self.width,
            parsed,
        }
    }

    fn count_bits(&self) -> Vec<usize> {
        self.parsed
            .iter()
            .fold(vec![0; self.width], |counters: Vec<usize>, line| {
                let new_counters = counters
                    .iter()
                    .zip(line)
                    .map(|(counter, bit)| match bit {
                        true => counter + 1,
                        false => counter + 0,
                    })
                    .collect::<Vec<usize>>();

                new_counters
            })
    }

    fn find_most_common(&self, tie: bool) -> Vec<bool> {
        let counters = self.count_bits();
        let total = self.parsed.len();

        counters
            .iter()
            .map(|c| {
                if total - *c == *c {
                    tie
                } else {
                    total - *c < *c
                }
            })
            .collect()
    }

    fn gamma_rate(&self) -> u64 {
        let most_common = self.find_most_common(false);

        bools_to_int(&most_common)
    }

    fn epsilon_rate(&self) -> u64 {
        let most_common = self.find_most_common(false);
        let inverted = most_common.iter().map(|b| !b).collect::<Vec<bool>>();

        bools_to_int(&inverted)
    }

    fn power_consumption(&self) -> u64 {
        let gamma = self.gamma_rate();
        let epsilon = self.epsilon_rate();

        gamma * epsilon
    }

    fn filter_report(&self, criteria: bool) -> Vec<bool> {
        let mut report = self.with_new_data(self.parsed.to_vec());

        for i in 0..report.width {
            let mut most_common = report.find_most_common(true);
            if !criteria {
                most_common = most_common.iter().map(|a| !a).collect();
            }

            report = report.with_new_data(
                report
                    .parsed
                    .iter()
                    .cloned()
                    .filter(|line| line[i] == most_common[i])
                    .collect(),
            );

            if report.parsed.len() == 1 {
                break;
            }
        }

        report.parsed[0].to_vec()
    }

    fn oxygen_generator_rating(&self) -> u64 {
        let filtered = self.filter_report(true);

        bools_to_int(&filtered)
    }

    fn co2_scrubber_rating(&self) -> u64 {
        let filtered = self.filter_report(false);

        bools_to_int(&filtered)
    }

    fn life_support_rating(&self) -> u64 {
        let o2_generator = self.oxygen_generator_rating();
        let co2_scrubber = self.co2_scrubber_rating();

        o2_generator * co2_scrubber
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        let report = DiagnosticsReport::from(input);

        assert_eq!(report.parsed.len(), 12);
        assert_eq!(report.width, 5);

        // part1
        assert_eq!(report.gamma_rate(), 22);
        assert_eq!(report.epsilon_rate(), 9);
        assert_eq!(report.power_consumption(), 198);

        // part2
        assert_eq!(report.oxygen_generator_rating(), 23);
        assert_eq!(report.co2_scrubber_rating(), 10);
        assert_eq!(report.life_support_rating(), 230);
    }
}
