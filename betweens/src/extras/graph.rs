use std::fs::File;
use std::collections::VecDeque;

type ListOfEdges = Vec<(usize, usize)>;
type AdjacencyLists = Vec<Vec<usize>>;

//graph struct and implements
#[derive(Debug)]
pub struct Graph {
    pub n: usize,
    pub outedges: AdjacencyLists,
}

impl Graph {
    pub fn add_edges(&mut self, edges: &ListOfEdges) {
        for (u, v) in edges {
            self.outedges[*u].push(*v);
        }
    }

    pub fn make_graph(n: usize, edges: &ListOfEdges) -> Graph {
        let mut graph = Graph {
            n,
            outedges: vec![vec![]; n],
        };

        graph.add_edges(edges);
        graph
    }

    pub fn bfs(g: &Graph, start: usize) -> Vec<usize> {
        //for a given node (start), run a breadth-first search to calculate
        //paths and distances of the paths
        let mut dist = vec![-1; g.n];
        let mut count = vec![0; g.n];
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(start);
        dist[start] = 0;
        count[start] = 1;

        while let Some(node) = queue.pop_front() {
            for &neigh in g.outedges[node].iter() {
                if dist[neigh] == -1 {
                    dist[neigh] = dist[node] + 1;
                    queue.push_back(neigh);
                }

                if dist[neigh] == dist[node] + 1 {
                    count[neigh] += count[node];
                }
            }
        }

        count
    }

    pub fn betweenness_centrality(g: &Graph) -> Vec<f64> {
        let mut centrality = vec![0.0; g.n];

        //run bfs for each node and calculate centralities for each node
        for node in 0..g.n {
            let count = Graph::bfs(&g, node);

            for i in 0..g.n {
                if i == node {
                    continue;
                }

                let node_dist = count[i] as f64;
                let total_count = count[node] as f64;
                let ratio = node_dist / total_count;
                centrality[i] += ratio;
            }
        }

        centrality.iter().map(|&c| c / 2.0).collect()
    }


    pub fn sort_centrality(centrality: &Vec<f64>) -> Vec<(usize, f64)> {
        //sort by node centrality and create a vec with format (node, centrality)
        let mut indexed_centrality: Vec<(usize, f64)> = centrality.iter().enumerate().map(|(i, &c)| (i, c)).collect();
        indexed_centrality.sort_by(|(_, c1), (_, c2)| c2.partial_cmp(c1).unwrap());
        indexed_centrality
    }

}