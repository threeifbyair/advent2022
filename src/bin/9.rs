use advent2022::Support;
use std::collections::hash_set::HashSet;
use std::io;

fn main() -> io::Result<()> {
    let sup = Support::new()?;

    let ropelen = if sup.args.part_two { 10 } else { 2 };
    let mut rope = vec![(0, 0); ropelen];
    let mut tailrecord: HashSet<(i32, i32)> = HashSet::new();
    tailrecord.insert(rope[ropelen - 1].clone());
    for line in sup.lines {
        let line = line?;
        let spl: Vec<&str> = line.split(' ').collect();
        let dir = spl[0];
        let dist: i32 = spl[1].parse().unwrap();
        for _ in 0..dist {
            match dir {
                "R" => rope[0].0 += 1,
                "L" => rope[0].0 -= 1,
                "U" => rope[0].1 += 1,
                "D" => rope[0].1 -= 1,
                _ => panic!("Unknown direction"),
            }
            for i in 1..ropelen {
                if rope[i - 1].0 == rope[i].0 + 2 {
                    rope[i].0 += 1;
                    rope[i].1 += (rope[i - 1].1 - rope[i].1).signum();
                } else if rope[i - 1].0 == rope[i].0 - 2 {
                    rope[i].0 -= 1;
                    rope[i].1 += (rope[i - 1].1 - rope[i].1).signum();
                } else if rope[i - 1].1 == rope[i].1 + 2 {
                    rope[i].1 += 1;
                    rope[i].0 += (rope[i - 1].0 - rope[i].0).signum();
                } else if rope[i - 1].1 == rope[i].1 - 2 {
                    rope[i].1 -= 1;
                    rope[i].0 += (rope[i - 1].0 - rope[i].0).signum();
                }
            }
            tailrecord.insert(rope[ropelen - 1].clone());
        }
    }
    println!("Count: {}", tailrecord.len());
    Ok(())
}
