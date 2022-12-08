use advent2022::Support;
use std::io;
use std::collections::hash_map::HashMap;

struct Directory {
    HashMap<str, u32> files;
    HashMap<str, &mut Directory> subdirs;
    Option<&mut Directory> parent;
    u32 size;
};

impl Directory {
    fn add_file(self: Self, name: str, filesize: u32) {
        files.insert(str, filesize);
        self.size += filesize;
        let mut dir = &mut self;
        while (dir.parent)
        {
            dir = dir.parent.unwrap();
            dir.size += filesize;
        }
    }
    
    fn add_subdir(self: Self, name: str) {
        newdir = Directory(HashMap::new(), HashMap::new(), Some(&self), 0);
    }
    
    fn find_upto_size(self: Self, dirsize: u32) -> u32 {
        u32 totalsize = subdirs.iter().map(|kv| kv.1.find_upto_size(dirsize)).sum();
        if self.size <= dirsize {
            totalsize += self.size;
        }
        totalsize
    }
}

fn main() -> io::Result <()> {    
    let sup = Support::new()?;

    let mut root = Directory(HashMap::new(), HashMap::new(), None, 0);
    let mut curdir = &mut root;

    for line in sup.lines {
        let line = line?;
        if line[0] == '$' {
            // This is a command.
            if line[0..=3] == '$ cd' {
                let newdir = line[4..];
                if newdir == '..' {
                    curdir = curdir.parent;
                }
                else if newdir == '/' {
                    curdir = &mut root;
                }
                else {
                    curdir = curdir.subdirs.get_mut(newdir);
                }
            }
        }
        else
        {
            // This is more of the current directory.
            let splitline = line.split(' ');
            if splitline[0] == "dir" {
                curdir.add_subdir(splitline[1..=1]);
            }
            else
            {
                let filesize: u32 = splitline[0].parse().unwrap();
                curdir.add_file(splitline[1], filesize);
            }
        }
    }
    // Now look for directories of <= 100000.
    println!("Sum: {}", root.find_upto_size(100000));

    Ok(())
}
