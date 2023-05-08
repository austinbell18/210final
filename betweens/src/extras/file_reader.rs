use std::fs::File;
use std::io::{BufRead, BufReader};

type ListOfEdges = Vec<(usize, usize)>;

//function to read the file in and calculate the number of nodes
pub fn read_file() -> (usize, Vec<(usize, usize)>) {
    let mut result: Vec<(usize, usize)> = Vec::new();
    let file = File::open("C:\\users\\austi\\Desktop\\Email-Enron.txt").expect("Could not open file");
    let buf_reader = BufReader::new(file).lines();
    
    //read each line, unwrap line and split at the tab (\t)
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split('\t').collect();
        let x: usize = v[0].parse::<usize>().unwrap();
        let y: usize = v[1].parse::<usize>().unwrap();
        result.push((x, y));
    }
    
    //iterate through list of edges and find the node w/ the highest value
    //add 1 to find length (first node is 0)
    let max_node = result.iter().map(|&(x, y)| usize::max(x, y)).max().unwrap();
    let n = max_node + 1;
    
    (n, result)
}