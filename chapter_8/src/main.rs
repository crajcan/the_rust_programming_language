use std::collections::HashMap;

fn main() {}

fn mean(ints: Vec<i32>) -> Option<i32> {
    if ints.len() == 0 {
        return None;
    }

    Some(ints.clone().into_iter().sum::<i32>() / ints.len() as i32)
}

fn median(ints: Vec<i32>) -> Option<i32> {
    if ints.len() == 0 {
        return None;
    }

    match ints.len() % 2 {
        1 => Some(*ints.get(ints.len() / 2).unwrap()),
        _ => {
            let right = ints[(ints.len() / 2)];
            let left = ints[(ints.len() / 2) - 1];
            Some((left + right) / 2)
        }
    }
}

fn freqs(ints: &Vec<i32>) -> HashMap<i32, i32> {
    let mut result: HashMap<i32, i32> = HashMap::new();

    for i in ints {
        let count = result.entry(*i).or_insert(0);
        *count += 1;
    }

    result
}

fn most_frequent_key(freqs: HashMap<i32, i32>) -> i32 {
    let mut mode = 0;
    let mut mode_freq = 0;

    for (k, v) in &freqs {
        if v > &mode_freq {
            mode = *k;
            mode_freq = *v;
        }
    }

    mode
}

fn mode(ints: Vec<i32>) -> Option<i32> {
    match ints.len() {
        0 => return None,
        _ => Some(most_frequent_key(freqs(&ints))),
    }
}

fn pig_latin(word: &str) -> String {
    let first = word.chars().nth(0).unwrap();
    match first {
        'a' => word.to_owned() + "-hay",
        'e' => word.to_owned() + "-hay",
        'i' => word.to_owned() + "-hay",
        'o' => word.to_owned() + "-hay",
        'u' => word.to_owned() + "-hay",
        _ => {
            let mut result = word[1..].to_owned();
            result.push(first);
            result.push_str("ay");
            result
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mean() {
        assert_eq!(mean(vec![]), None);
        assert_eq!(mean(vec![7]), Some(7));
        assert_eq!(mean(vec![2, 2, 4, 5]), Some(3));
        assert_eq!(mean(vec![2, 2, 4, 5, 12]), Some(5));
        assert_eq!(mean(vec![-1, 2, 4, 5]), Some(2));
    }

    #[test]
    fn test_median() {
        assert_eq!(median(vec![]), None);
        assert_eq!(median(vec![7]), Some(7));
        assert_eq!(median(vec![2, 2, 4, 5]), Some(3));
        assert_eq!(median(vec![2, 2, 4, 5, 12]), Some(4));
        assert_eq!(median(vec![-1, 2, 4, 5]), Some(3));
    }

    #[test]
    fn test_mode() {
        assert_eq!(mode(vec![]), None);
        assert_eq!(mode(vec![7]), Some(7));
        assert_eq!(mode(vec![3, 4, -2, 5, -2, 11]), Some(-2));
    }

    #[test]
    fn test_pig_latin() {
        assert_eq!(pig_latin("first"), "irstfay".to_string(),);
        assert_eq!(pig_latin("apple"), "apple-hay".to_string(),);
    }
}
