pub mod function {
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Graph {
    pub edges: Vec<(String, String)>,
}

impl Graph {
    pub fn new(edges: Vec<(String, String)>) -> Self {
        Graph { edges }
    }

    fn build_adjacency_list(edges: &[(String, String)]) -> HashMap<String, Vec<String>> {
        let mut adjacency_list: HashMap<String, Vec<String>> = HashMap::new();

        for &(ref src, ref dst) in edges {
            adjacency_list.entry(src.clone()).or_insert_with(Vec::new).push(dst.clone());
            adjacency_list.entry(dst.clone()).or_insert_with(Vec::new).push(src.clone());
        }

        adjacency_list
    }

    // Six degrees of separation 
    pub fn six_degrees_of_separation(&self, start_station: &str, target_station: &str) -> Result<u32, String> {
        // Check if start_station and target_station exist in the graph
        if !self.edges.iter().any(|(src, _)| src == start_station) {
            return Err(format!("Start station '{}' not found in the graph", start_station));
        }
        if !self.edges.iter().any(|(_, dst)| dst == target_station) {
            return Err(format!("Target station '{}' not found in the graph", target_station));
        }

        let adjacency_list = Self::build_adjacency_list(&self.edges);

        let mut visited: HashSet<String> = HashSet::new();
        let mut queue: VecDeque<(&str, u32)> = VecDeque::new();

        queue.push_back((&start_station, 0));

        while let Some((current_station, distance)) = queue.pop_front() {
            if current_station == target_station {
                return Ok(distance);
            }

            if visited.contains(current_station) {
                continue;
            }

            visited.insert(current_station.to_string());

            if let Some(neighbors) = adjacency_list.get(current_station) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        queue.push_back((neighbor.as_str(), distance + 1));
                    }
                }
            }
        }

        Err("Target station is not within six degrees of separation".to_string())
    }

    // Mean degree of separation
    pub fn mean_degree_of_separation(&self) -> f64 {
        let total_degrees: f64 = self.edges.iter()
            .map(|(src, _)| self.average_degree_of_separation(src))
            .sum();

        let total_nodes = self.edges.len() as f64;
        if total_nodes > 0.0 {
            let mean_degree = total_degrees / total_nodes;
            (mean_degree * 100.0).round() / 100.0
        } else {
            0.0
        }
    }

    // Median degree of separation
    pub fn median_degree_of_separation(&self) -> f64 {
        let mut degrees: Vec<f64> = self.edges.iter()
            .map(|(src, _)| self.average_degree_of_separation(src))
            .collect();

        degrees.sort_by(|a, b| a.partial_cmp(b).unwrap());

        if degrees.is_empty() {
            0.0
        } else if degrees.len() % 2 == 1 {
            degrees[degrees.len() / 2]
        } else {
            let mid = degrees.len() / 2;
            let median = (degrees[mid - 1] + degrees[mid]) / 2.0;
            median
        }
    }   

    // Average degree of separation
    pub fn average_degree_of_separation(&self, node: &str) -> f64 {
        let adjacency_list = Self::build_adjacency_list(&self.edges);

        let mut total_distance = 0;
        let mut total_paths = 0;

        let mut visited: HashSet<String> = HashSet::new();
        let mut queue: VecDeque<(String, u32)> = VecDeque::new();

        queue.push_back((node.to_string(), 0));

        while let Some((ref current_node, distance)) = queue.pop_front() {
            if visited.contains(current_node) {
                continue;
            }

            visited.insert(current_node.clone());

            if distance > 0 {
                total_distance += distance;
                total_paths += 1;
            }

            if let Some(neighbors) = adjacency_list.get(current_node) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        queue.push_back((neighbor.clone(), distance + 1));
                    }
                }
            }
        }

        if total_paths > 1 {
            (((total_distance as f64) / ((total_paths - 1) as f64)) * 100.0).round() / 100.0
        } else {
            0.0
        }
    }
}
}
