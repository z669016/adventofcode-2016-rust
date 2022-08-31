use std::collections::HashSet;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
struct IP7 {
    sub: Vec<String>,
}

impl IP7 {
    fn from(input: &String) -> Result<IP7, String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\w+").unwrap();
        }

        let mut sub: Vec<String> = Vec::new();
        for m in RE.find_iter(input) {
            sub.push(m.as_str().to_string());
        }

        if sub.is_empty() {
            return Err(format!("Invalid input {}", input));
        }

        return Ok(IP7 { sub });
    }

    fn to_string(&self) -> String {
        let mut str = String::new();

        let mut hyper = false;
        for s in &self.sub {
            if hyper {
                str.push('[');
            }
            str.push_str(&s);
            if hyper {
                str.push(']');
            }
            hyper = !hyper;
        }

        str
    }

    fn is_abba(part: &String) -> bool {
        let list: Vec<char> = part.chars().collect();

        for idx in 0..list.len() - 3 {
            if list[idx] == list[idx + 3] && list[idx + 1] == list[idx + 2] && list[idx] != list[idx + 1] {
                return true;
            }
        }

        false
    }

    fn supports_tls(&self) -> bool {
        let mut i = 1;
        while i < self.sub.len() {
            if IP7::is_abba(&self.sub.get(i).unwrap()) {
                return false;
            }
            i += 2;
        }

        i = 0;
        while i < self.sub.len() {
            if IP7::is_abba(&self.sub.get(i).unwrap()) {
                return true;
            }
            i += 2;
        }

        false
    }

    fn find_aba(part : &String, abas : &mut HashSet<String>)  {
        let list: Vec<char> = part.chars().collect();

        for idx in 0..list.len() - 2 {
            if list[idx] == list[idx + 2] && list[idx] != list[idx + 1] {
                let mut aba = String::new();
                aba.push(list[idx]);
                aba.push(list[idx+1]);
                aba.push(list[idx+2]);
                abas.insert(aba);
            }
        }
    }

    fn has_bab(part : &String, abas : &HashSet<String>) -> bool {
        let list: Vec<char> = part.chars().collect();

        for idx in 0..list.len() - 2 {
            if list[idx] == list[idx + 2] && list[idx] != list[idx + 1] {
                let mut aba = String::new();
                aba.push(list[idx+1]);
                aba.push(list[idx]);
                aba.push(list[idx+1]);

                if abas.contains(&aba) {
                    return true;
                }
            }
        }

        false
    }

    fn supports_ssl(&self) -> bool {
        let mut abas : HashSet<String> = HashSet::new();

        let mut i = 0;
        while i < self.sub.len() {
            IP7::find_aba(&self.sub.get(i).unwrap(), &mut abas);
            i += 2;
        }

        i = 1;
        while i < self.sub.len() {
            if IP7::has_bab(&self.sub.get(i).unwrap(), &abas) {
                return true;
            }
            i += 2;
        }

        false
    }
}


pub fn part1(input: &Vec<String>) -> usize {
    input.iter()
        .map(|text| IP7::from(text).unwrap())
        .filter(|ip| ip.supports_tls())
        .count()
}

pub fn part2(input: &Vec<String>) -> usize {
    input.iter()
        .map(|text| IP7::from(text).unwrap())
        .filter(|ip| ip.supports_ssl())
        .count()
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use rstest::rstest;
    use crate::day7::IP7;
    use crate::ioc::lines_from_file;

    #[rstest]
    #[case("abba[mnop]qrst")]
    #[case("abcd[bddb]xyyx")]
    fn test_from(#[case] input: &str) {
        assert_eq!(input.to_string(), IP7::from(&input.to_string()).unwrap().to_string());
    }

    #[rstest]
    #[case("abba[mnop]qrst", true)]
    #[case("dabba[mnop]qrst", true)]
    #[case("abca[mnop]dabba", true)]
    #[case("abcd[bddb]xyyx", false)]
    #[case("aaaa[qwer]tyui", false)]
    #[case("ioxxoj[asdfgh]zxcvbn", true)]
    fn test_is_abba(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(IP7::from(&input.to_string()).unwrap().supports_tls(), expected);
    }

    #[rstest]
    #[case("aba[bab]xyz", true)]
    #[case("xyx[xyx]xyx", false)]
    #[case("aaa[kek]eke", true)]
    #[case("zazbz[bzb]cdb", true)]
    fn test_has_bab(#[case] input : &str, #[case] expected : bool) {
        assert_eq!(IP7::from(&input.to_string()).unwrap().supports_ssl(), expected);
    }
}