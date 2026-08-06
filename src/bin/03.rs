advent_of_code::solution!(3);

use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64
}

fn move_point(point: &mut Point, direction: &u8) {
    match direction {
            b'^' => point.y += 1,
            b'v' => point.y -= 1,
            b'>' => point.x += 1,
            b'<' => point.x -= 1,
            _ => ()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut point_set: HashSet<Point> = HashSet::new();

    let mut p = Point { x: 0 , y: 0 };
    point_set.insert(Point { ..p });

    for b in input.as_bytes() {
        move_point(&mut p, b);
        point_set.insert(Point { ..p });
    }

    Some(point_set.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut point_set: HashSet<Point> = HashSet::new();
    point_set.insert(Point { x: 0, y: 0 });

    let mut p1 = Point { x: 0, y: 0 };
    let mut p2 = Point { x: 0, y: 0 };

    let arr = input.as_bytes();
    for i in 0..input.len() {
        let p = if i % 2 == 0 {
            &mut p1
        } else {
            &mut p2
        };

        move_point(p, &arr[i]);
        point_set.insert(Point { x: p.x, y: p.y });
    }

    Some(point_set.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(2));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(4));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 4));
        assert_eq!(result, Some(3));
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 5));
        assert_eq!(result, Some(3));
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 6));
        assert_eq!(result, Some(11));

    }
}
