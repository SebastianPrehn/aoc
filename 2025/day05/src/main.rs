use std::time::Instant;

const INPUT: &str = include_str!("../../inputs/2025-05.txt");

fn parse(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let (ranges, ids) = input.split_once("\n\n").unwrap();
    
    let mut range = Vec::new();
    for line in ranges.lines() {
        let (a, b) = line.split_once('-').unwrap();
        range.push((a.parse().unwrap(), b.parse().unwrap()));
    }

    let mut id = Vec::new();
    for line in ids.lines() {
        id.push(line.parse::<u64>().unwrap());
    }
    (range, id)
}

fn p1(input: &str) -> usize {
    let (ranges, ids) = parse(input);
    let mut fresh = 0;
    for id in ids {
        for (start, end) in &ranges {
            if id >= *start && id <= *end {
                fresh += 1;
                break;
            }
        }
    }
    fresh
}

fn p2(input: &str) -> usize {
    let (mut ranges, _) = parse(input);
    ranges.sort_unstable_by_key(|&(start, _end)| start);

    let mut total = 0;

    let (mut current_start, mut current_end) = ranges[0];

    for &(start, end) in &ranges[1..] {
        if start <= current_end + 1 {
            if end > current_end {
                current_end = end;
            }
        } else {
            total += (current_end - current_start + 1) as usize;
            current_start = start;
            current_end = end;
        }
    }

    total += (current_end - current_start + 1) as usize;
    total
}

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT);
    let elapsed = now.elapsed();
    println!("p1: {:?} {}", elapsed, solution);

    let now = Instant::now();
    let solution = p2(INPUT);
    let elapsed = now.elapsed();
    println!("p2: {:.6} {}", elapsed.as_secs_f64() * 1000.0, solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
    "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    #[test]
    fn test_p1() {
        assert_eq!(p1(TEST_INPUT), 3);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(TEST_INPUT), 14);
    }
}
