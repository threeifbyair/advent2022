use advent2022::Support;
use std::io;

fn main() -> io::Result<()> {
    let sup = Support::new()?;
    let mut v: Vec<i32> = vec![];
    let mut cur: i32 = 0;
    for line in sup.lines {
        let line = line?;
        if line.is_empty() {
            v.push(cur);
            cur = 0;
        } else {
            let upd: i32 = line.parse().unwrap();
            cur += upd;
        }
    }
    if cur > 0 {
        v.push(cur);
    }
    v.sort();
    v.reverse();
    if sup.args.part_two {
        println!("Top 3: {}", v[0] + v[1] + v[2]);
    } else {
        println!("Max: {}", v[0]);
    }
    Ok(())
}
