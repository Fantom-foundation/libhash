use crate::errors::Error;
use serde::{Deserialize, Serialize};

// Hash type used in
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Hash(pub [u8; 32]);

pub trait Hashing {
    /// creates new Hash out of serialisable data provided
    fn hash<D: Serialize>(data: &D) -> Result<Hash, Error>;
    // creates new Hash out of digest provided
    fn new(digest: &[u8]) -> Hash {
        let mut a: [u8; 32] = [0; 32];
        a.copy_from_slice(&digest[0..32]);
        Hash(a)
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

pub mod errors;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
