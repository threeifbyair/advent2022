use advent2022::Support;
use std::io;
use regex::Regex;

fn main() -> io::Result <()> {    
    let sup = Support::new()?;

    let mut stacks: Vec<Vec<char>> = vec![];
    let mut gotblank = false; 
    let movere = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    for line in sup.lines {
        let line = line?;
        if gotblank {
            // Interpret move commands.
            let caps = movere.captures(&line).unwrap();
            let mut amt = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let src = caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
            let dst = caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;
            let mut localstack: Vec<char> = vec![];
            while amt > 0 {
                let c = stacks[src].pop().unwrap();
                localstack.push(c);
                amt -= 1;
            }
            if sup.args.part_two {
                localstack.reverse();
            }
            for c in localstack.iter() {
                stacks[dst].push(*c);
            }
        }
        else 
        {
            if line.len() == 0 {
                // OK. Time to switch to move commands.
                gotblank = true;
                let mut newstacks: Vec<Vec<char>> = vec![];
                for mut s in stacks {
                    s.reverse();
                    newstacks.push(s);
                }
                stacks = newstacks;
            }
            else
            {
                // This will be a representation of the stack contents
                let mut offset = 0;
                let linelen = line.len();
                let linechars: Vec<char> = line.chars().collect();
                while offset < linelen 
                {
                    let c = linechars[offset+1];
                    while stacks.len() <= offset/4 {
                        stacks.push(vec![]);
                    }
                    if c != ' ' {                    
                        stacks[offset/4].push(c);
                    }
                    offset += 4;
                }
            }
        }

    }
    let mut finalstack: Vec<char> = vec![];
    for mut s in stacks {
            finalstack.push(s.pop().unwrap());
    }
    println!("Final stack: {:?}", finalstack);
    Ok(())
}
