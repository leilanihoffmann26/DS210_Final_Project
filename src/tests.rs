#[cfg(test)]
mod tests {
    use crate::sixd::Graph;

    #[test]
    fn test_six_degrees_of_separation() {
        // Test graph
        let edges = vec![
            ("Kennington Lane Rail Bridge, Vauxhall".to_string(), "Albert Embankment, Vauxhall".to_string()),
            ("Kennington Lane Rail Bridge, Vauxhall".to_string(), "Albert Embankment, Vauxhall".to_string()),
            ("Euston Road, Euston".to_string(), "Baldwin Street, St. Luke's".to_string()),
            ("Old Brompton Road, South Kensington".to_string(), "Grosvenor Road, Pimlico".to_string()),
            ("Lower Marsh, Waterloo".to_string(), "Vauxhall Walk, Vauxhall".to_string()),
            ("Lower Marsh, Waterloo".to_string(), "Black Prince Road, Vauxhall".to_string()),
            ("Grant Road East, Clapham Junction".to_string(), "Usk Road, Clapham Junction".to_string()),
            ("Gascoyne Road, Victoria Park".to_string(), "Cadogan Place, Knightsbridge".to_string()),
            ("Kings Gate House, Westminster".to_string(), "Nevern Place, Earl's Court".to_string()),
            ("Kennington Road  , Vauxhall".to_string(), "Kennington Cross, Kennington".to_string()),
            ("Chicheley Street, South Bank".to_string(), "Tysoe Street, Clerkenwell".to_string()),
            ("Granby Street, Shoreditch".to_string(), "Reardon Street, Wapping".to_string()),
            ("Chicheley Street, South Bank".to_string(), "Tysoe Street, Clerkenwell".to_string()),
        ];
        let graph = Graph::new(edges);

        // Test case: Stations "Kennington Lane Rail Bridge, Vauxhall" and "Albert Embankment, Vauxhall" are connected within six degrees of separation
        assert_eq!(
            graph.six_degrees_of_separation("Kennington Lane Rail Bridge, Vauxhall", "Albert Embankment, Vauxhall"),
            Some(5)
        );
    }

    #[test]
    fn test_average_degree_of_separation() {
        // Test graph
        let edges = vec![
            ("Kennington Lane Rail Bridge, Vauxhall".to_string(), "Albert Embankment, Vauxhall".to_string()),
            ("Kennington Lane Rail Bridge, Vauxhall".to_string(), "Albert Embankment, Vauxhall".to_string()),
            ("Euston Road, Euston".to_string(), "Baldwin Street, St. Luke's".to_string()),
            ("Old Brompton Road, South Kensington".to_string(), "Grosvenor Road, Pimlico".to_string()),
            ("Lower Marsh, Waterloo".to_string(), "Vauxhall Walk, Vauxhall".to_string()),
            ("Lower Marsh, Waterloo".to_string(), "Black Prince Road, Vauxhall".to_string()),
            ("Grant Road East, Clapham Junction".to_string(), "Usk Road, Clapham Junction".to_string()),
            ("Gascoyne Road, Victoria Park".to_string(), "Cadogan Place, Knightsbridge".to_string()),
            ("Kings Gate House, Westminster".to_string(), "Nevern Place, Earl's Court".to_string()),
            ("Kennington Road  , Vauxhall".to_string(), "Kennington Cross, Kennington".to_string()),
            ("Chicheley Street, South Bank".to_string(), "Tysoe Street, Clerkenwell".to_string()),
            ("Granby Street, Shoreditch".to_string(), "Reardon Street, Wapping".to_string()),
            ("Chicheley Street, South Bank".to_string(), "Tysoe Street, Clerkenwell".to_string()),
        ];
        let graph = Graph::new(edges);
    
        // Test case: Average degree of separation for "Kennington Lane Rail Bridge, Vauxhall"
        let avg_degree_start_station = graph.average_degree_of_separation("Kennington Lane Rail Bridge, Vauxhall");
        assert_eq!(avg_degree_start_station, 4.5);
    
        // Test case: Average degree of separation for "Chicheley Street, South Bank"
        let avg_degree_end_station = graph.average_degree_of_separation("Chicheley Street, South Bank");
        assert_eq!(avg_degree_end_station, 2.67);
    }

    #[test]
    fn test_mean_and_median_degree_of_separation() {
        // Define a test graph
        let edges = vec![
            ("Kennington Lane Rail Bridge, Vauxhall".to_string(), "Albert Embankment, Vauxhall".to_string()),
            ("Kennington Lane Rail Bridge, Vauxhall".to_string(), "Albert Embankment, Vauxhall".to_string()),
            ("Euston Road, Euston".to_string(), "Baldwin Street, St. Luke's".to_string()),
            ("Old Brompton Road, South Kensington".to_string(), "Grosvenor Road, Pimlico".to_string()),
            ("Lower Marsh, Waterloo".to_string(), "Vauxhall Walk, Vauxhall".to_string()),
            ("Lower Marsh, Waterloo".to_string(), "Black Prince Road, Vauxhall".to_string()),
            ("Grant Road East, Clapham Junction".to_string(), "Usk Road, Clapham Junction".to_string()),
            ("Gascoyne Road, Victoria Park".to_string(), "Cadogan Place, Knightsbridge".to_string()),
            ("Kings Gate House, Westminster".to_string(), "Nevern Place, Earl's Court".to_string()),
            ("Kennington Road  , Vauxhall".to_string(), "Kennington Cross, Kennington".to_string()),
            ("Chicheley Street, South Bank".to_string(), "Tysoe Street, Clerkenwell".to_string()),
            ("Granby Street, Shoreditch".to_string(), "Reardon Street, Wapping".to_string()),
            ("Chicheley Street, South Bank".to_string(), "Tysoe Street, Clerkenwell".to_string()),
        ];
        let graph = Graph::new(edges);

        // Test case: Mean and Median degree of separation
        let mean_degree = graph.mean_degree_of_separation();
        let median_degree = graph.median_degree_of_separation();

        assert_eq!(mean_degree, 3.38);
        assert_eq!(median_degree, 3.0);
    }
}