mod matrix;
mod printing;
mod row_operations;

use crate::matrix::COLUMNS;
use crate::matrix::ROWS;

fn into_rref(mut matrix: [[f64; COLUMNS]; ROWS]) -> [[f64; COLUMNS]; ROWS] {
    let mut p = 0;
    for r in 0..ROWS {
        if matrix[r][p] == 0.0 {
            matrix = row_operations::swap_rows(matrix, r, ROWS - 1)
        }
        matrix[r] = row_operations::set_pivot(matrix[r], p);
        matrix = row_operations::set_column_to_zero(matrix, r, p);

        p += 1;
    }
    return matrix;
}

fn main() {
    // proper RREF solution is 1, -1, 2
    let matrix = matrix::MATRIX;
    let matrix = into_rref(matrix);
    printing::print_matrix(matrix);
}
