use advent2022::Support;
use std::io;

fn main() -> io::Result <()> {    
    let sup = Support::new()?;

    for line in sup.lines {
        let line = line?;
        let distinct = if sup.args.part_two { 14 } else { 4 };
        for i in distinct-1..line.len() {
            let mut matchfound = false;
            for j in i+1-distinct..i {
                for k in j+1..=i {
                    if line[j..=j] == line[k..=k]
                    {
                        matchfound = true;
                        break;
                    }
                }
            }
            if !matchfound
            {
                println!("Marker at position {}", i+1);
                break;
            }
        }
    }
    Ok(())
}
