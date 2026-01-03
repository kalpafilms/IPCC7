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

    let mut g1 = InvPoly::default();
    let mut g2 = InvPoly::default();
    let m1 = 10u32;
    let m2 = *message / m1;

    let start = std::time::Instant::now();
    g1.gen_degree_k(
        &graph,
        true,
        &mut vertex_list,
        &mut value,
        config::K1,
        m1,
        config::K1,
        &mut rng,
    );
    vertex_list.fill(0);
    g2.gen_degree_k(
        &graph,
        true,
        &mut vertex_list,
        &mut value,
        config::K2,
        m2,
        config::K2,
        &mut rng,
    );
    let elapsed = start.elapsed();
    println!("> EncDegK time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);

    let start = std::time::Instant::now();
    cipher_text.combine_from(&g1, &g2);
    let elapsed = start.elapsed();
    println!("> Combine time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);

    let start = std::time::Instant::now();
    cipher_text.reduce_degree();
    let elapsed = start.elapsed();
    println!("> Reduce degrees time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);

    let start = std::time::Instant::now();
    cipher_text.reduce_terms(&graph);
    let elapsed = start.elapsed();
    println!("> Reduce term times: {:.3} ms", elapsed.as_secs_f64() * 1000.0);

    let start = std::time::Instant::now();
    cipher_text.sum_coefficients();
    let elapsed = start.elapsed();
    println!("> Sum coefficients time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);

    let start = std::time::Instant::now();
    cipher_text.shuffle(&mut rng);
    let elapsed = start.elapsed();
    println!("> Shuffle terms time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);
}
