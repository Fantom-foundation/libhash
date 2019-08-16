use crate::errors::Error;
use serde::Serialize;

pub trait Hashing {
    type Hash: AsRef<[u8]>;
    /// creates new Hash out of digest provided
    fn new(digest: &[u8]) -> Self::Hash;
    /// creates new Hash out of serialisable data provided
    fn hash<D: Serialize>(data: &D) -> Result<Self::Hash, Error>;
}

pub mod errors;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
