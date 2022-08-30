fn to_vec(input: &String) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for part in input.trim().split_whitespace() {
        let side: i32 = part.trim().parse().unwrap();
        result.push(side);
    }

    result
}

fn to_vec_from_string_vec(input: &Vec<String>) -> Vec<Vec<i32>> {
    input.iter().map(|s| to_vec(s)).collect()
}

fn sort_vec(v : &Vec<i32>) -> Vec<i32> {
    let mut v2 = v.clone();
    v2.sort();
    v2
}

fn possible(v: &Vec<i32>) -> bool {
    let v2 = sort_vec(v);
    *v2.get(0).unwrap() + *v2.get(1).unwrap() > *v2.get(2).unwrap()
}

pub fn part1(input: &Vec<String>) -> usize {
    to_vec_from_string_vec(input)
        .iter()
        .filter(|v| possible(v))
        .count()
}

pub fn part2(input: &Vec<String>) -> usize {
    let list = to_vec_from_string_vec(input);
    let mut count = 0;

    let mut y = 0;
    while y < list.len() {
        for x in 0..3 {
            let v = vec![
                *list.get(y + 0).unwrap().get(x).unwrap(),
                *list.get(y + 1).unwrap().get(x).unwrap(),
                *list.get(y + 2).unwrap().get(x).unwrap(),
            ];

            if possible(&v) {
                count += 1;
            }
        }
        y += 3;
    }

    count
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::day3::{to_vec};

    #[rstest]
    #[case("  2  3  1  ", vec ! [1, 2, 3])]
    #[case("2 3 1", vec ! [1, 2, 3])]
    fn test_to_vec(#[case] input: &str, #[case] expected: Vec<i32>) {
        assert_eq!(expected, *to_vec(&String::from(input)));
    }
}
