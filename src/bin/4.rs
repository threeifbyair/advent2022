use advent2022::Support;
use std::io;

fn main() -> io::Result<()> {
    let sup = Support::new()?;

    let mut score: u32 = 0;
    for line in sup.lines {
        let line = line?;
        // Split the line by commas.
        let commas = line.split(',');
        let mut limits: Vec<Vec<i32>> = vec![];
        for c in commas {
            let lim = c.split('-');
            let mut lvec: Vec<i32> = vec![];
            for l in lim {
                let l: i32 = l.parse().unwrap();
                lvec.push(l);
            }
            limits.push(lvec);
        }
        if (limits[0][0] <= limits[1][0] && limits[0][1] >= limits[1][1])
            || (limits[0][0] >= limits[1][0] && limits[0][1] <= limits[1][1])
            || (sup.args.part_two
                && ((limits[1][0] <= limits[0][0] && limits[1][1] >= limits[0][0])
                    || (limits[1][0] <= limits[0][1] && limits[1][1] >= limits[0][1])))
        {
            score += 1;
        }
    }
    println!("Score: {}", score);
    Ok(())
}
