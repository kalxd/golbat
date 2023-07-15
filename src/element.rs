use std::ffi::{c_char, CStr, CString};

use scraper::{
	element_ref::{Select, Text},
	node::{Attrs, Classes},
	ElementRef, Selector,
};

use crate::into_ptr;

#[repr(C)]
pub struct CTuple {
	pub fst: *const c_char,
	pub snd: *const c_char,
}

#[no_mangle]
pub extern "C" fn element_select<'a, 'b>(
	el: *const ElementRef<'a>,
	selector: *const Selector,
) -> *mut Select<'a, 'b> {
	let el = unsafe { el.as_ref().unwrap() };
	let selector = unsafe { selector.as_ref().unwrap() };

	let iter = el.select(&selector);
	into_ptr!(iter)
}

#[no_mangle]
pub extern "C" fn next_element_select<'a, 'b>(
	select: *mut Select<'a, 'b>,
) -> *const ElementRef<'a> {
	let iter = unsafe { select.as_mut().unwrap() };
	match iter.next() {
		Some(el) => into_ptr!(el),
		None => std::ptr::null(),
	}
}

#[no_mangle]
pub extern "C" fn element_html<'a>(el: *const ElementRef<'a>) -> *const c_char {
	let el = unsafe { el.as_ref().unwrap() };
	let html = el.html();
	CString::new(html).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn element_inner_html<'a>(el: *const ElementRef<'a>) -> *const c_char {
	let el = unsafe { el.as_ref().unwrap() };
	let html = el.inner_html();
	CString::new(html).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn element_text<'a>(el: *const ElementRef<'a>) -> *mut Text {
	let el = unsafe { el.as_ref().unwrap() };
	let text = el.text();
	into_ptr!(text)
}

#[no_mangle]
pub extern "C" fn next_element_text(text: *mut Text) -> *const c_char {
	let text = unsafe { text.as_mut().unwrap() };

	match text.next() {
		Some(s) => CString::new(s).unwrap().into_raw(),
		None => std::ptr::null(),
	}
}

#[no_mangle]
pub extern "C" fn element_name<'a>(el: *const ElementRef<'a>) -> *const c_char {
	let el = unsafe { el.as_ref().unwrap() };
	let cstring = CString::new(el.value().name()).unwrap();
	cstring.into_raw()
}

#[no_mangle]
pub extern "C" fn element_id<'a>(el: *const ElementRef<'a>) -> *const c_char {
	let el = unsafe { el.as_ref().unwrap() };
	match el.value().id() {
		Some(s) => CString::new(s).unwrap().into_raw(),
		None => std::ptr::null(),
	}
}

#[no_mangle]
pub extern "C" fn element_classes<'a>(el: *const ElementRef<'a>) -> *mut Classes {
	let el = unsafe { el.as_ref().unwrap() };
	into_ptr!(el.value().classes())
}

#[no_mangle]
pub extern "C" fn next_element_classes(iter: *mut Classes) -> *const c_char {
	let iter = unsafe { iter.as_mut().unwrap() };
	match iter.next() {
		Some(s) => CString::new(s).unwrap().into_raw(),
		None => std::ptr::null(),
	}
}

#[no_mangle]
pub extern "C" fn element_attr<'a>(
	el: *const ElementRef<'a>,
	attr: *const c_char,
) -> *const c_char {
	let el = unsafe { el.as_ref().unwrap() };
	let attr_name = unsafe { CStr::from_ptr(attr) };
	match el.value().attr(attr_name.to_str().unwrap()) {
		Some(s) => CString::new(s).unwrap().into_raw(),
		None => std::ptr::null(),
	}
}

#[no_mangle]
pub extern "C" fn element_attrs<'a>(el: *const ElementRef<'a>) -> *mut Attrs<'_> {
	let el = unsafe { el.as_ref().unwrap() };
	into_ptr!(el.value().attrs())
}

#[no_mangle]
pub extern "C" fn next_element_attrs<'a>(iter: *mut Attrs<'a>) -> *const CTuple {
	let iter = unsafe { iter.as_mut().unwrap() };
	match iter.next() {
		Some((a, b)) => {
			let fst = CString::new(a).unwrap().into_raw();
			let snd = CString::new(b).unwrap().into_raw();
			into_ptr!(CTuple { fst, snd })
		}
		None => std::ptr::null(),
	}
}
