advent_of_code::solution!(2);

struct Box {
    length: u64,
    height: u64,
    width: u64
}

fn extract_box(input: &str) -> Vec<Box> {
    input.split('\n')
        .map(|s| s.to_lowercase())
        .filter(|s| !s.is_empty())
        .map(|s| s.split('x').map(|s| s.parse()).map(|s| s.unwrap()).collect::<Vec<u64>>())
        .map(|b| Box { length: b[0], height: b[1], width: b[2] })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let boxes = extract_box(input);

    let mut surface = 0u64;
    for b in boxes {
        let mut sides = [b.length * b.height, b.height * b.width, b.width * b.length];
        sides.sort();

        surface += sides.iter().fold(0u64, |acc, s| acc + s) * 2 + sides[0] ;
    }

    Some(surface)
}

pub fn part_two(input: &str) -> Option<u64> {
    let boxes = extract_box(input);

    let mut length = 0u64;
    for b in boxes {


    }

    Some(length)
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
