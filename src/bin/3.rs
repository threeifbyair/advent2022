use advent2022::Support;
use std::io;
use std::collections::hash_set::HashSet;

fn main() -> io::Result <()> {    
    let sup = Support::new()?;

    let mut score: u32 = 0;
    let mut nelves: u32 = 0;
    let mut dataset: HashSet<char> = HashSet::new();
    let mut prevdataset: HashSet<char> = HashSet::new();
    let mut prevprevdataset: HashSet<char> = HashSet::new();
    for line in sup.lines {
        let innerline = line?;
        let linelen = innerline.len();
        for ch in innerline[0..linelen/2].chars() {
            dataset.insert(ch);
        }
        let mut priochr = '0';
        for ch in innerline[linelen/2..linelen].chars() {
            if dataset.contains(&ch) {
                priochr = ch;
            }
            dataset.insert(ch);
        }
        if sup.args.part_two {
            priochr = '0';
            nelves += 1;
            if nelves == 3
            {
                nelves = 0;
                let mut isect: HashSet<char> = HashSet::new();
                for ch in prevdataset {
                    if prevprevdataset.contains(&ch) {
                        isect.insert(ch);
                    }
                }
                for ch in dataset {
                    if isect.contains(&ch) {
                        priochr = ch;
                    }
                }
                prevprevdataset = HashSet::new();
                prevdataset = HashSet::new();
            }
            else
            {
                prevprevdataset = prevdataset;
                prevdataset = dataset;
            }
            dataset = HashSet::new();
        }
        
        let priority = match priochr {
            'a'..='z' => priochr as u32 - 'a' as u32 + 1,
            'A'..='Z' => priochr as u32 - 'A' as u32 + 27,
            '0' => 0,
            _ => panic!(),
        };
        score += priority;
          
        
    }
    println!("Score: {}", score);
    Ok(())
}
