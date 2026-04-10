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
 * Scale so index = 0
 * make all other entries in column 0
 * Repeat
 */

fn main() {
    // proper RREF solution is 1, -1, 2
    let matrix: [f64; 12] = [
        2.0, -5.0, 5.0, 17.0, 0.0, 1.0, 3.0, 5.0, 1.0, -2.0, 3.0, 9.0,
    ];
    let row_size = 4;
}
