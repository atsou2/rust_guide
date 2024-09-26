use rand::{prelude::Distribution, rngs::StdRng, Rng, SeedableRng};
use statrs::distribution::MultivariateNormal;
pub struct State<const N_DIM: usize> {
    // Fill this
}

fn log_likelihood<const N_DIM: usize>(arr: &[f64]) -> f64 {
    // Define the mean vector and covariance matrix
    let mean = vec![0.0; N_DIM]; // Assuming mean is a zero vector
    let cov = vec![vec![1.0; N_DIM]; N_DIM]; // Assuming identity matrix for covariance

    // Create a MultivariateNormal distribution
    let mvn = MultivariateNormal::new(mean, cov).unwrap();

    // Compute the log likelihood
    mvn.logpdf(arr)
}

impl<const N_DIM: usize> State<N_DIM> {
    pub fn new(seed: u64) -> Self {
    // Fill this
    }

    pub fn take_step(&mut self) {
    // Fill this
    }
}
