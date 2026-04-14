use super::matrix::ROWS;
use super::matrix::COLUMNS;

// sets all the numbers in a column, aside from the given row, to 0 using the given row and pivot
// assumes pivot is 1 (as it should be)
pub fn set_column_to_zero(
    mut matrix: [[f64; COLUMNS]; ROWS],
    row: usize,
    pivot: usize,
) -> [[f64; COLUMNS]; ROWS] {
    for r in 0..ROWS {
        if r == row {
            continue;
        }
        if matrix[r][pivot] == 0.0 {
            continue;
        }
        let d = -matrix[r][pivot];
        for c in 0..COLUMNS {
            matrix[r][c] = matrix[r][c] + d * matrix[row][c];
        }
    }
    return matrix;
}

// swaps two rows places
pub fn swap_rows(
    mut matrix: [[f64; COLUMNS]; ROWS],
    row1: usize,
    row2: usize,
) -> [[f64; COLUMNS]; ROWS] {
    let temp_row: [f64; COLUMNS] = matrix[row1];
    matrix[row1] = matrix[row2];
    matrix[row2] = temp_row;
    return matrix;
}

// sets pivot to 1 in given pivot
pub fn set_pivot(mut row: [f64; COLUMNS], pivot: usize) -> [f64; COLUMNS] {
    let pivot_value = row[pivot];
    for i in 0..COLUMNS {
        row[i] = row[i] / pivot_value;
    }
    return row;
}
