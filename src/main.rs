const ROWS: usize = 3;
const COLUMNS: usize = 4;

// sets all the numbers in a column, aside from the given row, to 0 using the given row and pivot
// assumes pivot is 1 (as it should be)
fn set_column_to_zero(
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

fn into_rref(mut matrix: [[f64; COLUMNS]; ROWS]) -> [[f64; COLUMNS]; ROWS] {
    let mut p = 0;
    for r in 0..ROWS {
        if matrix[r][p] == 0.0 {
            matrix = swap_rows(matrix, r, ROWS - 1)
        }
        matrix[r] = set_pivot(matrix[r], p);
        matrix = set_column_to_zero(matrix, r, p);

        p += 1;
    }
    return matrix;
}

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
    let matrix: [[f64; COLUMNS]; ROWS] = [
        [2.0, -4.0, 3.0, 5.0],
        [-1.0, 3.0, -1.0, -3.0],
        [2.0, -0.0, -4.0, 6.0],
    ];
    let matrix = into_rref(matrix);
    print_matrix(matrix);
}
