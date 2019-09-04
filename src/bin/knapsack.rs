use std::cmp::max;

fn knapsack(wt: &Vec<i32>, val: &Vec<i32>, b: i32) -> i32 {
    let mut grid = vec![vec![0; (b + 1) as usize]; (wt.len() + 1) as usize];

    for i in 1..(wt.len() + 1) {
        for j in 1..((b + 1)) {
            if j - wt[i-1] >= 0 {
                grid[i as usize][j as usize] = max(grid[(i-1) as usize][(j) as usize], grid[(i-1) as usize][(j-wt[i-1]) as usize] + val[i-1]);
            } else {
                grid[i as usize][j as usize] = grid[(i-1) as usize][j as usize];
            }
        }
    }
    println!("{:?}", grid[wt.len() as usize][b as usize]);
    grid[wt.len() as usize][b as usize]
}

fn main() {
    knapsack(&vec![6,3,4,2], &vec![30, 14, 16, 9], 10);
}