use rand::random;
use rand::seq::SliceRandom;
use std::collections::HashMap;

#[allow(dead_code)]
fn create_tour(mut nodes: Vec<i32>) -> Vec<(i32, i32)> {
    let mut connected: Vec<i32> = vec![];
    let mut tour = vec![];
    let mut degree = HashMap::new();

    let index_x = (random::<f32>() * nodes.len() as f32).floor() as usize;
    let x = nodes.remove(index_x);
    degree.insert(x, 1);

    let index_y = (random::<f32>() * nodes.len() as f32).floor() as usize;
    let y = nodes.remove(index_y);
    degree.insert(y, 1);

    connected.push(x);
    connected.push(y);
    tour.push((x, y));

    while nodes.len() > 0 {
        let x = connected.choose(&mut rand::thread_rng()).unwrap().clone();
        let index_y = (random::<f32>() * nodes.len() as f32).floor() as usize;
        let y = nodes.remove(index_y);
        connected.push(y);
        degree.insert(y, 1);
        tour.push((x, y));
        let current_degree = degree.entry(x).or_insert(0);
        *current_degree += 1;
    }

    let mut odd_nodes = vec![];
    let mut even_nodes = vec![];

    for (k, v) in &degree {
        if v % 2 != 0 {
            odd_nodes.push(*k);
        } else {
            even_nodes.push(*k);
        }
    }

    while odd_nodes.len() > 0 {
        let index_x = (random::<f32>() * odd_nodes.len() as f32).floor() as usize;
        let x = odd_nodes.remove(index_x);

        match unconnected(x, odd_nodes.clone(), &tour) {
            Some(y) => {
                even_nodes.push(x);
                even_nodes.push(y);
                tour.push((x, y));
                let index_y = odd_nodes.iter().position(|i| *i == y).unwrap();
                odd_nodes.remove(index_y);
            }
            None => match unconnected(x, even_nodes.clone(), &tour) {
                Some(z) => {
                    even_nodes.push(x);
                    let index_z = even_nodes.iter().position(|i| *i == z).unwrap();
                    even_nodes.remove(index_z);
                    odd_nodes.push(z);
                    tour.push((x, z));
                }
                None => (()),
            },
        }
    }

    return tour.to_vec();
}

fn unconnected(x: i32, nodes: Vec<i32>, tour: &Vec<(i32, i32)>) -> Option<i32> {
    nodes.into_iter().find(|&y| !is_connected(tour, x, y))
}

fn is_connected(tour: &Vec<(i32, i32)>, x: i32, y: i32) -> bool {
    let mut result = false;
    for (a, b) in tour {
        if (x == *a && y == *b) || (x == *b && y == *a) {
            result = true;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_eulerian_tour() {
        let nodes: Vec<i32> = vec![20, 21, 22, 23, 24, 25, 26, 27];
        let tour = create_tour(nodes);
        assert_eq!(2, tour.len());
    }
}

fn main() {
    let nodes: Vec<i32> = vec![20, 21, 22, 23, 24, 25, 26, 27];
    let tour = create_tour(nodes);
    println!("Eulerian tour graph: {:?}", tour);
}
