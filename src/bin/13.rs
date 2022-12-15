use advent2022::Support;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::io;

#[derive(Debug, PartialEq, PartialOrd, Eq)]
enum ComparableList {
    Number(u32),
    List(Vec<ComparableList>),
}

impl Ord for ComparableList {
    fn cmp(&self, other: &Self) -> Ordering {
        //println!("Comparing {:?} to {:?}", self, other);
        if let (ComparableList::Number(n1), ComparableList::Number(n2)) = (self, other) {
            let retval = n1.cmp(n2);
            //println!("{:?} {} {:?} (numbers)", self, {match retval { Ordering::Less => '<', Ordering::Equal => '=', Ordering::Greater => '>' }}, other);
            return retval;
        }
        let n1 = match self {
            ComparableList::Number(n) => vec![ComparableList::Number(*n)],
            ComparableList::List(_) => vec![],
        };
        let n2 = match other {
            ComparableList::Number(n) => vec![ComparableList::Number(*n)],
            ComparableList::List(_) => vec![],
        };
        let l1 = match self {
            ComparableList::Number(_) => &n1,
            ComparableList::List(l) => l,
        };
        let l2 = match other {
            ComparableList::Number(_) => &n2,
            ComparableList::List(l) => l,
        };
        for (i, item) in l1.iter().enumerate() {
            if i >= l2.len() {
                //println!("{:?} > {:?} (equal then longer)", l1, l2);
                return Ordering::Greater;
            }
            let ord = item.cmp(&l2[i]);
            if ord != Ordering::Equal {
                //println!("{:?} {} {:?} (item {})", l1, {if ord == Ordering::Less { '<' } else { '>' }}, l2, i);
                return ord;
            }
        }
        if l1.len() < l2.len() {
            //println!("{:?} < {:?} (equal then shorter)", l1, l2);
            return Ordering::Less;
        }
        //println!("{:?} = {:?}", l1, l2);
        Ordering::Equal
    }
}

fn build_list(chrs: VecDeque<char>) -> (ComparableList, VecDeque<char>) {
    let mut list = vec![];
    let mut chrs = chrs;
    loop {
        let mut chr = chrs.pop_front();
        match chr {
            None => break,
            Some(' ') => continue,
            Some(',') => continue,
            Some(']') => break,
            Some('[') => {
                let (sublist, newchrs) = build_list(chrs);
                chrs = newchrs;
                list.push(sublist);
            }
            Some('0'..='9') => {
                let mut num: u32 = 0;
                loop {
                    match chr {
                        None => break,
                        Some('0'..='9') => {}
                        _ => {
                            chrs.push_front(chr.unwrap());
                            break;
                        }
                    }
                    num = num * 10 + chr.unwrap().to_digit(10).unwrap();
                    chr = chrs.pop_front();
                }
                list.push(ComparableList::Number(num));
            }
            _ => panic!("Unexpected character: {:?}", chr),
        }
    }
    (ComparableList::List(list), chrs)
}

fn main() -> io::Result<()> {
    let sup = Support::new()?;

    let mut lists: Vec<ComparableList> = vec![];
    for line in sup.lines {
        let line = line?;
        if line.len() == 0 {
            continue;
        }
        let chrs = line
            .chars()
            .collect::<VecDeque<char>>();
        let (list, _newchrs) = build_list(chrs);
        lists.push(list);
    }

    if sup.args.part_two {
        lists.push(ComparableList::List(vec![ComparableList::List(vec![ComparableList::Number(2)])]));
        lists.push(ComparableList::List(vec![ComparableList::List(vec![ComparableList::Number(6)])]));
        lists.sort_by(|a,b| { 
            //println!("+++++\nComparing {:?} to {:?}", a, b);
            let retval = a.cmp(b);  
            //println!("RESULT: {:?} {} {:?}\n", a, {match retval { Ordering::Less => '<', Ordering::Equal => '=', Ordering::Greater => '>' }}, b);
            retval
        });

        let mut score = 1;
        let two = ComparableList::List(vec![ComparableList::List(vec![ComparableList::Number(2)])]);
        let six = ComparableList::List(vec![ComparableList::List(vec![ComparableList::Number(6)])]);
        for (i, list) in lists.iter().enumerate() {
            //println!("{}: {:?}\n", i, list);
            if list == &two {
                score *= i + 1;
            }
            if list == &six {
                score *= i + 1;
            }
        }
        println!("Score: {}", score);
    } else {
        let mut score = 0;
        let mut holding: Option<ComparableList> = None;
        let lists: Vec<(ComparableList, ComparableList)> =
            lists.into_iter().enumerate().filter_map(|(i, x)| {
                if i % 2 == 0 {
                    holding = Some(x);
                    None
                }
                else {
                    Some((holding.take().unwrap(), x))
                }
            }).collect();

        for (i, (list1, list2)) in lists.iter().enumerate() {
            //println!("{}: {:?} vs {:?}", i, list1, list2);
            if list1.cmp(list2) == Ordering::Less {
                score += i + 1;
            }
        }
        println!("Score: {}", score);
    }


    Ok(())
}
