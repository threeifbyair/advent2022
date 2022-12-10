use advent2022::Support;
use std::io;

fn main() -> io::Result<()> {
    let sup = Support::new()?;
    let mut trees: Vec<Vec<i32>> = vec![];
    for line in sup.lines {
        let line = line?;
        let treeline: Vec<i32> = line.chars().map(|c| (c as i32) - ('0' as i32)).collect();
        trees.push(treeline);
    }
    if sup.args.part_two {
        let scores: Vec<Vec<i32>> = trees
            .iter()
            .enumerate()
            .map(|(i, row)| {
                if i == 0 || i == trees.len() - 1 {
                    vec![0; row.len()]
                } else {
                    row.iter()
                        .enumerate()
                        .map(|(j, &t)| {
                            if j == 0 || j == row.len() - 1 {
                                0
                            } else {
                                let scoreleft = {
                                    let mut score = 0;
                                    for k in (0..j).rev() {
                                        score += 1;
                                        if trees[i][k] >= t {
                                            break;
                                        }
                                    }
                                    score
                                };

                                let scoreright = {
                                    let mut score = 0;
                                    for k in j + 1..row.len() {
                                        score += 1;
                                        if trees[i][k] >= t {
                                            break;
                                        }
                                    }
                                    score
                                };

                                let scoreup = {
                                    let mut score = 0;
                                    for k in (0..i).rev() {
                                        score += 1;
                                        if trees[k][j] >= t {
                                            break;
                                        }
                                    }
                                    score
                                };

                                let scoredown = {
                                    let mut score = 0;
                                    for k in i + 1..trees.len() {
                                        score += 1;
                                        if trees[k][j] >= t {
                                            break;
                                        }
                                    }
                                    score
                                };
                                scoreleft * scoreright * scoreup * scoredown
                            }
                        })
                        .collect()
                }
            })
            .collect();
        let score = scores
            .iter()
            .map(|v| v.iter().max().unwrap())
            .max()
            .unwrap();
        println!("Score: {}", score);
    } else {
        let mut visible: usize = 0;
        for i in 0..trees.len() {
            visible += {
                if i == 0 || i == trees.len() - 1 {
                    trees[i].len()
                } else {
                    trees[i]
                        .iter()
                        .enumerate()
                        .filter(|(j, &t)| {
                            *j == 0
                                || *j == trees[i].len() - 1
                                || trees[i][0..*j].iter().max().unwrap() < &t
                                || trees[i][*j + 1..trees[i].len()].iter().max().unwrap() < &t
                                || trees[0..i].iter().map(|v| v[*j]).max().unwrap() < t
                                || trees[i + 1..trees.len()]
                                    .iter()
                                    .map(|v| v[*j])
                                    .max()
                                    .unwrap()
                                    < t
                        })
                        .count()
                }
            };
        }
        println!("Visible: {}", visible);
    }
    Ok(())
}
