mod rng;

use rng::*;
use types::*;

struct Slider {
    deltas: [(i8, i8); 4],
}

impl Slider {
    fn moves(&self, square: Square, blockers: BitBoard) -> BitBoard {
        let mut moves = BitBoard::EMPTY;
        for &(df, dr) in &self.deltas {
            let mut ray = square;
            while !blockers.has(ray) {
                if let Some(shifted) = ray.try_offset(df, dr) {
                    ray = shifted;
                    moves |= ray.bitboard();
                } else {
                    break;
                }
            }
        }
        moves
    }

    fn relevant_blockers(&self, square: Square) -> BitBoard {
        let mut blockers = BitBoard::EMPTY;
        for &(df, dr) in &self.deltas {
            let mut ray = square;
            while let Some(shifted) = ray.try_offset(df, dr) {
                blockers |= ray.bitboard();
                ray = shifted;
            }
        }
        blockers &= !square.bitboard();
        blockers
    }
}

const ROOK: Slider = Slider {
    deltas: [(1, 0), (0, -1), (-1, 0), (0, 1)],
};

const BISHOP: Slider = Slider {
    deltas: [(1, 1), (1, -1), (-1, -1), (-1, 1)],
};

struct MagicEntry {
    mask: BitBoard,
    magic: u64,
    shift: u8,
}

fn magic_index(entry: &MagicEntry, blockers: BitBoard) -> usize {
    let blockers = blockers & entry.mask;
    let hash = blockers.0.wrapping_mul(entry.magic);
    let index = (hash >> entry.shift) as usize;
    index
}

// Given a sliding piece and a square, finds a magic number that
// perfectly maps input blockers into its solution in a hash table
fn find_magic(
    slider: &Slider,
    square: Square,
    index_bits: u8,
    rng: &mut Rng
) -> (MagicEntry, Vec<BitBoard>) {
    let mask = slider.relevant_blockers(square);
    let shift = 64 - index_bits;
    loop {
        // Magics require a low number of active bits, so we AND
        // by two more random values to cut down on the bits set.
        let magic = rng.next_u64() & rng.next_u64() & rng.next_u64();
        let magic_entry = MagicEntry { mask, magic, shift };
        if let Ok(table) = try_make_table(slider, square, &magic_entry) {
            return (magic_entry, table);
        }
    }
}

struct TableFillError;

// Attempt to fill in a hash table using a magic number.
// Fails if there are any non-constructive collisions.
fn try_make_table(
    slider: &Slider,
    square: Square,
    magic_entry: &MagicEntry,
) -> Result<Vec<BitBoard>, TableFillError> {
    let index_bits = 64 - magic_entry.shift;
    let mut table = vec![BitBoard::EMPTY; 1 << index_bits];
    // Iterate all configurations of blockers
    let mut blockers = BitBoard::EMPTY;
    loop {
        let moves = slider.moves(square, blockers);
        let table_entry = &mut table[magic_index(magic_entry, blockers)];
        if table_entry.is_empty() {
            // Write to empty slot
            *table_entry = moves;
        } else if *table_entry != moves {
            // Having two different move sets in the same slot is a hash collision
            return Err(TableFillError);
        }
        
        // Carry-Rippler trick that enumerates all subsets of the mask, getting us all blockers.
        // https://www.chessprogramming.org/Traversing_Subsets_of_a_Set#All_Subsets_of_any_Set
        blockers.0 = blockers.0.wrapping_sub(magic_entry.mask.0) & magic_entry.mask.0;
        if blockers.is_empty() {
            // Finished enumerating all blocker configurations
            break;
        }
    }
    Ok(table)
}

fn find_and_print_all_magics(slider: &Slider, slider_name: &str, rng: &mut Rng) {
    println!(
        "pub const {}_MAGICS: &[MagicEntry; Square::NUM] = &[",
        slider_name
    );
    let mut total_table_size = 0;
    for &square in &Square::ALL {
        let index_bits = slider.relevant_blockers(square).popcnt() as u8;
        let (entry, table) = find_magic(slider, square, index_bits, rng);
        // In the final move generator, each table is concatenated into one contiguous table
        // for convenience, so an offset is added to denote the start of each segment.
        println!(
            "    MagicEntry {{ mask: 0x{:016X}, magic: 0x{:016X}, shift: {}, offset: {} }},",
            entry.mask.0, entry.magic, entry.shift, total_table_size
        );
        total_table_size += table.len();
    }
    println!("];");
    println!(
        "pub const {}_TABLE_SIZE: usize = {};",
        slider_name, total_table_size
    );
}

fn main() {
    let mut rng = Rng::default();
    find_and_print_all_magics(&ROOK, "ROOK", &mut rng);
    find_and_print_all_magics(&BISHOP, "BISHOP", &mut rng);
}
