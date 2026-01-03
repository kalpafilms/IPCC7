// Number of vertices
pub const NUM_VERTEX: usize = 256;
// 'r' of r-Regular graph
// const val REGULAR_SIZE = 3
// The number of PDSes in an r-regular graph is {r + 1}
// The size of each PDS is {n} / {r + 1}, where {n} is the number of vertices.
// We consider 3-regular graphs, i.e. r = 3
// NUM_VERTEX / 4 = 64
pub const NUM_PDS: usize = 64;
// 6 random one-to-one correspondences will be generated for each PDS
// 6 = 4C2 (Combination)
// NUM_PDS * 6 = 384
pub const NUM_PUB_KEY: usize = 384;

// Degree 7
pub const K: usize = 7;
// k' = floor(k/2)
pub const K1: usize = 3;
// k" = k - k'
pub const K2: usize = 4;

// (1 << (2 * K + 1)) * 2 = 65536
pub const TERM: usize = 65535;

// Each vertex from a 3-regular graph has 4 neighbourhoods
pub const NEIGHBOURHOOD: usize = 4;

// Message space \mathbb{Z}_p
pub const P: u32 = 2_147_483_648;

// A parameter used for generating invariant polynomials of degree 1
pub const N_E: u8 = 3;
