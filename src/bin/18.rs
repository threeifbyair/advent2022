use advent2022::Support;
use std::io;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct CubeMatrix {
    cubes: HashSet<(isize, isize, isize)>,
    xrange: (isize, isize),
    yrange: (isize, isize),
    zrange: (isize, isize),
}

impl CubeMatrix {
    fn new() -> Self {
        CubeMatrix {
            cubes: HashSet::new(),
            xrange: (0, 0),
            yrange: (0, 0),
            zrange: (0, 0),
        }
    }

    fn add_cube(&mut self, x: isize, y: isize, z: isize) {
        self.cubes.insert((x, y, z));
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
        if z < self.zrange.0 {
            self.zrange.0 = z;
        }
        if z > self.zrange.1 {
            self.zrange.1 = z;
        }
    }

    fn innerblow(&self, x: isize, y: isize, z: isize, cubes_seen: &mut HashSet<(isize, isize, isize)>, blowfn: fn(obsidian: &Self, x:isize, y:isize, z:isize) -> usize) -> usize {
        let mut blowtotal = 0;
        let mut cubes_to_check = vec![(x, y, z)];
        while cubes_to_check.len() > 0 {
            let (x, y, z) = cubes_to_check.pop().unwrap();
            if cubes_seen.contains(&(x, y, z)) {
                continue;
            }
            if self.cubes.contains(&(x, y, z)) {
                cubes_seen.insert((x, y, z));
                continue;
            }
            blowtotal += blowfn(self, x, y, z);
            cubes_seen.insert((x, y, z));
            if x < self.xrange.1 {
                cubes_to_check.push((x + 1, y, z));
            }
            if x > self.xrange.0 {
                cubes_to_check.push((x - 1, y, z));
            }
            if y < self.yrange.1 {
                cubes_to_check.push((x, y + 1, z));
            }
            if y > self.yrange.0 {
                cubes_to_check.push((x, y - 1, z));
            }
            if z < self.zrange.1 {
                cubes_to_check.push((x, y, z + 1));
            }
            if z > self.zrange.0 {
                cubes_to_check.push((x, y, z - 1));
            }
        }
        blowtotal
    }

    fn blow(&self, blowfn: fn(obsidian: &Self, x:isize, y:isize, z:isize) -> usize) -> usize {
        let mut cubes_seen = HashSet::new();
        let mut blowtotal = 0;
        for x in self.xrange.0..=self.xrange.1 {
            for y in self.yrange.0..=self.yrange.1 {
                for z in self.zrange.0..=self.zrange.1 {
                    if x != self.xrange.0 && x != self.xrange.1 && y != self.yrange.0 && y != self.yrange.1 && z != self.zrange.0 && z != self.zrange.1 {
                        continue;
                    }
                    if self.cubes.contains(&(x, y, z)) {
                        cubes_seen.insert((x, y, z));
                        continue;
                    }
                    blowtotal += self.innerblow(x, y, z, &mut cubes_seen, blowfn);
                }
            }
        }
        blowtotal
    }
}

fn main() -> io::Result<()> {
    let sup = Support::new()?;

    let mut obsidian = CubeMatrix::new();
    for line in sup.lines {
        let line = line?;
        let mut parts = line.split(',');
        let x = parts.next().unwrap().parse::<isize>().unwrap();
        let y = parts.next().unwrap().parse::<isize>().unwrap();
        let z = parts.next().unwrap().parse::<isize>().unwrap();
        obsidian.add_cube(x, y, z);
    }

    if sup.args.part_two {
        obsidian.xrange = (obsidian.xrange.0-1, obsidian.xrange.1+1);
        obsidian.yrange = (obsidian.yrange.0-1, obsidian.yrange.1+1);
        obsidian.zrange = (obsidian.zrange.0-1, obsidian.zrange.1+1);

        let blowtotal = obsidian.blow(|obsidian, x, y, z| {
            let mut blowtotal = 0;
            if obsidian.cubes.contains(&(x + 1, y, z)) {
                blowtotal += 1;
            }
            if obsidian.cubes.contains(&(x - 1, y, z)) {
                blowtotal += 1;
            }
            if obsidian.cubes.contains(&(x, y + 1, z)) {
                blowtotal += 1;
            }
            if obsidian.cubes.contains(&(x, y - 1, z)) {
                blowtotal += 1;
            }
            if obsidian.cubes.contains(&(x, y, z + 1)) {
                blowtotal += 1;
            }
            if obsidian.cubes.contains(&(x, y, z - 1)) {
                blowtotal += 1;
            }
            blowtotal
        });
        println!("Exposed faces: {}", blowtotal);
    }
    else {
        let faces = obsidian.cubes.iter().fold(0, |acc, (x, y, z)| {
            let mut faces = 0;
            if !obsidian.cubes.contains(&(x + 1, *y, *z)) {
                faces += 1;
            }
            if !obsidian.cubes.contains(&(x - 1, *y, *z)) {
                faces += 1;
            }
            if !obsidian.cubes.contains(&(*x, y + 1, *z)) {
                faces += 1;
            }
            if !obsidian.cubes.contains(&(*x, y - 1, *z)) {
                faces += 1;
            }
            if !obsidian.cubes.contains(&(*x, *y, z + 1)) {
                faces += 1;
            }
            if !obsidian.cubes.contains(&(*x, *y, z - 1)) {
                faces += 1;
            }
            acc + faces
        });
        println!("Faces: {}", faces);
    }
    Ok(())
}
