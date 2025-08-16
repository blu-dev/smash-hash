use std::hash::{BuildHasherDefault, Hash, Hasher};

pub type Hash40Set = std::collections::HashSet<Hash40, BuildHasherDefault<PassHasher>>;
pub type Hash40Map<T> = std::collections::HashMap<Hash40, T, BuildHasherDefault<PassHasher>>;

mod const_hash;

#[derive(Default)]
pub struct PassHasher(u64);

impl Hasher for PassHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, _: &[u8]) {
        unimplemented!("PassHasher only supports direct u64");
    }

    fn write_u64(&mut self, i: u64) {
        self.0 = i;
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hash40(u64);

impl Hash40 {
    pub const fn from_raw(value: u64) -> Self {
        Self(value & 0x000000FF_FFFFFFFF)
    }

    pub const fn raw(self) -> u64 {
        self.0
    }

    pub const fn const_with(self, string: &str) -> Self {
        let other = Self::const_new(string);
        Self(const_hash::hash40_concat(self.0, other.0))
    }

    pub const fn const_with_bytes(self, bytes: &[u8]) -> Self {
        let other = Self::const_new_bytes(bytes);
        Self(const_hash::hash40_concat(self.0, other.0))
    }

    pub const fn const_new(string: &str) -> Self {
        Self(const_hash::hash40(string.as_bytes()))
    }

    pub const fn const_trim_trailing(self, string: &str) -> Self {
        Self(const_hash::hash40_undo(self.0, string.as_bytes()))
    }

    pub const fn const_trim_trailing_bytes(self, bytes: &[u8]) -> Self {
        Self(const_hash::hash40_undo(self.0, bytes))
    }

    pub const fn const_new_bytes(bytes: &[u8]) -> Self {
        Self(const_hash::hash40(bytes))
    }

    pub const fn length(&self) -> u8 {
        ((self.0 & 0x000000FF_00000000) >> 0x20) as u8
    }

    pub const fn crc32(&self) -> u32 {
        (self.0 & 0x00000000_FFFFFFFF) as u32
    }
}

impl Hash for Hash40 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.0);
    }
}
