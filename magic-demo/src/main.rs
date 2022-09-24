use types::*;

include!(concat!(env!("OUT_DIR"), "/magics.rs"));

struct MagicEntry {
    mask: u64,
    magic: u64,
    shift: u8,
    offset: u32,
}

fn magic_index(entry: &MagicEntry, blockers: BitBoard) -> usize {
    let blockers = blockers.0 & entry.mask;
    let hash = blockers.wrapping_mul(entry.magic);
    let index = (hash >> entry.shift) as usize;
    entry.offset as usize + index
}

fn get_rook_moves(square: Square, blockers: BitBoard) -> BitBoard {
    let magic = &ROOK_MAGICS[square as usize];
    BitBoard(ROOK_MOVES[magic_index(magic, blockers)])
}

fn get_bishop_moves(square: Square, blockers: BitBoard) -> BitBoard {
    let magic = &BISHOP_MAGICS[square as usize];
    BitBoard(BISHOP_MOVES[magic_index(magic, blockers)])
}

fn main() {
    let blockers = bitboard! {
        . . . X . . . X
        . . . . . . . .
        . . . X . . . .
        . . . . . . . .
        . . . . . . . X
        . . X . . . . .
        . . . X . X . .
        . . . . . . . .
    };
    let square = Square::D4;
    println!("Blockers: {:#?}", blockers);
    println!("Square: {:?}", square);
    println!("Rook moves: {:#?}", get_rook_moves(square, blockers));
    println!("Bishop moves: {:#?}", get_bishop_moves(square, blockers));
}
