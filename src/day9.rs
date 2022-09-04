use std::str::Chars;
use lazy_static::lazy_static;
use regex::Regex;

pub fn repetition(input: String) -> Result<(usize, usize), String> {
    lazy_static! {
            static ref XY: Regex = Regex::new(r"(\d+)x(\d+)").unwrap();
        }

    if let Some(caps) = XY.captures(&input) {
        let length = caps.get(1).map(|m| m.as_str()).unwrap().parse::<usize>().unwrap();
        let times = caps.get(2).map(|m| m.as_str()).unwrap().parse::<usize>().unwrap();

        return Ok((length, times));
    }

    Err(format!("Cannot convert {} into ?x?", input))
}

fn take(iter: &mut Chars, mut length: usize) -> String {
    let mut taken = String::new();
    while length > 0 {
        if let Some(ch) = iter.next() {
            taken.push(ch);
        }
        length -= 1;
    }

    taken
}

fn find_matching_close(iter: &mut Chars) -> String {
    let mut result: Vec<char> = Vec::new();
    while let Some(ch) = iter.next() {
        if ch == ')' {
            return result.iter().collect::<String>();
        }

        result.push(ch);
    }

    result.iter().collect::<String>()
}

fn decrypt(input: &String, recursive: bool) -> usize {
    let mut result = 0usize;
    let iter = &mut input.chars();
    while let Some(ch) = iter.next() {
        if ch == '(' {
            let (length, times) = repetition(find_matching_close(iter)).unwrap();
            let taken = take(iter, length);
            result += (if recursive { decrypt(&taken, true) } else { length }) * times;
        } else {
            result += 1;
        }
    }

    result
}

pub fn part1(input: &String) -> Result<usize, &str> {
    Ok(decrypt(input, false))
}

pub fn part2(input: &String) -> Result<usize, &str> {
    Ok(decrypt(input, true))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::day9::{repetition, decrypt};

    #[rstest]
    fn test_command() {
        let (x, y) = repetition("3x04".to_string()).unwrap();
        assert_eq!(x, 3);
        assert_eq!(y, 4);
    }

    #[rstest]
    #[case("ADVENT", "ADVENT")]
    #[case("A(1x5)BC", "ABBBBBC")]
    #[case("(3x3)XYZ", "XYZXYZXYZ")]
    #[case("A(2x2)BCD(2x2)EFG", "ABCBCDEFEFG")]
    #[case("(6x1)(1x3)A", "(1x3)A")]
    #[case("X(8x2)(3x3)ABCY", "X(3x3)ABC(3x3)ABCY")]
    fn test_decrypt(#[case] input: &str, #[case] expected_output: &str) {
        assert_eq!(decrypt(&input.to_string(), false), expected_output.len());
    }

    #[rstest]
    #[case("ADVENT", 6)]
    #[case("(3x3)XYZ", 9)]
    #[case("X(8x2)(3x3)ABCY", 20)]
    #[case("(27x12)(20x12)(13x14)(7x10)(1x12)A", 241920)]
    #[case("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN", 445)]
    fn test_decrypt2(#[case] input: &str, #[case] expected_length: usize) {
        assert_eq!(decrypt(&input.to_string(), true), expected_length);
    }
}
