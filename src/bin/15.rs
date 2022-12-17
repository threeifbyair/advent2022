use advent2022::Support;
use num::abs;
use regex::Regex;
use std::io;

#[derive(Debug, Clone)]
enum Block {
    Sensor(i32),
    Beacon(i32),
    NonBeacon(i32, i32),
}

#[derive(Debug, Clone, Default)]
struct Row {
    sensors: Vec<i32>,
    beacons: Vec<i32>,
    contents: Vec<(i32, i32)>,
}

#[derive(Debug, Default)]
struct Cave {
    xrange: (i32, i32),
    yrange: (i32, i32),
    rows: Vec<Row>,
}

impl Row {
    fn get(&self, x: i32) -> Option<Block> {
        if self.sensors.contains(&x) {
            return Some(Block::Sensor(x));
        }
        if self.beacons.contains(&x) {
            return Some(Block::Beacon(x));
        }
        for contents in self.contents.iter() {
            if x >= contents.0 && x <= contents.1 {
                return Some(Block::NonBeacon(contents.0, contents.1));
            }
            if x < contents.0 {
                return None;
            }
        }
        None
    }

    fn add(&mut self, block: Block) {
        let (low, high) = match block {
            Block::Sensor(x) => {
                self.sensors.push(x);
                self.sensors.sort();
                (x, x)
            },
            Block::Beacon(x) => {
                if !self.beacons.contains(&x) {
                    self.beacons.push(x);
                    self.beacons.sort();
                }
                (x, x)
            },
            Block::NonBeacon(x1, x2) => (x1, x2),
        };
        //println!("low {} high {} existing {:?}", low, high, self.contents);
        for (i, inblock) in self.contents.iter().enumerate() {
            if low > inblock.1 + 1 {
                continue;
            }
            if high < inblock.0 - 1{
                self.contents.insert(i, (low, high));
                //println!("RESULT1: {:?}", self.contents);
                return;
            }
            // OK, we have overlap (including a touch). Merge.
            let mut newlow = low;
            let mut newhigh = high;
            if low > inblock.0 {
                newlow = inblock.0;
            }
            if high < inblock.1 {
                newhigh = inblock.1;
            }
            self.contents.remove(i);
            self.contents.insert(i, (newlow, newhigh));
            // Now, we need to merge with any other blocks that overlap.
            let j = i + 1;
            while j < self.contents.len() {
                let inblock = self.contents[j];
                if newhigh < inblock.0 - 1 {
                    break;
                }
                if newhigh < inblock.1 {
                    newhigh = inblock.1;
                }
                self.contents.remove(j);
            }
            self.contents[i].1 = newhigh;
            //println!("RESULT2: {:?}", self.contents);
            return;
        }
        self.contents.push((low, high));
        //println!("RESULT3: {:?}", self.contents);
    }
}

impl Cave {
    fn update_range(&mut self, x: i32, y: i32) {
        if x < self.xrange.0 {
            self.xrange.0 = x;
        }
        if x > self.xrange.1 {
            self.xrange.1 = x;
        }
        //if y < self.yrange.0 {
        //    for _ in y..self.yrange.0 {
        //        self.rows.insert(0, Row { sensors: vec![], beacons: vec![], contents: vec![] });
        //    }
        //    self.yrange.0 = y;
        //}
        if y > self.yrange.1 {
            for _ in self.yrange.1..y {
                self.rows.push(Row { sensors: vec![], beacons: vec![], contents: vec![] });
            }
            self.yrange.1 = y;
        }
    }

    fn add_sensor(&mut self, sensor: (i32, i32), beacon: (i32, i32), row: Option<i32>) {
        self.update_range(sensor.0, sensor.1);
        self.update_range(beacon.0, beacon.1);

        if sensor.1 - self.yrange.0 >= 0 {
            let sensorrow = &mut self.rows[(sensor.1 - self.yrange.0) as usize];
            sensorrow.add(Block::Sensor(sensor.0));
        }
        if beacon.1 - self.yrange.0 >= 0 {
            let beaconrow = &mut self.rows[(beacon.1 - self.yrange.0) as usize];
            beaconrow.add(Block::Beacon(beacon.0));
        }

        let manhattan = abs(sensor.0 - beacon.0) + abs(sensor.1 - beacon.1);
        if row.is_some() && !(sensor.1 - manhattan..=sensor.1 + manhattan).contains(&row.unwrap()) {
            return;
        }
        for y in {
            match row {
                Some(r) => r..=r,
                None => sensor.1 - manhattan..=sensor.1 + manhattan,
            }
        } 
        {
            let ydist = abs(y - sensor.1);   
            let xmaxdist = manhattan - ydist;
            self.update_range(sensor.0 - xmaxdist, y);
            self.update_range(sensor.0 + xmaxdist, y);
            if y - self.yrange.0 >= 0 {
                self.rows[(y - self.yrange.0) as usize].add(Block::NonBeacon(sensor.0 - xmaxdist, sensor.0 + xmaxdist));
            }
        }
    }

    fn print(&self) {
        for y in self.yrange.0..=self.yrange.1 {
            for x in self.xrange.0..=self.xrange.1 {
                match self.rows[(y-self.yrange.0) as usize].get(x) {
                    Some(Block::NonBeacon(_, _)) => print!("#"),
                    Some(Block::Beacon(_)) => print!("B"),
                    Some(Block::Sensor(_)) => print!("S"),
                    None => print!("."),
                }
            }
            println!();
        }
    }
}

fn main() -> io::Result<()> {
    let sup = Support::new()?;

    println!("Making cave");

    let mut cave = Cave {
        rows: vec!( Row{ sensors: vec![], beacons: vec![], contents: vec![] }; (sup.args.argint*2+1).try_into().unwrap() ),
        xrange: (0, 0),
        yrange: (0, sup.args.argint*2),
    };
    let cavere =
        Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$")
            .unwrap();

    for line in sup.lines {
        let line = line?;
        let caps = cavere.captures(&line).unwrap();
        let sx = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let sy = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let bx = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let by = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();

        println!("Adding sensor at ({}, {}), beacon at ({}, {})", sx, sy, bx, by);

        cave.add_sensor((sx, sy), (bx, by), None);

    }

    if sup.args.argint < 100 {
        cave.print();
    }

    if sup.args.part_two {
        println!("Calculating position of beacon");

        for (y, row) in cave.rows.iter().enumerate() {
            for (x1, x2) in row.contents.iter() {
                if *x1 <= 0 && *x2 >= sup.args.argint * 2 {
                    // This is not our line.
                    break;
                }
                if *x2 < 0 {
                    // Not there yet
                    continue;
                }
                if *x1 > sup.args.argint * 2 {
                    // Too far
                    break;
                }
                let beaconx = { if *x1 > 0 { 0 } else { *x2 + 1 } };
                println!("Beacon at ({}, {}), frequency {}", beaconx, y as i32 - cave.yrange.0, (beaconx as u64 * 4000000 as u64) + (y as u64) - cave.yrange.0 as u64);
                return Ok(());
            }
        }

    }
    else {
        println!("Calculating nonbeacons");

        let row = &cave.rows[(sup.args.argint - cave.yrange.0) as usize];

        let nonbeacons = row.contents.iter().fold(0, |acc, (x1, x2)| {
            acc + (x2 - x1 + 1)
        }) - (row.beacons.len() as i32);

        println!("Nonbeacons: {}", nonbeacons);
    }

    Ok(())
}
