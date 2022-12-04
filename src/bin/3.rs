use advent2022::Support;
use std::io;
use std::collections::HashSet;

fn main() -> io::Result <()> {    
    let sup = Support::new()?;

    let mut score: u32 = 0;
    for line in sup.lines {
        let innerline = line?;
        let linelen = innerline.len();
        let mut dataset = HashSet::new();
        for ch in innerline[0..linelen/2].chars() {
            dataset.insert(ch);
        }
        let mut priochr = '0';
        for ch in innerline[linelen/2..linelen].chars() {
            if dataset.contains(&ch) {
                priochr = ch;
                break;
            }
        }
        let priority = match priochr {
            'a'..='z' => priochr as u32 - 'a' as u32 + 1,
            'A'..='Z' => priochr as u32 - 'A' as u32 + 27,
            _ => panic!(),
        };
        score += priority;
    }
    println!("Score: {}", score);
    Ok(())
}
