use euclid::Point2D;
use grid::{Grid, grid};

#[derive(Debug, Copy, Clone, PartialEq)]
enum Command {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

impl Command {
    fn from(command: char) -> Command {
        match command {
            'U' => Command::UP,
            'D' => Command::DOWN,
            'L' => Command::LEFT,
            'R' => Command::RIGHT,
            _ => panic!("Invalid command char '{}'", command)
        }
    }

    fn from_str(commands: &str) -> Vec<Command> {
        let mut list: Vec<Command> = Vec::new();
        for ch in commands.chars() {
            list.push(Command::from(ch));
        }

        list
    }

    fn from_string_list(commands: &Vec<String>) -> Vec<Vec<Command>> {
        let mut list: Vec<Vec<Command>> = Vec::new();
        for line in commands {
            list.push(Command::from_str(line));
        }

        list
    }
}

#[derive(Debug, Clone)]
struct KeyBoard {
    grid: Grid<char>,
    point: Point2D<usize, usize>,
}

impl KeyBoard {
    fn new(grid: Grid<char>, point: Point2D<usize, usize>) -> KeyBoard {
        let (x, y) = grid.size();
        if x % 2 == 0 || y % 2 == 0 {
            panic!("The grid must be uneven in size ({},{})", x, y);
        }

        if point.x >= x || point.y >= y {
            panic!("The initial point is off the grid {:?}", point);
        }

        KeyBoard {
            grid,
            point,
        }
    }

    fn at(&self) -> char {
        *self.grid.get(self.point.y, self.point.x).unwrap()
    }

    fn move_to(&mut self, command: &Command) -> char {
        let (max_y, max_x) = self.grid.size();

        let x = match command {
            Command::LEFT => if self.point.x > 0 { self.point.x - 1 } else { 0 },
            Command::RIGHT => if self.point.x < max_x - 1 { self.point.x + 1 } else { max_x - 1 },
            _ => self.point.x
        };
        let y = match command {
            Command::UP => if self.point.y > 0 { self.point.y - 1 } else { 0 },
            Command::DOWN => if self.point.y < max_y - 1 { self.point.y + 1 } else { max_y - 1 },
            _ => self.point.y
        };


        if let Some(c) = self.grid.get(y, x) {
            if *c != ' ' {
                self.point = Point2D::new(x, y);
            }
        }

        self.at()
    }

    fn move_to_list(&mut self, commands: &Vec<Command>) -> char {
        for command in commands {
            self.move_to(command);
        }

        self.at()
    }

    fn move_to_list_of_list(&mut self, commands: &Vec<Vec<Command>>) -> String {
        let mut result = String::new();

        for list in commands {
            result.push(self.move_to_list(list));
        }

        result
    }
}

fn three_by_three() -> Grid<char> {
    grid![
        ['1','2','3']
        ['4','5','6']
        ['7','8','9']
    ]
}

pub fn part1(input: &Vec<String>) -> String {
    let mut keyboard = KeyBoard::new(three_by_three(), Point2D::new(1, 1));
    let commands = Command::from_string_list(&input);
    keyboard.move_to_list_of_list(&commands)
}

fn five_by_five() -> Grid<char> {
    grid![
            [' ', ' ', '1', ' ', ' ']
            [' ', '2', '3', '4', ' ']
            ['5', '6', '7', '8', '9']
            [' ', 'A', 'B', 'C', ' ']
            [' ', ' ', 'D', ' ', ' ']
        ]
}

pub fn part2(input: &Vec<String>) -> String {
    let mut keyboard = KeyBoard::new(five_by_five(), Point2D::new(0, 2));
    let commands = Command::from_string_list(&input);
    keyboard.move_to_list_of_list(&commands)
}

#[cfg(test)]
mod tests {
    use euclid::Point2D;
    use rstest::rstest;
    use crate::day2::{Command, five_by_five, KeyBoard, three_by_three};

    #[rstest]
    fn test_commands() {
        let commands = Command::from_str("ULDR");

        assert_eq!(4, commands.len());
        assert_eq!(Command::UP, *commands.get(0).unwrap());
        assert_eq!(Command::LEFT, *commands.get(1).unwrap());
        assert_eq!(Command::DOWN, *commands.get(2).unwrap());
        assert_eq!(Command::RIGHT, *commands.get(3).unwrap());
    }

    #[rstest]
    fn test_keyboard() {
        let keyboard = KeyBoard::new(three_by_three(), Point2D::new(1, 1));
        assert_eq!('5', keyboard.at());
    }

    #[rstest]
    #[case("ULL", '1')]
    #[case("URR", '3')]
    #[case("LLL", '4')]
    #[case("RRR", '6')]
    #[case("DLL", '7')]
    #[case("DRR", '9')]
    fn test_keyboard_move_to_all(#[case] input: &str, #[case] expected: char) {
        let mut keyboard = KeyBoard::new(three_by_three(), Point2D::new(1, 1));
        let commands = Command::from_str(input);

        assert_eq!(expected, keyboard.move_to_list(&commands));
    }

    #[rstest]
    fn test_keyboard_move_to_all_list() {
        let input = vec![
            String::from("ULL"),
            String::from("RRDDD"),
            String::from("LURDL"),
            String::from("UUUUD"),
        ];
        let commands = Command::from_string_list(&input);

        let mut keyboard = KeyBoard::new(three_by_three(), Point2D::new(1, 1));
        assert_eq!("1985", keyboard.move_to_list_of_list(&commands).as_str());

        keyboard = KeyBoard::new(five_by_five(), Point2D::new(0, 2));
        assert_eq!("5DB3", keyboard.move_to_list_of_list(&commands).as_str());
    }
}
