fn main() {
    let input = include_str!("../input.txt");
    let map = parse_input(input);
    let lowspots = find_lowspots(&map);

    let risk_level = risk_level(&map, &lowspots);
    println!("RISK LEVEL: {}", risk_level);
}

struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
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

fn find_lowspots(map: &Vec<Vec<u32>>) -> Vec<Point> {
    let (width, height) = map_size(map);

    (0..height)
        .flat_map(|y| {
            (0..width).filter_map(move |x| {
                let val = map[y][x];

                let is_lowspot = [(-1isize, 0isize), (1, 0), (0, 1), (0, -1)]
                    .map(|(dy, dx)| ((y as isize + dy), (x as isize + dx)))
                    .iter()
                    .filter(|(ny, nx)| {
                        *nx >= 0 && *nx < width as isize && *ny >= 0 && *ny < height as isize
                    })
                    .all(|(ny, nx)| val < map[*ny as usize][*nx as usize]);

                if is_lowspot {
                    Some(Point::new(x, y))
                } else {
                    None
                }
            })
        })
        .collect()
}

//fn find_basins(map: &Vec<Vec<u32>>) -> Vec<Vec<(usize, usize)>> {
//}

fn risk_level(map: &Vec<Vec<u32>>, lowspots: &Vec<Point>) -> u32 {
    lowspots.iter().map(|point| map[point.y][point.x] + 1).sum()
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
        let lowspots = find_lowspots(&map);
        let risk_level = risk_level(&map, &lowspots);

        assert_eq!(risk_level, 15);
    }
}
