use crate::config::K;

/// Polynomial of degree K
#[derive(Copy, Clone, Debug)]
pub struct PolyD {
    pub v: [u8; K],
    pub coefficient: u32,
}

impl Default for PolyD {
    fn default() -> Self {
        PolyD {
            v: [0u8; K],
            coefficient: 0,
        }
    }
}
