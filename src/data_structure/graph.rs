use crate::config::NUM_VERTEX;
use crate::data_structure::neighbourhood::Neighbourhood;

/// Graph
#[derive(Copy, Clone, Debug)]
pub struct Graph {
    pub vertex: [Neighbourhood; NUM_VERTEX],
}

impl Default for Graph {
    fn default() -> Self {
        Graph {
            vertex: [Neighbourhood::default(); NUM_VERTEX],
        }
    }
}

impl Graph {
    /// Sort neighbourhoods per vertex.
    fn sort_neighbourhood(&mut self) {
        for vertex in self.vertex.iter_mut() {
            vertex.neighbour.sort_unstable();
        }
    }

    /// Generate the graph from the public key edges
    ///
    /// @param edges Array of the public key edges
    /// @return Graph
    pub fn generate_from(edges: &[[u8; 2]]) -> Self {
        // Index 0 is the vertex itself, so iterate from 1
        let mut idx = [1usize; NUM_VERTEX];
        let mut graph = Graph::default();

        for (i, v) in graph.vertex.iter_mut().enumerate() {
            v.neighbour[0] = i as u8;
        }

        for edge in edges {
            let a = edge[0] as usize;
            let b = edge[1] as usize;

            graph.vertex[a].neighbour[idx[a]] = b as u8;
            idx[a] += 1;
            graph.vertex[b].neighbour[idx[b]] = a as u8;
            idx[b] += 1;
        }

        // Sort neighbourhoods per vertex
        graph.sort_neighbourhood();

        graph
    }
}
