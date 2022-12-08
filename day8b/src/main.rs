fn main() {
    let grid: Vec<Vec<_>> = include_str!("../input")
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - 48).collect())
        .collect();

    let mut max_scenic_score = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let score = compute_scenic_score(i, j, &grid);
            max_scenic_score = max_scenic_score.max(score);
        }
    }

    println!("{}", max_scenic_score);
}

fn compute_scenic_score(i: usize, j: usize, grid: &Vec<Vec<u8>>) -> usize {
    let score_left = compute_scenic_score_left(i, j, grid);
    let score_right = compute_scenic_score_right(i, j, grid);
    let score_top = compute_scenic_score_top(i, j, grid);
    let score_bottom = compute_scenic_score_bottom(i, j, grid);

    score_left * score_right * score_top * score_bottom
}

fn compute_scenic_score_left(i: usize, j: usize, grid: &Vec<Vec<u8>>) -> usize {
    let mut score_left = 0;
    for k in (0..j).rev() {
        if grid[i][k] < grid[i][j] {
            score_left += 1;
        } else {
            score_left += 1;
            break;
        }
    }
    score_left
}

fn compute_scenic_score_right(i: usize, j: usize, grid: &Vec<Vec<u8>>) -> usize {
    let mut score_right = 0;
    for k in j + 1..grid.len() {
        if grid[i][k] < grid[i][j] {
            score_right += 1;
        } else {
            score_right += 1;
            break;
        }
    }
    score_right
}

fn compute_scenic_score_top(i: usize, j: usize, grid: &Vec<Vec<u8>>) -> usize {
    let mut score_top = 0;
    for k in (0..i).rev() {
        if grid[k][j] < grid[i][j] {
            score_top += 1;
        } else {
            score_top += 1;
            break;
        }
    }
    score_top
}

fn compute_scenic_score_bottom(i: usize, j: usize, grid: &Vec<Vec<u8>>) -> usize {
    let mut score_bottom = 0;
    for k in i + 1..grid.len() {
        if grid[k][j] < grid[i][j] {
            score_bottom += 1;
        } else {
            score_bottom += 1;
            break;
        }
    }
    score_bottom
}
