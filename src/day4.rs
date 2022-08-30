use std::cmp::Ordering::Equal;
use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug,PartialEq)]
struct Room {
    name: String,
    sector_id: i32,
    checksum: String,
}

impl Room {
    fn from(input: &str) -> Result<Room, String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([a-z\-]+)-(\d+)\[(\w+)\]").unwrap();
        }

        if let Some(caps) = RE.captures(input) {
            let name = String::from(caps.get(1).map(|m| m.as_str()).unwrap());
            let sector_id: i32 = caps.get(2).map(|m| m.as_str().parse::<i32>().unwrap()).unwrap();
            let checksum = String::from(caps.get(3).map(|m| m.as_str()).unwrap());

            return Ok(Room { name, sector_id, checksum });
        }

        Err(format!("Invalid input {}", input))
    }

    fn decrypt(&self) -> String {
        let secret = (self.sector_id % 26) as u8;
        let a = 'a' as u8;
        let z = 'z' as u8;

        self.name.chars()
            .map(|c| {
                if c == '-' {
                    return ' ';
                } else {
                    let i = (c as u8) + secret;
                    if i > z { (i - z + a - 1) as char } else { i as char }
                }
            })
            .collect::<String>()
    }

    fn check(&self) -> bool {
        let mut hash: HashMap<char, i32> = HashMap::new();
        for ch in self.name.chars() {
            if ch == '-' {
                continue;
            }

            if !hash.contains_key(&ch) {
                hash.insert(ch, 1);
            }
            if let Some(value) = hash.get_mut(&ch) {
                *value += 1
            }
        }

        let mut list : Vec<(&char, &i32)> = hash.iter()
            .map(|f| f)
            .collect();
        list.sort_by(|a,b| {
            if b.1.cmp(a.1) != Equal {
                b.1.cmp(a.1)
            } else {
                a.0.cmp(b.0)
            }
        });

        let mut result = String::new();
        for entry in list {
            result.push(*entry.0);
            if result.len() == 5 {
                break;
            }
        }

        result.eq(&self.checksum)
    }
}

pub fn part1(input: &Vec<String>) -> i32 {
    input.iter()
        .map(|text| Room::from(text).unwrap())
        .filter(|r| r.check())
        .map(|r| r.sector_id)
        .sum()
}

pub fn part2(input: &Vec<String>) -> Result<i32, &str> {
    for room in input.iter().map(|text| Room::from(text).unwrap()) {
        if room.decrypt().starts_with( "northpole object") {
            return Ok(room.sector_id);
        }
    }

    Err("North Pole objects room not found")
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::day4::Room;

    #[rstest]
    #[case("aaaaa-bbb-z-y-x-123[abxyz]", Room{name: "aaaaa-bbb-z-y-x".to_string(), sector_id: 123, checksum: "abxyz".to_string()})]
    fn test_room_from(#[case] input: String, #[case] expected: Room) {
        assert_eq!(Room::from(&input).unwrap(), expected);
    }

    #[rstest]
    #[case("aaaaa-bbb-z-y-x-123[abxyz]", true)]
    #[case("a-b-c-d-e-f-g-h-987[abcde]", true)]
    #[case("not-a-real-room-404[oarel]", true)]
    #[case("totally-real-room-200[decoy]", false)]
    fn test_room_check(#[case] input: String, #[case] expected: bool) {
        let room = Room::from(&input).unwrap();
        assert_eq!(room.check(), expected);
    }

    #[rstest]
    #[case("qzmt-zixmtkozy-ivhz-343[abxyz]", "very encrypted name")]
    fn test_room_decrypt(#[case] input: String, #[case] expected: &str) {
        let room = Room::from(&input).unwrap();
        assert_eq!(room.decrypt().as_str(), expected);
    }
}
