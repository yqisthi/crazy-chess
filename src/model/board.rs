use crate::model::piece::{Color, Piece, PieceKind};

pub type Board = [Option<Piece>; 64];

pub fn initial_board() -> Board {
    let mut board: Board = [None; 64];

    // Black Pieces
    board[0] = Some(Piece {
        kind: PieceKind::Rook,
        color: Color::Black,
    });
    board[1] = Some(Piece {
        kind: PieceKind::Knight,
        color: Color::Black,
    });
    board[2] = Some(Piece {
        kind: PieceKind::Bishop,
        color: Color::Black,
    });
    board[3] = Some(Piece {
        kind: PieceKind::Queen,
        color: Color::Black,
    });
    board[4] = Some(Piece {
        kind: PieceKind::King,
        color: Color::Black,
    });
    board[5] = Some(Piece {
        kind: PieceKind::Knight,
        color: Color::Black,
    });
    board[6] = Some(Piece {
        kind: PieceKind::Bishop,
        color: Color::Black,
    });
    board[7] = Some(Piece {
        kind: PieceKind::Rook,
        color: Color::Black,
    });

    for i in 8..16 {
        board[i] = Some(Piece {
            kind: PieceKind::Pawn,
            color: Color::Black,
        });
    }

    // White Pieces
    for i in 48..56 {
        board[i] = Some(Piece {
            kind: PieceKind::Pawn,
            color: Color::White,
        });
    }

    board[56] = Some(Piece {
        kind: PieceKind::Rook,
        color: Color::White,
    });
    board[57] = Some(Piece {
        kind: PieceKind::Knight,
        color: Color::White,
    });
    board[58] = Some(Piece {
        kind: PieceKind::Bishop,
        color: Color::White,
    });
    board[59] = Some(Piece {
        kind: PieceKind::Queen,
        color: Color::White,
    });
    board[60] = Some(Piece {
        kind: PieceKind::King,
        color: Color::White,
    });
    board[61] = Some(Piece {
        kind: PieceKind::Bishop,
        color: Color::White,
    });
    board[62] = Some(Piece {
        kind: PieceKind::Knight,
        color: Color::White,
    });
    board[63] = Some(Piece {
        kind: PieceKind::Rook,
        color: Color::White,
    });

    return board;
}
