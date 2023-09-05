#![allow(dead_code)]

use std::fmt;
use std::io::{self, Write};

const BLACK: bool = false;
const WHITE: bool = true;
const EVEN: u64 = 0b_10101010_01010101_10101010_01010101_10101010_01010101_10101010_01010101_u64;

fn main() {
    let mut board: Board = Board::new_starting_position();
    println!("{}", board);
    let (x, y) = get_cli_move_input();
    board.white_king = board.white_king | x | y;
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
        }
    }

    fn piece_char(&self, index: u32) -> char {
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

    //fn rook_moves_pseudo_legal
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "8 {} {} {} {} {} {} {} {}\n7 {} {} {} {} {} {} {} {}\n6 {} {} {} {} {} {} {} {}\n5 {} {} {} {} {} {} {} {}\n4 {} {} {} {} {} {} {} {}\n3 {} {} {} {} {} {} {} {}\n2 {} {} {} {} {} {} {} {}\n1 {} {} {} {} {} {} {} {}\n  a b c d e f g h", self.piece_char(56), self.piece_char(57), self.piece_char(58), self.piece_char(59), self.piece_char(60), self.piece_char(61), self.piece_char(62), self.piece_char(63), self.piece_char(48), self.piece_char(49), self.piece_char(50), self.piece_char(51), self.piece_char(52), self.piece_char(53), self.piece_char(54), self.piece_char(55), self.piece_char(40), self.piece_char(41), self.piece_char(42), self.piece_char(43), self.piece_char(44), self.piece_char(45), self.piece_char(46), self.piece_char(47), self.piece_char(32), self.piece_char(33), self.piece_char(34), self.piece_char(35), self.piece_char(36), self.piece_char(37), self.piece_char(38), self.piece_char(39), self.piece_char(24), self.piece_char(25), self.piece_char(26), self.piece_char(27), self.piece_char(28), self.piece_char(29), self.piece_char(30), self.piece_char(31), self.piece_char(16), self.piece_char(17), self.piece_char(18), self.piece_char(19), self.piece_char(20), self.piece_char(21), self.piece_char(22), self.piece_char(23), self.piece_char(8), self.piece_char(9), self.piece_char(10), self.piece_char(11), self.piece_char(12), self.piece_char(13), self.piece_char(14), self.piece_char(15), self.piece_char(0), self.piece_char(1), self.piece_char(2), self.piece_char(3), self.piece_char(4), self.piece_char(5), self.piece_char(6), self.piece_char(7))
    }
}

fn get_cli_move_input() -> (u64, u64) {
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
                (parse_pos(bytes[0], bytes[1]), parse_pos(bytes[2], bytes[3]))
            {
                return (inital_pos, final_pos);
            }
        }
        println!("Error parsing move, try again (example \"c2c4\").");
        buffer.clear();
    }
}

fn parse_pos(rank: u8, file: u8) -> Option<u64> {
    let mut pos = 1u64;
    pos <<= match rank {
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
    pos <<= match file {
        49 => 0,
        50 => 8,
        51 => 16,
        52 => 24,
        53 => 32,
        54 => 40,
        55 => 48,
        56 => 56,
        _ => return None,
    };
    Some(pos)
}
