fn parse(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|s| {
            if let Some(num) = s.strip_prefix("L") {
                -num.parse::<i64>().unwrap()
            } else if let Some(num) = s.strip_prefix("R") {
                num.parse().unwrap()
            } else {
                0
            }
        })
        .collect()
}

fn part1(puzzle: &[i64]) -> i64 {
    let mut acc = 50;
    let mut count = 0;

    for dir in puzzle {
        acc += dir;
        acc %= 100;

        if acc == 0 {
            count += 1;
        }
    }

    count
}

fn part2(puzzle: &[i64]) -> i64 {
    let mut acc = 50;
    let mut count = 0;

    for dir in puzzle {
        let steps = dir.abs();
        let sign = if *dir > 0 { 1 } else { -1 };

        for _ in 0..steps {
            acc += sign;
            acc %= 100;

            if acc == 0 {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let input = include_str!("../../input/input01.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part1 = {}", part2(&input));
}

#[test]
fn test_day01() {
    let input = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    let input = parse(input);

    assert_eq!(part1(&input), 3);
    assert_eq!(part2(&input), 6);
}
