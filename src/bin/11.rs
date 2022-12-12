use advent2022::Support;
use std::io;
use std::mem;
use num::Integer;

#[derive(Debug, Default)]
struct Monkeys {
    monkeys: Vec<Monkey>,
    relief: u32,
    lcm: u64,
}

#[derive(Debug)]
enum MonkeyOp {
    Add(u32),
    Multiply(u32),
    Square
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    op: MonkeyOp,
    divisor: u32,
    actions: u32,
    target: Vec<usize>,
}

impl Monkeys {
    fn take_turn(&mut self, monkey: usize) {
        let items = mem::take(&mut self.monkeys[monkey].items);
        //println!("  Monkey {} inspecting {:?}", monkey, items);
        for item in items {
            self.monkeys[monkey].actions += 1;
            let worry = ({
                match self.monkeys[monkey].op {
                    MonkeyOp::Add(x) => item + (x as u64),
                    MonkeyOp::Multiply(x) => item * (x as u64),
                    MonkeyOp::Square => &item * &item,
                }
            } / (self.relief as u64)) % self.lcm;
            let decision = (&worry % (self.monkeys[monkey].divisor as u64)) == 0;
            let target = self.monkeys[monkey].target[decision as usize];
            //println!("    Monkey {} took {} and gave {} to monkey {}", monkey, item, worry, target);
            self.monkeys[target].items.push(worry);
        }
        self.monkeys[monkey].items = vec![];
    }

}

fn main() -> io::Result<()> {
    let sup = Support::new()?;

    let relief = { if sup.args.part_two { 1 } else { 3 } };
    let rounds = { if sup.args.part_two { 10000 } else { 20 } };
    let mut tree = Monkeys { monkeys: vec![], relief, lcm: (relief as u64) };
    let mut lines = sup.lines;
    let mut lcm = vec![relief as u64];
    while lines.len() > 0 {
        let monkeylines: Vec<Result<String, io::Error>> = lines.drain(0..7).collect();
        let monkeyvec: Vec<Vec<&str>> = monkeylines.iter().map(|l| l.as_ref().unwrap().split(' ').collect::<Vec<&str>>().clone()).collect();
        //println!("Monkeyvec {:?}", monkeyvec);
        let items: Vec<u64> = monkeyvec[1][4..].iter().map(|i| i.replace(",", "").parse::<u64>().unwrap()).collect();
        let op = match (monkeyvec[2][6], monkeyvec[2][7])  {
            ("+", addend) => MonkeyOp::Add(addend.parse().unwrap()),
            ("*", "old") => MonkeyOp::Square,
            ("*", multiplier) => MonkeyOp::Multiply(multiplier.parse().unwrap()),
            (_, _) => panic!("Unknown operation"),
        };
        let divisor: u32 = monkeyvec[3][5].parse().unwrap();
        let target: Vec<usize> = vec![monkeyvec[5][9].parse().unwrap(), monkeyvec[4][9].parse().unwrap()];
        let monkey = Monkey {
            items,
            op,
            divisor,
            actions: 0,
            target,
        };
        //println!("Monkey {:?}", monkey);
        lcm.push(monkey.divisor as u64);
        tree.monkeys.push(monkey);

    }
    
    tree.lcm = lcm.iter().fold(1, |acc, x| acc.lcm(x));

    for _round in 1..=rounds {
        //println!("Round {}", round);
        for monkey in 0..tree.monkeys.len() {
            tree.take_turn(monkey);
        }
    }

    let mut inspections: Vec<u32> = tree.monkeys.iter().map(|m| m.actions).collect();
    inspections.sort();
    inspections.reverse();
    println!("Inspections: {:?}", inspections);
    println!("Monkey business: {}", (inspections[0] as u64) * (inspections[1] as u64));


    Ok(())
}
