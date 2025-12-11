use std::collections::HashMap;

fn parse(input: &str) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::new();

    for line in input.lines() {
        if let Some((from, to)) = line.split_once(':') {
            graph.insert(
                from.to_owned(),
                to.split_ascii_whitespace()
                    .map(|s| s.to_owned())
                    .collect::<Vec<_>>(),
            );
        }
    }

    graph
}

fn part1(puzzle: &HashMap<String, Vec<String>>) -> i64 {
    fn walk(puzzle: &HashMap<String, Vec<String>>, curr: &str) -> i64 {
        if curr == "out" {
            1
        } else {
            puzzle
                .get(curr)
                .unwrap()
                .iter()
                .map(|n| walk(puzzle, n))
                .sum()
        }
    }

    walk(puzzle, "you")
}

fn part2(puzzle: &HashMap<String, Vec<String>>) -> i64 {
    let mut cache = HashMap::new();

    fn walk<'a>(
        puzzle: &'a HashMap<String, Vec<String>>,
        curr: &'a str,
        mut seen: i64,
        cache: &mut HashMap<(i64, &'a str), i64>,
    ) -> i64 {
        if curr == "fft" || curr == "dac" {
            seen += 1;
        }

        let res = if curr == "out" {
            if seen == 2 { 1 } else { 0 }
        } else {
            if let Some(hit) = cache.get(&(seen, curr)) {
                return *hit;
            }

            puzzle
                .get(curr)
                .unwrap()
                .iter()
                .map(|n| walk(puzzle, n, seen, cache))
                .sum()
        };

        cache.insert((seen, curr), res);

        res
    }

    walk(puzzle, "svr", 0, &mut cache)
}

fn main() {
    let input = include_str!("../../input/input11.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day11() {
    let input1 = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
    let input1 = parse(input1);

    assert_eq!(part1(&input1), 5);

    let input2 = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
    let input2 = parse(input2);

    assert_eq!(part2(&input2), 2);
}
