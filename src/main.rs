use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::Rng;
mod sixd;
mod tests;

fn main() {
    let file = File::open("bike_journeys.csv").unwrap();
    let reader = BufReader::new(file);

    let mut edges = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        // Ignoring comment lines
        if line.starts_with("#") {
            continue;
        }

        let parts: Vec<_> = line.split(",").collect();
        if parts.len() >= 8 { 
            let start_station = parts[3].trim().to_string(); 
            let end_station = parts[6].trim().to_string(); 
            edges.push((start_station, end_station)); 
        }
    }

    let graph = sixd::Graph::new(edges);
    let mut rng = rand::thread_rng();
    let random_node1 = rng.gen::<usize>() % graph.edges.len(); 
    let random_node2 = rng.gen::<usize>() % graph.edges.len();

    // Degree between 2 random nodes 
    if let Some(distance) = graph.six_degrees_of_separation(&graph.edges[random_node1].0, &graph.edges[random_node2].1) {
        println!(
            "The distance between nodes {} and {} is: {}",
            graph.edges[random_node1].0, graph.edges[random_node2].1, distance
        );
    } else {
        println!("Nodes are not connected within six degrees of separation.");
    }

    // Average degree of separation for specific nodes
    let average_degree_node_2 = graph.average_degree_of_separation("Moore Street, Soho"); 
    println!("Average degree of separation for Moore Street, Soho: {:.2}", average_degree_node_2);

    // Mean degree of separation
    let mean_degree = graph.mean_degree_of_separation();
    println!("Mean degree of separation: {:.2}", mean_degree);

    // Median degree of separation
    let median_degree = graph.median_degree_of_separation();
    println!("Median degree of separation: {:.2}", median_degree);
}
