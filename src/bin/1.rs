use advent2022::Support;
use std::io;

fn main() -> io::Result <()> {
    let sup = Support::new()?;
    let mut v: Vec<i32> = vec![];
    let mut cur: i32 = 0;
    for line in sup.lines {
        let innerline = line?;
        if innerline.len() == 0 {
            v.push(cur);
            cur = 0;
        }
        else
        {
            let upd: i32 = innerline.parse().unwrap();
            cur += upd;
        }
    }
    if cur > 0 {
        v.push(cur);
    }
    println!("Vec: {:?} Max: {:?}", v, v.iter().max());
    Ok(())
}
