use advent2022::Support;
use std::collections::HashSet;
use std::fmt::Debug;
use std::io;

#[derive(Debug, Copy, Clone)]
enum Rock {
    Minus,
    Plus,
    Corner,
    Bar,
    Square,
}

impl Rock {
    fn value(&self) -> HashSet<(usize, usize)> {
        match self {
            Rock::Minus => {
                HashSet::from_iter(vec![(0, 0), (1, 0), (2, 0), (3, 0)].into_iter())
            }
            Rock::Plus => {
                HashSet::from_iter(vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)].into_iter())
            }
            Rock::Corner => {
                HashSet::from_iter(vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)].into_iter())
            }
            Rock::Bar => {
                HashSet::from_iter(vec![(0, 0), (0, 1), (0, 2), (0, 3)].into_iter())
            }
            Rock::Square => {
                HashSet::from_iter(vec![(0, 0), (1, 0), (0, 1), (1, 1)].into_iter())
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Jet {
    Left,
    Right,
}

struct Cave<'a> {
    locations: HashSet<(usize, usize)>,
    xrange: (usize, usize),
    yrange: (usize, usize),
    jetblast: std::iter::Cycle<std::slice::Iter<'a, Jet>>,
}

impl Debug for Cave<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cave")
            .field("locations", &self.locations)
            .field("xrange", &self.xrange)
            .field("yrange", &self.yrange)
            .finish()
    }
}

impl Cave<'_> {
    fn drop(&mut self, rock: &Rock) -> usize {
        let mut coords = (3, self.yrange.1 + 3);
        let val = rock.value();
        let mut gusts = 0;
        //println!("Dropping {:?} at {:?}", rock, coords);
        //self.print(Some((rock.clone(), coords.0, coords.1)), Some(15));
        loop {
            // First blast it with jets.
            let jet = self.jetblast.next().unwrap();
            gusts += 1;
            let propx = match jet {
                Jet::Left => coords.0 - 1,
                Jet::Right => coords.0 + 1,
            };
            let interfered = val
                .iter()
                .filter(|(x, y)| {
                    let newx = propx + x;
                    let newy = coords.1 + y;
                    self.locations.contains(&(newx, newy))
                        || newx < self.xrange.0
                        || newx > self.xrange.1
                })
                .fold(0, |acc, _| acc + 1);
            if interfered == 0 {
                coords.0 = propx;
            }
            // Now drop it.
            let interfered = val
                .iter()
                .filter(|(x, y)| {
                    let newx = coords.0 + x;
                    let newy = coords.1 + y - 1;
                    self.locations.contains(&(newx, newy)) || newy < self.yrange.0
                })
                .fold(0, |acc, _| acc + 1);
            if interfered == 0 {
                coords.1 -= 1;
            } else {
                break;
            }
            //self.print(Some((rock.clone(), coords.0, coords.1)), Some(10));
        }
        // We've come to rest. Add the locations to the cave.
        for (x, y) in val {
            if self.locations.contains(&(coords.0 + x, coords.1 + y)) { 
                println!("Dropping {rock:?} failed"); 
                self.print(None, Some(20)); 
                println!("With {rock:?}"); 
                self.print(Some((*rock, coords.0, coords.1)), Some(20)); 
                panic!("Overlapping locations: {:?} at {:?}", rock, coords);
            }
            self.locations.insert((coords.0 + x, coords.1 + y));
            if coords.1 + y + 1 > self.yrange.1 {
                //println!("Old height: {:?}, adding {rock:?} at {coords:?}, new height: {}", self.yrange.1, coords.1 + y + 1);
                self.yrange.1 = coords.1 + y + 1;
            }
        }
        //self.print(Some((*rock, coords.0, coords.1)), Some(15)); 
        //self.print(None);
        gusts
    }

    fn print(&self, rock: Option<(Rock, usize, usize)>, lines: Option<usize>) {
        let height = if let Some((rock, _, _)) = rock {
            self.yrange.1 + 3 + rock.value().iter().map(|v| v.1).max().unwrap()
        } else {
            self.yrange.1
        };
        for (i, y) in (self.yrange.0..=height).rev().enumerate() {
            print!("{:4}: ", y);
            for x in self.xrange.0..=self.xrange.1 {
                if let Some((rock, rockx, rocky)) = rock {
                    if x >= rockx && y >= rocky && rock.value().contains(&(x - rockx, y - rocky)) {
                        print!("@");
                        continue;
                    }
                }
                if self.locations.contains(&(x, y)) {
                    print!("#");
                    continue;
                }
                print!(".");
            }
            println!("");
            if let Some(lines) = lines {
                if i == lines {
                    break;
                }
            }
        }
        println!("\n");
    }
}

fn main() -> io::Result<()> {
    let mut sup = Support::new()?;

    let xrange = (1, 7);
    let yrange = (1, 1);
    let line = sup.lines.pop_front().unwrap();
    let line = line?;
    let jetblast1: Vec<Jet> = line
        .chars()
        .map(|v| match v {
            '<' => Jet::Left,
            '>' => Jet::Right,
            _ => panic!("Unrecognized jet"),
        })
        .collect();
    let jetblast = jetblast1.iter().cycle();

    let rocks1 = vec![
        Rock::Minus,
        Rock::Plus,
        Rock::Corner,
        Rock::Bar,
        Rock::Square,
    ];

    let rocks = rocks1.iter().cycle();

    let mut cave = Cave {
        locations: HashSet::new(),
        xrange,
        yrange,
        jetblast,
    };

    if sup.args.part_two {
        let gustperiod = line.len();
        let mut heights: Vec<usize> = Vec::new();
        let mut gusts = 0;
        let mut reps = vec![];
        let mut refgust = None;
        for (i, rock) in rocks.enumerate() {
            //if i % 10000 == 1001 {
            //    println!("Dropping {i}th rock, gusts {gusts} period {gustperiod}");
            //}
            heights.push(cave.yrange.1 - cave.yrange.0);
            gusts += cave.drop(rock);
            if i == 1000 {
                refgust = Some(gusts % gustperiod);
            }
            if refgust.is_some() && gusts % gustperiod == refgust.unwrap() {
                //println!("Gusts: {}  Rocks: {}", gusts, i);
                reps.push((gusts, i));
                if reps.len() == 15 {
                    break;
                }
            }
        }
        // Verify that the pattern repeats.
        let comparisonheight = heights[reps[8].1] - heights[reps[3].1];
        //println!("Reps: {:?} Compheight: {}", reps, comparisonheight);

        for i in reps[3].1..reps[8].1 {
            if heights[i+reps[8].1-reps[3].1] - heights[i] != comparisonheight {
                panic!("Height mismatch at {}", i);
            }
        }
        //println!("Height pattern repeats every {} rocks", reps[8].1-reps[3].1);
        //println!("Height pattern repeats every {} gusts", reps[8].0-reps[3].0);

        // So now we can calculate any height.
        // Height is (height of first reps[3].1 rocks) + (comparisonheight * (val - reps[3].1) / (reps[8].1 - reps[3].1)) + (height of last n rocks, where n is the remainder from the division)
        // So we drop rocks for reps[3].1, then for the remainder, then add the repeating height.
        let nrocks:u64 = 10_u64.pow(12);
        
        let firstrocks = reps[3].1;
        let repeatrocks = reps[8].1 - reps[3].1;

        let nonfirstrocks = nrocks - firstrocks as u64;
        let repeatcount = nonfirstrocks / repeatrocks as u64;
        let remainderrocks = nonfirstrocks % repeatrocks as u64;

        let rockstodrop = firstrocks + remainderrocks as usize;
        let jetblast = jetblast1.iter().cycle();
        let rocks = rocks1.iter().cycle();

        let mut secondcave = Cave {
            locations: HashSet::new(),
            xrange,
            yrange,
            jetblast,
        };
        for (i, rock) in rocks.enumerate() {
            if i == rockstodrop {
                break;
            }
            secondcave.drop(rock);
        }
        let mut height = secondcave.yrange.1 - secondcave.yrange.0;

        height += comparisonheight * repeatcount as usize;

        println!("Cave height: {}", height);

    }
    else {
        for (i, rock) in rocks.enumerate() {
            if i == 2022 {
                break;
            }
            cave.drop(rock);
        }

        cave.print(None, None);

        println!("Cave height: {}", cave.yrange.1 - cave.yrange.0);

    }

    Ok(())
}
