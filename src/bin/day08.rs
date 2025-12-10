#[derive(Clone, Copy, Debug)]
struct Point(f64, f64, f64);

fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|l| {
            let nums: Vec<_> =
                l.split(',').flat_map(|n| n.parse::<f64>()).collect();

            Point(nums[0], nums[1], nums[2])
        })
        .collect()
}

fn dist(p1: &Point, p2: &Point) -> f64 {
    ((p1.0 - p2.0).powf(2.0)
        + (p1.1 - p2.1).powf(2.0)
        + (p1.2 - p2.2).powf(2.0))
    .sqrt()
}

fn solve(puzzle: &[Point], num: usize) -> usize {
    let mut dists = Vec::with_capacity((puzzle.len() * puzzle.len()) / 2);

    for (from, p_from) in puzzle.iter().enumerate() {
        for (to, p_to) in puzzle.iter().enumerate() {
            if from < to {
                dists.push((from, to, dist(p_from, p_to)));
            }
        }
    }

    dists.sort_by(|d1, d2| d1.2.partial_cmp(&d2.2).unwrap());

    let mut clusters = puzzle
        .iter()
        .map(|p| vec![*p])
        .enumerate()
        .collect::<Vec<_>>();

    for (from, to, _dist) in dists.iter().take(num) {
        let mut to = *to;
        let mut from = *from;

        let x1 = puzzle[to].0;
        let x2 = puzzle[from].0;

        while clusters[to].1.is_empty() {
            to = clusters[to].0;
        }

        while clusters[from].1.is_empty() {
            from = clusters[from].0;
        }

        if from != to {
            // part2
            if clusters[from].1.len() + clusters[to].1.len() == puzzle.len() {
                return (x1 * x2) as usize;
            }

            let mut placeholder = (from, vec![]);
            std::mem::swap(&mut placeholder, &mut clusters[to]);
            clusters[from].1.extend(placeholder.1);
        }
    }

    clusters.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

    // part1
    clusters[0..3].iter().map(|(_, ps)| ps.len()).product()
}

fn main() {
    let input = include_str!("../../input/input08.txt");
    let input = parse(input);
    println!("part1 = {}", solve(&input, 1000));
    println!("part2 = {}", solve(&input, usize::MAX));
}

#[test]
fn test_day08() {
    let input = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
    let input = parse(input);

    assert_eq!(solve(&input, 10), 40);
    assert_eq!(solve(&input, usize::MAX), 25272);
}
