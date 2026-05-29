#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PieceKind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: Color,
}

impl Piece {
    pub fn unicode(&self) -> &'static str {
        match (&self.kind, &self.color) {
            (PieceKind::Pawn, Color::White) => "♙",
            (PieceKind::Rook, Color::White) => "♖",
            (PieceKind::Knight, Color::White) => "♘",
            (PieceKind::Bishop, Color::White) => "♗",
            (PieceKind::Queen, Color::White) => "♕",
            (PieceKind::King, Color::White) => "♔",
            (PieceKind::Pawn, Color::Black) => "♟",
            (PieceKind::Rook, Color::Black) => "♜",
            (PieceKind::Knight, Color::Black) => "♞",
            (PieceKind::Bishop, Color::Black) => "♝",
            (PieceKind::Queen, Color::Black) => "♛",
            (PieceKind::King, Color::Black) => "♚",
        }
    }
}
