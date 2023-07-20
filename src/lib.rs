mod cstringpair;
mod element;
mod html;
mod rule;
mod select;
mod selector;

use std::ffi::{c_char, CString};

pub use cstringpair::*;
pub use element::*;
pub use html::*;
pub use select::*;
pub use selector::*;

#[no_mangle]
pub unsafe extern "C" fn free_cstring(ptr: *mut c_char) {
	let _ = unsafe { CString::from_raw(ptr) };
}
