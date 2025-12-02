fn parse(input: &str) -> Vec<(u64, u64)> {
    input
        .split(',')
        .map(|r| {
            let mut r = r.split('-');
            let begin = r.next().unwrap().trim();
            let end = r.next().unwrap().trim();

            (begin.parse().unwrap(), end.parse().unwrap())
        })
        .collect::<Vec<_>>()
}

fn is_invalid1(num: u64) -> bool {
    let digits = num.ilog10() + 1;
    let half_digits = digits / 2;

    let first = num % 10_u64.pow(half_digits);
    let second = num / 10_u64.pow(half_digits);

    first == second
}

fn part1(puzzle: &[(u64, u64)]) -> u64 {
    let mut sum = 0;
    for &(begin, end) in puzzle {
        for n in begin..=end {
            if is_invalid1(n) {
                sum += n;
            }
        }
    }

    sum
}

fn is_invalid2(num: u64) -> bool {
    let digits = num.ilog10() + 1;

    for size in 1..=digits / 2 {
        let mut rest = num;
        let chunk = 10_u64.pow(size);

        let mut prev = None;
        let mut invalid = true;

        if !digits.is_multiple_of(size) {
            continue;
        }

        while rest > 0 {
            if let Some(prev) = prev
                && rest % chunk != prev
            {
                invalid = false;
                break;
            }

            prev = Some(rest % chunk);

            rest /= chunk;
        }

        if invalid {
            return invalid;
        }
    }

    false
}

fn part2(puzzle: &[(u64, u64)]) -> u64 {
    let mut sum = 0;
    for &(begin, end) in puzzle {
        for n in begin..=end {
            if is_invalid2(n) {
                sum += n;
            }
        }
    }

    sum
}

fn main() {
    let input = include_str!("../../input/input02.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day02() {
    let input = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";
    let input = parse(input);

    assert_eq!(part1(&input), 1227775554);
    assert_eq!(part2(&input), 4174379265);
}
