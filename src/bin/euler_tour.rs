use rand::random;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::collections::HashSet;

fn create_tour(mut nodes: &mut Vec<i32>) -> Vec<(i32, i32)> {
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

fn get_degree(tour: &Vec<(i32, i32)>) -> HashMap<&i32, i32> {
    let mut degree = HashMap::new();
    for (x, y) in tour {
        let x_degree = degree.entry(x).or_insert(0);
        *x_degree += 1;
        let y_degree = degree.entry(y).or_insert(0);
        *y_degree += 1;
    }
    degree
}

fn check_edge(edge: &(i32, i32), node: &i32, nodes: &HashSet<i32>) -> Option<i32> {
    match edge {
        (x, y) if x == node => {
            if !nodes.contains(&y) {
                return Some(*y);
            } else {
                return None;
            }
        }
        (x, y) if y == node => {
            if !nodes.contains(&x) {
                return Some(*x);
            } else {
                return None;
            }
        }
        _ => None,
    }
}

fn connected_nodes(tour: &Vec<(i32, i32)>) -> HashSet<i32> {
    let first_vertex = tour[0].0;
    let mut nodes: HashSet<i32> = vec![first_vertex].into_iter().collect();
    let mut explore: HashSet<i32> = vec![first_vertex].into_iter().collect();
    while &explore.len() > &0 {
        let explored_node = explore
            .take(&explore.clone().into_iter().collect::<Vec<i32>>()[0])
            .unwrap();
        for edge in tour {
            let node = check_edge(&edge, &explored_node, &nodes);
            match node {
                Some(x) => {
                    nodes.insert(x);
                    explore.insert(x);
                }
                _ => (()),
            }
        }
    }

    nodes
}

pub fn is_eulerian_tour(nodes: &Vec<i32>, tour: &Vec<(i32, i32)>) -> bool {
    let mut is_eulerian = false;
    // All vertexes must have even valencies
    // Every node must be in the graph
    let degree = get_degree(tour);
    for node in nodes {
        let vertex_valency = degree.get(&node);
        match vertex_valency {
            Some(v) => {
                if v % 2 != 0 {
                    println!("Node {:?} has odd degree", node);
                    return is_eulerian;
                } else {
                    is_eulerian = true;
                }
            }
            _ => {
                println!("Node {:?} was not in your tour", node);
                return is_eulerian;
            }
        }
    }
    let connected = connected_nodes(tour);
    if connected.len() != nodes.len() {
        println!("Your graph was not connected");
        is_eulerian = false;
    }

    is_eulerian
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_eulerian() {
        let nodes: Vec<i32> = vec![20, 21, 22, 23, 24, 25, 26, 27];
        let tour = create_tour(&mut nodes.clone());
        assert_eq!(true, is_eulerian_tour(&nodes, &tour));
    }
}

fn main() {
    let mut nodes: &mut Vec<i32> = &mut vec![20, 21, 22, 23, 24, 25, 26, 27];
    let tour = create_tour(&mut nodes);
    println!("Eulerian tour graph: {:?}", tour);
}
