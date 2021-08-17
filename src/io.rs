//! DOS inputs/outputs
//!
//! Provides the definitions for all the inputs and outputs used by DOS

use core::fmt::Debug;
use serde::{Deserialize, Serialize};
use std::fmt;

/// IO Error type
#[derive(Debug)]
pub enum IOError<T> {
    Missing(IO<T>),
}
impl<T: Debug> fmt::Display for IOError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing(v) => write!(f, "{:?} is missing", v),
        }
    }
}
impl<T: Debug> std::error::Error for IOError<T> {}

dosio_proc::ad_hoc! {}
impl<T, U: Debug> std::ops::Index<IO<U>> for Vec<IO<T>> {
    type Output = IO<T>;
    fn index(&self, io: IO<U>) -> &Self::Output {
        self.iter()
            .position(|x| *x == io)
            .ok_or_else(|| format!("No {:?} entry in `Vec<IO>`", io))
            .map(|i| &self[i])
            .unwrap()
    }
}
impl<T, U: Debug> std::ops::Index<&IO<U>> for Vec<IO<T>> {
    type Output = IO<T>;
    fn index(&self, io: &IO<U>) -> &Self::Output {
        self.iter()
            .position(|x| x == io)
            .ok_or_else(|| format!("No {:?} entry in `Vec<IO>`", io))
            .map(|i| &self[i])
            .unwrap()
    }
}
impl<T, U: Debug> std::ops::IndexMut<IO<U>> for Vec<IO<T>> {
    fn index_mut(&mut self, io: IO<U>) -> &mut Self::Output {
        self.iter()
            .position(|x| *x == io)
            .ok_or_else(|| format!("No {:?} entry in `Vec<IO>`", io))
            .map(move |i| &mut self[i])
            .unwrap()
    }
}

/// A type for empty `IO`
pub type Tags = IO<()>;
