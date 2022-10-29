use scraper::{html::Select, ElementRef, Html, Selector};
use std::ffi::{c_char, CStr};

use crate::raw_point;

#[no_mangle]
pub extern "C" fn parse_html(content: *const c_char) -> *const Html {
	let cstr = unsafe { CStr::from_ptr(content) };
	let html = Html::parse_document(cstr.to_str().unwrap());
	raw_point!(html)
}

#[no_mangle]
pub extern "C" fn parse_fragment(char_ptr: *const c_char) -> *const Html {
	let cstr = unsafe { CStr::from_ptr(char_ptr) };
	let html = Html::parse_fragment(cstr.to_str().unwrap());
	raw_point!(html)
}

#[no_mangle]
pub extern "C" fn html_select<'a, 'b>(
	html: *const Html,
	selector: *const Selector,
) -> *mut Select<'a, 'b> {
	let html = unsafe { html.as_ref().unwrap() };
	let selector = unsafe { selector.as_ref().unwrap() };
	raw_point!(html.select(&selector))
}

#[no_mangle]
pub extern "C" fn next_html_select<'a, 'b>(select: *mut Select<'a, 'b>) -> *const ElementRef<'a> {
	let select: &mut Select = unsafe { select.as_mut().unwrap() };

	match select.next() {
		Some(s) => raw_point!(s),
		None => std::ptr::null(),
	}
}
