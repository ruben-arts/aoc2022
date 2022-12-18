use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::{complete, map, map_res},
    multi::{many0, separated_list0},
    sequence::{delimited, preceded, separated_pair, tuple},
};
use std::path::Path;

fn main() {
    // Read input file
    let day = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
    let input = std::fs::read_to_string(format!("inputs/{day}.txt")).unwrap();
    let input = complete(many0(parse_monkey))(&input).unwrap().1;

    let mut monkeys = input.clone();
    for _round in 0..20 {
        for i in 0..monkeys.len() {
            let packages = monkeys[i].do_round(3, 0);
            for package in packages {
                monkeys[package.monkey_id].items.push(package.worry_level);
            }
        }
    }

    let mut monkey_business: Vec<_> = monkeys.iter().map(|m| m.items_inspected).collect();
    monkey_business.sort();
    monkey_business.reverse();
    let monkey_business = monkey_business[0] * monkey_business[1];

    println!("Solution day 11 part 1: {}", monkey_business);

    let mut monkeys = input;
    let common_denominator = monkeys.iter().map(|m| m.test_divisible_by).product();
    for _round in 0..10000 {
        for i in 0..monkeys.len() {
            let packages = monkeys[i].do_round(1, common_denominator);
            for package in packages {
                monkeys[package.monkey_id].items.push(package.worry_level);
            }
        }
    }

    let mut monkey_business: Vec<_> = monkeys.iter().map(|m| m.items_inspected).collect();
    monkey_business.sort();
    monkey_business.reverse();
    let monkey_business = monkey_business[0] * monkey_business[1];

    println!("Solution day 11 part 2: {}", monkey_business);
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Expr {
    Old,
    Value(usize),
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Operation {
    Multiple(Expr, Expr),
    Add(Expr, Expr),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test_divisible_by: usize,
    true_result: usize,
    false_result: usize,
    items_inspected: usize,
}

struct Package {
    monkey_id: usize,
    worry_level: usize,
}
impl Monkey {
    fn do_round(&mut self, divide_by: usize, modulo_by: usize) -> Vec<Package> {
        let items = std::mem::take(&mut self.items);
        let mut packages = vec![];
        for worry_level in items {
            let new = self.operation.execute(worry_level) / divide_by;
            let new = if modulo_by > 0 { new % modulo_by } else { new };
            let monkey_id = if new % self.test_divisible_by == 0 {
                self.true_result
            } else {
                self.false_result
            };
            self.items_inspected += 1;
            packages.push(Package {
                monkey_id,
                worry_level: new,
            })
        }
        packages
    }
}

impl Operation {
    pub fn execute(&self, old: usize) -> usize {
        match self {
            Operation::Multiple(left, right) => left.eval(old) * right.eval(old),
            Operation::Add(left, right) => left.eval(old) + right.eval(old),
        }
    }
}

impl Expr {
    pub fn eval(&self, old: usize) -> usize {
        match self {
            Expr::Old => old,
            Expr::Value(v) => *v,
        }
    }
}

fn parse_monkey(input: &str) -> nom::IResult<&str, Monkey> {
    map(
        tuple((
            preceded(multispace0, parse_monkey_label),
            preceded(multispace0, parse_starting_items),
            preceded(multispace0, parse_operation),
            preceded(multispace0, parse_test),
            preceded(multispace0, parse_if_true),
            preceded(multispace0, parse_if_false),
        )),
        |(_, items, operation, test_divisible_by, true_result, false_result)| Monkey {
            items,
            operation,
            test_divisible_by,
            true_result,
            false_result,
            items_inspected: 0,
        },
    )(input)
}

// Parses: "Monkey 2:"
fn parse_monkey_label(input: &str) -> nom::IResult<&str, usize> {
    delimited(tag("Monkey "), map_res(digit1, str::parse), tag(":"))(input)
}

// Parses: "Starting items: 91, 58, 52, 69, 95, 54"
fn parse_starting_items(input: &str) -> nom::IResult<&str, Vec<usize>> {
    preceded(
        tag("Starting items: "),
        separated_list0(tag(", "), map_res(digit1, str::parse)),
    )(input)
}
// Parses: "Operation: new = x (+/*) y"
fn parse_operation(input: &str) -> nom::IResult<&str, Operation> {
    preceded(
        tag("Operation: new = "),
        alt((
            map(
                separated_pair(parse_expr, tag(" * "), parse_expr),
                |(left, right)| Operation::Multiple(left, right),
            ),
            map(
                separated_pair(parse_expr, tag(" + "), parse_expr),
                |(left, right)| Operation::Add(left, right),
            ),
        )),
    )(input)
}

// Parses: "4" or "old"
fn parse_expr(input: &str) -> nom::IResult<&str, Expr> {
    alt((
        map(tag("old"), |_| Expr::Old),
        map_res(digit1, |value: &str| value.parse().map(Expr::Value)),
    ))(input)
}

// Parses: "Test: divisible by 13"
fn parse_test(input: &str) -> nom::IResult<&str, usize> {
    preceded(tag("Test: divisible by "), map_res(digit1, str::parse))(input)
}

// Parses:  "If true: throw to monkey x"
fn parse_if_true(input: &str) -> nom::IResult<&str, usize> {
    preceded(
        tag("If true: throw to monkey "),
        map_res(digit1, str::parse),
    )(input)
}

// Parses: "If false: throw to monkey x"
fn parse_if_false(input: &str) -> nom::IResult<&str, usize> {
    preceded(
        tag("If false: throw to monkey "),
        map_res(digit1, str::parse),
    )(input)
}

#[cfg(test)]
mod test {
    use super::{parse_monkey_label, parse_operation, parse_starting_items};
    use crate::{parse_expr, parse_monkey, Expr, Monkey, Operation};

    #[test]
    fn test_parse_monkey_label() {
        assert_eq!(parse_monkey_label("Monkey 2:").unwrap().1, 2);
    }

    #[test]
    fn test_parse_starting_items() {
        assert_eq!(
            parse_starting_items("Starting items: 91, 58, 52, 69, 95, 54")
                .unwrap()
                .1,
            vec![91, 58, 52, 69, 95, 54]
        );
    }

    #[test]
    fn test_parse_operation() {
        assert_eq!(
            parse_operation("Operation: new = old + 3").unwrap().1,
            Operation::Add(Expr::Old, Expr::Value(3))
        );
        assert_eq!(
            parse_operation("Operation: new = old * 3").unwrap().1,
            Operation::Multiple(Expr::Old, Expr::Value(3))
        );
    }

    #[test]
    fn test_parse_expr() {
        assert_eq!(parse_expr("old").unwrap().1, Expr::Old);
        assert_eq!(parse_expr("6").unwrap().1, Expr::Value(6));
    }
    #[test]
    fn test_parse_monkey() {
        assert_eq!(
            parse_monkey(
                "Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"
            )
            .unwrap()
            .1,
            Monkey {
                items: vec![74],
                operation: Operation::Add(Expr::Old, Expr::Value(3)),
                test_divisible_by: 17,
                true_result: 0,
                false_result: 1,
                items_inspected: 0
            }
        );
    }
}
