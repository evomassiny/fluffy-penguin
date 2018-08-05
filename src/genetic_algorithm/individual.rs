

use rand::distributions::StandardNormal;
use rand::{thread_rng, Rng};
use cge::network::Network;
use cge::node::Node;

const LEARNING_RATE_THREASHOLD: f32 = 0.01;

pub struct Specimen<T> {
    input_size: usize,
    output_size: usize,
    // The ANN.
    pub ann: Network<T>,
    // Symbolizes how well an individual solves a problem.
    pub fitness: T,
}

impl Specimen<f32> {
    fn new(input_size: usize, output_size: usize) -> Self {
        Specimen { 
            input_size,
            output_size,
            ann: Network::<f32>::new(input_size, output_size),
            fitness: 0.0,
        }
    }


    /// The exploitation phase researches the optimal weight of each Node in the current artificial
    /// neural network.
    pub fn exploitation(&mut self) {
        // Number of chromosome defining the ANN genome.
        let n: f32 = self.ann.genome.len() as f32;

        // The proportionality constant.
        let tho: f32 = 1.0 / ( 2.0 * n.sqrt() ).sqrt();
        let tho_p: f32 = 1.0 / ( 2.0 * n ).sqrt();
    
        // Denotes a draw from the standard normal distribution.
        let draw_snd: f32 = thread_rng().sample(StandardNormal) as f32;
        

        for mut node in &mut self.ann.genome {
            // Learning rate value of the current chromosome.
            let rho: f32 = node.learning_rate;

            // denotes a separate draw from the standard normal distribution for each node.
            let draw_snd_i: f32 = thread_rng().sample(StandardNormal) as f32;

            // Compute the learning rate matated value.
            let mut rho_p: f32 = rho * ( tho_p * draw_snd + tho * draw_snd_i ).exp() as f32;

            // Since standard deviations very close to zero are unwanted (they will have on average
            // a negligible effect), the following boundary rule is used to force step
            // sizes to be no smaller than a pre-defined threshold.
            if rho_p < LEARNING_RATE_THREASHOLD { rho_p = LEARNING_RATE_THREASHOLD; }

            // Compute a new mutated connection weight.
            let w_p: f32 = node.w + rho_p * draw_snd_i;

            // Assign the new mutated learning rate value to the Node.
            node.learning_rate = rho_p;
            // Assign the new mutated weight to the Node.
            node.w = w_p;
        }

    }
}