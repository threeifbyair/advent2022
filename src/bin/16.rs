use advent2022::Support;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::hash::{Hash, Hasher};
use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Location {
    loc: (char, char),
}

impl Debug for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.loc.0, self.loc.1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LocationSet {
    st: HashSet<Location>,
}

impl Hash for LocationSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut selfiter: Vec<(char, char)> = self.st.iter().map(|v| v.loc).collect();
        selfiter.sort();
        for loc in selfiter {
            loc.hash(state);
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Valve {
    rate: usize,
    dests: Vec<Location>,
}

#[derive(Debug, Clone)]
enum Move {
    Open(Location),
    Move(Location),
    Stay,
}

#[derive(Debug, Clone)]
struct Set {
    location: Location,
    openvalves: LocationSet,
    currentflow: usize,
    totalflow: usize,
    history: Vec<Move>,
}

fn check_and_add_set(
    sets_done: &mut HashMap<LocationSet, HashMap<Location, usize>>,
    set: &Set,
) -> bool {
    if sets_done.contains_key(&set.openvalves) {
        let compare_map = &mut sets_done.get_mut(&set.openvalves).unwrap();
        if compare_map.contains_key(&set.location) {
            let val = compare_map.get_mut(&set.location).unwrap();
            if *val < set.totalflow {
                *val = set.totalflow;
                //println!("Updating {:?} {:?}", set.openvalves, set.location);
                return true;
            } else {
                //println!("Already done {:?} {:?}", set.openvalves, set.location);
                return false;
            }
        }
        compare_map.insert(set.location, set.totalflow);
    } else {
        let mut locations: HashMap<Location, usize> = HashMap::new();
        locations.insert(set.location, set.totalflow);
        sets_done.insert(set.openvalves.clone(), locations);
    }
    //println!("Adding {:?} {:?}", set.openvalves, set.location);
    return true;
}

fn main() -> io::Result<()> {
    let sup = Support::new()?;

    println!("Making valves");

    let mut valves: HashMap<Location, Valve> = HashMap::new();
    let valvere =
        Regex::new(r"^Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? (.+)$")
            .unwrap();

    for line in sup.lines {
        let line = line?;
        let caps = valvere.captures(&line).unwrap();
        let valve: Vec<char> = caps.get(1).unwrap().as_str().chars().collect();
        let rate = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let dests: Vec<Location> = caps
            .get(3)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|s| {
                let ch: Vec<char> = s.chars().collect();
                Location { loc: (ch[0], ch[1]) }
            })
            .collect();

        println!(
            "Valve {:?} has flow rate {} and goes to {:?}",
            valve, rate, dests
        );
        valves.insert(Location { loc: (valve[0], valve[1]) }, Valve { rate, dests });
    }

    let mut sets: Vec<Set> = Vec::new();
    sets.push(Set {
        location: Location { loc: ('A', 'A') },
        openvalves: LocationSet { st: HashSet::new() },
        currentflow: 0,
        totalflow: 0,
        history: vec![],
    });

    let mut sets_done: HashMap<LocationSet, HashMap<Location, usize>> = HashMap::new();
    let max_flow = valves.iter().fold(0, |acc, (_, v)| acc + v.rate);

    println!("\n\nStarting search with max flow {}", max_flow);

    for i in 1..=30 {
        println!("Iteration {} with {} sets", i, sets.len());
        let mut newsets: Vec<Set> = Vec::new();
        let best_flow = sets[0].totalflow;
        for set in sets.clone() {
            let mut set = set;
            //println!("\nConsidering set {:?}", set);
            set.totalflow += set.currentflow;
            let valve = valves.get(&set.location).unwrap();
            if !set.openvalves.st.contains(&set.location) && valve.rate > 0 {
                // This valve is closed, so we should open it
                let mut newset = set.clone();
                newset.openvalves.st.insert(set.location);
                newset.history.push(Move::Open(set.location));
                newset.currentflow += valve.rate;
                if check_and_add_set(&mut sets_done, &newset) {
                    //println!("Opening valve {:?}", set.location);
                    newsets.push(newset);
                }
                else {
                    //println!("Not opening valve {:?} because it's already been done", set.location);
                }
            }
            // Now try moving
            let mut newset = set.clone();
            newset.history.push(Move::Stay);
            if (newset.totalflow + max_flow*(30-i)) >= best_flow {
                //println!("Sticking, still OK because we can get to {} at {}", best_flow, newset.totalflow + max_flow*(30-i));                                                                                          
                newsets.push(newset); 
            }
            else {
                //println!("Sticking, but we can't get to {} at {}", best_flow, newset.totalflow + max_flow*(30-i));
            }
            for dest in &valve.dests {
                let mut newset = set.clone();
                newset.location = *dest;
                newset.history.push(Move::Move(*dest));
                if check_and_add_set(&mut sets_done, &newset) {
                    //println!("Moving to {:?}", dest);
                    newsets.push(newset);
                }
                else {
                    //println!("Not moving to {:?} because it's already been done", dest);
                }
            }
        }
        sets = newsets;
        sets.sort_by(|a, b| b.totalflow.cmp(&a.totalflow));
    }                                                                                                                       

    println!("Done with iterations");
    println!("Best: {:?} with flow {}", sets[0], sets[0].totalflow);

    Ok(())
}
