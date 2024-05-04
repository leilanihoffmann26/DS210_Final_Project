# DS210_Final_Project

Intro

For this project, I analyzed London's bike-share usage data that contains information about 776,527 different individual bike rides from the Transport for London Cycle Hire System. This dataset was sourced from https://www.kaggle.com/datasets/kalacheva/london-bike-share-usage-dataset?resource=download. In order to construct a graph, I assigned each station as a node and the distance traveled between stations as an edge using the names of the start and end stations provided in the dataset. I thought that this was an interesting project because the results can provide valuable insights into urban transportation methods, bike station performance, and potentially shape sustainable endeavors of other large cities similar to London. 

function.rs

The graph struct in this file consists of functions that determine the connectivity of the nodes of the graph. At its core, the ‘Graph’ struct comprises a vector of edges that represents the connections between two nodes. One key function, ‘six_degrees_of_separation’ calculates the shortest path distance between two specific nodes using breadth-first search. I also implemented error handling to ensure that both the start and end stations exist in the graph, which returns an ‘Err’ if the station is not found and returns ‘Ok(distance)’ if the end station is found within six degrees of separation. If the end station is not reachable within six degrees of separation, an appropriate ‘Err’ statement is returned. Additionally, the ‘average_degree_of_separation’ function calculates the average distance from a given node to all other nodes in the graph, which helps to determine centrality. Lastly, ‘mean_degree_of_separation’ and ‘median_degree_of_separation’ are functions that compute the mean and median distances, respectively, utilizing all nodes in the graph. The ‘build_adjacency_list’ supporting function is a private function that allows for efficient traversal during breadth-first search by mapping each node to its list of neighboring nodes. 

main.rs

The code in the main.rs reads bike_journeys.csv and parses through the file. The unwrap_or_else method handles potential errors with the file opening. The code then constructs a graph where the edges represent the connection between nodes, which are selected at random. The functions for distance between nodes, average degree of separation, and mean and median degree of separation are all called, and the results are implemented on this file. 




tests.rs

The attribute #[cfg(test)] in this file ensures that the function is only used as a test function. There are three test functions that I decided to use for this project, which are ‘test_six_degrees_of_separation’, ‘test_average_degree_of_separation,’ and ‘test_mean_and_median_degree_of_separation’. The first test checks that two stations are connected within six degrees of separation within the network of bicycle transportation by creating a graph with specific edges and stating that the distance between two stations is 5. The other functions test the validity of the average, mean, and median degree of separation from graphs with specific edges. 


Results

For the results of my project, I have included a screenshot of the tests below. The six_degrees_of_separation test passed, while the other two tests failed. Although the code runs, I was unable to determine the issue with the failed tests. Overall, I learned a lot from this project, and I believe that the methods and results can be applied to other data sets in order to learn more about the efficiency of transportation systems in certain cities.
<img width="520" alt="Screenshot 2024-05-04 at 7 51 59 AM" src="https://github.com/leilanihoffmann26/DS210_Final_Project/assets/167572755/30ae34de-881f-467c-9b5d-230ccbfa7a67">
