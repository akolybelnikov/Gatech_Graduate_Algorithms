use std::cmp::max;

fn knapsack(wt: &Vec<i32>, val: &Vec<i32>, b: i32) -> Vec<Vec<i32>> {
    let mut grid = vec![vec![0; (b + 1) as usize]; (wt.len() + 1) as usize];

    for i in 1..(wt.len() + 1) as usize {
        for j in 1..(b + 1) {
            if wt[i - 1] <= j {
                grid[i][j as usize] = max(
                    grid[i - 1][j as usize],
                    grid[i - 1][(j - wt[i - 1]) as usize] + val[i - 1],
                );
            } else {
                grid[i][j as usize] = grid[i - 1][j as usize];
            }
        }
    }

    grid
}

fn get_knapsack_value(grid: &Vec<Vec<i32>>, w: usize, h: usize) -> i32 {
    grid[w][h]
}

fn get_knapsack_items(grid: &Vec<Vec<i32>>, wt: &Vec<i32>, mut h: usize) -> Vec<usize> {
    let mut items: Vec<usize> = Vec::new();
    let mut i: usize = wt.len();
    while i > 0 {
        if grid[i][h] == grid[i - 1][h] {
            items.insert(0, i - 2);
            h -= wt[i - 2] as usize;
        } else {
            if grid[i][h] == grid[i - 1][h] {
                items.insert(0, i - 2);
                h -= wt[i - 2] as usize;
            } else {
                items.insert(0, i - 1);
            }
        }
        i -= 1
    }
    items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knapsack_1() {
        assert_eq!(
            get_knapsack_value(
                &knapsack(&vec![6, 3, 4, 2], &vec![30, 14, 16, 9], 10),
                [6, 3, 4, 2].len(),
                10
            ),
            46
        );
    }

    #[test]
    fn test_knapsack_2() {
        assert_eq!(
            get_knapsack_value(
                &knapsack(
                    &vec![4, 2, 3, 5, 5, 6, 9, 7, 8, 10],
                    &vec![22, 20, 15, 30, 24, 54, 21, 32, 18, 25],
                    30
                ),
                [4, 2, 3, 5, 5, 6, 9, 7, 8, 10].len(),
                30
            ),
            182
        );
    }
}

fn main() {
    let weights = &vec![6, 3, 2, 4];
    let values = &vec![30, 14, 9, 16];
    let max_weight = 10;
    let knapsack_grid = &knapsack(weights, values, max_weight);
    let knapsack_value = get_knapsack_value(knapsack_grid, weights.len(), max_weight as usize);
    println!("{}", knapsack_value);
    let knapsack_items = get_knapsack_items(knapsack_grid, weights, max_weight as usize);
    println!("{:?}", knapsack_items);
}
