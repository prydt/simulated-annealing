use rand::Rng;
use std::f64;

pub trait SimulatedAnnealingProblem: Clone {
    fn calculate_energy(&self) -> u32;
    fn generate_neighbor(&self) -> Self;
    fn get_initial_state(puzzle_input: &str) -> Self;
}

pub fn solve_simulated_annealing<P: SimulatedAnnealingProblem>(
    initial_problem: P,
    initial_temp: f64,
    cooling_rate: f64,
    max_iterations: u32,
) -> P {
    let mut current_problem = initial_problem;
    let mut current_energy = current_problem.calculate_energy();
    let mut best_problem = current_problem.clone();
    let mut best_energy = current_energy;

    let mut temp = initial_temp;
    let mut rng = rand::rng();

    for i in 0..max_iterations {
        if current_energy == 0 {
            println!("Solution found at iteration {}", i);
            break;
        }

        let neighbor_problem = current_problem.generate_neighbor();
        let neighbor_energy = neighbor_problem.calculate_energy();

        let delta_e = neighbor_energy as f64 - current_energy as f64;

        if delta_e < 0.0 || rng.random::<f64>() < f64::exp(-delta_e / temp) {
            current_problem = neighbor_problem;
            current_energy = neighbor_energy;
        }

        if current_energy < best_energy {
            best_problem = current_problem.clone();
            best_energy = current_energy;
        }

        temp *= cooling_rate;

        if i % 1000 == 0 {
            println!("Iteration: {}, Temp: {:.2}, Current Energy: {}, Best Energy: {}", i, temp, current_energy, best_energy);
        }
    }

    best_problem
}