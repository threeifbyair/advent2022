use advent2022::Support;
use num::abs;
use regex::Regex;
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
enum Block {
    Sensor((i32, i32)),
    Beacon,
    NonBeacon(bool),
}

#[derive(Debug, Default)]
struct Cave {
    locations: HashMap<(i32, i32), Block>,
    xrange: (i32, i32),
    yrange: (i32, i32),
}

impl Cave {
    fn update_range(&mut self, x: i32, y: i32) {
        if x < self.xrange.0 {
            self.xrange.0 = x;
        }
        if x > self.xrange.1 {
            self.xrange.1 = x;
        }
        if y < self.yrange.0 {
            self.yrange.0 = y;
        }
        if y > self.yrange.1 {
            self.yrange.1 = y;
        }
    }

    fn add_sensor(&mut self, sensor: (i32, i32), beacon: (i32, i32), row: Option<i32>) {
        match self.locations.get(&sensor) {
            Some(Block::Sensor(_)) => panic!("Sensor at sensor location"),
            Some(Block::Beacon) => panic!("Beacon at sensor location"),
            Some(Block::NonBeacon(_)) => (),
            None => (),
        }
        match self.locations.get(&beacon) {
            Some(Block::Sensor(_)) => panic!("Sensor at beacon location"),
            Some(Block::Beacon) => (),
            Some(Block::NonBeacon(true)) => (),
            Some(Block::NonBeacon(false)) => {
                self.print();
                panic!("Non-edge non-beacon at beacon location")
            }
            None => (),
        }
        self.locations.insert(sensor, Block::Sensor(beacon));
        self.locations.insert(beacon, Block::Beacon);
        self.update_range(sensor.0, sensor.1);
        self.update_range(beacon.0, beacon.1);

        let manhattan = abs(sensor.0 - beacon.0) + abs(sensor.1 - beacon.1);
        if row.is_some() && !(sensor.1 - manhattan..=sensor.1 + manhattan).contains(&row.unwrap()) {
            return;
        }
        for y in {
            match row {
                Some(r) => r..=r,
                None => sensor.1 - manhattan..=sensor.1 + manhattan,
            }
        } {
            for x in sensor.0 - manhattan..=sensor.0 + manhattan {
                let edge = manhattan - (abs(x - sensor.0) + abs(y - sensor.1));
                if edge >= 0 {
                    if self.locations.get(&(x, y)).is_none() {
                        self.update_range(x, y);
                        self.locations.insert((x, y), Block::NonBeacon(edge == 0));
                    }
                }
            }
        }
    }

    fn print(&self) {
        for y in self.yrange.0..=self.yrange.1 {
            for x in self.xrange.0..=self.xrange.1 {
                match self.locations.get(&(x, y)) {
                    Some(Block::NonBeacon(true)) => print!("X"),
                    Some(Block::NonBeacon(false)) => print!("#"),
                    Some(Block::Beacon) => print!("B"),
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

    let mut cave = Cave {
        locations: HashMap::new(),
        xrange: (0, 0),
        yrange: (0, 0),
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

        cave.add_sensor((sx, sy), (bx, by), Some(sup.args.argint));
    }

    //cave.print();

    let nonbeacons = cave
        .locations
        .iter()
        .filter(|(k, v)| {
            k.1 == sup.args.argint
                && match v {
                    Block::NonBeacon(_) => true,
                    Block::Sensor(_) => true,
                    _ => false,
                }
        })
        .count();

    println!("Nonbeacons: {}", nonbeacons);

    Ok(())
}
