mod config;
mod data_structure;
mod decrypt;
mod encrypt;
mod key_gen;
mod utils;

use crate::data_structure::inv_poly::InvPoly;
use decrypt::decrypt;
use encrypt::encrypt;
use key_gen::key_gen;

use tick_counter::TickCounter;

fn main() {
    // Message to encrypt from argument
    let args: Vec<String> = std::env::args().collect();
    let message: u32 = args[1].clone().parse().unwrap();

    // Public and secret key arrays
    let mut public_key = [[0u8; 2]; config::NUM_PUB_KEY];
    let mut secret_key = [0u8; config::NUM_PDS];

    // Cipher text
    let mut cipher_text = InvPoly::default();

    // Counters
    let total_start = std::time::Instant::now();
    let cycle_start = TickCounter::current();

    // Key generation
    let start = std::time::Instant::now();
    key_gen(&mut public_key, &mut secret_key);
    let elapsed = start.elapsed();
    println!("Key generation time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);

    // Encryption
    let start = std::time::Instant::now();
    encrypt(&mut cipher_text, &message, &public_key);
    let elapsed = start.elapsed();
    println!("Encryption time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);

    // Decryption
    let start = std::time::Instant::now();
    let decrypted = decrypt(&cipher_text, &secret_key);
    let elapsed = start.elapsed();
    println!("Decryption time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);

    let total_elapsed = total_start.elapsed();
    println!("Total execution time: {:.3} ms", total_elapsed.as_secs_f64() * 1000.0);
    let cycle_elapsed = cycle_start.elapsed();
    println!("Total clock cycle: {}", cycle_elapsed);

    println!(">> Decrypted message: {}", decrypted);
}
