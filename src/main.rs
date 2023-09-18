#![allow(dead_code)]

use std::fmt;
use std::io::{self, Write};

const BLACK: bool = false;
const WHITE: bool = true;
const EVEN: u64 = 0b_10101010_01010101_10101010_01010101_10101010_01010101_10101010_01010101_u64;
const FILES: [u64; 8] = [
    0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111_u64,
    0b_00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000_u64,
    0b_00000000_00000000_00000000_00000000_00000000_11111111_00000000_00000000_u64,
    0b_00000000_00000000_00000000_00000000_11111111_00000000_00000000_00000000_u64,
    0b_00000000_00000000_00000000_11111111_00000000_00000000_00000000_00000000_u64,
    0b_00000000_00000000_11111111_00000000_00000000_00000000_00000000_00000000_u64,
    0b_00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000_u64,
    0b_11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000_u64,
];
const RANKS: [u64; 8] = [
    0b_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_u64,
    0b_00000010_00000010_00000010_00000010_00000010_00000010_00000010_00000010_u64,
    0b_00000100_00000100_00000100_00000100_00000100_00000100_00000100_00000100_u64,
    0b_00001000_00001000_00001000_00001000_00001000_00001000_00001000_00001000_u64,
    0b_00010000_00010000_00010000_00010000_00010000_00010000_00010000_00010000_u64,
    0b_00100000_00100000_00100000_00100000_00100000_00100000_00100000_00100000_u64,
    0b_01000000_01000000_01000000_01000000_01000000_01000000_01000000_01000000_u64,
    0b_10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000_u64,
];
const EVEN_FORWARD_DIAG: [u64; 7] = [
    0b_00000000_00000000_00000000_00000000_00000000_00000000_10000000_01000000_u64,
    0b_00000000_00000000_00000000_00000000_10000000_01000000_00100000_00010000_u64,
    0b_00000000_00000000_10000000_01000000_00100000_00010000_00001000_00000100_u64,
    0b_10000000_01000000_00100000_00010000_00001000_00000100_00000010_00000001_u64,
    0b_00100000_00010000_00001000_00000100_00000010_00000001_00000000_00000000_u64,
    0b_00001000_00000100_00000010_00000001_00000000_00000000_00000000_00000000_u64,
    0b_00000010_00000001_00000000_00000000_00000000_00000000_00000000_00000000_u64,
];
const ODD_FORWARD_DIAG: [u64; 6] = [
    0b_00000000_00000000_00000000_00000000_00000000_10000000_01000000_00100000_u64,
    0b_00000000_00000000_00000000_10000000_01000000_00100000_00010000_00001000_u64,
    0b_00000000_10000000_01000000_00100000_00010000_00001000_00000100_00000010_u64,
    0b_01000000_00100000_00010000_00001000_00000100_00000010_00000001_00000000_u64,
    0b_00010000_00001000_00000100_00000010_00000001_00000000_00000000_00000000_u64,
    0b_00000100_00000010_00000001_00000000_00000000_00000000_00000000_00000000_u64,
];
const EVEN_BACKWARD_DIAG: [u64; 6] = [
    0b_00000000_00000000_00000000_00000000_00000000_00000001_00000010_00000100_u64,
    0b_00000000_00000000_00000000_00000001_00000010_00000100_00001000_00010000_u64,
    0b_00000000_00000001_00000010_00000100_00001000_00010000_00100000_01000000_u64,
    0b_00000010_00000100_00001000_00010000_00100000_01000000_10000000_00000000_u64,
    0b_00001000_00010000_00100000_01000000_10000000_00000000_00000000_00000000_u64,
    0b_00100000_01000000_10000000_00000000_00000000_00000000_00000000_00000000_u64,
];
const ODD_BACKWARD_DIAG: [u64; 7] = [
    0b_00000000_00000000_00000000_00000000_00000000_00000000_00000001_00000010_u64,
    0b_00000000_00000000_00000000_00000000_00000001_00000010_00000100_00001000_u64,
    0b_00000000_00000000_00000001_00000010_00000100_00001000_00010000_00100000_u64,
    0b_00000001_00000010_00000100_00001000_00010000_00100000_01000000_10000000_u64,
    0b_00000100_00001000_00010000_00100000_01000000_10000000_00000000_00000000_u64,
    0b_00010000_00100000_01000000_10000000_00000000_00000000_00000000_00000000_u64,
    0b_01000000_10000000_00000000_00000000_00000000_00000000_00000000_00000000_u64,
];

fn main() {
    let mut board = Board::new_empty();
    board.black_king |= 0b_00100100_u64;
    println!("{}", board);
    board.update_pieces();
    board.black_king = 0;
    let input_pos = get_cli_move_input();
    board.white_pawn |= board.king_moves_pseudo_legal(&input_pos.0, BLACK);
    println!("{}", board);
}

struct Board {
    white_king: u64,
    black_king: u64,
    white_queen: u64,
    black_queen: u64,
    white_rook: u64,
    black_rook: u64,
    white_bishop: u64,
    black_bishop: u64,
    white_knight: u64,
    black_knight: u64,
    white_pawn: u64,
    black_pawn: u64,
    white_pieces: u64,
    black_pieces: u64,
}

impl Board {
    fn new_empty() -> Self {
        Self {
            white_king:
                0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_u64,
            black_king:
                0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_u64,
            white_queen:
                0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_u64,
            black_queen:
                0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_u64,
            white_rook:
                0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_u64,
            black_rook:
                0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_u64,
            white_bishop:
                0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_u64,
            black_bishop:
                0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_u64,
            white_knight:
                0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_u64,
            black_knight:
                0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_u64,
            white_pawn:
                0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_u64,
            black_pawn:
                0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_u64,
            white_pieces:
                0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_u64,
            black_pieces:
                0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_u64,
        }
    }

    fn new_starting_position() -> Self {
        Self {
            white_king:
                0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000_u64,
            black_king:
                0b_00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_u64,
            white_queen:
                0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000_u64,
            black_queen:
                0b_00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_u64,
            white_rook:
                0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001_u64,
            black_rook:
                0b_10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000_u64,
            white_bishop:
                0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100_u64,
            black_bishop:
                0b_00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000_u64,
            white_knight:
                0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010_u64,
            black_knight:
                0b_01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000_u64,
            white_pawn:
                0b_00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000_u64,
            black_pawn:
                0b_00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000_u64,
            white_pieces:
                0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_u64,
            black_pieces:
                0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_u64,
        }
    }

    fn piece_char(&self, index: u8) -> char {
        match 1 << index {
            i if i & self.white_king != 0 => '♚',
            i if i & self.black_king != 0 => '♔',
            i if i & self.white_queen != 0 => '♛',
            i if i & self.black_queen != 0 => '♕',
            i if i & self.white_rook != 0 => '♜',
            i if i & self.black_rook != 0 => '♖',
            i if i & self.white_bishop != 0 => '♝',
            i if i & self.black_bishop != 0 => '♗',
            i if i & self.white_knight != 0 => '♞',
            i if i & self.black_knight != 0 => '♘',
            i if i & self.white_pawn != 0 => '♟',
            i if i & self.black_pawn != 0 => '♙',
            i if i & EVEN != 0 => '#',
            _ => ' ',
        }
    }

    fn update_pieces(&mut self) {
        self.white_pieces = self.white_king
            | self.white_queen
            | self.white_rook
            | self.white_bishop
            | self.white_knight
            | self.white_pawn;
        self.black_pieces = self.black_king
            | self.black_queen
            | self.black_rook
            | self.black_bishop
            | self.black_knight
            | self.black_pawn;
    }

    fn rook_moves_pseudo_legal(&self, pos: &Pos, white: bool) -> u64 {
        let (same_color, opp_color) = if white {
            (self.white_pieces, self.black_pieces)
        } else {
            (self.black_pieces, self.white_pieces)
        };
        
        let bitboard_pos = pos.to_bitboard();
        let mut moves = 0_u64;
        
        // left
        add_sliding_moves(pos.rank, -1, &mut moves, bitboard_pos, &same_color, &opp_color);
        // right
        add_sliding_moves(7 - pos.rank, 1, &mut moves, bitboard_pos, &same_color, &opp_color);
        // down
        add_sliding_moves(pos.file, -8, &mut moves, bitboard_pos, &same_color, &opp_color);
        // up
        add_sliding_moves(7 - pos.file, 8, &mut moves, bitboard_pos, &same_color, &opp_color);
        
        moves
    }

    fn bishop_moves_pseudo_legal(&self, pos: &Pos, white: bool) -> u64 { 
        let (same_color, opp_color) = if white {
            (self.white_pieces, self.black_pieces)
        } else {
            (self.black_pieces, self.white_pieces)
        };
        
        let bitboard_pos = pos.to_bitboard();
        let mut moves = 0_u64;

        use core::cmp::min;

        // left + down
        add_sliding_moves(min(pos.rank, pos.file), -9, &mut moves, bitboard_pos, &same_color, &opp_color);
        // right + down
        add_sliding_moves(min(7 - pos.rank, pos.file), -7, &mut moves, bitboard_pos, &same_color, &opp_color);
        // left + up
        add_sliding_moves(min(pos.rank, 7 - pos.file), 7, &mut moves, bitboard_pos, &same_color, &opp_color);
        // right + up
        add_sliding_moves(min(7 - pos.rank, 7 - pos.file), 9, &mut moves, bitboard_pos, &same_color, &opp_color);
        
        moves
    }

    fn queen_moves_pseudo_legal(&self, pos: &Pos, white: bool) -> u64 {
       self.rook_moves_pseudo_legal(pos, white) | self.bishop_moves_pseudo_legal(pos, white)
    }

    fn king_moves_pseudo_legal(&self, pos: &Pos, white: bool) -> u64 {
        let same_color = if white {
            self.white_pieces
        } else {
            self.black_pieces
        };

        let left =  pos.rank > 0;
        let right = pos.rank < 7;
        let down = pos.file > 0;
        let up = pos.file < 7;

        let bitboard_pos = pos.to_bitboard();
        let mut moves = 0_u64;

        if left {
            add_move_if_not_same_color(bitboard_pos >> 1, &mut moves, &same_color); 
            if down {
                add_move_if_not_same_color(bitboard_pos >> 9, &mut moves, &same_color); 
            }
            if up {
                add_move_if_not_same_color(bitboard_pos << 7, &mut moves, &same_color); 
            }
        }
        if right {
            add_move_if_not_same_color(bitboard_pos << 1, &mut moves, &same_color); 
            if down {
                add_move_if_not_same_color(bitboard_pos >> 7, &mut moves, &same_color); 
            }
            if up {
                add_move_if_not_same_color(bitboard_pos << 9, &mut moves, &same_color); 
            }
        }
        if down {
            add_move_if_not_same_color(bitboard_pos >> 8, &mut moves, &same_color); 
        }
        if up {
            add_move_if_not_same_color(bitboard_pos << 8, &mut moves, &same_color); 
        }

        moves
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "8 {} {} {} {} {} {} {} {}\n7 {} {} {} {} {} {} {} {}\n6 {} {} {} {} {} {} {} {}\n5 {} {} {} {} {} {} {} {}\n4 {} {} {} {} {} {} {} {}\n3 {} {} {} {} {} {} {} {}\n2 {} {} {} {} {} {} {} {}\n1 {} {} {} {} {} {} {} {}\n  a b c d e f g h", self.piece_char(56), self.piece_char(57), self.piece_char(58), self.piece_char(59), self.piece_char(60), self.piece_char(61), self.piece_char(62), self.piece_char(63), self.piece_char(48), self.piece_char(49), self.piece_char(50), self.piece_char(51), self.piece_char(52), self.piece_char(53), self.piece_char(54), self.piece_char(55), self.piece_char(40), self.piece_char(41), self.piece_char(42), self.piece_char(43), self.piece_char(44), self.piece_char(45), self.piece_char(46), self.piece_char(47), self.piece_char(32), self.piece_char(33), self.piece_char(34), self.piece_char(35), self.piece_char(36), self.piece_char(37), self.piece_char(38), self.piece_char(39), self.piece_char(24), self.piece_char(25), self.piece_char(26), self.piece_char(27), self.piece_char(28), self.piece_char(29), self.piece_char(30), self.piece_char(31), self.piece_char(16), self.piece_char(17), self.piece_char(18), self.piece_char(19), self.piece_char(20), self.piece_char(21), self.piece_char(22), self.piece_char(23), self.piece_char(8), self.piece_char(9), self.piece_char(10), self.piece_char(11), self.piece_char(12), self.piece_char(13), self.piece_char(14), self.piece_char(15), self.piece_char(0), self.piece_char(1), self.piece_char(2), self.piece_char(3), self.piece_char(4), self.piece_char(5), self.piece_char(6), self.piece_char(7))
    }
}

fn add_sliding_moves(distance: u8, bitshift: i8, moves: &mut u64, mut bitboard_pos: u64, same_color: &u64, opp_color: &u64)  {
    for _ in 0..distance {
        if bitshift < 0 {
            bitboard_pos >>= bitshift.abs();
        } else {
            bitboard_pos <<= bitshift;
        }
        if same_color & bitboard_pos != 0 {
            break;
        }
        *moves |= bitboard_pos;
        if opp_color & bitboard_pos != 0 {
            break;
        }
    }
}

fn add_move_if_not_same_color(bitboard_pos: u64, moves: &mut u64, same_color: &u64) {
   if bitboard_pos & same_color == 0 {
        *moves |= bitboard_pos;
   } 
}

struct Pos {
    rank: u8,
    file: u8,
}

impl Pos {
    fn new(rank: u8, file: u8) -> Self {
        Self {
            rank,
            file,
        }
    }

    fn from_ascii(rank_char: u8, file_char: u8) -> Option<Self> {
        let rank = match rank_char {
            97 => 0,
            98 => 1,
            99 => 2,
            100 => 3,
            101 => 4,
            102 => 5,
            103 => 6,
            104 => 7,
            _ => return None,
        };
        let file = match file_char {
            49 => 0,
            50 => 1,
            51 => 2,
            52 => 3,
            53 => 4,
            54 => 5,
            55 => 6,
            56 => 7,
            _ => return None,
        };
        Some(Self {
            rank,
            file,
        })
    }

    fn to_bitboard(&self) -> u64 {
        1_u64 << (self.rank + (self.file * 8))
    }
}

fn get_cli_move_input() -> (Pos, Pos) {
    let mut buffer = String::new();
    print!("Input move: ");
    let _ = io::stdout().flush();
    loop {
        io::stdin()
            .read_line(&mut buffer)
            .expect("Error reading from stdin.");
        if buffer.len() == 5 && buffer.is_ascii() {
            let bytes = buffer.as_bytes();
            if let (Some(inital_pos), Some(final_pos)) =
                (Pos::from_ascii(bytes[0], bytes[1]), Pos::from_ascii(bytes[2], bytes[3]))
            {
                return (inital_pos, final_pos);
            }
        }
        println!("Error parsing move, try again (example \"c2c4\").");
        buffer.clear();
    }
}
