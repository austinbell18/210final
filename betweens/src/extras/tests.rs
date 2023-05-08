use crate::extras::graph::Graph;

#[test]
//tests to check computation and sorting algorithms
fn test_compute() {
    let example = vec![(0,1), (0,2), (1,2), (1,3), (2,3), (2,4)];
    let graph = Graph::make_graph(5, &example);
    let centrality = Graph::betweenness_centrality(&graph);
    let expected_centrality = vec![0.0, 0.5, 3.5, 0.0, 0.0];

    for i in 0..5 {
        assert!((centrality[i] - expected_centrality[i]).abs() < 1e-9);
    }
}

#[test]
fn test_sort() {
    let centrality = vec![1.0, 4.0, 2.0, 5.0, 3.0];
    let sorted_centrality = Graph::sort_centrality(&centrality);
    let expected_sorted_centrality = vec![(3, 5.0), (1, 4.0), (4, 3.0), (2, 2.0), (0, 1.0)];
    assert_eq!(sorted_centrality, expected_sorted_centrality);
}