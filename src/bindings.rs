pub const BYTES_PER_BLOB: usize = 131072;

#[doc = " A basic blob data."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Blob {
    pub bytes: [u8; 131072usize],
}

#[doc = " An array of 48 bytes. Represents an untrusted\n (potentially invalid) commitment/proof."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Bytes48 {
    pub bytes: [u8; 48usize],
}

#[doc = " An array of 32 bytes"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Bytes32 {
    pub bytes: [u8; 32usize],
}

extern "C" {
    pub fn verify_blob(
        ok: *mut bool,
        blobs: *const Blob,
        commitments_bytes: *const Bytes48,
        proofs_bytes: *const Bytes48,
    );
}
