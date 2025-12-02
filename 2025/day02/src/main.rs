use std::time::Instant;

const INPUT: &str = include_str!("../../inputs/2025-02.txt");

fn parse(input: &str) -> impl Iterator<Item = (u64, u64)> + '_ {
    input.trim().split(',').map(|r| {
        let (start, end) = r.split_once('-').unwrap();
        (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
    })
}

fn repeated_twice(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();
    if len % 2 != 0 {
        return false;
    }
    let half = len / 2;
    s[..half] == s[half..]
}

fn repeated_n_times(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();
    
    for chunk in 1..=len/2 {
        if len % chunk != 0 {
            continue;
        }
        let t = &s[..chunk];
        if t.repeat(len / chunk) == s {
            return true;
        }
    }
    false
}

fn p1(input: &str) -> u64 {
    parse(input)
        .flat_map(|(start, end)| start..=end)
        .filter(|&n| repeated_twice(n))
        .sum()
}

fn p2(input: &str) -> u64 {
    parse(input)
        .flat_map(|(start, end)| start..=end)
        .filter(|&n| repeated_n_times(n))
        .sum()
}

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT);
    println!("p1: {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT);
    println!("p2: {:?} {}", now.elapsed(), solution);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "11-22,95-155,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(p1(input), 1227775554);
    }

    #[test]
    fn test_p2() {
        let input = "11-22,95-155,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(p2(input), 4174379265);
    }
}
