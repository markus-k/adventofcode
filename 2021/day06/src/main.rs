fn main() {
    let input = include_str!("../input.txt");
    let mut sim = LanternFishSim::from(input);

    let days = 80;
    let days2 = 256;
    for _ in 0..days {
        sim.simulate_step();
    }

    println!("Fishies after {} days: {}", days, sim.count());

    for _ in days..days2 {
        sim.simulate_step();
    }

    println!("Fishies after {} days: {}", days2, sim.count());
}

const MAX_LIFETIME: usize = 9;

#[derive(Debug)]
struct LanternFishSim {
    lifetimes: [u64; MAX_LIFETIME],
}

impl From<&str> for LanternFishSim {
    fn from(input: &str) -> Self {
        let mut lifetimes = [0; MAX_LIFETIME];

        for fish in input.split(",") {
            let lifetime: usize = fish.trim().parse().unwrap();

            lifetimes[lifetime] += 1;
        }

        LanternFishSim { lifetimes }
    }
}

impl LanternFishSim {
    fn simulate_step(&mut self) {
        let mut lifetimes = self.lifetimes.clone();

        lifetimes[8] = self.lifetimes[0];
        lifetimes[7] = self.lifetimes[8];
        lifetimes[6] = self.lifetimes[0] + self.lifetimes[7];
        lifetimes[5] = self.lifetimes[6];
        lifetimes[4] = self.lifetimes[5];
        lifetimes[3] = self.lifetimes[4];
        lifetimes[2] = self.lifetimes[3];
        lifetimes[1] = self.lifetimes[2];
        lifetimes[0] = self.lifetimes[1];

        self.lifetimes = lifetimes;
    }

    fn count(&self) -> u64 {
        self.lifetimes.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let input = "3,4,3,1,2";
        let mut sim = LanternFishSim::from(input);

        for _ in 0..80 {
            sim.simulate_step();
        }

        assert_eq!(sim.count(), 5934);

        for _ in 80..256 {
            sim.simulate_step();
        }

        assert_eq!(sim.count(), 26984457539);
    }
}
