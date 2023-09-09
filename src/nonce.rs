use candid::CandidType;
use serde::{Deserialize, Serialize};
mod test;

#[derive(
    CandidType,
    Default,
    Hash,
    Serialize,
    Deserialize,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Debug,
    Clone,
)]
pub struct Nonce(pub u64);

impl Nonce {
    pub fn new(from: Option<u64>) -> Self {
        Self(from.unwrap_or(0))
    }

    pub fn from_le_bytes(bytes: [u8; 8]) -> Self {
        Nonce(u64::from_le_bytes(bytes))
    }

    pub fn to_le_bytes(&self) -> [u8; 8] {
        self.0.to_le_bytes()
    }

    pub fn zero() -> Self {
        Self(0)
    }

    pub fn current(&self) -> Nonce {
        self.clone()
    }

    pub fn get(&self) -> u64 {
        self.0
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }

    /// increment the counter and return the new value
    pub fn next(&mut self) -> Nonce {
        self.increment();

        self.current()
    }
}

impl TryFrom<&[u8]> for Nonce {
    type Error = std::array::TryFromSliceError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self(u64::from_le_bytes(value.try_into()?)))
    }
}

impl From<u64> for Nonce {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Nonce> for u64 {
    fn from(value: Nonce) -> Self {
        value.0
    }
}

impl From<Nonce> for Vec<u8> {
    fn from(value: Nonce) -> Self {
        value.0.to_le_bytes().to_vec()
    }
}
