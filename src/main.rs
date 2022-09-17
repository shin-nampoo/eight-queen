use std::env;

#[derive(Debug, Clone)]
struct Row {
    cols: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Board {
    size: usize,
    rows: Vec<Row>,
}

fn left_above(board: &mut Board, col: usize, row: usize) -> bool {
    if col == 0 {return true;}
    if row == 0 {return true;}
    for i in 0..col {
        if i == row {return true;}
        if board.rows[row - i - 1].cols[col - i - 1] == 1 {
            return false;
        }
    }
    return true;
}

fn left_below(board: &mut Board, col: usize, row: usize) -> bool {
    if col == 0 {return true;}
    if row == board.size - 1 {return true;}
    for i in 0..col {
        if board.size == row + i + 1 {return true;}
        if board.size == col - i - 1 {return true;}
        if board.rows[row + i + 1].cols[col - i - 1] == 1 {
            return false;
        }
    }
    return true;
}

fn right_above(board: &mut Board, col: usize, row: usize) -> bool {
    if col == board.size - 1 {return true;}
    if row == 0 {return true;}
    for i in 0..board.size {
        if row < i + 1 {return true;}
        if board.size == col + i + 1 {return true;}
        if i > row {return true;}
        if board.rows[row - i - 1].cols[col + i + 1] == 1 {
            return false;
        }
    }
    return true;
}

fn right_below(board: &mut Board, col: usize, row: usize) -> bool {
    if col == board.size - 1 {return true;}
    if row == board.size - 1 {return true;}
    for i in 0..board.size {
        if board.size == i + col + 1 {return true;}
        if board.size == i + row + 1 {return true;}
        if board.rows[row + i + 1].cols[col + i + 1] == 1 {
            return false;
        }
    }
    return true;
}

fn set_q(board: &mut Board, col: usize, row: usize) -> bool {
    let mut work_board = board.clone();
    for i in 0..board.size {
        if board.rows[row].cols[i] == 1 || board.rows[i].cols[col] == 1 {
            return false;
        } 
    }
    work_board.rows[row].cols[col] = 1;
    if !left_above(&mut work_board, col, row){
        return false;
    }
    if !left_below(&mut work_board, col, row){
        return false;
    }
    if !right_above(&mut work_board, col, row){
        return false;
    }
    if !right_below(&mut work_board, col, row){
        return false;
    }
    //work_board.rows[row].cols[col] = 0;
    *board = work_board;
    return true;
}

fn prt_board(board: &Board){
    for i in 0..board.size {
        println!("{}:{:?}", i, board.rows[i].cols);
    }
}

fn queen_get(result: &mut Vec<Board>, board: &mut Board, row: usize, cols: &mut Vec<u8>, c_count: u64) {
    let sv_cols = cols.clone();
    let bk_board = board.clone();
    for i in 0..board.size {
        if cols[i] != 0 {
            continue;
        }
        if set_q(board, i, row) {
            cols[i] = 1;
            if c_count == board.size as u64 {
                result.push(board.clone());
            }
            queen_get(result, board, row + 1, cols, c_count + 1);
            *board = bk_board.clone();   
            cols[i] = 0;
        }
    }
    *cols = sv_cols;
    return;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let size: usize = args[1].parse().unwrap();
    let mut board = Board{
        size: size,
        rows: Vec::new(),
    };
    let mut cols: Vec<u8> = vec![0; size];
    let mut result: Vec<Board> = Vec::new();
    for _i in 0..size {
        let row: Row = Row {
            cols: cols.clone(),
        };
        board.rows.push(row); 
    }
    queen_get(&mut result, &mut board, 0, &mut cols, 1);
    let mut ct = 0;
    for bd in result.iter() {
        ct += 1;
        println!("解答{}--------------", ct);
        prt_board(&bd);
    }
}
