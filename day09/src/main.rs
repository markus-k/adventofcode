fn main() {
    let input = include_str!("../input.txt");
    let map = HeightMap::parse_input(input);
    let lowspots = map.find_lowspots();

    let risk_level = map.risk_level(&lowspots);
    println!("RISK LEVEL: {}", risk_level);

    println!("Map:");
    map.display(|_| false);
    println!();

    println!("Lowspots marked:");
    map.display(|p| lowspots.contains(&p));
    println!();

    let basins = map.find_basins(&lowspots);

    println!("Basins marked:");
    for basin in basins.iter() {
        map.display(|p| basin.contains(&p));
        println!();
    }

    println!(
        "Product of largst three basins: {}",
        map.largest_basin_product(&basins)
    );
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

    fn explore_from_lowspot(&self, start: &Point, points: &Vec<Point>) -> Vec<Point> {
        let mut new_points: Vec<Point> = [(-1isize, 0isize), (1, 0), (0, 1), (0, -1)]
            .map(|(dy, dx)| ((start.y as isize + dy), (start.x as isize + dx)))
            .iter()
            .filter(|(ny, nx)| self.is_in_bounds(*nx, *ny))
            .filter_map(|(ny, nx)| {
                let x = *nx as usize;
                let y = *ny as usize;
                let val = self.map[y][x];
                let p = Point::new(x, y);

                if val != 9 && val > self.map[start.y][start.x] && !points.contains(&p) {
                    Some(p)
                } else {
                    None
                }
            })
            .collect();

        let mut recursed_points = new_points
            .iter()
            .flat_map(|point| self.explore_from_lowspot(&point, &new_points))
            .collect::<Vec<Point>>();

        new_points.append(&mut recursed_points);
        new_points.push(start.clone());

        new_points
    }

    fn find_basins(&self, lowspots: &Vec<Point>) -> Vec<Vec<Point>> {
        lowspots
            .iter()
            .map(|lowspot| {
                let points: Vec<Point> = vec![lowspot.clone()];
                let mut new_points = self.explore_from_lowspot(lowspot, &points);

                // this could probably be avoided by being a little smarter up there
                new_points.sort();
                new_points.dedup();

                new_points
            })
            .collect()
    }

    fn risk_level(&self, lowspots: &Vec<Point>) -> u32 {
        lowspots
            .iter()
            .map(|point| self.map[point.y][point.x] + 1)
            .sum()
    }

    fn largest_basin_product(&self, basins: &Vec<Vec<Point>>) -> usize {
        let mut sizes = basins
            .iter()
            .map(|basin| basin.len())
            .collect::<Vec<usize>>();

        sizes.sort_by(|a, b| b.cmp(a));

        sizes[0..3].iter().product()
    }

    fn display<F>(&self, mark: F)
    where
        F: Fn(Point) -> bool,
    {
        for y in 0..self.height {
            for x in 0..self.width {
                if mark(Point::new(x, y)) {
                    print!("*");
                } else {
                    print!("{}", self.map[y][x]);
                }
            }

            println!()
        }
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

        println!("Map:");
        map.display(|_| false);
        println!();

        println!("Lowspots marked:");
        map.display(|p| lowspots.contains(&p));
        println!();

        let basins = map.find_basins(&lowspots);

        println!("Basins marked:");
        for basin in basins.iter() {
            map.display(|p| basin.contains(&p));
            println!();
        }

        println!("{:?}", basins);

        assert_eq!(map.largest_basin_product(&basins), 1134);
    }
}
