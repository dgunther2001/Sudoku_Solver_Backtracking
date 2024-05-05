mod sudoku_solver;
use sudoku_solver::sudoku_solve;


fn main() {
    let my_arr_1: [[i32; 9]; 9] = [[0,0,0,0,4,0,0,0,6],
                                 [4,1,0,5,2,0,0,0,0],
                                 [0,8,3,7,0,0,5,0,0],
                                 [3,0,0,8,0,0,0,0,0],
                                 [0,0,0,0,7,0,4,3,0],
                                 [0,2,4,0,0,0,0,6,7],
                                 [7,4,0,0,9,3,6,5,8],
                                 [0,0,0,0,8,1,0,0,2],
                                 [2,9,8,0,5,7,1,4,0]
                                 ];

    let mut my_solver = sudoku_solve::init_solver(my_arr_1);
    my_solver.print_puzzle();

    my_solver.solve();

    println!();

    my_solver.print_puzzle();


}

