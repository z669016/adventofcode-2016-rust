use std::cmp::Ordering;
use std::collections::HashMap;

fn count_char(input : &Vec<String>) -> Vec<HashMap<char,i32>> {
    let mut count : Vec<HashMap<char,i32>> = Vec::new();

    for word in input {
        let mut pos = 0;
        for ch in word.chars() {
            if pos == count.len() {
                count.push(HashMap::new());
            }
            let map = count.get_mut(pos).unwrap();
            map.entry(ch)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);

            pos += 1;
        }
    }

    count
}

fn compile_message(count : &Vec<HashMap<char,i32>>, compare: fn(&i32, &i32) -> Ordering) -> String {
    let mut result = String::new();

    for map in count {
        let mut list : Vec<(&char,&i32)>= map.iter()
            .collect();
        list.sort_by(|a, b| compare(a.1, b.1));

        result.push(*list.get(0).unwrap().0);
    }

    result
}

pub fn part1(input: &Vec<String>) -> String {
    let count = count_char(&input);
    compile_message(&count, |a, b| b.cmp(a))
}

pub fn part2(input: &Vec<String>) -> String {
    let count = count_char(&input);
    compile_message(&count, |a, b| a.cmp(b))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::day6::{compile_message, count_char};

    #[rstest]
    fn test() {
        let input : Vec<String> = vec![
            "eedadn".to_string(),
            "drvtee".to_string(),
            "eandsr".to_string(),
            "raavrd".to_string(),
            "atevrs".to_string(),
            "tsrnev".to_string(),
            "sdttsa".to_string(),
            "rasrtv".to_string(),
            "nssdts".to_string(),
            "ntnada".to_string(),
            "svetve".to_string(),
            "tesnvt".to_string(),
            "vntsnd".to_string(),
            "vrdear".to_string(),
            "dvrsen".to_string(),
            "enarar".to_string(),
        ];
        let count = count_char(&input);
        let word = compile_message(&count, |a, b| b.cmp(a));
        assert_eq!("easter", word.as_str());

        let word = compile_message(&count, |a, b| a.cmp(b));
        assert_eq!("advent", word.as_str());
    }
}
