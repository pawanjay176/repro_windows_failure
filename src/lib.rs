include!("bindings.rs");

/// Converts a hex string (with or without the 0x prefix) to bytes.
pub fn hex_to_bytes(hex_str: &str) -> Vec<u8> {
    let trimmed_str = hex_str.strip_prefix("0x").unwrap_or(hex_str);
    hex::decode(trimmed_str).expect("invalid hex")
}

impl Blob {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        if bytes.len() != BYTES_PER_BLOB {
            panic!("Invalid bytes");
        }
        let mut new_bytes = [0; BYTES_PER_BLOB];
        new_bytes.copy_from_slice(bytes);
        Self { bytes: new_bytes }
    }

    pub fn from_hex(hex_str: &str) -> Self {
        Self::from_bytes(&hex_to_bytes(hex_str))
    }
}

impl Bytes32 {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        if bytes.len() != 32 {
            panic!("invalid bytes");
        }
        let mut new_bytes = [0; 32];
        new_bytes.copy_from_slice(bytes);
        Self { bytes: new_bytes }
    }

    pub fn from_hex(hex_str: &str) -> Self {
        Self::from_bytes(&hex_to_bytes(hex_str))
    }
}

impl Bytes48 {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        if bytes.len() != 48 {
            panic!("invalid bytes");
        }
        let mut new_bytes = [0; 48];
        new_bytes.copy_from_slice(bytes);
        Self { bytes: new_bytes }
    }

    pub fn from_hex(hex_str: &str) -> Self {
        Self::from_bytes(&hex_to_bytes(hex_str))
    }

    pub fn into_inner(self) -> [u8; 48] {
        self.bytes
    }
}

use std::ops::{Deref, DerefMut};

impl Deref for Bytes32 {
    type Target = [u8; 32];
    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

impl Deref for Bytes48 {
    type Target = [u8; 48];
    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

impl Deref for Blob {
    type Target = [u8; BYTES_PER_BLOB];
    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

impl DerefMut for Blob {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bytes
    }
}

impl From<[u8; BYTES_PER_BLOB]> for Blob {
    fn from(value: [u8; BYTES_PER_BLOB]) -> Self {
        Self { bytes: value }
    }
}

impl From<[u8; 32]> for Bytes32 {
    fn from(value: [u8; 32]) -> Self {
        Self { bytes: value }
    }
}

impl From<[u8; 48]> for Bytes48 {
    fn from(value: [u8; 48]) -> Self {
        Self { bytes: value }
    }
}

pub fn verify_blob_safe(blob: Blob, commitment: Bytes48, proof: Bytes48) -> bool {
    let mut res = false;
    unsafe {
        verify_blob(&mut res, &blob, &commitment, &proof);
    }
    res
}
