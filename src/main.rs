/*
 * Get a 1 in top left
 * Add mults of row to get coll of 0s
 * Move down 1 and right until you hit a non-zero
 *  if no nonzero, move that row to last and repeat
 * Repeat 1 and 2
 * Repeat
 * Left with REF
 *
 * Start at top left
 * For each column:
 *  find row at or below curr with nonzero entry in column
 *  None? move right one
 *  Found? Swap row with curr row
 * Scale so pivot = 1
 * make all other entries in column 0
 * Repeat
 */

const ROWS: usize = 3;
const COLUMNS: usize = 4;

// sets all the numbers in a column, aside from the given row, to 0 using the given row and pivot
// assumes pivot is 1 (as it should be)
fn set_rows_to_zero(
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
fn swap_rows(
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
fn set_pivot(mut row: [f64; COLUMNS], pivot: usize) -> [f64; COLUMNS] {
    let pivot_value = row[pivot];
    for i in 0..COLUMNS {
        row[i] = row[i] / pivot_value;
    }
    return row;
}

// start function for REF
fn into_ref(matrix: [[f64; COLUMNS]; ROWS]) {}

// start function for RREF
fn into_rref(matrix: [[f64; COLUMNS]; ROWS]) {}

fn print_matrix(matrix: [[f64; COLUMNS]; ROWS]) {
    for i in 0..ROWS {
        print_row(matrix[i]);
    }
}

fn print_row(row: [f64; COLUMNS]) {
    let row_as_str: String = row
        .iter()
        .map(|f| f.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    println!("{}", row_as_str);
}

fn main() {
    // proper RREF solution is 1, -1, 2
    let mut matrix: [[f64; COLUMNS]; ROWS] = [
        [2.0, -5.0, 5.0, 17.0],
        [0.0, 1.0, 3.0, 5.0],
        [1.0, -2.0, 3.0, 9.0],
    ];
}
