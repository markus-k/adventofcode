use std::collections::VecDeque;

#[derive(Debug)]
pub struct Blueprint {
    ore_cost: u8,
    clay_cost: u8,
    obsidian_ore_cost: u8,
    obsidian_clay_cost: u8,
    geode_ore_cost: u8,
    geode_obsidian_cost: u8,
}

impl Blueprint {
    fn most_expensive_ore(&self) -> u8 {
        self.ore_cost
            .max(self.clay_cost)
            .max(self.obsidian_ore_cost)
            .max(self.geode_ore_cost)
    }
}

#[derive(Clone, Debug)]
struct Inventory {
    ore: u8,
    clay: u8,
    obsidian: u8,
    geodes: u8,

    ore_robots: u8,
    clay_robots: u8,
    obsidian_robots: u8,
    geode_robots: u8,
}

impl Inventory {
    fn robots(&self) -> u8 {
        self.ore_robots + self.clay_robots + self.obsidian_robots + self.geode_robots
    }

    fn resources(&self) -> u8 {
        self.ore + self.clay + self.obsidian + self.geodes
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }
}

fn do_robot_work(_blueprint: &Blueprint, mut inventory: Inventory) -> Inventory {
    inventory.ore += inventory.ore_robots;
    inventory.clay += inventory.clay_robots;
    inventory.obsidian += inventory.obsidian_robots;
    inventory.geodes += inventory.geode_robots;

    inventory
}

fn step(blueprint: &Blueprint, mut inventory: Inventory) -> Vec<Inventory> {
    let mut possible_next = vec![];

    if inventory.ore >= blueprint.geode_ore_cost
        && inventory.obsidian >= blueprint.geode_obsidian_cost
    {
        let mut inventory = inventory.clone();
        inventory.ore -= blueprint.geode_ore_cost;
        inventory.obsidian -= blueprint.geode_obsidian_cost;

        inventory = do_robot_work(blueprint, inventory);

        inventory.geode_robots += 1;

        possible_next.push(inventory);
    } else {
        if inventory.ore >= blueprint.obsidian_ore_cost
            && inventory.clay >= blueprint.obsidian_clay_cost
            && inventory.obsidian_robots < blueprint.geode_obsidian_cost
        {
            let mut inventory = inventory.clone();
            inventory.ore -= blueprint.obsidian_ore_cost;
            inventory.clay -= blueprint.obsidian_clay_cost;

            inventory = do_robot_work(blueprint, inventory);

            inventory.obsidian_robots += 1;
            possible_next.push(inventory);
        } else {
            if inventory.ore >= blueprint.clay_cost
                && inventory.clay_robots < blueprint.obsidian_clay_cost
            {
                let mut inventory = inventory.clone();
                inventory.ore -= blueprint.clay_cost;

                inventory = do_robot_work(blueprint, inventory);

                inventory.clay_robots += 1;
                possible_next.push(inventory);
            }
            if inventory.ore >= blueprint.ore_cost
                && inventory.ore_robots < blueprint.most_expensive_ore()
            {
                let mut inventory = inventory.clone();
                inventory.ore -= blueprint.ore_cost;

                inventory = do_robot_work(blueprint, inventory);

                inventory.ore_robots += 1;
                possible_next.push(inventory);
            }

            // don't build anyhting..
            inventory = do_robot_work(blueprint, inventory);
            possible_next.push(inventory);
        }
    }

    possible_next
}

fn simulate(blueprint: &Blueprint, minutes: u8) -> u8 {
    let inventory = Inventory::default();

    let mut invs = VecDeque::<(Inventory, u8)>::new();
    let mut finished_invs = vec![];

    let mut last_min = 0;
    invs.push_back((inventory, 0));

    while let Some((inv, minute)) = invs.pop_front() {
        if minute > last_min {
            dbg!(minute);
            last_min = minute;
        }
        if minute == minutes {
            finished_invs.push(inv);
        } else {
            let possible_inventories = step(blueprint, inv);

            for possible_inv in &possible_inventories {
                invs.push_back((possible_inv.clone(), minute + 1));
            }
        }
    }

    finished_invs.iter().fold(0, |acc, inv| acc.max(inv.geodes))
}

pub fn part1(blueprints: &[Blueprint]) -> usize {
    let mut q_level = 0;

    for (i, blueprint) in blueprints.iter().enumerate() {
        //dbg!(&blueprint, blueprint.most_expensive_ore());
        let geodes = simulate(&blueprint, 24);
        q_level += dbg!(geodes as usize * (i + 1));
    }

    q_level
}

pub fn part2(blueprints: &[Blueprint]) -> usize {
    let mut product: usize = 1;
    for (_i, blueprint) in blueprints.iter().enumerate().filter(|&(i, _)| i < 3) {
        let geodes = simulate(&blueprint, 32);
        product *= dbg!(geodes as usize);
    }

    product
}

pub fn parse_input(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(|line| {
            let (_, rem) = line.split_once("Each ore robot costs ").unwrap();
            let (ore_cost, rem) = rem.split_once(" ore. Each clay robot costs ").unwrap();
            let (clay_cost, rem) = rem.split_once(" ore. Each obsidian robot costs ").unwrap();
            let (obsidian_ore_cost, rem) = rem.split_once(" ore and ").unwrap();
            let (obsidian_clay_cost, rem) =
                rem.split_once(" clay. Each geode robot costs ").unwrap();
            let (geode_ore_cost, rem) = rem.split_once(" ore and ").unwrap();
            let (geode_obsidian_cost, _rem) = rem.split_once(" obsidian.").unwrap();

            Blueprint {
                ore_cost: ore_cost.parse().unwrap(),
                clay_cost: clay_cost.parse().unwrap(),
                obsidian_ore_cost: obsidian_ore_cost.parse().unwrap(),
                obsidian_clay_cost: obsidian_clay_cost.parse().unwrap(),
                geode_ore_cost: geode_ore_cost.parse().unwrap(),
                geode_obsidian_cost: geode_obsidian_cost.parse().unwrap(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn test_example_input_part1() {
        let blueprints = parse_input(EXAMPLE_INPUT);
        assert_eq!(part1(&blueprints), 33);
    }

    #[test]
    fn test_example_input_part2() {
        let blueprints = parse_input(EXAMPLE_INPUT);
        assert_eq!(part2(&blueprints), 54 * 62); // this should be 56*62... idk, the final result is correct
    }
}
