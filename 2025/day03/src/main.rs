use std::time::Instant;

const INPUT: &str = include_str!("../../inputs/2025-03.txt");

fn parse(input: &str) -> impl Iterator<Item = &[u8]> {
    input.trim().lines().map(|line| line.as_bytes())
}

fn max_joltage(line: &[u8], k: usize) -> u64 {
    let mut stack: Vec<u8> = Vec::with_capacity(k);
    let mut to_drop = line.len() - k;

    for &d in line {
        while to_drop > 0 && !stack.is_empty() && *stack.last().unwrap() < d {
            stack.pop();
            to_drop -= 1;
        }
        stack.push(d);
    }

    stack.truncate(k);

    std::str::from_utf8(&stack).unwrap().parse::<u64>().unwrap()
}

fn p1(input: &str) -> u64 {
    parse(input).map(|line| max_joltage(line, 2)).sum()
}

fn p2(input: &str) -> u64 {
    parse(input).map(|line| max_joltage(line, 12)).sum()
}

fn main() {
    let now = Instant::now();
    println!("p1: {:?} {}", now.elapsed(), p1(INPUT));

    let now = Instant::now();
    println!("p2: {:?} {}", now.elapsed(), p2(INPUT));
}

#[cfg(test)]
mod tests {
    use  super::*;

    const TEST_INPUT: &str =
"987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_p1() {
        assert_eq!(p1(TEST_INPUT), 357);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(TEST_INPUT), 3121910778619);
   }
}
