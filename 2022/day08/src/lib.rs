use std::ops::Index;

use take_until::TakeUntilExt;

pub struct Map2D<T> {
    map: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Map2D<T> {
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.map.get(y * self.height + x)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl<T, II> FromIterator<II> for Map2D<T>
where
    II: IntoIterator<Item = T>,
{
    fn from_iter<I: IntoIterator<Item = II>>(iter: I) -> Self {
        let mut height = 0;
        let mut iter = iter.into_iter();

        // try to guess the final size we need to allocate by getting the first line
        let map = if let Some(first) = iter.next() {
            height = 1;

            let first = first.into_iter();
            let mut map = Vec::from_iter(first);

            // assume we have a square
            map.reserve(map.len() * (map.len() - 1));

            // this is *a lot* faster than .flatten() for some reason
            for subiter in iter.inspect(|_| height += 1) {
                map.extend(subiter);
            }

            // discard any excess memory
            map.shrink_to_fit();

            map
        } else {
            // iter is empty anyway..
            Vec::new()
        };

        let width = map.len() / height;

        Self { map, width, height }
    }
}

impl<T> Index<(usize, usize)> for Map2D<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.get(index.0, index.1).unwrap()
    }
}

pub type TreeMap = Map2D<u8>;

pub fn parse_input(input: &str) -> TreeMap {
    input
        .lines()
        .map(|line| line.as_bytes().iter().copied().map(|c| c as u8 - b'0'))
        .collect::<Map2D<_>>()
}

pub fn visible_trees(map: &TreeMap) -> usize {
    let mut trees = 0;

    let y_len = map.height();
    let x_len = map.width();

    // this is quite the bad algorithm
    for y in 1..(y_len - 1) {
        for x in 1..(x_len - 1) {
            let tree = map[(x, y)];

            if (0..x).all(|x1| map[(x1, y)] < tree)
                || ((x + 1)..x_len).all(|x1| map[(x1, y)] < tree)
                || (0..y).all(|y1| map[(x, y1)] < tree)
                || ((y + 1)..y_len).all(|y1| map[(x, y1)] < tree)
            {
                trees += 1;
            }
        }
    }

    trees + y_len * 2 + x_len * 2 - 4
}

pub fn best_score(map: &TreeMap) -> usize {
    let y_len = map.height();
    let x_len = map.width();

    let mut best_score = 0;

    for y in 1..(y_len - 1) {
        for x in 1..(x_len - 1) {
            best_score = scenic_score(x, y, &map).max(best_score);
        }
    }

    best_score
}

fn scenic_score(x: usize, y: usize, map: &TreeMap) -> usize {
    let tree = map[(x, y)];

    //dbg!(x, y, tree);

    let y_len = map.height();
    let x_len = map.width();

    let left_score = (0..x).rev().take_until(|x1| map[(*x1, y)] >= tree).count();
    let right_score = ((x + 1)..x_len)
        .take_until(|x1| map[(*x1, y)] >= tree)
        .count();

    let up_score = (0..y).rev().take_until(|y1| map[(x, *y1)] >= tree).count();
    let down_score = ((y + 1)..y_len)
        .take_until(|y1| map[(x, *y1)] >= tree)
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

    #[test]
    fn test_parse_input() {
        let map = parse_input(EXAMPLE_INPUT);

        assert_eq!(map.width(), 5);
        assert_eq!(map.height(), 5);
    }
}
