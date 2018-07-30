//! This is the core of our A.I. engine. "EANT2 with prune" is the algorithm we choose to implement and some
//! documentation can be found in this repository.
//! We choose EANT2 because of its capability to evolve its own structure and prune the connections
//! that are unnecessary. I find it very cool and powerful and I invit you to learn more about this
//! beautiful peace of art =].

extern crate rand;
pub mod activation;
pub mod cge;


#[cfg(test)]
mod activation_tests {
    use activation;

    #[test]
    fn activation_test_relu() {
        assert_eq!(0.5703423, activation::relu(0.5703423_f32));
        assert_eq!(0_f32, activation::relu(-10.5703423_f32));
    }


    #[test]
    fn activation_test_sigmoids() {
        assert_eq!(0.7310586_f32, activation::sigmoids(1_f32));
    }

}


#[cfg(test)]
mod network {
    use cge::network::Network;

    #[test]
    fn partial_eq() {
        assert_eq!(Network::new(16_i32, 9_i32), Network::new(16_i32, 9_i32));
    }
}


#[cfg(test)]
mod node_tests {
    use cge::node;

    #[test]
    fn neuron() {
        node::Neuron::new(0 as usize, 0.3_f32, 1_i32);
    }
}
