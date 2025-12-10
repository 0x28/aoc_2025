fn parse(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .flat_map(|l| {
            let mut nums = l.split(',').flat_map(|n| n.parse());
            Some((nums.next()?, nums.next()?))
        })
        .collect()
}

fn area((x1, y1): &(i64, i64), (x2, y2): &(i64, i64)) -> i64 {
    ((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1)
}

fn part1(puzzle: &[(i64, i64)]) -> i64 {
    let mut max = 0;

    for (i, p1) in puzzle.iter().enumerate() {
        for (j, p2) in puzzle.iter().enumerate() {
            if i < j {
                max = max.max(area(p1, p2));
            }
        }
    }

    max
}

// https://wrfranklin.org/Research/Short_Notes/pnpoly.html
fn inside_polygon(vertices: &[(i64, i64)], point: &(f64, f64)) -> bool {
    let mut prev = vertices.len() - 1;
    let mut inside = false;

    for curr in 0..vertices.len() {
        let vcurr = (vertices[curr].0 as f64, vertices[curr].1 as f64);
        let vprev = (vertices[prev].0 as f64, vertices[prev].1 as f64);

        if (vcurr.1 > point.1) != (vprev.1 > point.1)
            && (point.0
                < (vprev.0 - vcurr.0) * (point.1 - vcurr.1)
                    / (vprev.1 - vcurr.1)
                    + vcurr.0)
        {
            inside = !inside;
        }

        prev = curr;
    }

    inside
}

fn lines_cross(
    line1: ((i64, i64), (i64, i64)),
    line2: ((i64, i64), (i64, i64)),
) -> bool {
    // no diagonal lines
    let is_horizontal1 = line1.0.1 == line1.1.1;
    let is_horizontal2 = line2.0.1 == line2.1.1;

    if is_horizontal1 == is_horizontal2 {
        return false;
    }

    let (hline, vline) = if is_horizontal1 {
        (line1, line2)
    } else if is_horizontal2 {
        (line2, line1)
    } else {
        unreachable!()
    };

    let ybegin = vline.0.1.min(vline.1.1);
    let yend = vline.0.1.max(vline.1.1);

    let xbegin = hline.0.0.min(hline.1.0);
    let xend = hline.0.0.max(hline.1.0);

    xbegin < vline.0.0
        && vline.0.0 < xend
        && ybegin < hline.0.1
        && hline.0.1 < yend
}

fn rect_contained(
    vertices: &[(i64, i64)],
    p1: (i64, i64),
    p2: (i64, i64),
) -> bool {
    let l1 = (p1, (p2.0, p1.1));
    let l2 = ((p2.0, p1.1), p2);
    let l3 = (p2, (p1.0, p2.1));
    let l4 = ((p1.0, p2.1), p1);

    let mut prev = vertices.len() - 1;

    for curr in 0..vertices.len() {
        let vert = (vertices[prev], vertices[curr]);
        if lines_cross(vert, l1)
            || lines_cross(vert, l2)
            || lines_cross(vert, l3)
            || lines_cross(vert, l4)
        {
            return false;
        }

        prev = curr;
    }

    // account for problems with inside_polygon algorithm on boundaries
    let ymin = p1.1.min(p2.1) as f64 + 0.1;
    let ymax = p1.1.max(p2.1) as f64 - 0.1;
    let xmin = p1.0.min(p2.0) as f64 + 0.1;
    let xmax = p1.0.max(p2.0) as f64 - 0.1;

    inside_polygon(vertices, &(xmin, ymin))
        && inside_polygon(vertices, &(xmin, ymax))
        && inside_polygon(vertices, &(xmax, ymin))
        && inside_polygon(vertices, &(xmax, ymax))
}

fn part2(puzzle: &[(i64, i64)]) -> i64 {
    let mut max = 0;

    for (i, p1) in puzzle.iter().enumerate() {
        for (j, p2) in puzzle.iter().enumerate() {
            if i < j && rect_contained(puzzle, *p1, *p2) {
                max = max.max(area(p1, p2));
            }
        }
    }

    max
}

fn main() {
    let input = include_str!("../../input/input09.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day09() {
    let input = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
    let input = parse(input);

    assert_eq!(part1(&input), 50);
    assert_eq!(part2(&input), 24);
}
