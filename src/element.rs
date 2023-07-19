use std::ffi::{c_char, CStr, CString};

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

#[no_mangle]
pub extern "C" fn element_has_class<'a>(ptr: *const ElementRef<'a>, pstr: *const c_char) -> bool {
	let el = unsafe { &*ptr };
	let cstr = unsafe { CStr::from_ptr(pstr) }.to_str().unwrap();
	el.value()
		.has_class(cstr, scraper::CaseSensitivity::CaseSensitive)
}

#[no_mangle]
pub extern "C" fn element_attr<'a>(
	ptr: *const ElementRef<'a>,
	pstr: *const c_char,
) -> *const c_char {
	let el = unsafe { &*ptr };
	let cstr = unsafe { CStr::from_ptr(pstr) }.to_str().unwrap();
	match el.value().attr(cstr) {
		Some(s) => CString::new(s).unwrap().into_raw(),
		None => std::ptr::null(),
	}
}
