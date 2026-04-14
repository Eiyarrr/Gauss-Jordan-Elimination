use super::matrix::ROWS;
use super::matrix::COLUMNS;

pub fn print_matrix(matrix: [[f64; COLUMNS]; ROWS]) {
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
