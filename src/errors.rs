/// Hashing errors.
#[derive(Debug)]
pub enum Error {
    /// A serialization error in `compute_hash`.
    //#[fail(display = "Serialization error: {}", _0)]
    ComputeHashSerialize(String),
}
