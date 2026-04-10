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

// sets all the numbers in a column, aside from the given row, to 0 using the given row
fn set_columns_to_zero(matrix: [[f64; COLUMNS]; ROWS], row: i32) {}

// swaps two rows places
fn swap_rows(matrix: [[f64; COLUMNS]; ROWS], row1: i32, row2: i32) {}

// sets pivot to 1 in given pivot
fn set_pivot(row: [f64; COLUMNS], pivot: i32) {}

// start function for REF
fn into_ref(matrix: [[f64; COLUMNS]; ROWS]) {}

// start function for RREF
fn into_rref(matrix: [[f64; COLUMNS]; ROWS]) {}

fn main() {
    // proper RREF solution is 1, -1, 2
    let matrix: [[f64; COLUMNS]; ROWS] = [
        [2.0, -5.0, 5.0, 17.0],
        [0.0, 1.0, 3.0, 5.0],
        [1.0, -2.0, 3.0, 9.0],
    ];
}
