use candid::CandidType;

pub use b3_stable_structures::{
    cell::InitError as ExternalCellInitError, log::InitError as ExternalLogInitError,
    memory_manager::VirtualMemory, BoundedStorable, DefaultMemoryImpl, FileMemory, Memory,
    RestrictedMemory, StableBTreeMap, StableCell, StableLog, StableMinHeap, StableVec, Storable,
    VectorMemory,
};

use crate::{nonce::Nonce, NanoTimeStamp, Subaccount};

#[derive(CandidType, Clone, Debug)]
pub struct PartitionDetail {
    pub name: String,
    pub len: u64,
}

pub type DefaultVM = VirtualMemory<DefaultMemoryImpl>;

pub type DefaultVMMap<K, V> = StableBTreeMap<K, V, DefaultVM>;
pub type DefaultVMVec<T> = StableVec<T, DefaultVM>;
pub type DefaultVMLog<T> = StableLog<T, DefaultVM, DefaultVM>;
pub type DefaultVMCell<T> = StableCell<T, DefaultVM>;
pub type DefaultVMHeap<T> = StableMinHeap<T, DefaultVM>;

impl Storable for Subaccount {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        self.0.to_bytes()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Subaccount::from_slice(&bytes).unwrap()
    }
}

impl BoundedStorable for Subaccount {
    const IS_FIXED_SIZE: bool = true;
    const MAX_SIZE: u32 = 32;
}

impl Storable for NanoTimeStamp {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        self.to_le_bytes().to_vec().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        NanoTimeStamp::from_le_bytes(bytes[0..8].try_into().unwrap())
    }
}

impl BoundedStorable for NanoTimeStamp {
    const IS_FIXED_SIZE: bool = true;
    const MAX_SIZE: u32 = 8;
}

impl Storable for Nonce {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        self.to_le_bytes().to_vec().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Nonce::from_le_bytes(bytes[0..8].try_into().unwrap())
    }
}

impl BoundedStorable for Nonce {
    const IS_FIXED_SIZE: bool = true;
    const MAX_SIZE: u32 = 8;
}
