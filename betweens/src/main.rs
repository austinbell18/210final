use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;
use crate::extras::file_reader::read_file;
use crate::extras::tests;
use crate::extras::graph::Graph;

mod extras;

fn main() {
    //read file and call functions to initialize graph,
    //compute betweenness centrality, and sort it
    let (n, edges) = read_file();
    let graph = Graph::create_undirected(n, &edges);
    let centrality = Graph::betweenness_centrality(&graph);
    let sorted_centrality = Graph::sort_centrality(&centrality);
    
    //print nodes with the 10 highest centralities
    println!("The 10 highest centralities are: ");
    for (node, score) in sorted_centrality.iter().take(10) {
        println!("Node {}: {}", node, score);
    }
}