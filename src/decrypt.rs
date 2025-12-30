use crate::config;
use crate::data_structure::inv_poly::InvPoly;

/// Convert a secret key to a perfect dominating set(PDS) array
///
/// @param `secret_key`: The secret key used for decryption.
/// @return: The PDS array.
pub fn get_pds(secret_key: &[u8]) -> [u8; config::NUM_VERTEX] {
    // PDS array of the secret key
    let mut pds = [0u8; config::NUM_VERTEX];

    // Set a flag on the secret key vertex
    for &vertex in secret_key {
        if let Some(key) = pds.get_mut(vertex as usize) {
            *key = 1;
        }
    }

    pds
}

/// Decrypts the given `cipher_text` using the provided `secret_key` and returns the decrypted value.
///
/// @param `cipher_text`: The encrypted data that is represented by an `InvPoly` instance.
/// @param `secret_key`: The secret key used for decryption.
///
/// @return The decrypted value.
pub fn decrypt(cipher_text: &InvPoly, secret_key: &[u8]) -> u32 {
    let pds = get_pds(secret_key);

    cipher_text.decrypt_by(pds)
}
