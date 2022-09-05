use std::fmt;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Rect { dx: usize, dy: usize },
    RotateRow { y: usize, dx: usize },
    RotateColumn { x: usize, dy: usize },
}

impl Command {
    pub fn from(input: &String) -> Result<Command, String> {
        lazy_static! {
            static ref RECT: Regex = Regex::new(r"rect (\d+)x(\d+)").unwrap();
            static ref ROTATE_COLUMN: Regex = Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap();
            static ref ROTATE_ROW: Regex = Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap();
        }

        if input.starts_with("rect ") {
            if let Some(caps) = RECT.captures(&input) {
                let dx = caps.get(1).map(|m| m.as_str()).unwrap().parse::<usize>().unwrap();
                let dy = caps.get(2).map(|m| m.as_str()).unwrap().parse::<usize>().unwrap();

                return Ok(Command::Rect { dx, dy });
            }
        }

        if input.starts_with("rotate column ") {
            if let Some(caps) = ROTATE_COLUMN.captures(&input) {
                let x = caps.get(1).map(|m| m.as_str()).unwrap().parse::<usize>().unwrap();
                let dy = caps.get(2).map(|m| m.as_str()).unwrap().parse::<usize>().unwrap();

                return Ok(Command::RotateColumn { x, dy });
            }
        }

        if input.starts_with("rotate row ") {
            if let Some(caps) = ROTATE_ROW.captures(&input) {
                let y = caps.get(1).map(|m| m.as_str()).unwrap().parse::<usize>().unwrap();
                let dx = caps.get(2).map(|m| m.as_str()).unwrap().parse::<usize>().unwrap();

                return Ok(Command::RotateRow { y, dx });
            }
        }

        Err(format!("Cannot convert {} into command", input))
    }
}

#[derive(Debug, Clone)]
struct Screen {
    pixels: Vec<Vec<char>>,
}

const ON: char = '#';
const OFF: char = '.';

impl Screen {
    fn new() -> Screen {
        Screen {
            pixels: vec![vec![OFF; 50]; 6],
        }
    }

    fn rect(&mut self, dx: usize, dy: usize) -> Result<(),String>{
        if dx > self.pixels.get(0).unwrap().len() {
            return Err(format!("Invalid value {} for dx, must be in range 0..{}", dx, self.pixels.get(0).unwrap().len()));
        }

        if dy > self.pixels.len() {
            return Err(format!("Invalid value {} for dy, must be in range 0..{}", dy, self.pixels.len()));
        }

        for y in 0..dy {
            for x in 0..dx {
                self.pixels[y][x] = ON;
            }
        }

        Ok(())
    }

    fn on_count(&self) -> usize {
        self.pixels.iter().map(|row| row.iter().filter(|p| **p == ON).count()).sum()
    }

    fn rotate_row(&mut self, y: usize, mut dx: usize) -> Result<(), String> {
        let len = self.pixels.len();
        if y >= len {
            return Err(format!("Invalid row {}, must be 0..{}", y, len));
        }

        let row = self.pixels.get_mut(y).unwrap();
        let max = row.len() - 1;
        while dx > 0 {
            let tmp = *row.get(max).unwrap();
            for x in (1..row.len()).rev() {
                *row.get_mut(x).unwrap() = *row.get_mut(x - 1).unwrap();
            }
            *row.get_mut(0).unwrap() = tmp;
            dx -= 1;
        }

        Ok(())
    }


    fn rotate_column(&mut self, x: usize, mut dy: usize) -> Result<(), String> {
        let len = self.pixels.get(0).unwrap().len();
        if x >= len {
            return Err(format!("Invalid column {}, must be 0..{}", x, len));
        }

        let max = self.pixels.len() - 1;
        while dy > 0 {
            let tmp = *self.pixels.get(max).unwrap().get(x).unwrap();

            for y in (1..self.pixels.len()).rev() {
                *self.pixels.get_mut(y).unwrap().get_mut(x).unwrap() = *self.pixels.get(y - 1).unwrap().get(x).unwrap();
            }
            *self.pixels.get_mut(0).unwrap().get_mut(x).unwrap() = tmp;
            dy -= 1;
        }

        Ok(())
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.pixels {
            write!(f, "{}\n", row.iter().collect::<String>()).expect("unable to display Screen");
        }

        Ok(())
    }
}

fn process(input: &Vec<Command>) -> Screen {
    let mut screen = Screen::new();

    for command in input {
        match command {
            Command::Rect{dx, dy} => screen.rect(*dx, *dy),
            Command::RotateRow {y, dx} => screen.rotate_row(*y, *dx),
            Command::RotateColumn {x, dy} => screen.rotate_column(*x, *dy),
        }.expect(format!("Invalid command {:?}", command).as_str());
    }

    screen
}


pub fn part1(input: &Vec<Command>) -> usize {
    let screen = process(input);
    screen.on_count()
}

pub fn part2(input: &Vec<Command>) -> String {
    let screen = process(input);
    screen.to_string()
}

#[cfg(test)]
impl Screen {
    fn with_size(x: usize, y: usize) -> Screen {
        Screen {
            pixels: vec![vec![OFF; x]; y],
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::day8::{Command, Screen};

    #[rstest]
    fn test_display() {
        println!("{}", Screen::new());
        assert_eq!(Screen::new().on_count(), 0);
    }

    #[rstest]
    fn test_count_on() {
        let mut screen = Screen::new();
        screen.rect(3, 2).expect("invalid rect");
        assert_eq!(screen.on_count(), 6);
        println!("{}", screen);
    }

    #[rstest]
    fn test_rotate() {
        let mut screen = Screen::with_size(7, 3);
        screen.rect(3, 2).expect("invalid rect");
        screen.rotate_column(1, 1).expect("could not rotate column");
        println!("{}", screen);
        screen.rotate_row(0, 4).expect("could not rotate row");
        println!("{}", screen);
        screen.rotate_column(1, 1).expect("could not rotate column");
        println!("{}", screen);
    }

    #[rstest]
    fn test_command_rect() {
        let command = Command::from(&"rect 3x2".to_string());
        assert_eq!(command.unwrap(), Command::Rect {dx : 3, dy : 2})
    }

    #[rstest]
    fn test_command_rotate_row() {
        let command = Command::from(&"rotate row y=2 by 7".to_string());
        assert_eq!(command.unwrap(), Command::RotateRow {y : 2, dx : 7})
    }

    #[rstest]
    fn test_command_rotate_column() {
        let command = Command::from(&"rotate column x=12 by 3".to_string());
        assert_eq!(command.unwrap(), Command::RotateColumn{x : 12, dy : 3})
    }
}
