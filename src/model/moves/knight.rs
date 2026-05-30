use crate::model::board::Board;

pub fn knight_moves(board: &Board, from: usize) -> Vec<usize> {
    let mut moves = Vec::new();
    let col = (from % 8) as i32;
    let row = (from / 8) as i32;

    // top-left
    if is_valid_knight_tile(col - 1, row + 2) {
        let target = get_knight_tile(col - 1, row + 2);
        if board[target].is_none() {
            moves.push(target);
        }
    }

    // left-top
    if is_valid_knight_tile(col - 2, row + 1) {
        let target = get_knight_tile(col - 2, row + 1);
        if board[target].is_none() {
            moves.push(target);
        }
    }

    // left-bottom
    if is_valid_knight_tile(col - 2, row - 1) {
        let target = get_knight_tile(col - 2, row - 1);
        if board[target].is_none() {
            moves.push(target);
        }
    }

    // bottom-left
    if is_valid_knight_tile(col - 1, row - 2) {
        let target = get_knight_tile(col - 1, row - 2);
        if board[target].is_none() {
            moves.push(target);
        }
    }

    // top-right
    if is_valid_knight_tile(col + 1, row + 2) {
        let target = get_knight_tile(col + 1, row + 2);
        if board[target].is_none() {
            moves.push(target);
        }
    }

    // right-top
    if is_valid_knight_tile(col + 2, row + 1) {
        let target = get_knight_tile(col + 2, row + 1);
        if board[target].is_none() {
            moves.push(target);
        }
    }

    // right-bottom
    if is_valid_knight_tile(col + 2, row - 1) {
        let target = get_knight_tile(col + 2, row - 1);
        if board[target].is_none() {
            moves.push(target);
        }
    }

    // bottom-right
    if is_valid_knight_tile(col + 1, row - 2) {
        let target = get_knight_tile(col + 1, row - 2);
        if board[target].is_none() {
            moves.push(target);
        }
    }

    return moves;
}

fn get_knight_tile(col: i32, row: i32) -> usize {
    return ((row * 8) + col) as usize;
}

fn is_valid_knight_tile(col: i32, row: i32) -> bool {
    if col < 0 || row < 0 || col > 8 || row > 8 {
        return false;
    }
    return true;
}
