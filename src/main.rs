/*
 * Get a 1 in top left
 * Add mults of r1 to get coll of 0s
 * Move down 1 and right until you hit a non-zero
 *  if no nonzero, move that row to last and repeat
 * Repeat 1 and 2
 * Repeat all
 * Left with REF
 *
 * Start at bottom right-most one
 * Add mults until coll of 0s
 * Move up and left
 * Repeat
 * Left with RREF
 */

fn main() {
    // proper RREF solution is 1, -1, 2
    let matrix: [f64; 12] = [
        2.0, -5.0, 5.0, 17.0, 0.0, 1.0, 3.0, 5.0, 1.0, -2.0, 3.0, 9.0,
    ];
    let row_size = 4;
}
