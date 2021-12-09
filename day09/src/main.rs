fn main() {
    let input = include_str!("../input.txt");
    let map = parse_input(input);

    let risk_level = find_lowspots(&map);
    println!("RISK LEVEL: {}", risk_level);
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn map_size(map: &Vec<Vec<u32>>) -> (usize, usize) {
    (map[0].len(), map.len())
}

fn find_lowspots(map: &Vec<Vec<u32>>) -> u32 {
    let (width, height) = map_size(map);
    let mut risk_level = 0;

    for y in 0..height {
        for x in 0..width {
            let val = map[y][x];

            let is_lowspot = [(-1isize, 0isize), (1, 0), (0, 1), (0, -1)]
                .map(|(dy, dx)| ((y as isize + dy), (x as isize + dx)))
                .iter()
                .filter(|(ny, nx)| {
                    *nx >= 0 && *nx < width as isize && *ny >= 0 && *ny < height as isize
                })
                .all(|(ny, nx)| val < map[*ny as usize][*nx as usize]);

            if is_lowspot {
                risk_level += val + 1;
            }
        }
    }

    risk_level
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example_input() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
        let map = parse_input(input);
        let risk_level = find_lowspots(&map);

        assert_eq!(risk_level, 15);
    }
}
