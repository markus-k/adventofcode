use take_until::TakeUntilExt;

pub type TreeMap = Vec<Vec<u8>>;

pub fn parse_input(input: &str) -> TreeMap {
    input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .copied()
                .map(|c| c as u8 - b'0')
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn visible_trees(map: &TreeMap) -> usize {
    let mut trees = 0;

    let y_len = map.len();
    let x_len = map[0].len();

    for y in 0..y_len {
        for x in 0..x_len {
            let tree = map[y][x];

            if (0..x).all(|x1| map[y][x1] < tree)
                || ((x + 1)..x_len).all(|x1| map[y][x1] < tree)
                || (0..y).all(|y1| map[y1][x] < tree)
                || ((y + 1)..y_len).all(|y1| map[y1][x] < tree)
            {
                trees += 1;
            }
        }
    }

    trees
}

pub fn best_score(map: &TreeMap) -> usize {
    let y_len = map.len();
    let x_len = map[0].len();

    let mut best_score = 0;

    for y in 0..y_len {
        for x in 0..x_len {
            best_score = scenic_score(x, y, &map).max(best_score);
        }
    }

    best_score
}

fn scenic_score(x: usize, y: usize, map: &TreeMap) -> usize {
    let tree = map[y][x];

    //dbg!(x, y, tree);

    let y_len = map.len();
    let x_len = map[0].len();

    let left_score = (0..x).rev().take_until(|x1| map[y][*x1] >= tree).count();
    let right_score = ((x + 1)..x_len)
        .take_until(|x1| map[y][*x1] >= tree)
        .count();

    let up_score = (0..y).rev().take_until(|y1| map[*y1][x] >= tree).count();
    let down_score = ((y + 1)..y_len)
        .take_until(|y1| map[*y1][x] >= tree)
        .count();

    //dbg!(left_score, right_score, up_score, down_score);

    left_score * right_score * up_score * down_score
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_example_part1() {
        let map = parse_input(EXAMPLE_INPUT);

        assert_eq!(visible_trees(&map), 21);
    }

    #[test]
    fn test_example_part2() {
        let map = parse_input(EXAMPLE_INPUT);

        assert_eq!(best_score(&map), 8);
    }

    #[test]
    fn test_scenic_score() {
        let map = parse_input(EXAMPLE_INPUT);

        assert_eq!(scenic_score(2, 1, &map), 4);
        assert_eq!(scenic_score(2, 3, &map), 8);
    }
}
