use crate::config;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

/// Generate public and secret keys
///
/// @param `public_key`: 2D array to store the generated public key
/// @param `secret_key`: 1D array to store the generated secret key
pub fn key_gen(public_key: &mut [[u8; 2]], secret_key: &mut [u8]) {
    // Use the cryptographic RNG(CSPRNG)
    let mut rng = StdRng::from_os_rng();

    // Generate shuffled vertices for PDSes
    let mut shuffled_index: [u8; config::NUM_VERTEX] = std::array::from_fn(|i| i as u8);
    shuffled_index.shuffle(&mut rng);

    // Generate 4 PDSes (r + 1 PDSes)
    let mut pdses = [[0u8; config::NUM_PDS]; config::NEIGHBOURHOOD];
    // Since `shuffled_index` is already shuffled, so handle vertices as chunks instead of single elements.
    // Set `chunks_exact` as the outer loop to prevent re-calculating the chunks.
    for (i, chunk) in shuffled_index.chunks_exact(config::NEIGHBOURHOOD).enumerate() {
        for (pds, &vertex) in pdses.iter_mut().zip(chunk) {
            pds[i] = vertex;
        }
    }

    // Choose a PDS, which does not contain 0, as the secret key
    // This guarantees that only one PDS contains vertex 0, since the vertex indices are 0 to 255.
    let selected = pdses.iter()
        .find(|pds| !pds.contains(&0))
        .expect("At least one PDS does not contain vertex 0");
    secret_key.copy_from_slice(selected);

    // Six random one-to-one correspondences between the PDSes
    let mut key_chunks = public_key.chunks_mut(config::NUM_PDS);
    // (pds1, pds2): [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]
    for pds1 in 0..config::NEIGHBOURHOOD {
        for pds2 in (pds1 + 1)..config::NEIGHBOURHOOD {
            if let Some(chunk) = key_chunks.next() {
                for i in 0..config::NUM_PDS {
                    let v1 = pdses[pds1][i];
                    let v2 = pdses[pds2][i];
                    chunk[i] = if v1 < v2 { [v1, v2] } else { [v2, v1] };
                }
            }
        }
    }
}
