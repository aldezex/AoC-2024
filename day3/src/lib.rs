fn workworkwork() -> (i32, i32) {
    let input = include_str!("../puzzle.txt");
    let muls = find_muls(input);
    let does_muls = find_does_muls(input);

    (add_muls(muls), add_muls(does_muls))
}

fn find_muls(input: &str) -> Vec<&str> {
    let mut muls = Vec::new();
    let re = regex::Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    for cap in re.captures_iter(input) {
        muls.push(cap.get(0).unwrap().as_str());
    }

    muls
}

fn find_does_muls(input: &str) -> Vec<&str> {
    let mut muls = Vec::new();
    let mut enabled = true;
    let re = regex::Regex::new(r"(?:mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\))").unwrap();

    for cap in re.find_iter(input) {
        let instruction = cap.as_str();
        match instruction {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ if instruction.starts_with("mul") && enabled => {
                muls.push(instruction);
            }
            _ => {}
        }
    }

    muls
}

fn add_muls(muls: Vec<&str>) -> i32 {
    let mut sum = 0;
    for mul in muls {
        let nums = mul
            .trim_start_matches("mul(")
            .trim_end_matches(")")
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        sum += nums[0] * nums[1];
    }

    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_workworkwork() {
        assert_eq!(super::workworkwork(), (175015740, 112272912));
    }
}
