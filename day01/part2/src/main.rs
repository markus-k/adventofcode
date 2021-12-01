fn main() {
    let contents = include_str!("../input.txt");
    let increase_counter = count_increases_with_moving_average(contents);

    println!("Depth increased {} times.", increase_counter);
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
        let input = "607
618
618
617
647
716
769
792";
        let count = count_increases_with_moving_average(input);

        assert_eq!(count, 5);
    }
}
