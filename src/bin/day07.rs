use std::collections::HashMap;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn part1(puzzle: &Vec<Vec<char>>) -> i64 {
    let start = puzzle[0].iter().position(|c| *c == 'S').unwrap();

    let mut rays = vec![false; puzzle[0].len()];

    rays[start] = true;

    let mut split = 0;

    for line in puzzle {
        let mut new_rays = vec![false; line.len()];

        for (ray, active) in rays.iter().enumerate() {
            if !active {
                continue;
            }

            match line[ray] {
                '.' | 'S' => new_rays[ray] = true,
                '^' => {
                    split += 1;
                    new_rays[ray - 1] = true;
                    new_rays[ray + 1] = true;
                }
                _ => unreachable!(),
            }
        }

        std::mem::swap(&mut rays, &mut new_rays);
    }

    split
}

fn part2(puzzle: &Vec<Vec<char>>) -> i64 {
    let start = puzzle[0].iter().position(|c| *c == 'S').unwrap();

    let mut cache = HashMap::new();

    fn recur(
        puzzle: &Vec<Vec<char>>,
        ray: usize,
        depth: usize,
        cache: &mut HashMap<(usize, usize), i64>,
    ) -> i64 {
        if let Some(res) = cache.get(&(ray, depth)) {
            return *res;
        }

        if depth >= puzzle.len() {
            return 1;
        }

        let res = match puzzle[depth][ray] {
            '.' | 'S' => recur(puzzle, ray, depth + 1, cache),
            '^' => {
                recur(puzzle, ray - 1, depth + 1, cache)
                    + recur(puzzle, ray + 1, depth + 1, cache)
            }
            _ => unreachable!(),
        };

        cache.insert((ray, depth), res);

        res
    }

    recur(puzzle, start, 0, &mut cache)
}

fn main() {
    let input = include_str!("../../input/input07.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day07() {
    let input = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
    let input = parse(input);

    assert_eq!(part1(&input), 21);
    assert_eq!(part2(&input), 40);
}
