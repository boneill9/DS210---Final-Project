use graphlib::Graph;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Clone, Deserialize)]
struct TikToker {
    name: String,
    likes: u32,
    comments: u32,
    shares: u32,
}

pub fn read_tiktokers_from_csv(file_path: &str) -> Result<Vec<TikToker>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(file_path)?;
    let mut tiktokers = Vec::new();

    for result in reader.deserialize() {
        let tiktoker: TikToker = result?;
        tiktokers.push(tiktoker);
    }

    Ok(tiktokers)
}

pub fn build_similarity_graph(tiktokers: Vec<TikToker>, thresholds: (u32, u32, u32)) -> Graph<String> {
    let mut graph = Graph::new();

    // Add nodes
    let mut node_indices = Vec::new();
    for tiktoker in &tiktokers {
        let node_index = graph.add_node(tiktoker.name.clone());
        node_indices.push((tiktoker, node_index));
    }

    // Add edges based on similarity
    let (like_threshold, comment_threshold, share_threshold) = thresholds;
    for i in 0..tiktokers.len() {
        for j in (i + 1)..tiktokers.len() {
            let t1 = &tiktokers[i];
            let t2 = &tiktokers[j];

            if (t1.likes as i32 - t2.likes as i32).abs() <= like_threshold as i32 &&
               (t1.comments as i32 - t2.comments as i32).abs() <= comment_threshold as i32 &&
               (t1.shares as i32 - t2.shares as i32).abs() <= share_threshold as i32 {
                // Add an edge between the nodes
                graph.add_edge(node_indices[i].1, node_indices[j].1);
            }
        }
    }

    graph
}

pub fn visualize_graph(graph: &Graph<String>, output_path: &str) {
    let mut dot_output = String::new();

    // Generate DOT format
    dot_output.push_str("graph G {\n");
    for node in graph.nodes() {
        dot_output.push_str(&format!("    {};\n", node.data));
    }
    for (a, b) in graph.edges() {
        dot_output.push_str(&format!("    {} -- {};\n", a.data, b.data));
    }
    dot_output.push_str("}\n");

    // Write DOT file to disk
    std::fs::write(output_path, dot_output).expect("Unable to write DOT file");
}