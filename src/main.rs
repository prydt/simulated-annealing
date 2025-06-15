mod sa_solver;
mod sudoku;

use sudoku::Sudoku;
use crate::sa_solver::SimulatedAnnealingProblem;

fn main() {
    // Example Sudoku puzzle (0 represents empty cells)
    let puzzle = "300002478924007030000354200248010000000598040095040780002801900807400053001900000";
    // Alternative representation: 
    // "53..7....\n6..195...\n.98....6.\n8...6...3\n4..8.3..1\n7...2...6\n.6....28.\n...419..5\n....8..79"

    println!("Initial Sudoku puzzle:");
    let initial_sudoku = Sudoku::get_initial_state(puzzle);
    initial_sudoku.display();
    println!("Initial energy: {}", initial_sudoku.calculate_energy());

    // Simulated annealing parameters
    let initial_temp = 10.0;
    let cooling_rate = 0.9999999;
    let max_iterations = 500_000;

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