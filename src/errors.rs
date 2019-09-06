//! This file defines an error variant which can be used for the [`Hash`](https://github.com/Fantom-foundation/libhash/blob/master/src/lib.rs) trait.
#[derive(Debug)]
pub enum Error {
    /// A serialization error in `compute_hash`.
    //#[fail(display = "Serialization error: {}", _0)]
    ComputeHashSerialize(String),
}
