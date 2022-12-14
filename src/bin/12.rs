use advent2022::Support;
use std::io;
use std::collections::HashMap;

#[derive(Debug, Default)]
struct Maze {
    locations: HashMap<(usize, usize), u32>,
    dimensions: (usize, usize),
    endpoint: (usize, usize),
    startpoint: (usize, usize),
    paths: HashMap<(usize, usize), u32>,
}

impl Maze {
    fn solve(&mut self) -> Option<(u32, Vec<(usize, usize)>)> { 
        let mut paths = vec![vec![self.startpoint]];
        let mut depth = 1;
        loop {
            let mut newpaths = vec![];
            for path in paths {
                let current = path.last().unwrap();


                
                let mut neighbors = vec![];
                if current.0 > 0 {
                    neighbors.push((current.0 - 1, current.1));
                }
                if current.1 > 0 {
                    neighbors.push((current.0, current.1 - 1));
                }
                if current.0 < self.dimensions.0 - 1{
                    neighbors.push((current.0 + 1, current.1));
                }
                if current.1 < self.dimensions.1 - 1 {
                    neighbors.push((current.0, current.1 + 1));
                }
                for neighbor in neighbors {
                    if self.locations[&neighbor] < self.locations[current] + 2 {
                        if self.paths.get(&neighbor).is_none() {
                            self.paths.insert(neighbor, depth);
                            if neighbor == self.endpoint {
                                return Some((depth, path));
                            }
                            let mut newpath = path.clone();
                            newpath.push(neighbor);
                            newpaths.push(newpath);
                        }
                    }
                }
            }
            paths = newpaths;
            depth += 1;
            if paths.len() == 0 {
                return None;
            }
        }
    }

}

fn main() -> io::Result<()> {
    let sup = Support::new()?;

    let mut locs: HashMap<(usize, usize), u32> = HashMap::new();
    let lines = sup.lines;
    let mut startpoint: Option<(usize, usize)> = None;
    let mut endpoint: Option<(usize, usize)> = None;

    let mut xdimension = 0;
    let ydimension = lines.len();

    for (j, line) in lines.into_iter().enumerate() {
        let line = line?;
        xdimension = line.len();
        for (i,ch) in line.chars().enumerate() {
            let height = { match ch {
                'S' => { 
                    startpoint = Some((i,j));
                    1
                },
                'E' => { 
                    endpoint = Some((i,j));
                    26
                },
                'a'..='z' => (ch as u32) - ('a' as u32) + 1,
                _ => panic!("Unknown character {}", ch),
            }};
            locs.insert((i, j), height);
        }
    }
    let startpoints:Vec<(usize, usize)> = { if sup.args.part_two {
        locs.iter().filter_map(|(k, v)| if *v == 1 { Some(*k) } else { None }).collect()
    } else {
        vec![startpoint.unwrap()]

    }};
    let mut mindepth: Option<u32> = None;
    for startpoint in startpoints {
        let mut maze = Maze {
            locations: locs.clone(),
            dimensions: (xdimension, ydimension),
            startpoint,
            endpoint: endpoint.unwrap(),
            paths: HashMap::new(),
        };

        if let Some((depth, _path)) = maze.solve() {
            if mindepth.is_none() || depth < mindepth.unwrap() {
                mindepth = Some(depth);
            }
        };
    }
    println!("Depth: {}", mindepth.unwrap());

    Ok(())
}
