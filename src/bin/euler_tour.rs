use rand::random;
use rand::seq::SliceRandom;
use std::collections::HashMap;

#[allow(dead_code)]
fn create_tour(mut nodes: Vec<i32>) -> Vec<(i32, i32)> {
    let mut connected: Vec<i32> = vec![];
    let mut tour = vec![];
    let mut degree = HashMap::new();

    // We start by connecting two random nodes
    let first_node_random = remove_random(&mut nodes);
    let second_node_random = remove_random(&mut nodes);
    // And storing the first edge of the tour
    tour.push((first_node_random, second_node_random));
    // We register the two graphs as connected
    connected.push(first_node_random);
    connected.push(second_node_random);
    // And increase their valency / degree
    degree.insert(first_node_random, 1);
    degree.insert(second_node_random, 1);

    while nodes.len() > 0 {
        // We continue connecting loose nodes to registered graphs randomly
        let connected_random = connected.choose(&mut rand::thread_rng()).unwrap().clone();
        let loose_random = remove_random(&mut nodes);
        // and registering the new edges and vertexes
        connected.push(loose_random);
        tour.push((connected_random, loose_random));
        // updating the existing graphs' valencies and adding the new ones
        degree.insert(loose_random, 1);
        let current_degree = degree.entry(connected_random).or_insert(0);
        *current_degree += 1;
        // until there's no more loose nodes
    }

    // We split the connected graphs into odd and even ones by their valency
    let mut odd_nodes = vec![];
    let mut even_nodes = vec![];

    for (k, v) in &degree {
        if v % 2 != 0 {
            odd_nodes.push(*k);
        } else {
            even_nodes.push(*k);
        }
    }
    // Finally we search for unconnected graphs and update their valency
    // by connecting them until all graphs have even valencies
    while odd_nodes.len() > 0 {
        let connected_odd_random = remove_random(&mut odd_nodes);
        // by preference we connect graphs with odd valencies
        match unconnected(connected_odd_random, odd_nodes.clone(), &tour) {
            Some(odd_vertex) => {
                even_nodes.push(connected_odd_random);
                even_nodes.push(odd_vertex);
                tour.push((connected_odd_random, odd_vertex));
                remove_by_index(odd_vertex, &mut odd_nodes);
            }
            // and continue with vertexes with even valencies, if all odd ones are connected
            None => match unconnected(connected_odd_random, even_nodes.clone(), &tour) {
                Some(even_vertex) => {
                    even_nodes.push(connected_odd_random);
                    odd_nodes.push(even_vertex);
                    tour.push((connected_odd_random, even_vertex));
                    remove_by_index(even_vertex, &mut even_nodes);
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

fn remove_by_index(node: i32, nodes: &mut Vec<i32>) {
    let index = nodes.iter().position(|i| *i == node).unwrap();
    nodes.remove(index);
}

fn remove_random(nodes: &mut Vec<i32>) -> i32 {
    let index = (random::<f32>() * nodes.len() as f32).floor() as usize;
    nodes.remove(index)
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
