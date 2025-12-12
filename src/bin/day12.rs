use rustc_hash::{FxHashMap, FxHashSet};

const SHAPE_SIZE: usize = 3;
type Shape = [[bool; SHAPE_SIZE]; SHAPE_SIZE];
const EMPTY_SHAPE: Shape = [[false; SHAPE_SIZE]; SHAPE_SIZE];

#[derive(Debug)]
struct Puzzle {
    shapes: Vec<Shape>,
    regions: Vec<(usize, usize, Vec<usize>)>,
}

fn parse(input: &str) -> Puzzle {
    let mut sections = input.split("\n\n").peekable();
    let mut shapes = vec![];
    let mut regions = vec![];

    while let Some(section) = sections.next() {
        if sections.peek().is_some() {
            let mut shape = EMPTY_SHAPE;
            for (idx, line) in section.lines().skip(1).enumerate() {
                let bytes = line.as_bytes();
                for x in 0..shape[idx].len() {
                    shape[idx][x] = bytes[x] == b'#';
                }
            }

            shapes.push(shape);
        } else {
            for line in section.lines() {
                let (size, quantities) = line.split_once(':').unwrap();
                let (width, height) = size.split_once('x').unwrap();
                let indexes: Vec<usize> = quantities
                    .split_ascii_whitespace()
                    .flat_map(|n| n.parse())
                    .collect();

                regions.push((
                    width.parse().unwrap(),
                    height.parse().unwrap(),
                    indexes,
                ));
            }
        }
    }

    Puzzle { shapes, regions }
}

fn rotate(shape: &Shape) -> Shape {
    let mut rotated = EMPTY_SHAPE;

    for y in 0..rotated.len() {
        for x in 0..rotated[y].len() {
            rotated[y][x] = shape[rotated[y].len() - x - 1][y];
        }
    }

    rotated
}

fn flip(shape: &Shape) -> Shape {
    let mut flipped = EMPTY_SHAPE;

    for y in 0..flipped.len() {
        for x in 0..flipped[y].len() {
            flipped[y][x] = shape[y][flipped[y].len() - x - 1];
        }
    }

    flipped
}

fn orientations(shape: &Shape) -> FxHashSet<Shape> {
    let mut res = FxHashSet::default();

    let mut current = *shape;
    for _ in 0..4 {
        res.insert(current);
        current = rotate(&current);
    }

    let mut flipped = flip(shape);
    for _ in 0..4 {
        res.insert(flipped);
        flipped = rotate(&flipped);
    }

    res
}

fn can_place(x: usize, y: usize, shape: &Shape, region: &[Vec<bool>]) -> bool {
    for (pos_y, shape_row) in shape.iter().enumerate() {
        for (pos_x, pixel) in shape_row.iter().enumerate() {
            if *pixel && region[pos_y + y][pos_x + x] {
                return false;
            }
        }
    }

    true
}

fn toggle_place(x: usize, y: usize, shape: &Shape, region: &mut [Vec<bool>]) {
    for pos_y in 0..SHAPE_SIZE {
        for pos_x in 0..SHAPE_SIZE {
            if shape[pos_y][pos_x] {
                region[pos_y + y][pos_x + x] = !region[pos_y + y][pos_x + x];
            }
        }
    }
}

fn shape_size(shape: &Shape) -> usize {
    shape.iter().flatten().filter(|&&c| c).count()
}

fn recur(
    cache: &mut FxHashMap<(usize, Vec<Vec<bool>>), bool>,
    orientations: &[FxHashSet<Shape>],
    region: &mut [Vec<bool>],
    shape: usize,
    count: usize,
    rest: &[usize],
) -> bool {
    let res = if let Some(hit) = cache.get(&(shape, region.to_vec())) {
        *hit
    } else if rest.is_empty() && count == 0 {
        true
    } else if count == 0 {
        recur(cache, orientations, region, shape + 1, rest[0], &rest[1..])
    } else {
        // find position/orientation
        for o in &orientations[shape] {
            for y in 0..=region.len() - SHAPE_SIZE {
                for x in 0..=region[0].len() - SHAPE_SIZE {
                    if can_place(x, y, o, region) {
                        toggle_place(x, y, o, region);

                        if recur(
                            cache,
                            orientations,
                            region,
                            shape,
                            count - 1,
                            rest,
                        ) {
                            return true;
                        }

                        // undo
                        toggle_place(x, y, o, region);
                    }
                }
            }
        }

        false
    };

    cache.insert((shape, region.to_vec()), res);
    res
}

fn solve(puzzle: &Puzzle) -> i64 {
    let mut can_fit = 0;

    let orientations: Vec<_> = puzzle.shapes.iter().map(orientations).collect();
    let shape_sizes: Vec<usize> =
        puzzle.shapes.iter().map(shape_size).collect();

    for (width, height, quantities) in &puzzle.regions {
        let mut cache = FxHashMap::default();
        let mut region = vec![vec![false; *width]; *height];

        let max_size = region.len() * region[0].len();

        if max_size
            >= SHAPE_SIZE * SHAPE_SIZE * quantities.iter().sum::<usize>()
        {
            can_fit += 1; // will always fit
        } else if max_size
            < quantities
                .iter()
                .enumerate()
                .map(|(idx, q)| shape_sizes[idx] * q)
                .sum::<usize>()
        {
            continue; // will never fit
        } else if recur(
            &mut cache,
            &orientations,
            &mut region,
            0,
            quantities[0],
            &quantities[1..],
        ) {
            can_fit += 1;
        }
    }

    can_fit
}

fn main() {
    let input = include_str!("../../input/input12.txt");
    let input = parse(input);
    println!("solution = {}", solve(&input));
}

#[test]
fn test_day12() {
    let input = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";
    let input = parse(input);
    assert_eq!(solve(&input), 2);
}
