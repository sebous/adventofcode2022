use itertools::Itertools;

#[derive(PartialEq, Eq, Debug, Clone)]
enum Symbol {
    List(Vec<Symbol>),
    Num(u8),
}

fn collect_list(line: &str) -> (Symbol, usize) {
    let mut open_brac = 0;
    let mut close_brac = 0;

    let mut symbols = vec![];

    let mut buffer = String::new();

    let flush_buffer = |symbols: &mut Vec<Symbol>, num_char_buffer: &mut String| {
        if num_char_buffer.len() > 0 {
            symbols.push(Symbol::Num(num_char_buffer.parse::<u8>().unwrap()));
            num_char_buffer.clear();
        }
    };

    let mut iter = line.chars().enumerate();

    while let Some((i, ch)) = iter.next() {
        match ch {
            dig if dig.is_digit(10) => buffer.push(dig),
            ',' => {
                flush_buffer(&mut symbols, &mut buffer);
            }
            '[' => {
                if open_brac == 1 {
                    let (syms, end_i) = collect_list(&line[i..]);
                    symbols.push(syms);
                    iter.nth(end_i - 1);
                    continue;
                } else {
                    open_brac += 1;
                }
            }
            ']' => {
                close_brac += 1;
                flush_buffer(&mut symbols, &mut buffer);
            }
            _ => continue,
        }
        if open_brac == close_brac {
            // println!("{:?}", &symbols);
            return (Symbol::List(symbols), i);
        }
    }
    unimplemented!()
}

fn parse(input: &str) -> Vec<(Symbol, Symbol)> {
    input
        .split("\n\n")
        .map(|pair| {
            pair.lines()
                .map(|line| collect_list(line).0)
                .next_tuple()
                .unwrap()
        })
        .collect_vec()
}

enum CmpResult {
    Ok,
    Rev,
}

fn compare_symbols((left, right): (&Symbol, &Symbol)) -> CmpResult {
    match (left, right) {
        (Symbol::List(a), Symbol::List(b)) => {
            for a_index in 0..a.len() {
                if a_index == b.len() {
                    return CmpResult::Rev;
                }
                match compare_symbols((&a[a_index], &b[a_index])) {
                    CmpResult::Ok => continue,
                    CmpResult::Rev => return CmpResult::Rev,
                }
            }
            CmpResult::Ok
        }
        (Symbol::List(_), Symbol::Num(b)) => {
            compare_symbols((left, &Symbol::List(vec![Symbol::Num(*b)])))
        }
        (Symbol::Num(a), Symbol::List(_)) => {
            compare_symbols((&Symbol::List(vec![Symbol::Num(*a)]), right))
        }
        (Symbol::Num(a), Symbol::Num(b)) => {
            if a <= b {
                CmpResult::Ok
            } else {
                CmpResult::Rev
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let pairs = parse(input);
    pairs.iter().for_each(|p| {
        println!("P1: {:?}", p.0);
        println!("P2: {:?}", p.1);
        println!("-----");
    });
    let cnt = pairs
        .iter()
        .map(|(a, b)| compare_symbols((a, b)))
        .filter(|res| matches!(res, CmpResult::Ok))
        .count();
    Some(cnt)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
