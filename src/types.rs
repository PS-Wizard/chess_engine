#![allow(dead_code)]

use crate::utils;

type Bitboard = u64;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Piece {
    color: Color,
    piece_type: PieceType,
}

impl Piece {
    fn new(color: Color, piece_type: PieceType) -> Self {
        Self { color, piece_type }
    }
}

struct Board {
    white_pawns: Bitboard,
    white_rooks: Bitboard,
    white_knights: Bitboard,
    white_bishops: Bitboard,
    white_queens: Bitboard,
    white_king: Bitboard,
    black_pawns: Bitboard,
    black_rooks: Bitboard,
    black_knights: Bitboard,
    black_bishops: Bitboard,
    black_queens: Bitboard,
    black_king: Bitboard,
}

impl Board {
    fn new() -> Self {
        Self {
            white_pawns: 0x0000_0000_0000_FF00,
            white_rooks: 0x0000_0000_0000_0081,
            white_knights: 0x0000_0000_0000_0042,
            white_bishops: 0x0000_0000_0000_0024,
            white_queens: 0x0000_0000_0000_0008,
            white_king: 0x0000_0000_0000_0010,
            black_pawns: 0x00FF_0000_0000_0000,
            black_rooks: 0x8100_0000_0000_0000,
            black_knights: 0x4200_0000_0000_0000,
            black_bishops: 0x2400_0000_0000_0000,
            black_queens: 0x0800_0000_0000_0000,
            black_king: 0x1000_0000_0000_0000,
        }
    }

    fn get_bitboard_mut(&mut self, color: Color, piece_type: PieceType) -> &mut Bitboard {
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

    fn white_pieces(&self) -> Bitboard {
        self.white_pawns
            | self.white_rooks
            | self.white_knights
            | self.white_bishops
            | self.white_queens
            | self.white_king
    }

    fn black_pieces(&self) -> Bitboard {
        self.black_pawns
            | self.black_rooks
            | self.black_knights
            | self.black_bishops
            | self.black_queens
            | self.black_king
    }

    fn draw_board(&self) {
        println!("===================================");
        let files = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        for rank in (0..8).rev() {
            print!("{} ", rank + 1);
            for file in 0..8 {
                let idx = rank * 8 + file;
                let mask = 1u64 << idx;
                let piece = self.get_piece_by_mask(mask);
                let symbol = match piece {
                    Some(Piece {
                        color: Color::White,
                        piece_type,
                    }) => match piece_type {
                        PieceType::Pawn => 'P',
                        PieceType::Rook => 'R',
                        PieceType::Knight => 'N',
                        PieceType::Bishop => 'B',
                        PieceType::Queen => 'Q',
                        PieceType::King => 'K',
                    },
                    Some(Piece {
                        color: Color::Black,
                        piece_type,
                    }) => match piece_type {
                        PieceType::Pawn => 'p',
                        PieceType::Rook => 'r',
                        PieceType::Knight => 'n',
                        PieceType::Bishop => 'b',
                        PieceType::Queen => 'q',
                        PieceType::King => 'k',
                    },
                    None => '.',
                };
                print!("{} ", symbol);
            }
            println!();
        }
        print!("  ");
        for file in files {
            print!("{} ", file);
        }
        println!();
        println!("===================================");
    }

    fn get_piece_by_mask(&self, mask: u64) -> Option<Piece> {
        for &(color, piece_type, bb) in &[
            (Color::White, PieceType::Pawn, self.white_pawns),
            (Color::White, PieceType::Rook, self.white_rooks),
            (Color::White, PieceType::Knight, self.white_knights),
            (Color::White, PieceType::Bishop, self.white_bishops),
            (Color::White, PieceType::Queen, self.white_queens),
            (Color::White, PieceType::King, self.white_king),
            (Color::Black, PieceType::Pawn, self.black_pawns),
            (Color::Black, PieceType::Rook, self.black_rooks),
            (Color::Black, PieceType::Knight, self.black_knights),
            (Color::Black, PieceType::Bishop, self.black_bishops),
            (Color::Black, PieceType::Queen, self.black_queens),
            (Color::Black, PieceType::King, self.black_king),
        ] {
            if bb & mask != 0 {
                return Some(Piece::new(color, piece_type));
            }
        }
        None
    }

    fn get_piece_at(&self, pos: &str) -> Option<Piece> {
        let idx = utils::notation_to_idx(pos).unwrap();
        let mask = 1u64 << idx;
        self.get_piece_by_mask(mask)
    }

    fn move_piece(&mut self, from: &str, to: &str) -> Result<(), String> {
        let from_idx = utils::notation_to_idx(from).unwrap();
        let to_idx = utils::notation_to_idx(to).unwrap();
        let from_mask = 1u64 << from_idx;
        let to_mask = 1u64 << to_idx;

        let piece = self
            .get_piece_by_mask(from_mask)
            .ok_or(format!("No piece at {}", from))?;

        if match piece.color {
            Color::White => self.white_pieces(),
            Color::Black => self.black_pieces(),
        } & to_mask
            != 0
        {
            return Err("Cannot capture your own piece".to_string());
        }

        // Remove source
        *self.get_bitboard_mut(piece.color, piece.piece_type) &= !from_mask;

        // Remove captured
        for pt in [
            PieceType::Pawn,
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
        ] {
            *self.get_bitboard_mut(Color::White, pt) &= !to_mask;
            *self.get_bitboard_mut(Color::Black, pt) &= !to_mask;
        }

        // Place piece at destination
        *self.get_bitboard_mut(piece.color, piece.piece_type) |= to_mask;

        println!(
            "Moved {:?} {:?} from {} to {}",
            piece.color, piece.piece_type, from, to
        );

        Ok(())
    }
}

#[cfg(test)]
mod types_tests {
    use super::*;

    #[test]
    fn test_draw_board() {
        let board = Board::new();
        board.draw_board();
    }

    #[test]
    fn test_get_piece() {
        let board = Board::new();
        assert_eq!(
            board.get_piece_at("a2"),
            Some(Piece::new(Color::White, PieceType::Pawn))
        );
        assert_eq!(
            board.get_piece_at("a8"),
            Some(Piece::new(Color::Black, PieceType::Rook))
        );
        assert_eq!(board.get_piece_at("a4"), None);
    }

    #[test]
    fn test_move_piece() {
        let mut board = Board::new();

        // Move white pawn from a2 to a4
        assert!(board.move_piece("a2", "a4").is_ok());
        board.draw_board();
        assert_eq!(board.get_piece_at("a2"), None);
        assert_eq!(
            board.get_piece_at("a4"),
            Some(Piece::new(Color::White, PieceType::Pawn))
        );

        // Move black pawn from a7 to a5
        assert!(board.move_piece("a7", "a5").is_ok());
        board.draw_board();
        assert_eq!(board.get_piece_at("a7"), None);
        assert_eq!(
            board.get_piece_at("a5"),
            Some(Piece::new(Color::Black, PieceType::Pawn))
        );

        // Move white knight from b1 to c3
        assert!(board.move_piece("b1", "c3").is_ok());
        board.draw_board();
        assert_eq!(board.get_piece_at("b1"), None);
        assert_eq!(
            board.get_piece_at("c3"),
            Some(Piece::new(Color::White, PieceType::Knight))
        );

        // Move black knight from b8 to c6
        assert!(board.move_piece("b8", "c6").is_ok());
        board.draw_board();
        assert_eq!(board.get_piece_at("b8"), None);
        assert_eq!(
            board.get_piece_at("c6"),
            Some(Piece::new(Color::Black, PieceType::Knight))
        );

        // Invalid move: no piece on e3
        assert!(board.move_piece("e3", "e4").is_err());
        board.draw_board();

        // Valid capture: white pawn captures black pawn at a5
        assert!(board.move_piece("a4", "a5").is_ok());
        board.draw_board();
        assert_eq!(
            board.get_piece_at("a5"),
            Some(Piece::new(Color::White, PieceType::Pawn))
        );
        assert_eq!(board.get_piece_at("a4"), None);

        // Invalid move: try to capture own knight
        assert!(board.move_piece("c3", "a5").is_err()); // can't take friendly pawn
        board.draw_board();
    }
}
