pub fn part1(input: &str) -> i64 {
    decrypt(input, 1, 1)
}

pub fn part2(input: &str) -> i64 {
    decrypt(input, 811589153, 10)
}

fn decrypt(input: &str, key: i64, rounds: usize) -> i64 {
    let original_numbers = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut indices = (0..original_numbers.len()).collect::<Vec<_>>();

    for _round in 0..rounds {
        for i in 0..original_numbers.len() {
            let pos = indices.iter().position(|&p| p == i).unwrap();

            let num = original_numbers[i] * key;

            let ir = indices.remove(pos);
            assert_eq!(ir, i);

            let new_index = (pos as i64 + num).rem_euclid(original_numbers.len() as i64 - 1);
            // dbg!(new_index, num);
            indices.insert(new_index as usize, i);

            // dbg!(indices
            //     .iter()
            //     .map(|&i| original_numbers[i])
            //     .collect::<Vec<_>>());
        }
    }

    let numbers = indices
        .iter()
        .map(|&i| original_numbers[i])
        .collect::<Vec<_>>();

    let zero = numbers.iter().position(|&n| n == 0).unwrap();
    let indices = [1000, 2000, 3000];

    indices
        .iter()
        .map(|offset| numbers[(zero + offset) % numbers.len()] * key)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn test_example_input_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 3);
    }

    #[test]
    fn test_example_input_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 1623178306);
    }
}
