use std::{fmt::Display, ops::{Add, Sub, Div, Mul}};

use num_traits::{Float, PrimInt, Num, NumOps, NumRef, ToPrimitive, One, Zero};
use crate::error::InterpError;

use super::error::Result;

pub trait InterpNum: Num + NumOps + NumRef + PartialOrd + Display + ToPrimitive + Copy {}
pub trait InterpFloat: Mul<Output=Self> + Add<Output=Self> + Sub<Output=Self> + Div<Output=Self> + One + Zero + Copy {}

impl<Q: Num + NumOps + NumRef + PartialOrd + Display + ToPrimitive + Copy> InterpNum for Q {}
impl<Q: Mul<Output=Self> + Add<Output=Self> + Sub<Output=Self> + Div<Output=Self> + One + Zero + Copy> InterpFloat for Q {}

#[derive(Debug, Clone, Copy)]
pub struct Pair<T: Num, U: InterpFloat> {
    pub(crate) x: T,
    pub(crate) y: U,
}

impl<T: InterpNum, U: InterpFloat> PartialOrd for Pair<T, U> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.x.partial_cmp(&other.x)
    }
}

impl<T: InterpNum, U: InterpFloat> Eq for Pair<T, U> { }

impl<T: InterpNum, U: InterpFloat> PartialEq for Pair<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.x.eq(&other.x)
    }
}

impl<T: Float + InterpNum, U: InterpFloat> Pair<T, U> {
    pub(crate) fn from_float(pair: (T, U)) -> Result<Pair<T, U>> {
        if !pair.0.is_finite() { return Err(InterpError::InvalidData) }
        Ok(Pair { x: pair.0, y: pair.1 })
    }
}

impl<T: PrimInt + InterpNum, U: InterpFloat> Pair<T, U> {
    pub(crate) fn from_int(pair: (T, U)) -> Pair<T, U> {
        Pair { x: pair.0, y: pair.1 }
    }
}


