use std::fmt::Display;

use num_traits::{Float, PrimInt, Num, NumOps, NumRef, ToPrimitive};
use crate::error::InterpError;

use super::error::Result;

pub trait InterpNum: Num + NumOps + NumRef + PartialOrd + Display + ToPrimitive + Copy {}

impl<Q: Num + NumOps + NumRef + PartialOrd + Display + ToPrimitive + Copy> InterpNum for Q {}

#[derive(Debug, Clone, Copy)]
pub struct Pair<T: Num, U: Float> {
    pub(crate) x: T,
    pub(crate) y: U,
}

impl<T: InterpNum, U: Float> PartialOrd for Pair<T, U> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.x.partial_cmp(&other.x)
    }
}

impl<T: InterpNum, U: Float> Eq for Pair<T, U> { }

impl<T: InterpNum, U: Float> PartialEq for Pair<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.x.eq(&other.x)
    }
}

impl<T: Float + InterpNum, U: Float> Pair<T, U> {
    pub(crate) fn from_float(pair: (T, U)) -> Result<Pair<T, U>> {
        if !pair.0.is_finite() { return Err(InterpError::InvalidData) }
        Ok(Pair { x: pair.0, y: pair.1 })
    }
}

impl<T: PrimInt + InterpNum, U: Float> Pair<T, U> {
    pub(crate) fn from_int(pair: (T, U)) -> Pair<T, U> {
        Pair { x: pair.0, y: pair.1 }
    }
}


