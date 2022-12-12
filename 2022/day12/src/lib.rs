use std::str::FromStr;

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

        offsets
            .into_iter()
            .filter_map(move |(dx, dy)| {
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
            .filter(move |(nx, ny)| {
                let prev = self.get(x, y);
                let new = self.get(*nx, *ny);

                new <= prev + 1
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

fn dijkstra(map: &Map, start: (usize, usize), end: (usize, usize)) -> usize {
    let size = map.size();
    let mut dist = vec![vec![usize::MAX / 2 /* bloody hell */; size.0]; size.1];
    let mut prev = vec![vec![None::<(usize, usize)>; size.0]; size.1];
    let mut q = (0..size.0)
        .flat_map(|x| (0..size.1).map(move |y| (x, y)))
        .collect::<Vec<_>>();

    dist[start.1][start.0] = 0;

    while !q.is_empty() {
        let (i, u) = q
            .iter()
            .enumerate()
            .min_by_key(|(_i, u)| dist[u.1][u.0])
            .map(|(i, u)| (i, u.clone()))
            .unwrap();
        q.remove(i);

        for v in map.neighbors(u.0, u.1).filter(|n| q.contains(n)) {
            let alt = dist[u.1][u.0] + 1;
            if alt < dist[v.1][v.0] {
                dist[v.1][v.0] = alt;
                prev[v.1][v.0] = Some(u.clone());
            }
        }
    }

    let mut s = 0;
    let mut u = Some(end);

    while let Some(up) = u {
        //s.push(up);
        s += 1;
        u = prev.get(up.1).and_then(|v| v.get(up.0)).unwrap().clone();
    }

    s - 1
}

pub fn parse_input(input: &str) -> Map {
    input.parse().expect("Parsing map failed")
}

pub fn part1(map: &Map) -> usize {
    dijkstra(map, map.start(), map.end())
}

pub fn part2(map: &Map) -> usize {
    // this is stupid and i shall be punished (but it works)
    let mut shortest = map
        .map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, v)| ((x, y), v)))
        .filter(|((_x, _y), &v)| v == 0)
        .map(|((x, y), _)| (x, y))
        .map(|(x, y)| dijkstra(map, (x, y), map.end()))
        .collect::<Vec<_>>();
    shortest.sort();

    *shortest.iter().filter(|&s| *s > 0).next().unwrap()
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
