fn main() {
    let contents = include_str!("../input.txt");
    let increase_counter = count_increases(contents);

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
    }
}
