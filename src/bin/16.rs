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

#[derive(Debug, Clone, PartialEq, Eq)]
struct LocationPair {
    pair: Vec<Location>,
}

impl Hash for LocationPair {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut selfiter = self.pair.clone();
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
    location: LocationPair,
    openvalves: LocationSet,
    currentflow: usize,
    totalflow: usize,
    history: Vec<(Move, Move)>,
}

impl Set {
    fn apply_move(&mut self, m: Move, which: usize, valves: &HashMap<Location, Valve>) {
        match m {
            Move::Open(loc) => {
                self.openvalves.st.insert(loc);
                self.currentflow += valves.get(&loc).unwrap().rate;
            },
            Move::Move(loc) => {
                self.location.pair[which] = loc;
            },
            Move::Stay => {
            },
        };
        if which == 0 {
            self.history.push((m, Move::Stay));
        }
        else {
            let oldmove = self.history.pop().unwrap();
            self.history.push((oldmove.0, m));
        }

    }

}

fn check_and_add_set(
    sets_done: &mut HashMap<LocationSet, HashMap<LocationPair, usize>>,
    set: &Set,
) -> bool {
    let mut compare_key = LocationPair { pair: set.location.pair.clone() };
    compare_key.pair.sort();
    if sets_done.contains_key(&set.openvalves) {
        let compare_map = &mut sets_done.get_mut(&set.openvalves).unwrap();
        if compare_map.contains_key(&compare_key) {
            let val = compare_map.get_mut(&compare_key).unwrap();
            if *val < set.totalflow {
                *val = set.totalflow;
                //println!("Updating {:?} {:?}", set.openvalves, set.location);
                return true;
            } else {
                //println!("Already done {:?} {:?}", set.openvalves, set.location);
                return false;
            }
        }
        compare_map.insert(compare_key, set.totalflow);
    } else {
        let mut locations: HashMap<LocationPair, usize> = HashMap::new();
        locations.insert(compare_key, set.totalflow);
        sets_done.insert(set.openvalves.clone(), locations);
    }
    //println!("Adding {:?} {:?}", set.openvalves, set.location);
    return true;
}

fn find_moves(set: &Set, valves: &HashMap<Location, Valve>, which: usize) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let valve = valves.get(&set.location.pair[which]).unwrap();
    if !set.openvalves.st.contains(&set.location.pair[which]) && valve.rate > 0 {
        // This valve is closed, so we should open it
        moves.push(Move::Open(set.location.pair[which]));
    }
    moves.push(Move::Stay);
    for dest in &valve.dests {
        moves.push(Move::Move(*dest));
    }
    moves
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
        location: LocationPair { pair: vec![Location { loc: ('A', 'A') } , Location { loc: ('A', 'A') } ] },
        openvalves: LocationSet { st: HashSet::new() },
        currentflow: 0,
        totalflow: 0,
        history: vec![],
    });

    let mut sets_done: HashMap<LocationSet, HashMap<LocationPair, usize>> = HashMap::new();
    let max_flow = valves.iter().fold(0, |acc, (_, v)| acc + v.rate);

    println!("\n\nStarting search with max flow {}", max_flow);

    let minutes = if sup.args.part_two { 26 } else { 30 };

    for i in 1..=minutes {
        println!("Iteration {} with {} sets", i, sets.len());
        let mut newsets: Vec<Set> = Vec::new();
        let best_flow = sets[0].totalflow;
        for set in sets.clone() {
            let mut set = set;
            //println!("\nConsidering set {:?}", set);
            set.totalflow += set.currentflow;
            let moves = find_moves(&set, &valves, 0);
            let elephantmoves = 
            if sup.args.part_two {
                find_moves(&set, &valves, 1)
            }
            else {
                vec![Move::Stay]
            };
            //println!("Moves: {:?}", moves);
            for m in moves.into_iter() {
                for n in elephantmoves.clone() {
                    let mut newset = set.clone();
                    let tm = m.clone();
                    newset.apply_move(tm, 0, &valves);
                    newset.apply_move(n, 1, &valves);
                    if check_and_add_set(&mut sets_done, &newset) && (newset.totalflow + max_flow*(minutes-i)) >= best_flow {
                        newsets.push(newset);
                    }
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
