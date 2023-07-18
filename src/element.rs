use std::ffi::{c_char, CString};

use scraper::ElementRef;

use crate::rule::drop_ptr;

#[no_mangle]
pub unsafe extern "C" fn free_element_ref<'a>(ptr: *mut ElementRef<'a>) {
	drop_ptr(ptr)
}

#[no_mangle]
pub extern "C" fn element_id<'a>(ptr: *const ElementRef<'a>) -> *const c_char {
	let el = unsafe { &*ptr };
	match el.value().id() {
		None => std::ptr::null(),
		Some(s) => CString::new(s).unwrap().into_raw(),
	}
}
