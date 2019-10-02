//! This file defines an error variant which can be used for the [`Hash`](https://github.com/Fantom-foundation/libhash/blob/master/src/lib.rs) trait.
use failure::Error as FailureError;

pub type Result<T> = std::result::Result<T, FailureError>;

#[derive(Debug, Fail)]
pub enum Error {
    /// A serialization error in `compute_hash`.
    #[fail(display = "Serialization error: {}", _0)]
    ComputeHashSerialize(String),
}
