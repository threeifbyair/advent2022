use advent2022::Support;
use std::io;

struct Rps {
    val: u8,
    beats: u8,
}

impl Rps {
    pub fn new(val: u8, beats: u8) -> Self {
        Self { val, beats }
    }

    pub fn score(&self, other: &Self) -> u32 {
        let outcome = match other.val {
            _ if other.val == self.val => 3,
            _ if other.val == self.beats => 6,
            _ => 0,
        };
        (self.val + outcome) as u32
    }
}

fn main() -> io::Result<()> {
    let sup = Support::new()?;

    let rock = Rps::new(1, 3);
    let paper = Rps::new(2, 1);
    let scissors = Rps::new(3, 2);

    let mut score: u32 = 0;
    for line in sup.lines {
        let line = line?;
        let outcome = {
            if sup.args.part_two {
                match line {
                    // This time, X means 'you must lose'
                    _ if line == *"A X" => scissors.score(&rock),
                    _ if line == *"B X" => rock.score(&paper),
                    _ if line == *"C X" => paper.score(&scissors),
                    _ if line == *"A Y" => rock.score(&rock),
                    _ if line == *"B Y" => paper.score(&paper),
                    _ if line == *"C Y" => scissors.score(&scissors),
                    _ if line == *"A Z" => paper.score(&rock),
                    _ if line == *"B Z" => scissors.score(&paper),
                    _ if line == *"C Z" => rock.score(&scissors),
                    _ => panic!(),
                }
            } else {
                match line {
                    _ if line == *"A X" => rock.score(&rock),
                    _ if line == *"B X" => rock.score(&paper),
                    _ if line == *"C X" => rock.score(&scissors),
                    _ if line == *"A Y" => paper.score(&rock),
                    _ if line == *"B Y" => paper.score(&paper),
                    _ if line == *"C Y" => paper.score(&scissors),
                    _ if line == *"A Z" => scissors.score(&rock),
                    _ if line == *"B Z" => scissors.score(&paper),
                    _ if line == *"C Z" => scissors.score(&scissors),
                    _ => panic!(),
                }
            }
        };
        score += outcome;
    }
    println!("Score: {}", score);
    Ok(())
}
