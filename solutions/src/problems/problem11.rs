use std::fs::read_to_string;

pub fn problem11() -> i64 {
    let size = 20;
    let stride = 4;
    let content = read_lines(&String::from("assets/problem11.txt"));
    let mut best = 0;

    for start_row in 0..size {
        for start_col in 0..size {
            if start_col < size-stride {
                let row = get_stride(start_row, start_col, 0, 1, stride, &content);
                if row > best {
                    best = row;
                }
            }

            if start_row < size-stride {
                let col = get_stride(start_row, start_col, 1, 0, stride, &content);
                if col > best {
                    best = col;
                }
            }

            if start_row < size-stride && start_col < size-stride {
                let diag_dr = get_stride(start_row, start_col, 1, 1, stride, &content);
                if diag_dr > best {
                    best = diag_dr;
                }
            }

            if start_row < size-stride && start_col >= stride-1 {
                let diag_dl = get_stride(start_row, start_col, 1, -1, stride, &content);
                if diag_dl > best {
                    best = diag_dl;
                }
            }
        }
    }

    best as i64
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn get_value(row_index: usize, col_index: usize, content: &Vec<String>) -> i32 {
    let row = &content[row_index];
    let col = &row[col_index*3..col_index*3+2];
    return col.parse().unwrap();
}

fn get_stride(row_index: i32, col_index: i32, row_delta: i32, col_delta: i32, stride: i32, content: &Vec<String>) -> i32 {
    let mut product = 1;
    for idx in 0..stride {
        let row = (row_index + row_delta * idx) as usize;
        let col = (col_index + col_delta * idx) as usize;
        let value = get_value(row, col, content);
        product *= value;
    }
    product
}