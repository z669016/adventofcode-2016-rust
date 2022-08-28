use std::collections::HashSet;
use euclid::Point2D;
use crate::day1::LocationDirection::ZERO;

#[derive(Debug, Copy, Clone)]
enum CommandDirection {
    LEFT,
    RIGHT,
}

impl CommandDirection {
    fn of(cd: char) -> CommandDirection {
        if cd == 'R' {
            CommandDirection::RIGHT
        } else {
            CommandDirection::LEFT
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Command {
    direction: CommandDirection,
    distance: i32,
}

impl Command {
    fn of(text: &str) -> Command {
        let cd = text.chars().next().unwrap();
        let distance = text[1..].parse().unwrap();

        Command {
            direction: CommandDirection::of(cd),
            distance,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum LocationDirection {
    ZERO,
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl LocationDirection {
    fn move_to(&self, command_direction: &CommandDirection) -> LocationDirection {
        match (self, command_direction) {
            (LocationDirection::ZERO, CommandDirection::LEFT) => LocationDirection::WEST,
            (LocationDirection::ZERO, CommandDirection::RIGHT) => LocationDirection::EAST,
            (LocationDirection::NORTH, CommandDirection::LEFT) => LocationDirection::WEST,
            (LocationDirection::NORTH, CommandDirection::RIGHT) => LocationDirection::EAST,
            (LocationDirection::EAST, CommandDirection::LEFT) => LocationDirection::NORTH,
            (LocationDirection::EAST, CommandDirection::RIGHT) => LocationDirection::SOUTH,
            (LocationDirection::SOUTH, CommandDirection::LEFT) => LocationDirection::EAST,
            (LocationDirection::SOUTH, CommandDirection::RIGHT) => LocationDirection::WEST,
            (LocationDirection::WEST, CommandDirection::LEFT) => LocationDirection::SOUTH,
            (LocationDirection::WEST, CommandDirection::RIGHT) => LocationDirection::NORTH,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Location {
    direction: LocationDirection,
    point: Point2D<i32, i32>,
}

impl Location {
    fn origin() -> Location {
        Location {
            direction: ZERO,
            point: Point2D::zero(),
        }
    }

    fn move_to(&self, command: &Command) -> Vec<Location> {
        let mut list: Vec<Location> = Vec::new();
        let new_direction = self.direction.move_to(&command.direction);

        let mut distance = command.distance;
        let mut x = self.point.x;
        let mut y = self.point.y;

        while distance > 0 {
            distance -= 1;
            x = match &new_direction {
                LocationDirection::WEST => x - 1,
                LocationDirection::EAST => x + 1,
                _ => x,
            };
            y = match &new_direction {
                LocationDirection::NORTH => y + 1,
                LocationDirection::SOUTH => y - 1,
                _ => y,
            };

            list.push(Location {
                direction: new_direction,
                point: Point2D::new(x, y),
            });
        }

        list
    }

    fn manhattan_distance(&self) -> i32 {
        self.point.x.abs() + self.point.y.abs()
    }
}

pub fn part1(input: &str) -> Result<i32, &str> {
    let mut location = Location::origin();

    for part in input.split(", ") {
        location = *(location.move_to(&Command::of(part)).last().unwrap());
    }

    Ok(location.manhattan_distance())
}

pub fn part2(input: &str) -> Result<i32, &str> {
    let mut set: HashSet<Point2D<i32, i32>> = HashSet::new();
    let mut location = Location::origin();

    set.insert(location.point);
    for part in input.split(", ") {
        let list = location.move_to(&Command::of(part));
        for loc in list {
            if !set.insert(loc.point) {
                return Ok(loc.manhattan_distance());
            }
            location = loc;
        }
    }

    Err("No doubles")
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::day1::{part1, part2};

    #[rstest]
    #[case("R2, L3", 5)]
    #[case("R4, R4", 8)]
    #[case("R2, R2, R2", 2)]
    #[case("R5, L5, R5, R3", 12)]
    fn test_part1(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, part1(&input).unwrap())
    }

    #[rstest]
    #[case("R8, R4, R4, R8", 4)]
    fn test_part2(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, part2(&input).unwrap())
    }
}
