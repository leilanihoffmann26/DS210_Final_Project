use std::collections::{HashMap, HashSet, VecDeque};

pub struct Graph {
    pub edges: Vec<(String, String)>,
}

impl Graph {
    pub fn new(edges: Vec<(String, String)>) -> Self {
        Graph { edges }
    }

    fn build_adjacency_list(&self) -> HashMap<String, Vec<String>> {
        let mut adjacency_list: HashMap<String, Vec<String>> = HashMap::new();

        for &(ref src, ref dst) in &self.edges {
            adjacency_list.entry(src.clone()).or_insert_with(Vec::new).push(dst.clone());
            adjacency_list.entry(dst.clone()).or_insert_with(Vec::new).push(src.clone());
        }

        adjacency_list
    }

    // Function to calculate six degrees of separation 
    pub fn six_degrees_of_separation(&self, start_station: &str, target_station: &str) -> Option<u32> {
        let adjacency_list = self.build_adjacency_list();

        let mut visited: HashSet<String> = HashSet::new();
        let mut queue: VecDeque<(&String, u32)> = VecDeque::new();

        queue.push_back((&start_station.to_string(), 0));

        while let Some((current_station, distance)) = queue.pop_front() {
            if current_station == target_station {
                return Some(distance);
            }

            if visited.contains(current_station) {
                continue;
            }

            visited.insert(current_station.clone());

            if let Some(neighbors) = adjacency_list.get(current_station) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        queue.push_back((neighbor, distance + 1));
                    }
                }
            }
        }

        None
    }
}
