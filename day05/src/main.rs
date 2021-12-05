use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let input = include_str!("../input.txt");
    let field = VentField::from(input);

    let hv_field = field.with_only_ventkinds_vents(&[VentKind::Horzontal, VentKind::Vertical]);
    let hv_map = VentFieldMap::from(&hv_field);
    let hv_overlaps = hv_map.points_with_overlap(2).count();
    println!("Fields on H/V with at least 2 overlaps: {}", hv_overlaps);

    let hvd_field = field.with_only_ventkinds_vents(&[
        VentKind::Horzontal,
        VentKind::Vertical,
        VentKind::Diagonal,
    ]);
    let start = Instant::now();
    let hvd_map = VentFieldMap::from(&hvd_field);
    let duration_map = start.elapsed();

    let hvd_overlaps = hvd_map.points_with_overlap(2).count();
    println!("Fields on H/V/D with at least 2 overlaps: {}", hvd_overlaps);
    println!("(map generation took {:?})", duration_map);

    hvd_map.visualize();
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq)]
enum VentKind {
    Horzontal,
    Vertical,
    Diagonal,
    Other,
}

#[derive(Clone, Debug)]
struct Vent(Point, Point);

impl From<&str> for Vent {
    fn from(input: &str) -> Self {
        lazy_static! {
            // don't compile this regex for every single line
            static ref RE: Regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
        }
        let captures = RE.captures(input).unwrap();

        Self {
            0: Point::new(captures[1].parse().unwrap(), captures[2].parse().unwrap()),
            1: Point::new(captures[3].parse().unwrap(), captures[4].parse().unwrap()),
        }
    }
}

impl Vent {
    fn is_horizontal(&self) -> bool {
        self.0.x == self.1.x
    }

    fn is_vertical(&self) -> bool {
        self.0.y == self.1.y
    }

    fn is_diagonal(&self) -> bool {
        // a 45° degree diagonal vent has the same number of steps in both directions
        let steps_x = (self.0.x - self.1.x).abs();
        let steps_y = (self.0.y - self.1.y).abs();

        steps_x == steps_y
    }

    fn kind(&self) -> VentKind {
        if self.is_horizontal() {
            VentKind::Horzontal
        } else if self.is_vertical() {
            VentKind::Vertical
        } else if self.is_diagonal() {
            VentKind::Diagonal
        } else {
            VentKind::Other
        }
    }

    fn points_covered(&self) -> impl Iterator<Item = Point> {
        if self.kind() == VentKind::Other {
            panic!("Unsupported VentKind");
        }

        let (x0, x1) = (self.0.x, self.1.x);
        let (y0, y1) = (self.0.y, self.1.y);
        let x_steps = (x1 - x0).abs();
        let y_steps = (y1 - y0).abs();

        let steps = x_steps.max(y_steps);

        (0..(steps + 1)).map(move |step| {
            let x = if x_steps == 0 {
                x0
            } else {
                if x1 > x0 {
                    x0 + step
                } else {
                    x0 - step
                }
            };
            let y = if y_steps == 0 {
                y0
            } else {
                if y1 > y0 {
                    y0 + step
                } else {
                    y0 - step
                }
            };

            Point::new(x, y)
        })
    }
}

struct VentField {
    vents: Vec<Vent>,
}

impl From<&str> for VentField {
    fn from(input: &str) -> Self {
        let vents = input.lines().map(|line| Vent::from(line)).collect();

        Self { vents }
    }
}

impl VentField {
    fn with_only_ventkinds_vents(&self, ventkinds: &[VentKind]) -> Self {
        let filtered_vents = self
            .vents
            .iter()
            .filter(|vent| ventkinds.contains(&vent.kind()))
            .cloned()
            .collect();

        Self {
            vents: filtered_vents,
        }
    }
}

struct VentFieldMap(HashMap<Point, usize>);

impl From<&VentField> for VentFieldMap {
    fn from(field: &VentField) -> Self {
        let mut map = HashMap::<Point, usize>::new();

        for vent in field.vents.iter() {
            for point in vent.points_covered() {
                map.entry(point)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }

        Self { 0: map }
    }
}

impl VentFieldMap {
    fn points_with_overlap(&self, overlap: usize) -> impl Iterator<Item = Point> + '_ {
        self.0.iter().filter_map(move |(point, &n)| {
            if n >= overlap {
                Some(point.clone())
            } else {
                None
            }
        })
    }

    fn visualize(&self) {
        let width = self.0.keys().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x as u32;
        let height = self.0.keys().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y as u32;
        let max = *self.0.values().max().unwrap() as f32;

        let img = image::ImageBuffer::from_fn(width, height, |x,y| {
            if let Some(count) = self.0.get(&Point::new(x as i32,y as i32)) {
                image::Luma([(((*count as f32) / max) * 255.0) as u8])
            } else {
                image::Luma([0u8])
            }
        });

        img.save("viz.png");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        let field = VentField::from(input);
        let hv_field = field.with_only_ventkinds_vents(&[VentKind::Horzontal, VentKind::Vertical]);
        let hv_map = VentFieldMap::from(&hv_field);
        assert_eq!(hv_map.points_with_overlap(2).count(), 5);

        let hvd_field = field.with_only_ventkinds_vents(&[
            VentKind::Horzontal,
            VentKind::Vertical,
            VentKind::Diagonal,
        ]);
        let hvd_map = VentFieldMap::from(&hvd_field);
        assert_eq!(hvd_map.points_with_overlap(2).count(), 12);
    }

    #[test]
    fn test_parse_vent() {
        let vent_str = "1234,32 -> 1,15";
        let vent = Vent::from(vent_str);

        assert_eq!(vent.0.x, 1234);
        assert_eq!(vent.0.y, 32);
        assert_eq!(vent.1.x, 1);
        assert_eq!(vent.1.y, 15);
    }

    #[test]
    fn test_kind_and_covered_points() {
        let testcases = vec![
            (
                "1,1 -> 1,3",
                vec![Point::new(1, 1), Point::new(1, 2), Point::new(1, 3)],
                VentKind::Horzontal,
            ),
            (
                "1,3 -> 1,1",
                vec![Point::new(1, 3), Point::new(1, 2), Point::new(1, 1)],
                VentKind::Horzontal,
            ),
            (
                "9,7 -> 7,7",
                vec![Point::new(9, 7), Point::new(8, 7), Point::new(7, 7)],
                VentKind::Vertical,
            ),
            (
                "7,7 -> 9,7",
                vec![Point::new(7, 7), Point::new(8, 7), Point::new(9, 7)],
                VentKind::Vertical,
            ),
            (
                "1,1 -> 3,3",
                vec![Point::new(1, 1), Point::new(2, 2), Point::new(3, 3)],
                VentKind::Diagonal,
            ),
            (
                "9,7 -> 7,9",
                vec![Point::new(9, 7), Point::new(8, 8), Point::new(7, 9)],
                VentKind::Diagonal,
            ),
        ];
        for (vent, points, kind) in testcases {
            let vent = Vent::from(vent);

            assert_eq!(vent.kind(), kind);
            assert_eq!(vent.points_covered().collect::<Vec<Point>>(), points);
        }
    }
}
