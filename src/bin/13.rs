use std::cmp::{self, Ordering};

use itertools::Itertools;

#[derive(PartialEq, Eq, Debug, Clone)]
enum Symbol {
    List(Vec<Symbol>),
    Num(u8),
}

fn collect_list(line: &str) -> (Symbol, usize) {
    let mut open_brac_cnt = 0;
    let mut close_brac_cnt = 0;

    let mut symbols = vec![];

    let mut buffer = String::new();

    let flush_buffer = |symbols: &mut Vec<Symbol>, buffer: &mut String| {
        if buffer.len() > 0 {
            symbols.push(Symbol::Num(buffer.parse::<u8>().unwrap()));
            buffer.clear();
        }
    };

    let mut line_iter = line.chars().enumerate();

    while let Some((i, ch)) = line_iter.next() {
        match ch {
            dig if dig.is_numeric() => buffer.push(dig),
            ',' => {
                flush_buffer(&mut symbols, &mut buffer);
            }
            '[' => {
                if open_brac_cnt == 1 {
                    let (syms, end_i) = collect_list(&line[i..]);
                    symbols.push(syms);
                    line_iter.nth(end_i - 1);
                    continue;
                } else {
                    open_brac_cnt += 1;
                }
            }
            ']' => {
                close_brac_cnt += 1;
                flush_buffer(&mut symbols, &mut buffer);
            }
            _ => unreachable!(),
        }
        if open_brac_cnt == close_brac_cnt {
            return (Symbol::List(symbols), i);
        }
    }
    unreachable!()
}

fn compare_symbols((left, right): (&Symbol, &Symbol)) -> Ordering {
    match (left, right) {
        (Symbol::List(a), Symbol::List(b)) => {
            let list_max = cmp::max(a.len(), b.len());
            for i in 0..list_max {
                if i == a.len() {
                    return Ordering::Less;
                }
                if i == b.len() {
                    return Ordering::Greater;
                }
                match compare_symbols((&a[i], &b[i])) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Equal => continue,
                }
            }
            Ordering::Equal
        }
        (Symbol::List(_), Symbol::Num(b)) => {
            compare_symbols((left, &Symbol::List(vec![Symbol::Num(*b)])))
        }
        (Symbol::Num(a), Symbol::List(_)) => {
            compare_symbols((&Symbol::List(vec![Symbol::Num(*a)]), right))
        }
        (Symbol::Num(a), Symbol::Num(b)) => {
            if a < b {
                Ordering::Less
            } else if a == b {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let result = input
        .split("\n\n")
        .map(|pair| {
            pair.lines()
                .map(|line| collect_list(line).0)
                .next_tuple()
                .unwrap()
        })
        .map(|(a, b)| compare_symbols((&a, &b)))
        .enumerate()
        .filter(|(_, res)| matches!(res, Ordering::Less))
        .map(|(i, _)| i + 1)
        .sum::<usize>();
    Some(result)
}

const ADDITIONAL_PACKETS: [&'static str; 2] = ["[[2]]", "[[6]]"];

pub fn part_two(input: &str) -> Option<usize> {
    let result = input
        .lines()
        .filter(|l| l != &"")
        .chain(ADDITIONAL_PACKETS.into_iter())
        .map(|l| collect_list(l).0)
        .sorted_by(|a, b| compare_symbols((a, b)))
        .enumerate()
        .filter(|(_, packet)| match packet {
            Symbol::List(syms) => {
                if syms.len() != 1 {
                    return false;
                }
                match &syms[0] {
                    Symbol::List(syms) => {
                        if syms.len() != 1 {
                            return false;
                        }
                        match &syms[0] {
                            Symbol::Num(2 | 6) => true,
                            _ => false,
                        }
                    }
                    _ => false,
                }
            }
            _ => unreachable!(),
        })
        .fold(
            0,
            |total, (i, _)| if total == 0 { i + 1 } else { total * (i + 1) },
        );
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(4));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), None);
    }

    #[test]
    fn test_collect_list() {
        let input = "[1,1]";
        assert_eq!(
            collect_list(input).0,
            Symbol::List(vec![Symbol::Num(1), Symbol::Num(1)])
        );
        let input = "[[1],4]";
        assert_eq!(
            collect_list(input).0,
            Symbol::List(vec![Symbol::List(vec![Symbol::Num(1)]), Symbol::Num(4)])
        );
        let input = "[[8,7,6]]";
        assert_eq!(
            collect_list(input).0,
            Symbol::List(vec![Symbol::List(vec![
                Symbol::Num(8),
                Symbol::Num(7),
                Symbol::Num(6)
            ])])
        );
    }
}
