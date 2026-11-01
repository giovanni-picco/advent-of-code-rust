advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let mut floor = 0;
    for c in input.as_bytes() {
        floor += steps(c);
    }
    Some(floor)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut position = 1;
    let mut floor = 0;

    for c in input.as_bytes() {
        floor += steps(c);
        if floor < 0 {
            return Some(position);
        };
        position += 1;
    }
    None
}

fn steps(input: &u8) -> i32 {
    match input {
        b'(' => 1,
        b')' => -1,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_() {
        let result_1 = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result_1, Some(0));

        let result_1 = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result_1, Some(3));

        let result_1 = part_one(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result_1, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result_1 = part_two(&advent_of_code::template::read_file_part("examples", DAY, 4));
        assert_eq!(result_1, Some(1));
        let result_2 = part_two(&advent_of_code::template::read_file_part("examples", DAY, 5));
        assert_eq!(result_2, Some(5));
    }
}
