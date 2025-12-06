#[derive(Debug)]
enum Expr {
    Sum(Vec<i64>),
    Prod(Vec<i64>),
}

impl Expr {
    fn eval(&self) -> i64 {
        match self {
            Expr::Sum(items) => items.iter().sum(),
            Expr::Prod(items) => items.iter().product(),
        }
    }

    fn add_num(&mut self, num: i64) {
        let nums = match self {
            Expr::Sum(nums) | Expr::Prod(nums) => nums,
        };

        nums.push(num);
    }
}

fn parse1(input: &str) -> Vec<Expr> {
    let table: Vec<Vec<&str>> = input
        .lines()
        .map(|l| l.split_ascii_whitespace().collect())
        .collect();

    let mut exprs = vec![];

    for col in 0..table[0].len() {
        let mut nums: Vec<i64> = vec![];
        for row in table.iter().take(table.len() - 1) {
            nums.push(row[col].parse().unwrap());
        }

        if table[table.len() - 1][col] == "+" {
            exprs.push(Expr::Sum(nums));
        } else {
            exprs.push(Expr::Prod(nums));
        }
    }

    exprs
}

fn solve(puzzle: &[Expr]) -> i64 {
    puzzle.iter().map(Expr::eval).sum()
}

fn parse2(input: &str) -> Vec<Expr> {
    let lines: Vec<_> = input.lines().map(|l| l.as_bytes()).collect();

    let width = lines[0].len();
    let bottom = lines.len() - 1;

    let mut exprs = vec![];

    for w in 0..width {
        if (0..=bottom).all(|h| lines[h][w] == b' ') {
            continue;
        }

        if lines[bottom][w] != b' ' {
            // new expr
            if lines[bottom][w] == b'+' {
                exprs.push(Expr::Sum(vec![]));
            } else {
                exprs.push(Expr::Prod(vec![]));
            }
        }

        let mut num = 0;
        for row in lines.iter().take(bottom) {
            if row[w].is_ascii_digit() {
                num = num * 10 + (row[w] - b'0') as i64;
            }
        }

        if let Some(e) = exprs.last_mut() {
            e.add_num(num);
        }
    }

    exprs
}

fn main() {
    let input = include_str!("../../input/input06.txt");
    println!("part1 = {}", solve(&parse1(input)));
    println!("part2 = {}", solve(&parse2(input)));
}

#[test]
fn test_day06() {
    let input = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    assert_eq!(solve(&parse1(input)), 4277556);
    assert_eq!(solve(&parse2(input)), 3263827);
}
