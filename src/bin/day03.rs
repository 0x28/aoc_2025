use std::collections::HashMap;

fn parse(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect::<Vec<_>>()
}

fn part1(puzzle: &[String]) -> i64 {
    let mut sum = 0;

    for line in puzzle {
        let len = line.len();

        let mut first = -1i64;
        let mut idx_first = -1isize;

        for (idx, digit) in line[0..len - 1].char_indices() {
            let curr = digit.to_digit(10).unwrap() as i64;

            if curr > first {
                first = curr;
                idx_first = idx as isize;
            }
        }

        let mut second = 0;

        for digit in line.chars().skip(idx_first as usize + 1) {
            let curr = digit.to_digit(10).unwrap() as i64;

            if curr > second {
                second = curr;
            }
        }

        sum += first * 10 + second;
    }

    sum
}

// dynamic programming
fn max_value<'a>(
    cache: &mut HashMap<(usize, &'a [u64]), u64>,
    count: usize,
    digits: &'a [u64],
) -> u64 {
    if let Some(hit) = cache.get(&(count, digits)) {
        return *hit;
    }

    if count == 12 {
        return 0;
    }

    let res = if let [first, rest @ ..] = digits {
        // use digit
        (first * 10u64.pow(count as u32) + max_value(cache, count + 1, rest))
            // skip
            .max(max_value(cache, count, rest))
    } else {
        return 0;
    };

    cache.insert((count, digits), res);

    res
}

fn part2(puzzle: &[String]) -> i64 {
    let mut sum = 0;

    let mut cache = HashMap::new();

    // keep parsed digits alive for the lifetime of the cache
    let digit_lines = puzzle
        .iter()
        .map(|l| {
            l.chars()
                .flat_map(|d| d.to_digit(10).map(|n| n as u64))
                .rev()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for digits in &digit_lines {
        sum += max_value(&mut cache, 0, &digits[..]);
    }

    sum as i64
}

fn main() {
    let input = include_str!("../../input/input03.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day03() {
    let input = "\
987654321111111
811111111111119
234234234234278
818181911112111";
    let input = parse(input);

    assert_eq!(part1(&input), 357);
    assert_eq!(part2(&input), 3121910778619);
}
