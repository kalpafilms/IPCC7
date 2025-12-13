use crate::config::NEIGHBOURHOOD;

/// Neighbourhood of a vertex
#[derive(Copy, Clone, Debug)]
pub struct Neighbourhood {
    pub neighbour: [u8; NEIGHBOURHOOD],
}

impl Default for Neighbourhood {
    fn default() -> Self {
        Neighbourhood {
            neighbour: [0; NEIGHBOURHOOD],
        }
    }
}
