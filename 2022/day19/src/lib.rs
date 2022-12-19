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

impl Inventory {}

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

fn do_robot_work(blueprint: &Blueprint, mut inventory: Inventory) -> Inventory {
    inventory.ore += inventory.ore_robots;
    inventory.clay += inventory.clay_robots;
    inventory.obsidian += inventory.obsidian_robots;
    inventory.geodes += inventory.geode_robots;

    inventory
}

fn step(blueprint: &Blueprint, mut inventory: Inventory, minute: usize) -> Vec<Inventory> {
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
        {
            let mut inventory = inventory.clone();
            inventory.ore -= blueprint.obsidian_ore_cost;
            inventory.clay -= blueprint.obsidian_clay_cost;

            inventory = do_robot_work(blueprint, inventory);

            inventory.obsidian_robots += 1;
            possible_next.push(inventory);
        } else {
            if inventory.ore >= blueprint.clay_cost {
                let mut inventory = inventory.clone();
                inventory.ore -= blueprint.clay_cost;

                inventory = do_robot_work(blueprint, inventory);

                inventory.clay_robots += 1;
                possible_next.push(inventory);
            }
            if inventory.ore >= blueprint.ore_cost {
                let mut inventory = inventory.clone();
                inventory.ore -= blueprint.ore_cost;

                inventory = do_robot_work(blueprint, inventory);

                inventory.ore_robots += 1;
                possible_next.push(inventory);
            }
        }
    }

    // don't build anyhting..
    inventory = do_robot_work(blueprint, inventory);
    possible_next.push(inventory);

    possible_next
}

fn simulate(blueprint: &Blueprint) -> u8 {
    let mut inventory = Inventory::default();
    let minutes = 24;

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
            let possible_inventories = step(blueprint, inv, minute + 1);

            for possible_inv in possible_inventories {
                invs.push_back((possible_inv, minute + 1));
            }
        }
    }

    finished_invs.iter().fold(0, |acc, inv| acc.max(inv.geodes))
}

pub fn part1(blueprints: &[Blueprint]) -> usize {
    let mut q_level = 0;

    for (i, blueprint) in blueprints.iter().enumerate() {
        let inv = simulate(&blueprint);
        q_level += dbg!(inv as usize * (i + 1));
    }

    q_level
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
}
