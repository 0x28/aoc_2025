fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn access(field: &[Vec<char>], x: isize, y: isize) -> i64 {
    if x < 0 || y < 0 {
        return 0;
    }

    let elem = field.get(y as usize).and_then(|line| line.get(x as usize));
    if let Some(&'@') = elem { 1 } else { 0 }
}

fn part1(puzzle: &[Vec<char>]) -> i64 {
    let mut sum = 0;

    for y in 0..puzzle.len() {
        for x in 0..puzzle[0].len() {
            if puzzle[y][x] != '@' {
                continue;
            }

            let (x, y) = (x as isize, y as isize);

            let rolls = access(puzzle, x, y - 1)
                + access(puzzle, x, y + 1)
                + access(puzzle, x - 1, y - 1)
                + access(puzzle, x - 1, y)
                + access(puzzle, x - 1, y + 1)
                + access(puzzle, x + 1, y - 1)
                + access(puzzle, x + 1, y)
                + access(puzzle, x + 1, y + 1);

            if rolls < 4 {
                sum += 1;
            }
        }
    }

    sum
}

fn part2(puzzle: &[Vec<char>]) -> i64 {
    let mut sum = 0;

    let mut field = puzzle.to_vec();

    loop {
        let mut to_remove = vec![];

        for y in 0..field.len() {
            for x in 0..field[0].len() {
                if field[y][x] != '@' {
                    continue;
                }

                let (x, y) = (x as isize, y as isize);

                let rolls = access(&field, x, y - 1)
                    + access(&field, x, y + 1)
                    + access(&field, x - 1, y - 1)
                    + access(&field, x - 1, y)
                    + access(&field, x - 1, y + 1)
                    + access(&field, x + 1, y - 1)
                    + access(&field, x + 1, y)
                    + access(&field, x + 1, y + 1);

                if rolls < 4 {
                    to_remove.push((x as usize, y as usize));
                    sum += 1;
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for &(x, y) in &to_remove {
            field[y][x] = '.';
        }
    }

    sum
}

fn main() {
    let input = include_str!("../../input/input04.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day04() {
    let input = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    let input = parse(input);

    assert_eq!(part1(&input), 13);
    assert_eq!(part2(&input), 43);
}
