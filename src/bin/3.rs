use advent2022::Support;
use std::collections::hash_set::HashSet;
use std::io;

fn main() -> io::Result<()> {
    let sup = Support::new()?;

    let mut score: u32 = 0;
    let mut nelves: u32 = 0;
    let mut prevdataset: HashSet<char> = HashSet::new();
    let mut prevprevdataset: HashSet<char> = HashSet::new();
    for line in sup.lines {
        let line = line?;
        let linelen = line.len();
        let priochr: Vec<char> = {
            if sup.args.part_two {
                let dataset: HashSet<char> = HashSet::from_iter(line[0..linelen].chars());
                nelves += 1;
                if nelves == 3 {
                    nelves = 0;
                    let isect: HashSet<char> = HashSet::from_iter(
                        prevdataset
                            .into_iter()
                            .filter(|ch| prevprevdataset.contains(ch)),
                    );
                    let priochr: Vec<char> =
                        Vec::from_iter(dataset.into_iter().filter(|ch| isect.contains(ch)));
                    prevprevdataset = HashSet::new();
                    prevdataset = HashSet::new();
                    priochr
                } else {
                    prevprevdataset = prevdataset;
                    prevdataset = dataset;
                    vec![]
                }
            } else {
                let dataset: HashSet<char> = HashSet::from_iter(line[0..linelen / 2].chars());
                Vec::from_iter(
                    line[linelen / 2..linelen]
                        .chars()
                        .filter(|ch| dataset.contains(ch)),
                )
            }
        };

        let priority = if !priochr.is_empty() {
            match priochr[0] {
                'a'..='z' => priochr[0] as u32 - 'a' as u32 + 1,
                'A'..='Z' => priochr[0] as u32 - 'A' as u32 + 27,
                _ => panic!(),
            }
        } else {
            0
        };
        score += priority;
    }
    println!("Score: {}", score);
    Ok(())
}
