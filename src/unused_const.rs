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
