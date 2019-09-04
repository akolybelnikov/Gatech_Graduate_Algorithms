use std::cmp::max;

fn knapsack(wt: &Vec<i32>, val: &Vec<i32>, b: i32) -> i32 {
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
    println!("{}", grid[wt.len() as usize][b as usize]);
    grid[wt.len() as usize][b as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knapsack_1() {
        assert_eq!(knapsack(&vec![6, 3, 4, 2], &vec![30, 14, 16, 9], 10), 46);
    }

    #[test]
    fn test_knapsack_2() {
        assert_eq!(
            knapsack(
                &vec![4, 2, 3, 5, 5, 6, 9, 7, 8, 10],
                &vec![22, 20, 15, 30, 24, 54, 21, 32, 18, 25],
                30
            ),
            182
        );
    }
}

fn main() {
    knapsack(&vec![6, 3, 4, 2], &vec![30, 14, 16, 9], 10);
}
