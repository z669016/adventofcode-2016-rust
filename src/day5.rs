enum Strategy {
    FIXED,
    PICK,
}

pub fn part1(input: &str) -> String {
    password(input, Strategy::FIXED)
}

pub fn part2(input: &str) -> String {
    password(input, Strategy::PICK)
}


fn password(door_id: &str, strategy: Strategy) -> String {
    let mut password: Vec<char> = vec![' '; 8];
    let mut index = -1;
    let mut offset : usize = 0;
    let mut count = 0;

    loop {
        index += 1;
        let key = format!("{}{}", door_id, index);
        let digest = md5::compute(&key);
        let digest_str = format!("{:x}", &digest);

        if digest_str.starts_with("00000") {
            let mut ch = digest_str.chars().nth(5).unwrap();
            if matches!(strategy, Strategy::PICK) {
                let digit= ch.to_digit(10);
                if digit.is_none() {
                    continue;
                }

                offset = digit.unwrap() as usize;
                if offset > 7 {
                    continue;
                }

                ch = digest_str.chars().nth(6).unwrap();
            }

            if password[offset] == ' ' {
                password[offset] = ch;
                offset += 1;
                count += 1;
            }
        }

        if count == 8 {
            break;
        }
    }

    password.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::day5::{password, Strategy};

    #[rstest]
    fn test_fixed() {
        let password = password("abc", Strategy::FIXED);
        assert_eq!("18f47a30", password);
    }

    #[rstest]
    fn test_pick() {
        let password = password("abc", Strategy::PICK);
        assert_eq!("05ace8e3", password);
    }
}
