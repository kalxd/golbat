use std::ffi::{CStr, CString, c_char};

use scraper::{
	ElementRef, Selector,
	element_ref::Select,
	node::{Attrs, Classes},
};

use crate::{
	into_ptr,
	rule::{drop_ptr, show},
};

#[unsafe(no_mangle)]
extern "C" fn to_str(ptr: *const c_char) -> *const c_char {
	ptr
}

#[unsafe(no_mangle)]
extern "C" fn element_free<'a>(ptr: *mut ElementRef<'a>) {
	unsafe { drop_ptr(ptr) }
}

#[unsafe(no_mangle)]
extern "C" fn element_id<'a>(ptr: *const ElementRef<'a>) -> *const c_char {
	let el = unsafe { &*ptr };
	match el.value().id() {
		None => std::ptr::null(),
		Some(s) => CString::new(s).unwrap().into_raw(),
	}
}

#[unsafe(no_mangle)]
extern "C" fn element_has_class<'a>(ptr: *const ElementRef<'a>, pstr: *const c_char) -> bool {
	let el = unsafe { &*ptr };
	let cstr = unsafe { CStr::from_ptr(pstr) }.to_str().unwrap();
	el.value()
		.has_class(cstr, scraper::CaseSensitivity::CaseSensitive)
}

#[unsafe(no_mangle)]
extern "C" fn element_attr<'a>(pstr: *const c_char, ptr: *const ElementRef<'a>) -> *const c_char {
	let el = unsafe { &*ptr };
	let cstr = unsafe { CStr::from_ptr(pstr) }.to_str().unwrap();
	match el.value().attr(cstr) {
		Some(s) => CString::new(s).unwrap().into_raw(),
		None => std::ptr::null(),
	}
}

#[unsafe(no_mangle)]
extern "C" fn free_attrs<'a>(ptr: *mut Attrs<'a>) {
	unsafe { drop_ptr(ptr) }
}

#[unsafe(no_mangle)]
extern "C" fn element_attrs<'a>(ptr: *const ElementRef<'a>) -> *const Attrs<'a> {
	let el = unsafe { &*ptr };
	into_ptr!(el.value().attrs())
}

/*
#[unsafe(no_mangle)]
pub extern "C" fn element_attrs_next<'a>(ptr: *mut Attrs<'a>) -> *const CStringPair<'a> {
	let iter = unsafe { &mut *ptr };
	match iter.next() {
		Some((fst, snd)) => into_ptr!(CStringPair { fst, snd }),
		None => std::ptr::null(),
	}
	}
	*/

#[unsafe(no_mangle)]
extern "C" fn element_html<'a>(ptr: *const ElementRef<'a>) -> *const c_char {
	let el = unsafe { &*ptr };
	let html = el.html();
	CString::new(html).unwrap().into_raw()
}

#[unsafe(no_mangle)]
extern "C" fn element_inner_html<'a>(ptr: *const ElementRef<'a>) -> *const c_char {
	let el = unsafe { &*ptr };
	let html = el.inner_html();
	CString::new(html).unwrap().into_raw()
}

#[unsafe(no_mangle)]
extern "C" fn free_classes<'a>(ptr: *mut Classes<'a>) {
	unsafe { drop_ptr(ptr) }
}

#[unsafe(no_mangle)]
extern "C" fn element_classes<'a>(ptr: *const ElementRef<'a>) -> *const Classes<'a> {
	let el = unsafe { &*ptr };
	into_ptr!(el.value().classes())
}

#[unsafe(no_mangle)]
extern "C" fn element_classes_next<'a>(ptr: *mut Classes<'a>) -> *const c_char {
	let iter = unsafe { &mut *ptr };
	match iter.next() {
		Some(s) => CString::new(s).unwrap().into_raw(),
		None => std::ptr::null(),
	}
}

#[unsafe(no_mangle)]
extern "C" fn element_select<'a, 'b>(
	ptr: *const ElementRef<'a>,
	selector_ptr: *const Selector,
) -> *const Select<'a, 'b> {
	let el = unsafe { &*ptr };
	let selector = unsafe { &*selector_ptr };
	into_ptr!(el.select(&selector))
}

#[unsafe(no_mangle)]
extern "C" fn element_text<'a>(ptr: *const ElementRef<'a>) -> *const c_char {
	let el = unsafe { &*ptr };
	match el.text().next() {
		Some(s) => CString::new(s).unwrap().into_raw(),
		None => std::ptr::null(),
	}
}

#[unsafe(no_mangle)]
extern "C" fn element_show<'a>(ptr: *const ElementRef<'a>) -> *const c_char {
	let el = unsafe { &*ptr };
	show(el)
}
