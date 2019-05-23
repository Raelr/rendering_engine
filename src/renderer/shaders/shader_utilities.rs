// Use
use std::ffi::{CString};

// Creates a CString for our error
pub fn create_whitespace_cstring_with_len(len: usize) -> CString {

    // Allocate buffer to correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);

    // Fill buffer with len spaces
    buffer.extend([b' '].iter().cycle().take(len as usize));

    // Convert buffer to Cstring
    unsafe { CString::from_vec_unchecked(buffer) }
}