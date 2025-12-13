use crate::data_structure::inv_poly::InvPoly;

use crate::data_structure::graph::Graph;
use std::fmt::Write;

impl InvPoly {
    /// Print the given number of polynomials
    #[allow(dead_code)]
    fn print_terms(&self, length_to_print: usize) {
        let mut output = String::new();

        for d in self.d.iter().take(length_to_print) {
            if d.coefficient == 0 {
                continue;
            }

            write!(output, "{:>10} | ", d.coefficient).unwrap();

            for v in d.v.iter().filter(|&&v| v != 0) {
                write!(output, "{:3} ", v).unwrap();
            }
            output.push('\n');

            if output.len() > 8192 {
                print!("{}", output);
                output.clear();
            }
        }

        println!("{}", output);
    }

    /// Print the polynomial
    #[allow(dead_code)]
    pub fn print(&self) {
        self.print_terms(self.d.len());
    }

    /// Print the given number of polynomials
    #[allow(dead_code)]
    pub fn print_first(&self, length_to_print: usize) {
        self.print_terms(length_to_print);
    }

    /// Count terms
    #[allow(dead_code)]
    pub fn count_terms(&self) -> usize {
        self.d.iter().filter(|d| d.coefficient != 0).count()
    }

    /// Count terms and print the result
    #[allow(dead_code)]
    pub fn count_terms_and_print(&self) {
        println!("Number of terms: {}", self.count_terms());
    }
}

impl Graph {
    /// Print the graph
    #[allow(dead_code)]
    pub fn print(&self) {
        for v in self.vertex.iter() {
            println!("{:?}", v.neighbour);
        }
    }
}
