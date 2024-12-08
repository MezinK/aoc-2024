fn find_xmas(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let directions = [
        (0, 1),   // Right
        (0, -1),  // Left
        (1, 0),   // Down
        (-1, 0),  // Up
        (1, 1),   // Down-Right
        (1, -1),  // Down-Left
        (-1, 1),  // Up-Right
        (-1, -1), // Up-Left
    ];

    let word = "XMAS";
    let word_len = word.len();
    let word_chars: Vec<char> = word.chars().collect();

    let mut count = 0;

    for row_idx in 0..rows {
        for col_idx in 0..cols {
            for &(dir_r, dir_c) in &directions {
                let mut matched = true;
                for (i, _) in word_chars.iter().enumerate().take(word_len) {
                    let nr = row_idx as isize + i as isize * dir_r;
                    let nc = col_idx as isize + i as isize * dir_c;

                    if nr < 0 || nr >= rows as isize || nc < 0 || nc >= cols as isize {
                        matched = false;
                        break;
                    }

                    if grid[nr as usize][nc as usize] != word_chars[i] {
                        matched = false;
                        break;
                    }
                }
                if matched {
                    count += 1;
                }
            }
        }
    }

    count
}

fn find_x_mas(grid: &[Vec<char>]) -> usize {
    let height = grid.len();
    let width = grid.first().unwrap().len();

    let mut ans = 0;

    for i in 1..height - 1 {
        for j in 1..width - 1 {
            if grid[i][j] != 'A' {
                continue;
            }

            if (grid[i - 1][j - 1] == 'M' && grid[i + 1][j + 1] == 'S'
                || grid[i - 1][j - 1] == 'S' && grid[i + 1][j + 1] == 'M')
                && (grid[i - 1][j + 1] == 'M' && grid[i + 1][j - 1] == 'S'
                    || grid[i - 1][j + 1] == 'S' && grid[i + 1][j - 1] == 'M')
            {
                ans += 1;
            }
        }
    }

    ans
}

fn create_grid_from_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

pub fn part1(input: &str) -> usize {
    let grid = create_grid_from_input(input);
    find_xmas(&grid)
}

pub fn part2(input: &str) -> usize {
    let grid = create_grid_from_input(input);
    find_x_mas(&grid)
}
