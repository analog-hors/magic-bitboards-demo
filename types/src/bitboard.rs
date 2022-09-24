use super::*;

use std::ops::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct BitBoard(pub u64);

macro_rules! impl_math_ops {
    ($($trait:ident,$fn:ident;)*) => {$(
        impl $trait for BitBoard {
            type Output = Self;

            fn $fn(self, other: Self) -> Self::Output {
                Self($trait::$fn(self.0, other.0))
            }
        }
    )*};
}
impl_math_ops! {
    BitAnd, bitand;
    BitOr, bitor;
    BitXor, bitxor;
}

macro_rules! impl_math_assign_ops {
    ($($trait:ident,$fn:ident;)*) => {$(
        impl $trait for BitBoard {
            fn $fn(&mut self, other: Self) {
                $trait::$fn(&mut self.0, other.0)
            }
        }
    )*};
}
impl_math_assign_ops! {
    BitAndAssign, bitand_assign;
    BitOrAssign, bitor_assign;
    BitXorAssign, bitxor_assign;
}

impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl BitBoard {
    pub const EMPTY: Self = Self(0);

    pub fn popcnt(self) -> u32 {
        self.0.count_ones()
    }

    pub fn has(self, square: Square) -> bool {
        !(self & square.bitboard()).is_empty()
    }

    pub fn is_empty(self) -> bool {
        self == BitBoard::EMPTY
    }

    pub fn next_square(self) -> Option<Square> {
        Square::try_index(self.0.trailing_zeros() as usize)
    }
}

impl IntoIterator for BitBoard {
    type Item = Square;
    type IntoIter = BitBoardIter;

    fn into_iter(self) -> Self::IntoIter {
        BitBoardIter(self)
    }
}

pub struct BitBoardIter(BitBoard);

impl Iterator for BitBoardIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        let square = self.0.next_square();
        if let Some(square) = square {
            self.0 ^= square.bitboard();
        }
        square
    }
}

#[macro_export]
macro_rules! __bitboard {
    (
        $a8:tt $b8:tt $c8:tt $d8:tt $e8:tt $f8:tt $g8:tt $h8:tt
        $a7:tt $b7:tt $c7:tt $d7:tt $e7:tt $f7:tt $g7:tt $h7:tt
        $a6:tt $b6:tt $c6:tt $d6:tt $e6:tt $f6:tt $g6:tt $h6:tt
        $a5:tt $b5:tt $c5:tt $d5:tt $e5:tt $f5:tt $g5:tt $h5:tt
        $a4:tt $b4:tt $c4:tt $d4:tt $e4:tt $f4:tt $g4:tt $h4:tt
        $a3:tt $b3:tt $c3:tt $d3:tt $e3:tt $f3:tt $g3:tt $h3:tt
        $a2:tt $b2:tt $c2:tt $d2:tt $e2:tt $f2:tt $g2:tt $h2:tt
        $a1:tt $b1:tt $c1:tt $d1:tt $e1:tt $f1:tt $g1:tt $h1:tt
    ) => {
        $crate::__bitboard! { @__inner
            $a1 $b1 $c1 $d1 $e1 $f1 $g1 $h1
            $a2 $b2 $c2 $d2 $e2 $f2 $g2 $h2
            $a3 $b3 $c3 $d3 $e3 $f3 $g3 $h3
            $a4 $b4 $c4 $d4 $e4 $f4 $g4 $h4
            $a5 $b5 $c5 $d5 $e5 $f5 $g5 $h5
            $a6 $b6 $c6 $d6 $e6 $f6 $g6 $h6
            $a7 $b7 $c7 $d7 $e7 $f7 $g7 $h7
            $a8 $b8 $c8 $d8 $e8 $f8 $g8 $h8
        }
    };
    (@__inner $($occupied:tt)*) => {{
        let mut index = 0;
        let mut bitboard = $crate::BitBoard::EMPTY;
        $(
            if $crate::__bitboard!(@__square $occupied) {
                bitboard.0 |= 1 << index;
            }
            index += 1;
        )*
        let _ = index;
        bitboard
    }};
    (@__square X) => { true };
    (@__square .) => { false };
    (@__square $token:tt) => {
        compile_error!(
            concat!(
                "Expected only `X` or `.` tokens, found `",
                stringify!($token),
                "`"
            )
        )
    };
    ($($token:tt)*) => {
        compile_error!("Expected 64 squares")
    };
}
pub use __bitboard as bitboard;

impl std::fmt::Debug for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "bitboard! {{")?;
            for &rank in Rank::ALL.iter().rev() {
                write!(f, "\n   ")?;
                for &file in &File::ALL {
                    if self.has(Square::new(file, rank)) {
                        write!(f, " X")?;
                    } else {
                        write!(f, " .")?;
                    }
                }
            }
            write!(f, "\n}}")?;
            Ok(())
        } else {
            write!(f, "BitBoard({:#018X})", self.0)
        }
    }
}
