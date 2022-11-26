fn main() {
    let input = include_str!("../input.txt");

    let risk = parse_input(input, false);
    println!("Total risk: {risk}");

    let risk = parse_input(input, true);
    println!("Expanded risk: {risk}");
}

fn neighbors(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .iter()
        .map(move |(dy, dx)| (dy + y as isize, dx + x as isize))
        .filter(move |(y, x)| *x >= 0 && *y >= 0 && *x < width as isize && *y < height as isize)
        .map(|(y, x)| (y as usize, x as usize))
}

fn parse_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<_>>>()
}

fn expanded_risk(risk: u32, level: usize) -> u32 {
    (risk + level as u32 - 1) % 9 + 1
}

fn expand_grid(grid: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    const FACTOR: usize = 5;
    let new_grid_row: Vec<Vec<u32>> = grid
        .into_iter()
        .map(|row| {
            (0..FACTOR)
                .map(move |ex| row.clone().into_iter().map(move |r| expanded_risk(r, ex)))
                .flatten()
                .collect::<Vec<_>>()
        })
        .collect();

    let new_grid: Vec<Vec<u32>> = (0..FACTOR)
        .map(|ey| {
            new_grid_row
                .iter()
                .map(move |row| {
                    row.into_iter()
                        .map(move |r| expanded_risk(*r, ey))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<Vec<_>>>()
        })
        .flatten()
        .collect();

    new_grid
}

fn parse_input(input: &str, expand: bool) -> u32 {
    let mut grid = parse_grid(input);

    if expand {
        grid = expand_grid(grid)
    }

    let width = grid[0].len();
    let height = grid.len();

    for y in 0..height {
        for x in 0..width {
            print!("{}", grid[y][x])
        }

        println!();
    }

    let mut distances = vec![vec![u32::MAX; width]; height];
    let mut prev: Vec<Vec<Option<(usize, usize)>>> = vec![vec![None; width]; height];

    distances[0][0] = 0;

    // Q
    let mut list: Vec<(usize, usize)> = (0..width)
        .map(|x| (0..height).map(move |y| (y, x)))
        .flatten()
        .collect();

    while list.len() > 0 {
        let (u_index, u) = list
            .iter()
            .enumerate()
            .min_by_key(|(_, (y, x))| distances[*y][*x])
            .map(|(i, u)| (i, u.clone()))
            .unwrap();

        list.remove(u_index);

        for (y, x) in neighbors(u.1, u.0, width, height).filter(|v| list.contains(v)) {
            let alt = distances[u.0][u.1] + grid[y][x];

            if alt < distances[y][x] {
                distances[y][x] = alt;
                prev[y][x] = Some(u);
            }
        }
    }

    let mut seq = Vec::new();
    let mut u = Some((height - 1, width - 1));
    if prev[u.unwrap().0][u.unwrap().1].is_some() {
        while let Some((u_y, u_x)) = u {
            seq.insert(0, (u_y, u_x));
            u = prev[u_y][u_x];
        }
    }

    dbg!(&seq);

    let risk: u32 = seq.iter().skip(1).map(|(y, x)| grid[*y][*x]).sum();
    dbg!(risk);

    for y in 0..height {
        for x in 0..width {
            if seq.contains(&(y, x)) {
                print!("X");
            } else {
                print!("O");
            }
        }

        println!();
    }

    return risk;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

        let risk = parse_input(input, false);
        assert_eq!(risk, 40);

        let risk = parse_input(input, true);
        assert_eq!(risk, 315);
    }
}
