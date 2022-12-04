use advent2022::Support;
use std::io;

struct RPS {
    val: u8,
    beats: u8,
}

impl RPS {
    pub fn new(val: u8, beats: u8) -> Self {
        Self {
            val,
            beats
        }
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
       
fn main() -> io::Result <()> {
    let sup = Support::new()?;

    let rock = RPS::new(1, 3);
    let paper = RPS::new(2, 1);
    let scissors = RPS::new(3, 2);

    let mut score: u32 = 0;
    for line in sup.lines {
        let innerline = line?;
        let outcome = {
            if sup.args.part_two {
                match innerline {
                    // This time, X means 'you must lose'
                    _ if innerline == String::from("A X") => scissors.score(&rock),
                    _ if innerline == String::from("B X") => rock.score(&paper),
                    _ if innerline == String::from("C X") => paper.score(&scissors),
                    _ if innerline == String::from("A Y") => rock.score(&rock),
                    _ if innerline == String::from("B Y") => paper.score(&paper),
                    _ if innerline == String::from("C Y") => scissors.score(&scissors),
                    _ if innerline == String::from("A Z") => paper.score(&rock),
                    _ if innerline == String::from("B Z") => scissors.score(&paper),
                    _ if innerline == String::from("C Z") => rock.score(&scissors),
                    _ => panic!()
                }
            }
            else
            {
                match innerline {
                    _ if innerline == String::from("A X") => rock.score(&rock),
                    _ if innerline == String::from("B X") => rock.score(&paper),
                    _ if innerline == String::from("C X") => rock.score(&scissors),
                    _ if innerline == String::from("A Y") => paper.score(&rock),
                    _ if innerline == String::from("B Y") => paper.score(&paper),
                    _ if innerline == String::from("C Y") => paper.score(&scissors),
                    _ if innerline == String::from("A Z") => scissors.score(&rock),
                    _ if innerline == String::from("B Z") => scissors.score(&paper),
                    _ if innerline == String::from("C Z") => scissors.score(&scissors),
                    _ => panic!()
                }
            }
        };
        score += outcome;
    }
    println!("Score: {}", score);
    Ok(())
}
