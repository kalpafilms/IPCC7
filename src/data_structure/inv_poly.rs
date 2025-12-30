use crate::config;
use crate::data_structure::graph::Graph;
use crate::data_structure::poly_d::PolyD;

use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::Rng;
use std::fmt;

/// Invariant Polynomial
#[derive(Copy, Clone, Debug)]
pub struct InvPoly {
    // Polynomial terms
    pub d: [PolyD; config::TERM],
}

impl Default for InvPoly {
    fn default() -> Self {
        InvPoly {
            d: [PolyD::default(); config::TERM],
        }
    }
}

impl fmt::Display for InvPoly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for d in self.d.iter() {
            if d.coefficient == 0 {
                continue;
            }
            write!(f, "{:>10} | ", d.coefficient)?;

            for vertex in d.v.iter().filter(|&&v| v != 0) {
                write!(f, "{:3} ", vertex)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl InvPoly {
    /// Generate an Invariant Polynomial of degree 1
    fn gen_degree_1(
        &mut self,
        graph: &Graph,
        depend: bool,
        vertex_list: &mut [u8],
        value: &mut u32,
        rng: &mut StdRng,
    ) {
        let mut vertex_number = rng.random_range(0..=u8::MAX);
        let coefficient = rng.random_range(0..config::Q);
        // Find the first empty term in the polynomial
        let last_index = self
            .d
            .iter()
            .position(|d| d.coefficient == 0)
            .unwrap_or(config::TERM - config::NEIGHBOURHOOD);

        if depend {
            // Ensure vertex_number is not already in vertex_list
            while vertex_list.iter().any(|&v| v == vertex_number) {
                vertex_number = rng.random_range(0..=u8::MAX);
            }
            // Update vertex_list
            let base_idx = config::NEIGHBOURHOOD + (config::K - 1);
            for i in 0..config::NEIGHBOURHOOD {
                vertex_list[base_idx + i] = graph.vertex[vertex_number as usize].neighbour[i];
            }
        }

        // Add 4 new terms to the polynomial
        for i in 0..config::NEIGHBOURHOOD {
            let term = &mut self.d[last_index + i];
            term.v[0] = graph.vertex[vertex_number as usize].neighbour[i];
            term.coefficient = coefficient;
        }

        // Update the value
        *value += coefficient;
        *value %= config::Q;
    }

    /// Generate an Invariant Polynomial of degree k
    pub fn gen_degree_k(
        &mut self,
        graph: &Graph,
        depend: bool,
        vertex_list: &mut [u8],
        value: &mut u32,
        k: usize,
        message: &u32,
        max_k: usize,
        rng: &mut StdRng,
    ) {
        if k == 1 {
            let (cnt, idx) = if depend {
                let v = (rng.random_range(0..u8::MAX) % config::N_E) + 1;
                (v, Some(rng.random_range(0..v)))
            } else {
                (rng.random_range(0..config::NEIGHBOURHOOD as u8), None)
            };

            for i in 0..cnt {
                let is_depend = idx.map_or(false, |s| i == s);
                self.gen_degree_1(graph, is_depend, vertex_list, value, rng);
            }
        } else if k > 1 {
            let mp = if k == max_k {
                *message
            } else {
                rng.random_range(0..config::Q)
            };
            let vertex_number = if depend {
                std::iter::repeat_with(|| rng.random_range(0..=u8::MAX))
                    .find(|&v| !vertex_list.iter().any(|&vv| vv == v))
                    .unwrap()
            } else {
                rng.random_range(0..=u8::MAX)
            };
            if depend {
                // Update vertex_list for current dimension
                let base_idx = config::NEIGHBOURHOOD * (config::K - k);
                for i in 0..config::NEIGHBOURHOOD {
                    vertex_list[base_idx + i] = graph.vertex[vertex_number as usize].neighbour[i];
                }
            }

            let mut a = [0u32; config::NEIGHBOURHOOD];
            let s = rng.random_range(0..config::NEIGHBOURHOOD);
            for i in 0..config::NEIGHBOURHOOD {
                *value = 0;

                self.gen_degree_k(
                    graph,
                    depend && (i == s),
                    vertex_list,
                    value,
                    k - 1,
                    message,
                    max_k,
                    rng,
                );

                a[i] = (*value + config::Q - mp) % config::Q;
                *value = mp;

                // Update polynomial terms: append or update existing terms
                for term in self.d.iter_mut() {
                    if term.coefficient == 0 {
                        term.coefficient = config::Q - a[i];
                        for j in 0..k {
                            term.v[j] = graph.vertex[vertex_number as usize].neighbour[i];
                        }
                        break;
                    } else if term.v[k - 1] == 0 {
                        term.v[k - 1] = graph.vertex[vertex_number as usize].neighbour[i];
                    }
                }
            }
        }
    }

    /// Combine two polynomials
    /// @param `p1`: A reference to the first invariable polynomial to combine.
    /// @param `p2`: A reference to the second invariable polynomial to combine.
    pub fn combine_from(&mut self, p1: &Self, p2: &Self) {
        let mut cnt = 0;

        // Iterate through terms that have non-zero coefficients
        for d1 in p1.d.iter().filter(|d| d.coefficient != 0) {
            for d2 in p2.d.iter().filter(|d| d.coefficient != 0) {
                // Handle overflow: InvPoly.d.len() == config::TERM
                if cnt >= config::TERM {
                    return;
                }

                let target = &mut self.d[cnt];
                target.coefficient = ((d1.coefficient as u64 * d2.coefficient as u64) % config::Q as u64) as u32;
                target.v[..config::K1].copy_from_slice(&d1.v[..config::K1]);
                target.v[config::K1..].copy_from_slice(&d2.v[..config::K2]);

                cnt += 1;
            }
        }
    }

    /// Reduces the degree of terms by keeping only unique variables.
    pub fn reduce_degree(&mut self) {
        // Transfer values to a temporary object, and clear self.
        let source = std::mem::take(self);

        for i in 0..config::TERM {
            let mut src_term = source.d[i];
            if src_term.coefficient == 0 {
                break;
            }

            // Sort vertices ascending, but treat 0 as the largest value to push padding to the end
            src_term.v.sort_unstable_by_key(|&v| if v == 0 { u8::MAX } else { v });

            let mut offset = 0usize;
            let dest_term = &mut self.d[i];
            dest_term.coefficient = src_term.coefficient;

            let mut last_vertex: Option<u8> = None;
            for &v in src_term.v.iter() {
                // Break if the variable is 0, as it is considered as the padding value
                if v == 0 || offset == config::K {
                    break;
                }

                // Since `src_term.v` is sorted, we only need to compare the last added vertex
                if Some(v) != last_vertex {
                    dest_term.v[offset] = v;
                    offset += 1;
                    last_vertex = Some(v);
                }
            }
        }
    }

    /// Determine if two vertices are neighbours
    fn is_neighbour(v1: u8, v2: u8, graph: &Graph) -> bool {
        let n1 = graph.vertex[v1 as usize].neighbour;
        let n2 = graph.vertex[v2 as usize].neighbour;

        let mut i = 0usize;
        let mut j = 0usize;
        while i < config::NEIGHBOURHOOD && j < config::NEIGHBOURHOOD {
            if n1[i] == n2[j] {
                return true;
            } else if n1[i] < n2[j] {
                i += 1;
            } else {
                j += 1;
            }
        }

        false
    }

    /// Reduce neighbour terms (Proposition 4)
    /// @param `graph`: Representing the structure of the graph.
    pub fn reduce_terms(&mut self, graph: &Graph) {
        for term in self.d.iter_mut() {
            if term.coefficient == 0 {
                break;
            }

            'outer: for (idx, &v_j) in term.v.iter().enumerate() {
                if v_j == 0 || idx == config::K - 1 {
                    break;
                }

                for &v_k in term.v[idx + 1..].iter() {
                    if v_k == 0 {
                        break;
                    }

                    if Self::is_neighbour(v_j, v_k, graph) {
                        term.coefficient = 0;
                        break 'outer;
                    }
                }
            }
        }
    }

    /// Sum coefficients of polynomial terms to reduce duplicates
    pub fn sum_coefficients(&mut self) {
        let length = self.count_terms();

        // Identify and mark duplicates by setting coefficients to 0
        for i in 0..length - 1 {
            if self.d[i].coefficient == 0 {
                continue;
            }

            for j in i + 1..length {
                if self.d[j].coefficient == 0 {
                    continue;
                }

                let is_duplicate = self.d[i].v[0..config::K] == self.d[j].v[0..config::K];
                if is_duplicate {
                    // Sum the coefficient into the first occurrence and mark current as 0
                    let d_i_coeff = self.d[i].coefficient as u64;
                    let d_j_coeff = self.d[j].coefficient as u64;
                    self.d[i].coefficient = ((d_i_coeff + d_j_coeff) % config::Q as u64) as u32;
                    self.d[j].coefficient = 0;
                }
            }
        }

        // Transfer values to a temporary object, and clear self.
        let source = std::mem::take(self);

        // Copy non-zero terms to the result
        let mut cnt = 0usize;
        for i in 0..length {
            if source.d[i].coefficient != 0 {
                self.d[cnt] = source.d[i];
                cnt += 1;
            }
        }
    }

    /// Shuffle polynomial terms to hide the degree of the polynomial
    pub fn shuffle(&mut self, rng: &mut StdRng) {
        let length = self.count_terms();
        if length > 1 {
            self.d[0..length].shuffle(rng);
        }
    }

    /// Decrypt the cipher text polynomial
    pub fn decrypt(&self, pds: [u8; config::NUM_VERTEX]) -> u32 {
        let mut decrypted = 0u64;

        for i in 0..config::TERM {
            if i >= self.d.len() {
                break;
            }

            let term = &self.d[i];
            // Break if coefficients are zero (end of terms)
            if term.coefficient == 0 {
                break;
            }

            let mut sign = 1i64;
            for j in 0..config::K {
                let v_val = term.v[j];
                if v_val != 0 {
                    sign *= pds[v_val as usize] as i64;
                }
            }

            // Use rem_euclid to handle modular arithmetic with potential negative signs
            let temp = (term.coefficient as i64) * sign;
            let next_val = (decrypted as i64 + temp).rem_euclid(config::Q as i64);
            decrypted = next_val as u64;
        }

        decrypted as u32
    }
}
