use crate::model::board::Board;

pub fn rook_moves(board: &Board, from: usize) -> Vec<usize> {
    let mut moves = Vec::new();
    let col = from % 8;
    let row = from / 8;

    // down
    for i in (0..row).rev() {
        let target = (i * 8) + col;
        if board[target].is_none() {
            moves.push(target);
        } else {
            break;
        }
    }

    // up
    for i in (row + 1)..8 {
        let target = (i * 8) + col;
        if board[target].is_none() {
            moves.push(target);
        } else {
            break;
        }
    }

    // left
    for i in (0..col).rev() {
        let target = (8 * row) + i;
        if board[target].is_none() {
            moves.push(target);
        } else {
            break;
        }
    }

    // right
    for i in (col + 1)..8 {
        let target = (8 * row) + i;
        if board[target].is_none() {
            moves.push(target);
        } else {
            break;
        }
    }

    return moves;
}
