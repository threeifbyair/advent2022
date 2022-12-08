use advent2022::Support;
use std::io;
use std::collections::hash_set::HashSet;

fn main() -> io::Result <()> {    
    let sup = Support::new()?;

    for line in sup.lines {
        let line = line?;
        let distinct = if sup.args.part_two { 14 } else { 4 };
        let mut window: Vec<char> = vec![];
        for i in 0..line.len() {
            if i < distinct {
                window.push(line[i..=i].chars().next().unwrap());
            }
            else {
                window.remove(0);
                window.push(line[i..=i].chars().next().unwrap());
                let hash: HashSet<&char> = HashSet::from_iter(window.iter());
                if hash.len() == distinct {
                    println!("Marker at position {}", i+1);
                    break;
                }
            }   
        }
    }
    Ok(())
}
