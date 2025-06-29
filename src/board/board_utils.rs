use super::{
    board::{Bitboard, Board},
    piece::{Color, PieceType},
};

pub fn notation_to_idx(notation: &str) -> Option<usize> {
    let mut chars = notation.chars();
    let file_char = chars.next()?; // 'e'
    let rank_char = chars.next()?; // '2'

    let file = (file_char as u8).checked_sub(b'a')?;
    let rank = (rank_char as u8).checked_sub(b'1')?;

    if file > 7 || rank > 7 {
        return None;
    }

    Some((rank as usize) * 8 + (file as usize))
}

impl Board {
    pub fn get_bitboard_mut(&mut self, color: Color, piece_type: PieceType) -> &mut Bitboard {
        match (color, piece_type) {
            (Color::White, PieceType::Pawn) => &mut self.white_pawns,
            (Color::White, PieceType::Rook) => &mut self.white_rooks,
            (Color::White, PieceType::Knight) => &mut self.white_knights,
            (Color::White, PieceType::Bishop) => &mut self.white_bishops,
            (Color::White, PieceType::Queen) => &mut self.white_queens,
            (Color::White, PieceType::King) => &mut self.white_king,
            (Color::Black, PieceType::Pawn) => &mut self.black_pawns,
            (Color::Black, PieceType::Rook) => &mut self.black_rooks,
            (Color::Black, PieceType::Knight) => &mut self.black_knights,
            (Color::Black, PieceType::Bishop) => &mut self.black_bishops,
            (Color::Black, PieceType::Queen) => &mut self.black_queens,
            (Color::Black, PieceType::King) => &mut self.black_king,
        }
    }
}
