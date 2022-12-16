#![warn(rust_2018_idioms)]

use std::{
    collections::{HashMap, HashSet, VecDeque},
    str,
};

#[derive(Debug)]
pub struct Valve<'a> {
    name: &'a str,
    flow_rate: u32,
    leads_to: Vec<&'a str>,
}

pub fn parse_input(input: &str) -> HashMap<&str, Valve<'_>> {
    input
        .lines()
        .map(|line| {
            let (name, rem) = line
                .strip_prefix("Valve ")
                .unwrap()
                .split_once(" has flow rate=")
                .unwrap();
            let (flow_rate, rem) = rem
                .split_once("; tunnel leads to valve ")
                .or_else(|| rem.split_once("; tunnels lead to valves "))
                .unwrap();
            let flow_rate = flow_rate.parse().unwrap();
            let leads_to = rem.split(", ").collect();

            (
                name,
                Valve {
                    name,
                    flow_rate,
                    leads_to,
                },
            )
        })
        .collect()
}

fn explore_tunnel<'a>(
    valves: &HashMap<&'a str, Valve<'a>>,
    paths: &HashMap<(&str, &str), u32>,
    valve_name: &'a str,
    mut opened: HashSet<&'a str>,
    mut time_left: u32,
    mut pressure_release: u32,
) -> u32 {
    if time_left == 0 {
        return pressure_release;
    }

    //dbg!(&valve_name, time_left, pressure_release);

    let valve = &valves[valve_name];
    if valve.flow_rate > 0 && !opened.contains(valve.name) {
        opened.insert(valve.name);

        // spend one minute opening the valve
        time_left -= 1;

        pressure_release += time_left * valve.flow_rate;
    }

    if time_left >= 1 {
        let mut max = pressure_release;

        for unopened_valve in valves
            .values()
            .filter(|&valve| valve.flow_rate > 0 && !opened.contains(valve.name))
        {
            let steps_to_valve = paths[&(valve_name, unopened_valve.name)];

            if time_left > steps_to_valve {
                let path_release = explore_tunnel(
                    valves,
                    paths,
                    unopened_valve.name,
                    opened.clone(),
                    time_left - steps_to_valve,
                    pressure_release,
                );
                max = max.max(path_release);
            }
        }

        max
    } else {
        pressure_release
    }
}

fn bfs<'a>(valves: &HashMap<&'a str, Valve<'a>>, from: &'a str, to: &'a str) -> u32 {
    let mut q = VecDeque::<&str>::new();
    let mut prev = HashMap::<&str, &str>::new();
    let mut explored = HashSet::<&str>::new();
    q.push_back(from);
    while let Some(v) = q.pop_front() {
        if v == to {
            break;
        }

        for w in &valves[v].leads_to {
            if !explored.contains(w) {
                explored.insert(w);
                prev.insert(w, v);
                q.push_back(w);
            }
        }
    }

    //let mut path = Vec::<usize>::new();
    let mut v = Some(&to);
    let mut steps = 0;
    while let Some(p) = v {
        if *p == from {
            break;
        }

        let prev_v = prev.get(p);

        if prev_v.is_some() {
            //let Some(prev_v) = prev_v {
            /* let i = valves[prev_v]
                .leads_to
                .iter()
                .position(|valve| valve == p)
                .unwrap();

            path.push(i);*/
            steps += 1;
        }

        v = prev_v;
    }

    //path.reverse();

    //path
    steps
}

pub fn part1(valves: &HashMap<&str, Valve<'_>>) -> u32 {
    let opened = HashSet::<&str>::with_capacity(valves.len());
    let start_valve = "AA";

    // create a map with the number of steps required from each valve to another
    let mut steps_to_valve = HashMap::<(&str, &str), u32>::new();

    for valve in valves.values() {
        for valve2 in valves.values().filter(|v| v.name != valve.name) {
            steps_to_valve.insert(
                (valve.name, valve2.name),
                bfs(valves, valve.name, valve2.name),
            );
        }
    }

    explore_tunnel(valves, &steps_to_valve, start_valve, opened, 30, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_example_input_part1() {
        let valves = parse_input(EXAMPLE_INPUT);
        assert_eq!(part1(&valves), 1651);
    }
}
