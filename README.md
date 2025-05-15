# DS210_Final_Project

# 🚲 London Bike-Share Graph Analysis

This project analyzes bike ride data from the **Transport for London Cycle Hire System**, modeling the system as a graph to explore network connectivity, centrality, and urban mobility patterns.

## 📊 Dataset

- **Source:** [Kaggle - London Bike Share Usage Dataset](https://www.kaggle.com/datasets/kalacheva/london-bike-share-usage-dataset)
- **Rides:** 776,527 unique bike journeys
- **Fields Used:** Start station, end station, trip duration

## 📌 Project Summary

- Each **station** is treated as a node.
- Each **trip** is treated as a directed edge between nodes.
- Graph algorithms analyze:
  - Shortest path between stations
  - Average, mean, and median degrees of separation
  - Centrality of stations

## 🛠 File Descriptions

### `function.rs`

Contains the core `Graph` struct and key functions:
- `six_degrees_of_separation(start, end)`  
  Uses BFS to compute shortest path; returns `Ok(distance)` or error if unreachable.
- `average_degree_of_separation(start)`  
  Calculates average path length from a given station.
- `mean_degree_of_separation()` / `median_degree_of_separation()`  
  Aggregates global statistics on station connectivity.
- Private helper: `build_adjacency_list()` for BFS efficiency.

### `main.rs`

- Reads `bike_journeys.csv`
- Builds the graph using station names as nodes
- Calls graph analysis functions and prints results
- Handles I/O errors using `unwrap_or_else`

### `tests.rs`

Unit testing module with:
- `test_six_degrees_of_separation()`  
  Verifies correct distance between stations
- `test_average_degree_of_separation()`  
  Tests average separation logic
- `test_mean_and_median_degree_of_separation()`  
  Checks accuracy of summary statistics

## ✅ Results

- ✅ `six_degrees_of_separation` test: Passed  
- ❌ Average/Median separation tests: Failed (under review)

<img width="520" alt="Screenshot 2024-05-04 at 7 51 59 AM" src="https://github.com/leilanihoffmann26/DS210_Final_Project/assets/167572755/30ae34de-881f-467c-9b5d-230ccbfa7a67">

The code runs and produces output, but some metric calculations need debugging for full accuracy.

## 🔮 Future Work

- Debug and improve failing unit tests
- Add visualizations using Graphviz or Plotters
- Extend analysis to additional cities and datasets
- Explore real-time data applications

## 🧰 Technologies Used

- Rust (core language)
- CSV parsing (Rust standard library)
- Breadth-First Search (BFS)
- Unit testing with `#[cfg(test)]`

## 🙏 Acknowledgments

- [Kaggle Dataset by Kalacheva](https://www.kaggle.com/datasets/kalacheva/london-bike-share-usage-dataset)
- Rust documentation and community for graph algorithms and testing support

