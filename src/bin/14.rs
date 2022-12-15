use advent2022::Support;
use std::io;
use std::collections::HashMap;

#[derive(Debug)]
enum Block {
    Rock,
    Sand,
}

#[derive(Debug, Default)]
struct Cave {
    locations: HashMap<(usize, usize), Block>,
    xrange: (usize, usize),
    yrange: (usize, usize),
}

impl Cave {
    fn drop(&mut self) -> bool {
        let mut sand = (500, 0);
        if self.locations.get(&(sand.0, sand.1)).is_some() {
            return false;
        }
        while sand.0 >= self.xrange.0 && sand.0 <= self.xrange.1 && 
              sand.1 >= self.yrange.0 && sand.1 <= self.yrange.1 {
            //println!("Sand falling at: {:?}", sand);
            if self.locations.get(&(sand.0, sand.1 + 1)).is_none() {
                sand.1 += 1;
            } else if self.locations.get(&(sand.0 - 1, sand.1 + 1)).is_none() {
                sand.0 -= 1;
                sand.1 += 1;
            } else if self.locations.get(&(sand.0 + 1, sand.1 + 1)).is_none() {
                sand.0 += 1;
                sand.1 += 1;
            } else {
                self.locations.insert(sand, Block::Sand);
                //println!("Sand came to rest: {:?}", sand);
                return true;
            }
        }
        //println!("Sand fell out of bounds: {:?}", sand);
        return false;
    }
    fn print(&self) {
        for y in self.yrange.0..=self.yrange.1 {
            for x in self.xrange.0..=self.xrange.1 {
                match self.locations.get(&(x, y)) {
                    Some(Block::Rock) => print!("#"),
                    Some(Block::Sand) => print!("o"),
                    None => print!("."),
                }
            }
            println!();
        }
    }
}

fn main() -> io::Result<()> {
    let sup = Support::new()?;

    let mut walls: HashMap<(usize, usize), Block> = HashMap::new();
    let mut xrange = (500, 500);
    let mut yrange = (0, 0);
    for line in sup.lines {
        let line = line?;
        let parts = line.split(" -> ");
        let mut draw: Vec<(usize, usize)> = vec![];
        for part in parts {
            let mut part = part.split(',');
            let x = part.next().unwrap().parse::<usize>().unwrap();
            let y = part.next().unwrap().parse::<usize>().unwrap();
            if x < xrange.0 {
                xrange.0 = x;
            }
            if x > xrange.1 {
                xrange.1 = x;
            }
            if y < yrange.0 {
                yrange.0 = y;
            }
            if y > yrange.1 {
                yrange.1 = y;
            }
            draw.push((x, y));
        }
        let mut oldxy: Option<(usize, usize)> = None;
        for (x, y) in draw {
            if let Some((oldx, oldy)) = oldxy {
                if oldx == x {
                    for y in if oldy < y { oldy..=y } else { y..=oldy } {
                        walls.insert((x, y), Block::Rock);
                    }
                } else {
                    for x in if oldx < x { oldx..=x } else { x..=oldx }{
                        walls.insert((x, y), Block::Rock);
                    }
                }
            }
            oldxy = Some((x, y));
        }
    }

    if sup.args.part_two {
        println!("Part 2");
        yrange.1 += 2;
        if xrange.0 > (500 - yrange.1) {
            xrange.0 = 500 - yrange.1;
        }
        if xrange.1 < (500 + yrange.1) {
            xrange.1 = 500 + yrange.1;
        }
        for x in xrange.0..=xrange.1 {
            walls.insert((x, yrange.1), Block::Rock);
        }
    }

    let mut cave = Cave {
        locations: walls,
        xrange,
        yrange,
    };

    cave.print();

    let mut drops = 0;
    while cave.drop() {
        drops += 1;
        //cave.print();
    }

    cave.print();

    println!("Drops: {}", drops);

    Ok(())
}
