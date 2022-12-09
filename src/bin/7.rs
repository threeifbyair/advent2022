use advent2022::Support;
use std::io;
use std::collections::hash_map::HashMap;

#[derive(Debug, Default)]
struct DirTree
{
    arena: Vec<Directory>,
}

#[derive(Debug)]
struct Directory
{
    files: HashMap<String, u32>,
    size: u32,
    parent: Option<usize>,
    subdirs: HashMap<String, usize>,
}


impl DirTree {
    fn add_file(self: &mut Self, dir: usize, name: &str, filesize: u32) {
        let mut mydir = dir;
        self.arena[mydir].files.insert(String::from(name), filesize);
        self.arena[mydir].size += filesize;
        while self.arena[mydir].parent.is_some()
        {
            mydir = self.arena[mydir].parent.unwrap();
            self.arena[mydir].size += filesize;
        }
    }
    
    fn add_subdir(self: &mut Self, dir: usize, name: &str) {
        let dirlen = self.arena.len();
        let newdir = Directory {
            files: HashMap::new(), 
            size: 0,
            parent: Some(dir),
            subdirs: HashMap::new(),
        };
        self.arena.push(newdir);
        self.arena[dir].subdirs.insert(String::from(name), dirlen);
    }
    
    fn find_subdir(self: &Self, dir: usize, name: &str) -> Option<usize> {
        let dir = &self.arena[dir];
        if dir.subdirs.contains_key(name) {
            Some(dir.subdirs[name])
        }
        else {
            None
        }
    }
    
    fn find_parent(self: &Self, dir: usize) -> Option<usize> {
        let dir = &self.arena[dir];
        dir.parent
    }
    
    fn find_upto_size(self: &Self, dir: usize, dirsize: u32) -> u32 {
        let dir = &self.arena[dir];
        let totalsize = dir.subdirs.iter().map(|kv| self.find_upto_size(*kv.1, dirsize)).sum();
        if dir.size <= dirsize {
            totalsize + dir.size
        }
        else {
            totalsize
        }
    }
}

fn main() -> io::Result <()> {    
    let sup = Support::new()?;

    let root = Directory {
        files: HashMap::new(), 
        subdirs: HashMap::new(), 
        parent: None, 
        size: 0
    };
    let mut tree = DirTree { arena: vec![root] };
    let mut curdir: usize = 0;

    for line in sup.lines {
        let line = line?;
        if &line[0..=0] == "$" {
            // This is a command.
            if line.len() >= 5 && &line[0..=4] == "$ cd " {
                let newdir = &line[5..];
                if newdir == ".." {
                    curdir = tree.find_parent(curdir).unwrap();
                }
                else if newdir == "/" {
                    curdir = 0;
                }
                else {
                    curdir = tree.find_subdir(curdir, newdir).unwrap();
                }
            }
        }
        else
        {
            // This is more of the current directory.
            let splitline:Vec<&str> = line.split(' ').collect();
            if splitline[0] == "dir" {
                tree.add_subdir(curdir, splitline[1]);
            }
            else
            {
                let filesize: u32 = splitline[0].parse().unwrap();
                tree.add_file(curdir, splitline[1], filesize);
            }
        }
    }
    if sup.args.part_two {
        let free_space = 70000000 - tree.arena[0].size;
        let amount_to_delete = 30000000 - free_space;
        println!("Free space: {}  Amount to delete: {}", free_space, amount_to_delete);
        let mut smallvec = tree.arena.iter().filter_map(|d| if d.size >= amount_to_delete { Some(d.size) } else {None}).collect::<Vec<u32>>();
        smallvec.sort();
        println!("Smallest: {}", smallvec[0]);
    }
    else
    {
        // Now look for directories of <= 100000.
        println!("Sum: {}", tree.find_upto_size(0, 100000));
    }

    Ok(())
}
