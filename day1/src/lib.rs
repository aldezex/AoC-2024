pub fn day1() -> u32 {
    let input = include_str!("../puzzle.txt").trim();
    let parts = input.split("\n");

    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    for part in parts {
        let mut parts = part.split("   ");
        let first = parts.next().unwrap().parse().unwrap();
        let second = parts.next().unwrap().parse().unwrap();
        left.push(first);
        right.push(second);
    }

    left.sort();
    right.sort();

    let mut pairs: Vec<(u32, u32)> = Vec::new();
    for (l, r) in left.iter().zip(right.iter()) {
        pairs.push((*l, *r));
    }

    let mut count = 0;

    for pair in pairs {
        match (pair.0, pair.1) {
            (first, second) if first > second => {
                count += first - second;
            }
            _ => {
                count += pair.1 - pair.0;
            }
        }
    }

    count
}

fn similarity_score() -> u32 {
    let input = include_str!("../puzzle.txt").trim();
    let parts = input.split("\n");

    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for part in parts {
        let mut parts = part.split("   ");
        let first = parts.next().unwrap().parse().unwrap();
        let second = parts.next().unwrap().parse().unwrap();
        left.push(first);
        right.push(second);
    }

    let mut score = 0;

    for num in left.iter() {
        let count = count(right.clone(), *num as u32);
        score += num * count as u32;
    }

    score
}

fn count(list: Vec<u32>, num: u32) -> usize {
    list.iter().filter(|&&x| x == num).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1() {
        assert_eq!(day1(), 1765812);
    }

    #[test]
    fn test_similarity_score() {
        assert_eq!(similarity_score(), 20520794);
    }
}
