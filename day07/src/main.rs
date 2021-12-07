fn main() {
    let input = include_str!("../input.txt");
    let (position, fuel) = find_cheapest_position(input, simple_cost_fn);

    println!("Cheapest position: {}", position);
    println!("Total fuel: {}", fuel);
}

fn simple_cost_fn(final_pos: usize, current_pos: usize) -> usize {
    (final_pos as i64 - current_pos as i64).abs() as usize
}

/// Returns (cheapest_position, total_fuel)
fn find_cheapest_position<F>(input: &str, cost_fn: F) -> (usize, usize)
where
    F: Fn(usize, usize) -> usize,
{
    let positions = input
        .split(",")
        .map(|n| n.trim().parse().unwrap())
        .collect::<Vec<usize>>();

    let fuels = (0..positions.len())
        .map(|final_position| {
            positions
                .iter()
                .map(|&position| cost_fn(final_position, position))
                .sum()
        })
        .collect::<Vec<usize>>();

    fuels
        .iter()
        .enumerate()
        .min_by_key(|(_, &val)| val)
        .map(|(idx, &val)| (idx, val)) // this map is quite ugly, not sure how to avoid it
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let (position, fuel) = find_cheapest_position(input, simple_cost_fn);

        assert_eq!(position, 2);
        assert_eq!(fuel, 37);
    }
}
