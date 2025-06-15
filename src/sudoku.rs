use rand::Rng;
use rand::seq::SliceRandom;
use rand::prelude::IndexedMutRandom;
use rand::seq::IteratorRandom;

#[derive(Clone)]
pub struct Sudoku {
    board: [[u8; 9]; 9],
    fixed_cells: [[bool; 9]; 9],
}

impl Sudoku {
    pub fn new(board: [[u8; 9]; 9], fixed_cells: [[bool; 9]; 9]) -> Self {
        Sudoku { board, fixed_cells }
    }

    // Display implementation for pretty printing
    pub fn display(&self) {
        for (i, row) in self.board.iter().enumerate() {
            if i % 3 == 0 && i != 0 {
                println!("------+-------+------");
            }
            for (j, cell) in row.iter().enumerate() {
                if j % 3 == 0 && j != 0 {
                    print!("| ");
                }
                if *cell == 0 {
                    print!(". ");
                } else {
                    print!("{} ", cell);
                }
            }
            println!();
        }
    }
}

impl super::sa_solver::SimulatedAnnealingProblem for Sudoku {
    fn calculate_energy(&self) -> u32 {
        let mut energy = 0;

        // Helper function to count duplicates in a slice
        fn count_duplicates(slice: &[u8]) -> u32 {
            let mut seen = [false; 10];
            let mut duplicates = 0;
            for &val in slice {
                if val != 0 {
                    if seen[val as usize] {
                        duplicates += 1;
                    }
                    seen[val as usize] = true;
                }
            }
            duplicates
        }

        // Check rows
        for row in &self.board {
            energy += count_duplicates(row);
        }

        // Check columns
        for c in 0..9 {
            let col: Vec<u8> = (0..9).map(|r| self.board[r][c]).collect();
            energy += count_duplicates(&col);
        }

        // Check 3x3 blocks
        for block_row in 0..3 {
            for block_col in 0..3 {
                let mut block = Vec::new();
                for r in 0..3 {
                    for c in 0..3 {
                        block.push(self.board[block_row * 3 + r][block_col * 3 + c]);
                    }
                }
                energy += count_duplicates(&block);
            }
        }

        energy
    }

    fn generate_neighbor(&self) -> Self {
        let mut new_board = self.board;
        let mut rng = rand::rng();

        // Randomly select a 3x3 block
        let block_row = rng.random_range(0..3);
        let block_col = rng.random_range(0..3);

        // Find all non-fixed cells within this block
        let mut cells_in_block = Vec::new();
        for r_offset in 0..3 {
            for c_offset in 0..3 {
                let r = block_row * 3 + r_offset;
                let c = block_col * 3 + c_offset;
                if !self.fixed_cells[r][c] {
                    cells_in_block.push((r, c));
                }
            }
        }

        // If there are at least two non-fixed cells in the block, swap two of them
        if cells_in_block.len() >= 2 {
            let chosen_cells = cells_in_block.as_slice().iter().choose_multiple(&mut rng, 2).into_iter().cloned().collect::<Vec<(usize, usize)>>();
            
            let (r1, c1) = chosen_cells[0];
            let (r2, c2) = chosen_cells[1];

            // Swap the values
            let temp = new_board[r1][c1];
            new_board[r1][c1] = new_board[r2][c2];
            new_board[r2][c2] = temp;
        } else {
            // Fallback: if not enough non-fixed cells in the chosen block,
            // pick a random non-fixed cell and change its value (original logic)
            let mut non_fixed_cells = Vec::new();
            for r in 0..9 {
                for c in 0..9 {
                    if !self.fixed_cells[r][c] {
                        non_fixed_cells.push((r, c));
                    }
                }
            }
            if let Some(&mut (r, c)) = non_fixed_cells.as_mut_slice().choose_mut(&mut rng) {
                let current_val = new_board[r][c];
                let mut new_val = rng.random_range(1..=9);
                while new_val == current_val {
                    new_val = rng.random_range(1..=9);
                }
                new_board[r][c] = new_val;
            }
        }

        Sudoku {
            board: new_board,
            fixed_cells: self.fixed_cells,
        }
    }

    fn get_initial_state(puzzle: &str) -> Self {
        let mut board = [[0; 9]; 9];
        let mut fixed_cells = [[false; 9]; 9];
        let mut rng = rand::rng();

        // Parse puzzle string
        let mut chars = puzzle.chars().filter(|c| c.is_ascii_digit() || *c == '.');
        for i in 0..81 {
            let r = i / 9;
            let c = i % 9;
            if let Some(ch) = chars.next() {
                if ch.is_ascii_digit() {
                    board[r][c] = ch.to_digit(10).unwrap() as u8;
                    fixed_cells[r][c] = true;
                }
            }
        }

        // Fill each 3x3 block with valid numbers
        for block_row in 0..3 {
            for block_col in 0..3 {
                let mut available: Vec<u8> = (1..=9).collect();
                
                // Remove fixed numbers in this block
                for r in 0..3 {
                    for c in 0..3 {
                        let val = board[block_row * 3 + r][block_col * 3 + c];
                        if val != 0 {
                            if let Some(pos) = available.iter().position(|&x| x == val) {
                                available.remove(pos);
                            }
                        }
                    }
                }
                
                // Shuffle remaining numbers
                available.shuffle(&mut rng);
                
                // Fill empty cells
                let mut index = 0;
                for r in 0..3 {
                    for c in 0..3 {
                        let abs_r = block_row * 3 + r;
                        let abs_c = block_col * 3 + c;
                        if board[abs_r][abs_c] == 0 {
                            board[abs_r][abs_c] = available[index];
                            index += 1;
                        }
                    }
                }
            }
        }

        Sudoku::new(board, fixed_cells)
    }
}