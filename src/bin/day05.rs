use std::ops::RangeInclusive;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Interval {
    begin: i64,
    end: i64,
}

impl Interval {
    fn from_range(range: RangeInclusive<i64>) -> Self {
        Self {
            begin: *range.start(),
            end: *range.end(),
        }
    }

    fn union(&self, other: &Interval) -> Option<Self> {
        if self.end < other.begin || other.end < self.begin {
            None
        } else {
            Some(Self::from_range(
                self.begin.min(other.begin)..=self.end.max(other.end),
            ))
        }
    }

    fn size(&self) -> i64 {
        self.end - self.begin + 1
    }

    fn contains(&self, num: i64) -> bool {
        self.begin <= num && num <= self.end
    }
}

struct Puzzle {
    fresh: Vec<Interval>,
    ingredients: Vec<i64>,
}

fn parse(input: &str) -> Puzzle {
    let mut comps = input.split("\n\n");

    let fresh = comps
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut nums = l.split('-').flat_map(|d| d.parse::<i64>());

            Interval {
                begin: nums.next().unwrap(),
                end: nums.next().unwrap(),
            }
        })
        .collect();

    let ingredients = comps
        .next()
        .unwrap()
        .lines()
        .flat_map(|l| l.parse::<i64>())
        .collect();

    Puzzle { fresh, ingredients }
}

fn part1(puzzle: &Puzzle) -> i64 {
    let mut sum = 0;

    'outer: for i in &puzzle.ingredients {
        for interval in &puzzle.fresh {
            if interval.contains(*i) {
                sum += 1;
                continue 'outer;
            }
        }
    }

    sum
}

fn part2(puzzle: &Puzzle) -> i64 {
    let mut combined_intervals = vec![];

    let mut fresh = puzzle.fresh.clone();
    // ensure intervals are combined by avoiding gaps because of the order
    fresh.sort_by(|l, r| l.begin.cmp(&r.begin));

    for interval in &fresh {
        let mut found = false;
        for combined in combined_intervals.iter_mut() {
            if let Some(new) = interval.union(combined) {
                *combined = new;
                found = true;
                break;
            }
        }

        if !found {
            combined_intervals.push(*interval);
        }
    }

    let mut sum = 0;
    for interval in &combined_intervals {
        sum += interval.size();
    }

    sum
}

fn main() {
    let input = include_str!("../../input/input05.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day05() {
    let input = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    let input = parse(input);

    assert_eq!(
        Interval::from_range(10..=18)
            .union(&Interval::from_range(16..=20))
            .unwrap(),
        Interval::from_range(10..=20)
    );
    assert_eq!(part1(&input), 3);
    assert_eq!(part2(&input), 14);
}
