advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let mut i = 0u64;

    loop {
        let digest = md5::compute(format!("{}{i}", input.trim()));
        if format!("{:x}", digest).starts_with("00000") {
            break;
        }
        i += 1;
    }

    Some(i)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut i = 0u64;

    loop {
        let digest = md5::compute(format!("{}{i}", input.trim()));
        if format!("{:x}", digest).starts_with("000000") {
            break;
        }
        i += 1;
    }

    Some(i)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one("abcdef");
        assert_eq!(result, Some(609043));
        let result = part_one("pqrstuv");
        assert_eq!(result, Some(1048970));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
