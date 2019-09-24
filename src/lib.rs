//! This crate defines a trait which contains commonly used behaviours for hashing.
//! An example of an implementation of this trait can be found in the [`libhash-sha3`](https://github.com/Fantom-foundation/libhash-sha3) crate.
use crate::errors::Error;
use core::hash::Hash as CoreHash;
use serde::Serialize;
use to_vec::ToVec;

/// The Hashing trait. So far, the trait requires an array of 8-bit unsigned integers as a defined
/// type.
pub trait Hash: AsRef<[u8]> + ToVec<u8> + Default + CoreHash + Copy + PartialEq {
    /// Creates new Hash out of the provided digest.
    fn new(digest: &[u8]) -> Self
    where
        Self: Sized;
    /// Creates a new hash from the inputted data. Data must be serializable for hash to function.
    fn hash<D: Serialize>(data: &D) -> Result<Self, Error>
    where
        Self: Sized;
}

pub mod errors;

#[cfg(test)]
mod tests {}
