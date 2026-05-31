use crate::model::{
    game::Board,
    moves::{bishop::bishop_moves, rook::rook_moves}, piece::Color,
};

pub fn queen_moves(board: &Board, from: usize, color: Color) -> Vec<usize> {
    let mut moves = Vec::new();

    moves.append(&mut rook_moves(board, from, color));
    moves.append(&mut bishop_moves(board, from, color));

    return moves;
}
