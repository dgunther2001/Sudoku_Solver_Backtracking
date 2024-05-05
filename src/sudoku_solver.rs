pub struct sudoku_solve { // generates a struct that contains a 9x9 sudoku grid
    arr: [[i32; 9]; 9],
}

impl sudoku_solve {

    pub fn init_solver(input_arr: [[i32; 9]; 9]) -> Self {  // takes a 9x9 i32 matrix
        Self { arr: input_arr }
    }

    pub fn print_puzzle(&self) { // iterates over the entire grid and prints the value
        for i in 0..9 { // TODO => DO THIS AS A fmt::Display instead...
            print!("[");
            for j in 0..9 {
                match self.arr[i][j] {
                    0 => print!("_"),
                    _ => print!("{}", self.arr[i][j]),
                } 
                match j {
                    8 => print!(""),
                    _ => print!(","), 
                }    
            }
            println!("]");
        }
    }

    pub fn solve(&mut self) { // calls the helper function where the algorithm is actually implemented
        self.solve_helper(0, 0); // start in the the first cell...
    }

    fn solve_helper(&mut self, row_num: usize, col_num: usize) -> bool {
        if row_num == 9 { // if we are out of bounds on the rows, it means we have solved the entire puzzle, so we return true...
            return true;
        } else if (col_num == 9) { // if we are out of bounds for columns, we recursively call this helper function on the next row in the first column
            return self.solve_helper(row_num + 1, 0);
        } else if (self.arr[row_num][col_num] != 0) { // if we encounter a row and a column that already has a value, we recursively call this helper function on the next cell
            return self.solve_helper(row_num, col_num + 1);
        } else { // this is where we actually insert new values... 

            // fundamentally create a recursive stack of the entire puzzle for each cell, that gets unwound and reset if we find an entry that completely invalidates he puzzle. We do this until true is returned for all entries...
            // element of: theta(9^(#empty cells)) => NOT GOOD, but constrained since Sudoku is always a 9x9 puzzle

            for val in 1..=9 { // iterate over all possible values that can go into the empy cell
                if self.validate(row_num, col_num, val) { // if the entry is validated (standard sudoku rules)...
                    self.arr[row_num][col_num] = val; // inserts the value into the matrix at the proper location IF it has been validated
                    if self.solve_helper(row_num, col_num + 1) { // recursive call to the next cell
                        return true; // RETURNS TRUE SO LONG AS THE NEXT NUMBER CAN BE INSERTED
                    }
                    self.arr[row_num][col_num] = 0; // if the next call didn't work, reset it and try with the next value (val)
                }
            }
            return false; // returns false if entry didn't work...
        }
    }

    fn validate(&self, i: usize, j: usize, val: i32) -> bool{ // validates a specific cell
        // 1: if val not in the row i
        // 2: if not in the column j
        // 3: not in subgrid (from floor(i/3)*3 to floor(i/3)*3 + 3) and the same for j...


        for a in 0..9 {
            // condition 1 => verify it's not in the row
            if self.arr[i][a] == val {
                return false
            }

            // condition 2 => verify it's not in the column
            if self.arr[a][j] == val {
                return false
            }
        }

        // condition 3 => verify it's not in the subgrid elememt (3 x 3)
        for row_num in ((i/3)*3)..((i/3)*3) + 3 {
            for col_num in ((j/3)*3)..((j/3)*3) + 3 {
                if self.arr[row_num][col_num] == val {
                    return false;
                }
            }
        }

        return true; // returns true if none of the invalidation conditions were met...
    }
}