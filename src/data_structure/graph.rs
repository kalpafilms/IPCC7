use crate::config;
use crate::data_structure::neighbourhood::Neighbourhood;

/// Graph
#[derive(Copy, Clone, Debug)]
pub struct Graph {
    pub vertex: [Neighbourhood; config::NUM_VERTEX],
}

impl Default for Graph {
    fn default() -> Self {
        Graph {
            vertex: std::array::from_fn(|i| {
                let mut neighbour = Neighbourhood::default();
                neighbour.neighbour[0] = i as u8;
                neighbour
            }),
        }
    }
}

impl Graph {
    /// Sort neighbourhoods per vertex.
    #[allow(dead_code)]
    fn sort_neighbourhood(&mut self) {
        for vertex in self.vertex.iter_mut() {
            vertex.neighbour.sort_unstable();
        }
    }

    /// Generate the graph from the public key edges
    ///
    /// @param `public_key`: Array of the public key edges
    /// @return Graph
    pub fn generate_from(public_key: &[[u8; 2]]) -> Self {
        let mut graph = Graph::default();

        // Set neighbourhoods
        for edge in public_key {
            let v1 = edge[0] as usize;
            let v2 = edge[1] as usize;

            // TODO: Handle vertex 0 cases
            if let Some(position) = graph.vertex[v1].neighbour.iter().position(|&n| n == 0) {
                graph.vertex[v1].neighbour[position] = v2 as u8;
            }
            if let Some(position) = graph.vertex[v2].neighbour.iter().position(|&n| n == 0) {
                graph.vertex[v2].neighbour[position] = v1 as u8;
            }
        }

        graph
    }
}
