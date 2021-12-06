fn main() {
    let input = include_str!("../input.txt");
    let mut sim = LanternFishSim::from(input);

    let days = 80;
    for _ in 0..days {
        sim.simulate_step();
    }

    println!("Fishies after {} days: {}", days, sim.count());
}

#[derive(Debug)]
struct LanternFishSim {
    fishies: Vec<usize>,
}

impl From<&str> for LanternFishSim {
    fn from(input: &str) -> Self {
        let fishies = input.split(",").map(|num| num.trim().parse().unwrap()).collect();

        LanternFishSim { fishies }
    }
}

impl LanternFishSim {
    fn simulate_step(&mut self) {
        let mut new_fishies = 0;
        for fish in self.fishies.iter_mut() {
            if *fish == 0 {
                new_fishies += 1;
                *fish = 6;
            } else {
                *fish -= 1;
            }
        }

        let mut new_fishies = vec![8; new_fishies];
        self.fishies.append(&mut new_fishies);
    }

    fn count(&self) -> usize {
        self.fishies.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let input = "3,4,3,1,2";
        let mut sim = LanternFishSim::from(input);
        assert_eq!(sim.fishies, vec![3, 4, 3, 1, 2]);

        let steps = [
            vec![2, 3, 2, 0, 1],
            vec![1, 2, 1, 6, 0, 8],
            vec![0, 1, 0, 5, 6, 7, 8],
            vec![6, 0, 6, 4, 5, 6, 7, 8, 8],
            vec![5, 6, 5, 3, 4, 5, 6, 7, 7, 8],
        ];

        for step in steps.iter() {
            sim.simulate_step();
            assert_eq!(&sim.fishies, step);
        }

        for _ in steps.len()..80 {
            sim.simulate_step();
        }

        assert_eq!(sim.count(), 5934);
    }
}
