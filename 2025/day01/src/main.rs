use std::time::Instant;

const INPUT: &str = include_str!("../../inputs/2025-01.txt");
const LOCK_SIZE: isize = 100;

// Grab parse from https://gist.github.com/icub3d/dc8ef4474449d327fda2336f3fe79df9
fn parse(input: &str) -> impl Iterator<Item = (char, isize)> + '_ {
    input.lines().map(|l| {
        let (dir, dist) = l.split_at(1);
        (
            dir.chars().next().expect("Missing direction"),
            dist.parse::<isize>().expect("Invalid number"),
        )
    })
}

fn p1(input: &str) -> usize {
    parse(input)
        .scan(50isize, |acc, (dir, dist)| {
            *acc = match dir {
                'L' => (*acc - dist).rem_euclid(LOCK_SIZE),
                'R' => (*acc + dist).rem_euclid(LOCK_SIZE),
                _ => unreachable!(),
            };
            Some(*acc)
        })
        .filter(|&i| i == 0)
        .count()
}

fn p2(input: &str) -> isize {
    parse(input)
        .scan(50isize, |acc, (dir, dist)| {
            let end = match dir {
                'L' => *acc - dist,
                'R' => *acc + dist,
                _ => unreachable!(),
            };

            // How many times did we cross zero? Using div_euclid can help here especially when
            // going into negative space.
            let zeros = match dir {
                'L' => (*acc - 1).div_euclid(LOCK_SIZE) - (end - 1).div_euclid(LOCK_SIZE),
                'R' => end.div_euclid(LOCK_SIZE),
                _ => unreachable!(),
            };

            // Update accumulator like p1.
            *acc = end.rem_euclid(LOCK_SIZE);

            // Return the number of times we crossed zero to sum them.
            Some(zeros)
        })
        .sum()
}


fn main() {
    let now = Instant::now();
    println!("p1: {:?} {}", now.elapsed(), p1(INPUT));

    let now = Instant::now();
    println!("p2: {:?} {}", now.elapsed(), p2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
"L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
    
    #[test]
    fn test_p1() {
        assert_eq!(p1(TEST_INPUT), 3);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(TEST_INPUT), 6);
    }

}
