use advent2022::Support;
use std::io;

fn main() -> io::Result<()> {
    let sup = Support::new()?;

    let mut score = 0;
    let mut regx = 1;
    let mut targetcycle = 20;
    let mut cycle = 1;
    let mut curvec: Vec<bool> = vec![];
    let mut screen: Vec<Vec<bool>> = vec![];
    for line in sup.lines {
        let line = line?;
        //println!("{}: {} (x={})", cycle, line, regx);
        let spl: Vec<&str> = line.split(' ').collect();
        let opcode = spl[0];
        let (cycles, newregx) = match opcode {
            "addx" => {
                let val: i32 = spl[1].parse().unwrap();
                (2, regx + val)
            }
            "noop" => (1, regx),
            _ => {
                panic!("Unknown opcode: {}", opcode)
            }
        };
        if cycle <= targetcycle && cycle + cycles > targetcycle {
            score += targetcycle * regx;
            //println!("Cycle {}: {} -> {}", cycle, regx, targetcycle * regx);
            targetcycle += 40;
        }
        for _ in 0..cycles {
            let subcycle = (cycle - 1) % 40;
            //println!("{}: {} (x={}) #={}", cycle, subcycle, regx, (regx - subcycle).abs() <= 1);
            curvec.push((regx - subcycle).abs() <= 1);
            if curvec.len() == 40 {
                screen.push(curvec);
                curvec = vec![];
            }
            if cycle == targetcycle {
                score += targetcycle * regx;
                //println!("Cycle {}: {} -> {}", cycle, regx, targetcycle * regx);
                targetcycle += 40;
            }
            cycle += 1;
        }
        regx = newregx;
    }
    if sup.args.part_two {
        for y in 0..screen.len() {
            for x in 0..screen[y].len() {
                if screen[y][x] {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    } else {
        println!("Strength: {}", score);
    }
    Ok(())
}
