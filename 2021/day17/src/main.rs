use std::ops::RangeInclusive;

fn main() {
    let target = TargetArea {
        x: 135..=155,
        y: -102..=-78,
    };

    let (best_x, best_y, hits) = find_best_xy(&target);
    println!("best_y = {best_y}, x = {best_x}");
    println!("hits: {hits}");

    let (hit, max_y) = simulate(best_x, best_y, &target);
    println!("Hit: {hit}, max y: {max_y}");
}

struct TargetArea {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
}

#[derive(Debug)]
struct Simulator {
    x: i32,
    y: i32,
    v_x: i32,
    v_y: i32,
}

impl Simulator {
    fn new(v_x: i32, v_y: i32) -> Self {
        Self {
            x: 0,
            y: 0,
            v_x,
            v_y,
        }
    }

    fn step(&mut self) {
        self.x += self.v_x;
        self.y += self.v_y;
        if self.v_x > 0 {
            self.v_x -= 1;
        } else if self.v_x < 0 {
            self.v_x += 1;
        }
        self.v_y -= 1;
    }

    fn is_in_area(&self, target: &TargetArea) -> bool {
        target.x.contains(&self.x) && target.y.contains(&self.y)
    }

    fn is_past_area(&self, target: &TargetArea) -> bool {
        self.x > *target.x.end() || self.y < *target.y.start()
    }

    fn has_ended(&self, target: &TargetArea) -> bool {
        self.is_in_area(target) || self.is_past_area(target)
    }
}

fn simulate(v_x: i32, v_y: i32, target: &TargetArea) -> (bool, i32) {
    let mut sim = Simulator::new(v_x, v_y);
    let mut max_y = 0;

    while !sim.has_ended(target) {
        sim.step();
        max_y = max_y.max(sim.y);

        //println!("{:?}", sim);
    }

    (sim.is_in_area(target), max_y)
}

fn find_initial_vx(target_x: i32) -> i32 {
    for v_x in 0.. {
        let x = v_x + (v_x * (v_x - 1)) / 2;

        if x >= target_x {
            return v_x;
        }
    }

    0
}

fn find_best_xy(target: &TargetArea) -> (i32, i32, usize) {
    let mut best_y = 0;
    let mut best_x = 0;
    let mut hits = 0;

    //for x in target.x.clone() {
    //let v_x = find_initial_vx(x);

    for v_x in 0..1000 {
        //println!("v_x = {v_x} for x = {x}");

        for v_y in -1000..1000 {
            let (hit, max_y) = simulate(v_x, v_y, &target);
            if hit {
                println!("{v_x},{v_y}");
                hits += 1;

                if max_y > best_y {
                    best_y = v_y;
                    best_x = v_x;
                }
                //println!("Hit: {hit}, max y: {max_y}");
            }
        }
        //println!("best_y = {best_y}, x = {best_x}");
    }

    (best_x, best_y, hits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highest_y() {
        let target = TargetArea {
            x: 20..=30,
            y: -10..=-5,
        };

        let (best_x, best_y, hits) = find_best_xy(&target);
        println!("best_y = {best_y}, x = {best_x}");
        println!("hits: {hits}");

        let (hit, max_y) = simulate(best_x, best_y, &target);
        println!("Hit: {hit}, max y: {max_y}");

        assert!(hit);
        assert_eq!(max_y, 45);
        assert_eq!(hits, 112);
    }
}
