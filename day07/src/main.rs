fn main() {
    let input = include_str!("../input.txt");
    let (position, fuel) = find_cheapest_position(input);

    println!("Cheapest position: {}", position);
    println!("Total fuel: {}", fuel);
}

/// Returns (cheapest_position, total_fuel)
fn find_cheapest_position(input: &str) -> (usize, usize) {
    let positions = input
        .split(",")
        .map(|n| n.trim().parse().unwrap())
        .collect::<Vec<usize>>();
    let mut fuels: Vec<usize> = Vec::new();

    for final_pos in 0..(positions.len()) {
        let mut fuel = 0;
        for &pos in positions.iter() {
            fuel += (final_pos as i64 - pos as i64).abs() as usize;
        }

        fuels.push(fuel);
    }

    println!("{:?}", fuels);

    let fuel = *fuels.iter().min().unwrap();
    let pos = fuels.iter().position(|&x| x == fuel).unwrap();

    (pos, fuel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let (position, fuel) = find_cheapest_position(input);

        assert_eq!(position, 2);
        assert_eq!(fuel, 37);
    }
}
