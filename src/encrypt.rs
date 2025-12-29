use crate::config;
use crate::data_structure::graph::Graph;
use crate::data_structure::inv_poly::InvPoly;

use rand::rngs::StdRng;
use rand::SeedableRng;

pub fn encrypt(cipher_text: &mut InvPoly, message: &u32, public_key: &[[u8; 2]]) {
    // Use the cryptographic RNG(CSPRNG)
    let mut rng = StdRng::from_os_rng();

    let graph = Graph::generate_from(&public_key);
    let mut vertex_list = [0u8; config::NUM_VERTEX];
    let mut value = 0u32;

    let mut p1 = InvPoly::default();
    let mut p2 = InvPoly::default();
    let mut m1 = 10u32;
    let mut m2 = *message / m1;

    // Temporary InvPoly for hiding phase
    let mut tmp = InvPoly::default();

    let start = std::time::Instant::now();
    p1.gen_degree_k(
        &graph,
        true,
        &mut vertex_list,
        &mut value,
        config::K1,
        &mut m1,
        config::K1,
        &mut rng,
    );
    vertex_list.fill(0);
    p2.gen_degree_k(
        &graph,
        true,
        &mut vertex_list,
        &mut value,
        config::K2,
        &mut m2,
        config::K2,
        &mut rng,
    );
    let elapsed = start.elapsed();
    println!("> EncDegK time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);

    let start = std::time::Instant::now();
    cipher_text.combine_from(&p1, &p2);
    let elapsed = start.elapsed();
    println!("> Combine time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);

    let start = std::time::Instant::now();
    cipher_text.reduce_degree_to(&mut tmp);
    let elapsed = start.elapsed();
    println!("> Reduce degrees time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);

    let start = std::time::Instant::now();
    tmp.sort_variable();
    let elapsed = start.elapsed();
    println!("> Sort variables time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);

    let start = std::time::Instant::now();
    tmp.reduce_terms(&graph);
    let elapsed = start.elapsed();
    println!("> Reduce term times: {:.3} ms", elapsed.as_secs_f64() * 1000.0);

    let start = std::time::Instant::now();
    tmp.sum_coefficients(cipher_text);
    let elapsed = start.elapsed();
    println!("> Sum coefficients time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);

    let start = std::time::Instant::now();
    cipher_text.shuffle(&mut rng);
    let elapsed = start.elapsed();
    println!("> Shuffle terms time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);
}
