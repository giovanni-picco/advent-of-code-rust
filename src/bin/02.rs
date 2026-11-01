advent_of_code::solution!(2);

use std::array;

fn extract_box(input: &str) -> Vec<[u64; 3]> {
    let lines: Vec<&str> = input.split('\n')
        .filter(|s| !s.is_empty())
        .collect();

    let mut boxes = Vec::new();

    for line in lines {
        let mut iter = line.split('x');
        let mut b: [u64; 3] =  array::from_fn(|_| iter.next().unwrap().parse().unwrap());
        b.sort();
        boxes.push(b);
    }

    boxes
}

pub fn part_one(input: &str) -> Option<u64> {
    let boxes = extract_box(input);

    let mut surface = 0u64;
    for b in boxes {
        let sides = [b[0] * b[1], b[1] * b[2], b[2] * b[0]];
        surface += sides.iter().fold(0u64, |acc, s| acc + s) * 2 + sides[0] ;
    }

    Some(surface)
}

pub fn part_two(input: &str) -> Option<u64> {
    let boxes = extract_box(input);

    let mut lenght = 0u64;
    for b in boxes {

        lenght += (b[0] * 2) + (b[1] * 2);
        lenght += b[0] * b[1] * b[2];

    }
    Some(lenght)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(58));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(43));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(34));
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(14));
    }
}
