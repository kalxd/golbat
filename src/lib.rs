use std::ffi::{c_char, CStr};

use scraper::Selector;

mod element;
mod html;
mod rule;

pub use element::*;
pub use html::*;

#[no_mangle]
pub extern "C" fn build_selector(char_ptr: *const c_char) -> *const Selector {
    let cstr = unsafe { CStr::from_ptr(char_ptr) };
    raw_point!(Selector::parse(cstr.to_str().unwrap()).unwrap())
}
