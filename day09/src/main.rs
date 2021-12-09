fn main() {
    let input = include_str!("../input.txt");
    let map = HeightMap::parse_input(input);
    let lowspots = map.find_lowspots();

    let risk_level = map.risk_level(&lowspots);
    println!("RISK LEVEL: {}", risk_level);
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

struct HeightMap {
    map: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl HeightMap {
    fn parse_input(input: &str) -> Self {
        let map = input
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();
        let width = map[0].len();
        let height = map.len();

        Self { map, width, height }
    }

    fn is_in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize
    }

    fn is_lowspot(&self, x: usize, y: usize) -> bool {
        let val = self.map[y][x];

        [(-1isize, 0isize), (1, 0), (0, 1), (0, -1)]
            .map(|(dy, dx)| ((y as isize + dy), (x as isize + dx)))
            .iter()
            .filter(|(ny, nx)| self.is_in_bounds(*nx, *ny))
            .all(|(ny, nx)| val < self.map[*ny as usize][*nx as usize])
    }

    fn find_lowspots(&self) -> Vec<Point> {
        (0..self.height)
            .flat_map(|y| {
                (0..self.width).filter_map(move |x| {
                    if self.is_lowspot(x, y) {
                        Some(Point::new(x, y))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    fn explore_from_lowspot(&self, start: &Point) -> Vec<Point> {
        let points: Vec<Point> = vec![];

        points
    }

    fn find_basins(&self, lowspots: &Vec<Point>) -> Vec<Vec<Point>> {
        vec![]
    }

    fn risk_level(&self, lowspots: &Vec<Point>) -> u32 {
        lowspots
            .iter()
            .map(|point| self.map[point.y][point.x] + 1)
            .sum()
    }
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
        let map = HeightMap::parse_input(input);
        let lowspots = map.find_lowspots();
        let risk_level = map.risk_level(&lowspots);

        assert_eq!(risk_level, 15);
    }
}
