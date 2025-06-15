mod sa_solver;
mod sudoku;

use sudoku::Sudoku;
use crate::sa_solver::SimulatedAnnealingProblem;

fn main() {
    // NYT Easy June 15, 2025
    let puzzle = "300002478924007030000354200248010000000598040095040780002801900807400053001900000";

    println!("Initial Sudoku puzzle:");
    let initial_sudoku = Sudoku::get_initial_state(puzzle);
    initial_sudoku.display();
    println!("Initial energy: {}", initial_sudoku.calculate_energy());

    // Simulated annealing parameters
    let initial_temp = 90.0;
    let cooling_rate = 0.999999;
    let max_iterations = 5_000_000;

    println!("\nRunning simulated annealing...");
    let solved_sudoku = sa_solver::solve_simulated_annealing(
        initial_sudoku,
        initial_temp,
        cooling_rate,
        max_iterations
    );

    println!("\nFinal solution:");
    solved_sudoku.display();
    println!("Final energy: {}", solved_sudoku.calculate_energy());

    if solved_sudoku.calculate_energy() == 0 {
        println!("\nSUCCESS: Valid Sudoku solution found!");
    } else {
        println!("\nWARNING: Solution not perfect. Energy: {}", solved_sudoku.calculate_energy());
    }
}