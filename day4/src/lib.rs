fn find_xmas(grid: &Vec<Vec<char>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    let directions = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];

    for i in 0..rows {
        for j in 0..cols {
            for (dx, dy) in directions.iter() {
                let mut valid = true;
                let target = "XMAS";

                for (k, expected_char) in target.chars().enumerate() {
                    let new_i = i as i32 + dx * k as i32;
                    let new_j = j as i32 + dy * k as i32;

                    if new_i < 0
                        || new_i >= rows as i32
                        || new_j < 0
                        || new_j >= cols as i32
                        || grid[new_i as usize][new_j as usize] != expected_char
                    {
                        valid = false;
                        break;
                    }
                }

                if valid {
                    count += 1;
                }
            }
        }
    }

    count
}

fn save_the_world() -> (usize, usize) {
    let input = include_str!("../puzzle.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let result = find_xmas(&grid);

    (result, 0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_find_xmas() {
        assert_eq!(super::save_the_world(), (2462, 0));
    }
}
