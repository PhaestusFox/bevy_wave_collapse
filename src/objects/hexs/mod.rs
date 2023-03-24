pub mod river;
pub mod sand;
pub use trig::*;

use super::*;

mod trig {
    use fixed::{
        types::extra::{LeEqU32, LeEqU64},
        FixedI32, FixedI64,
    };

    pub trait HexTrig: Sized {
        const ROTATIONS_COS: [Self; 6];
        const ROTATIONS_SIN: [Self; 6];
    }
    impl<P: LeEqU32> HexTrig for FixedI32<P> {
        const ROTATIONS_COS: [FixedI32<P>; 6] = [
            FixedI32::<P>::lit("1."),
            FixedI32::<P>::lit("0.5"),
            FixedI32::<P>::lit("-0.5"),
            FixedI32::<P>::lit("-1."),
            FixedI32::<P>::lit("-0.5"),
            FixedI32::<P>::lit("0.5"),
        ];
        const ROTATIONS_SIN: [FixedI32<P>; 6] = [
            FixedI32::<P>::lit("0."),
            FixedI32::<P>::lit("0.86602540378"),
            FixedI32::<P>::lit("0.86602540378"),
            FixedI32::<P>::lit("0."),
            FixedI32::<P>::lit("-0.86602540378"),
            FixedI32::<P>::lit("-0.86602540378"),
        ];
    }

    impl<P: LeEqU64> HexTrig for FixedI64<P> {
        const ROTATIONS_COS: [FixedI64<P>; 6] = [
            FixedI64::<P>::lit("1."),
            FixedI64::<P>::lit("0.5"),
            FixedI64::<P>::lit("-0.5"),
            FixedI64::<P>::lit("-1."),
            FixedI64::<P>::lit("-0.5"),
            FixedI64::<P>::lit("0.5"),
        ];
        const ROTATIONS_SIN: [FixedI64<P>; 6] = [
            FixedI64::<P>::lit("0."),
            FixedI64::<P>::lit("0.86602540378"),
            FixedI64::<P>::lit("0.86602540378"),
            FixedI64::<P>::lit("0."),
            FixedI64::<P>::lit("-0.86602540378"),
            FixedI64::<P>::lit("-0.86602540378"),
        ];
    }
}
