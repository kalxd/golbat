mod cstringpair;
mod element;
mod html;
mod rule;
mod select;
mod selector;

use rule::drop_ptr;
use std::ffi::c_char;

pub use cstringpair::*;
pub use element::*;
pub use html::*;
pub use select::*;
pub use selector::*;

#[no_mangle]
pub unsafe extern "C" fn free_cstring(ptr: *mut c_char) {
	drop_ptr(ptr)
}
