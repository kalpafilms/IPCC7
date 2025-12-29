use crate::config;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

/// Generate public and secret keys
///
/// @param `public_key`: 2D array to store the generated public key
/// @param `secret_key`: 1D array to store the generated secret key
pub fn key_gen(public_key: &mut [[u8; 2]], secret_key: &mut [u8]) {
    // Generate for Secret Key PDS
    let mut shuffled_index = [0u8; config::NUM_VERTEX];
    shuffle(&mut shuffled_index);

    // Generate 4 PDSes (r + 1 PDSes)
    let mut pdses = [[0u8; config::NUM_PDS]; config::NEIGHBOURHOOD];
    for i in 0..config::NEIGHBOURHOOD {
        for j in 0..config::NUM_PDS {
            pdses[i][j] = shuffled_index[config::NEIGHBOURHOOD * j + i];
        }
    }

    // Choose a PDS, which does not contain 0, as the secret key
    let selected = pdses.iter().find(|pds| !pds.contains(&0u8)).unwrap();
    secret_key.copy_from_slice(selected);

    // Connect vertices between PDSes
    let pds_pairs = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
    for (pair_idx, (pds1_idx, pds2_idx)) in pds_pairs.iter().copied().enumerate() {
        for i in 0..config::NUM_PDS {
            add_edges(
                public_key,
                pair_idx * config::NUM_PDS + i,
                pdses[pds1_idx][i],
                pdses[pds2_idx][i],
            );
        }
    }
}

/// Shuffle the given array using the Fisher-Yates shuffle algorithm
///
/// @param `arr`: Array to shuffle
fn shuffle(arr: &mut [u8]) {
    // Use the cryptographic RNG(CSPRNG)
    let mut rng = StdRng::from_os_rng();

    // Fill the array from 0 to (length - 1)
    for i in 0..arr.len() {
        arr[i] = i as u8;
    }

    // Fisher-Yates shuffle algorithm
    for i in 0..(arr.len() - 1) {
        let index = rng.random_range(0..arr.len());
        arr.swap(i, index);
    }
}

/// Add a graph edge to the edge array.
///
/// @param `edges`: 2D array of edges
/// @param `idx`: Index of the edge array where to store the edge
/// @param `v1`: Vertex 1
/// @param `v2`: Vertex 2
fn add_edges(edges: &mut [[u8; 2]], idx: usize, v1: u8, v2: u8) {
    let high = v1.max(v2);
    let low = v1.min(v2);

    edges[idx] = [low, high];
}
