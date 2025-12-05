use std::time::Instant;

const INPUT: &str = include_str!("../../inputs/2025-04.txt");

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),           (0, 1),
    (1, -1),  (1, 0),  (1, 1)
];

fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

fn count_neighbors(grid: &[Vec<u8>], r: usize, c: usize) -> usize {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let r = r as isize;
    let c = c as isize;

    DIRECTIONS.iter()
        .filter(|&&(dr, dc)| {
            let nr = r + dr;
            let nc = c + dc;
            nr >= 0 &&
            nc >= 0 &&
            nr < rows &&
            nc < cols &&
            grid[nr as usize][nc as usize] == b'@'
        })
        .count()
}

fn p1(input: &str) -> usize {
    let grid = parse(input);
    let mut count = 0;    
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == b'@' && count_neighbors(&grid, r, c) < 4 {
                count += 1;
            }
        }
    }
    count
}

fn p2(input: &str) -> usize {
    let mut grid = parse(input);
    let mut removed = 0;
    loop {
        let mut to_remove = Vec::new();

        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == b'@' && count_neighbors(&grid, r, c) < 4 {
                    to_remove.push((r, c));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }
        
        for (r, c) in to_remove {
            grid[r][c] = b'.';
            removed += 1;
        }
    }
    removed
}

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT);
    let elapsed = now.elapsed();
    println!("p1: {:?} {}", elapsed, solution);

    let now = Instant::now();
    let solution = p2(INPUT);
    let elapsed = now.elapsed();
    println!("p2: {:?} {}", elapsed, solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    
    #[test]
    fn test_p1() {           
        assert_eq!(p1(TEST_INPUT), 13);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(TEST_INPUT), 43);
    }
}
