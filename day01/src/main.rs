fn main() {
    let contents = include_str!("../input.txt");

    let increase_counter = count_increases(contents);
    println!("Depth increased {} times.", increase_counter);

    let increase_counter = count_increases_with_moving_average(contents);
    println!("Depth increased {} times.", increase_counter);
}

fn count_increases(contents: &str) -> usize {
    let mut last_number = 0;
    let mut increase_counter = 0;

    for (i, line) in contents.lines().enumerate() {
        let number: i32 = line.parse().expect("Can't parse line to int");

        if i > 0 && number > last_number {
            increase_counter += 1;
        }

        last_number = number;
    }

    return increase_counter;
}

fn count_increases_with_moving_average(contents: &str) -> usize {
    let numbers = contents
        .lines()
        .map(|line| line.parse().expect("Can't parse line to int"))
        .collect::<Vec<i32>>();

    let windows = numbers
        .windows(3)
        .map(|window| window.iter().sum())
        .collect::<Vec<i32>>();

    windows.windows(2).filter(|x| x[1] > x[0]).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counts_correctly() {
        let input = "199
200
208
210
200
207
240
269
260
263";
        let count = count_increases(input);

        assert_eq!(count, 7);

        let count = count_increases_with_moving_average(input);
        assert_eq!(count, 5);
    }
}
