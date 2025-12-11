use itertools::Itertools;

use z3::SatResult;
use z3::{Optimize, ast::*};

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl Machine {
    fn new() -> Self {
        Self {
            lights: vec![],
            buttons: vec![],
            joltage: vec![],
        }
    }
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|l| {
            let mut machine = Machine::new();
            let mut in_button = false;
            let mut in_joltage = false;

            let mut num = 0;

            for c in l.chars() {
                match c {
                    '.' => {
                        machine.lights.push(false);
                    }
                    '#' => {
                        machine.lights.push(true);
                    }
                    '(' => {
                        machine.buttons.push(vec![]);
                        in_button = true;
                    }
                    '{' => {
                        in_joltage = true;
                    }
                    ')' => {
                        if let Some(b) = machine.buttons.last_mut() {
                            b.push(num);
                        }
                        num = 0;
                        in_button = false;
                    }
                    ',' => {
                        if in_button {
                            if let Some(b) = machine.buttons.last_mut() {
                                b.push(num);
                            }
                        } else if in_joltage {
                            machine.joltage.push(num);
                        }
                        num = 0;
                    }
                    '}' => {
                        machine.joltage.push(num);
                        num = 0;
                        in_joltage = false;
                    }
                    c if c.is_ascii_digit() => {
                        num = num * 10 + c.to_digit(10).unwrap() as usize;
                    }
                    _ => continue,
                }
            }

            machine
        })
        .collect()
}

fn toggle_lights(lights: &mut [bool], button: &[usize]) {
    for &wire in button {
        lights[wire] = !lights[wire];
    }
}

fn part1(puzzle: &Vec<Machine>) -> usize {
    let mut res = 0;

    for machine in puzzle {
        'outer: for c in 1.. {
            for comb in machine.buttons.iter().combinations_with_replacement(c)
            {
                let mut lights = vec![false; machine.lights.len()];
                for button in &comb {
                    toggle_lights(&mut lights, button);
                }

                if machine.lights == lights {
                    res += c;
                    break 'outer;
                }
            }
        }
    }

    res
}

// Let the theorem prover do its magic.
fn part2(puzzle: &Vec<Machine>) -> u64 {
    let mut res = 0;

    for machine in puzzle {
        let opt = Optimize::new();

        let button_vars = machine
            .buttons
            .iter()
            .enumerate()
            .map(|(idx, _)| Int::new_const(format!("b_{idx}")))
            .collect::<Vec<_>>();

        for (idx, &j) in machine.joltage.iter().enumerate() {
            let limit = Int::from_u64(j as u64);
            let mut sum = Int::from_u64(0);

            for (b_idx, b) in machine.buttons.iter().enumerate() {
                if b.contains(&idx) {
                    sum += &button_vars[b_idx];
                }
            }

            opt.assert(&sum.eq(&limit));
        }

        let mut button_sum = Int::from_u64(0);
        for bvar in &button_vars {
            opt.assert(&bvar.ge(Int::from_u64(0)));
            button_sum += bvar;
        }

        opt.minimize(&button_sum);

        if let SatResult::Sat = opt.check(&[]) {
            let model = opt.get_model().unwrap();

            for b in &button_vars {
                res += model.get_const_interp(b).unwrap().as_u64().unwrap_or(0);
            }

            // println!("{}", opt.get_model().unwrap());
        }
    }

    res
}

fn main() {
    let input = include_str!("../../input/input10.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day10() {
    let input = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
    let input = parse(input);

    assert_eq!(part1(&input), 7);
    assert_eq!(part2(&input), 33);
}
