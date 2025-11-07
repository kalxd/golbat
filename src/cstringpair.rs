use std::ffi::{c_char, CString};

use crate::rule::drop_ptr;

#[derive(Debug)]
pub struct CStringPair<'a> {
	pub fst: &'a str,
	pub snd: &'a str,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_cstring_pair(ptr: *mut CStringPair) {
	unsafe { drop_ptr(ptr) }
}

#[unsafe(no_mangle)]
pub extern "C" fn cstring_pair_first(ptr: *const CStringPair) -> *const c_char {
	let el = unsafe { &*ptr };
	CString::new(el.fst).unwrap().into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn cstring_pair_second(ptr: *const CStringPair) -> *const c_char {
	let el = unsafe { &*ptr };
	CString::new(el.snd).unwrap().into_raw()
}
