use std::fmt::Display;

fn main() {
    let input = include_str!("../input.txt");
    let mut grid = OctopusGrid::from(input);

    let flashes = grid.run_steps(100);
    println!("Flashes after 100 steps: {}", flashes);

    let mut grid = OctopusGrid::from(input);
    let steps = grid.run_til_the_supernova_happens();
    println!("Supernova after {} steps!", steps);
}

struct OctopusGrid {
    grid: Vec<Vec<u8>>,
}

const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 10;

impl From<&str> for OctopusGrid {
    fn from(input: &str) -> Self {
        Self {
            grid: input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as u8)
                        .collect()
                })
                .collect(),
        }
    }
}

impl Display for OctopusGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            for octopus in row.iter() {
                write!(f, "{:>2}", octopus)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl OctopusGrid {
    fn offset(&self, x: usize, y: usize, dx: isize, dy: isize) -> Option<(usize, usize)> {
        let nx = (x as isize) + dx;
        let ny = (y as isize) + dy;

        if nx >= 0 && nx < GRID_WIDTH as isize && ny >= 0 && ny < GRID_HEIGHT as isize {
            Some((nx as usize, ny as usize))
        } else {
            None
        }
    }

    fn flash(&mut self, x: usize, y: usize, exclude: &[(usize, usize)]) {
        for dy in [-1, 0, 1] {
            for dx in [-1, 0, 1] {
                if let Some((x, y)) = self.offset(x, y, dx, dy) {
                    if !exclude.contains(&(x, y)) {
                        self.grid[y][x] += 1;
                    }
                }
            }
        }
    }

    fn find_flashers(&mut self, flashed: &mut Vec<(usize, usize)>) -> usize {
        // we have to collect all flashers first
        // thanks borrow-checker :)
        let mut flashers: Vec<(usize, usize)> = vec![];

        for (y, row) in self.grid.iter().enumerate() {
            for (x, &octopus) in row.iter().enumerate() {
                if octopus > 9 {
                    flashers.push((x, y));
                }
            }
        }

        for &(x, y) in flashers.iter() {
            if !flashed.contains(&(x, y)) {
                self.flash(x, y, flashed);
                self.grid[y][x] = 0;
            }
        }

        let flashes = flashers.len();

        flashed.append(&mut flashers);

        flashes
    }

    fn step(&mut self) -> usize {
        for row in self.grid.iter_mut() {
            for octopus in row.iter_mut() {
                *octopus += 1;
            }
        }

        let mut total_flashes = 0;
        let mut flashers: Vec<(usize, usize)> = vec![];
        loop {
            let flashes = self.find_flashers(&mut flashers);
            total_flashes += flashes;
            println!("{:?}", flashers);

            for &(x, y) in flashers.iter() {
                self.grid[y][x] = 0;
            }
            if flashes == 0 {
                break;
            }
        }

        total_flashes
    }

    fn run_steps(&mut self, steps: usize) -> usize {
        let mut total = 0;
        for i in 0..steps {
            total += self.step();

            println!("After step {} ({} flashes):", i + 1, total);
            println!("{}", self);
        }

        total
    }

    fn run_til_the_supernova_happens(&mut self) -> usize {
        let mut steps = 1;
        while self.step() != 100 {
            steps += 1;
        }

        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let mut grid = OctopusGrid::from(input);
        println!("{}", grid);

        let flashes = grid.run_steps(10);

        assert_eq!(flashes, 204);

        let flashes = flashes + grid.run_steps(90);
        assert_eq!(flashes, 1656);

        let mut grid = OctopusGrid::from(input);
        assert_eq!(grid.run_til_the_supernova_happens(), 195);
    }
}
