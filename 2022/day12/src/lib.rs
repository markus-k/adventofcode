use std::{collections::VecDeque, str::FromStr};

pub struct Map {
    map: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Map {
    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.map[y][x]
    }

    pub fn start(&self) -> (usize, usize) {
        self.start
    }

    pub fn end(&self) -> (usize, usize) {
        self.end
    }

    pub fn size(&self) -> (usize, usize) {
        let height = self.map.len();
        let width = self.map[0].len();

        (width, height)
    }

    pub fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let (width, height) = self.size();

        offsets.into_iter().filter_map(move |(dx, dy)| {
            if (x == 0 && dx < 0)
                || (y == 0 && dy < 0)
                || (x == width - 1 && dx > 0)
                || (y == height - 1 && dy > 0)
            {
                None
            } else {
                Some((((x as isize + dx) as usize), (y as isize + dy) as usize))
            }
        })
    }

    pub fn neighbors_forward(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.neighbors(x, y).filter(move |(nx, ny)| {
            let prev = self.get(x, y);
            let new = self.get(*nx, *ny);

            new <= prev + 1
        })
    }

    pub fn neighbors_backward(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.neighbors(x, y).filter(move |(nx, ny)| {
            let prev = self.get(x, y);
            let new = self.get(*nx, *ny);

            new + 1 >= prev
        })
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start: Option<(usize, usize)> = None;
        let mut end: Option<(usize, usize)> = None;
        let map = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.as_bytes()
                    .iter()
                    .enumerate()
                    .map(|(x, c)| match c {
                        b'S' => {
                            start = Some((x, y));
                            0
                        }
                        b'E' => {
                            end = Some((x, y));
                            25
                        }
                        c @ b'a'..=b'z' => c - b'a',
                        _ => unreachable!(),
                    })
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<_>>();

        Ok(Self {
            map,
            start: start.ok_or(())?,
            end: end.ok_or(())?,
        })
    }
}

pub fn bfs<SI: Iterator<Item = (usize, usize)>>(
    map: &Map,
    start_iter: SI,
    end: (usize, usize),
) -> usize {
    let (width, height) = map.size();
    let mut prev = vec![vec![None::<(usize, usize)>; width]; height];
    let mut q = VecDeque::<(usize, usize)>::new();
    let mut explored = vec![vec![false; width]; height];

    let mut goal = None::<(usize, usize)>;

    for start in start_iter {
        q.push_back(start);
        explored[start.1][start.0] = true;
    }

    while let Some(v) = q.pop_front() {
        if v == end {
            goal = Some(v);
        }
        for w in map.neighbors_forward(v.0, v.1) {
            if !explored[w.1][w.0] {
                explored[w.1][w.0] = true;
                prev[w.1][w.0] = Some(v);
                q.push_back(w);
            }
        }
    }

    if goal.is_some() {
        let mut pos = goal;
        let mut steps = 0;
        while let Some(p) = pos {
            pos = prev[p.1][p.0];
            steps += 1;
        }

        steps - 1
    } else {
        usize::MAX
    }
}

pub fn parse_input(input: &str) -> Map {
    input.parse().expect("Parsing map failed")
}

pub fn part1(map: &Map) -> usize {
    bfs(map, std::iter::once(map.start()), map.end())
}

pub fn part2(map: &Map) -> usize {
    bfs(
        map,
        map.map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, v)| ((x, y), v)))
            .filter(|((_x, _y), &v)| v == 0)
            .map(|((x, y), _)| (x, y)),
        map.end(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_example_input_part1() {
        let map = parse_input(EXAMPLE_INPUT);
        assert_eq!(part1(&map), 31);
    }

    #[test]
    fn test_example_input_part2() {
        let map = parse_input(EXAMPLE_INPUT);
        assert_eq!(part2(&map), 29);
    }
}
