use scraper::{Html, Selector, html::Select};
use std::ffi::c_char;

use crate::into_ptr;
use crate::rule::{drop_ptr, show, unsafe_str};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn html_free(ptr: *mut Html) {
	unsafe {
		drop_ptr(ptr);
	}
}

#[unsafe(no_mangle)]
pub extern "C" fn html_parse_doc(content: *const c_char) -> *const Html {
	let content = unsafe { unsafe_str(content) };
	let html = Html::parse_document(content);
	into_ptr!(html)
}

#[unsafe(no_mangle)]
pub extern "C" fn html_parse_fragment(char_ptr: *const c_char) -> *const Html {
	let fragment = unsafe { unsafe_str(char_ptr) };
	let html = Html::parse_fragment(fragment);
	into_ptr!(html)
}

#[unsafe(no_mangle)]
pub extern "C" fn html_select<'a, 'b>(
	selector: *const Selector,
	html: *const Html,
) -> *const Select<'a, 'b> {
	let html = unsafe { html.as_ref().unwrap() };
	let selector = unsafe { selector.as_ref().unwrap() };
	into_ptr!(html.select(&selector))
}

#[unsafe(no_mangle)]
pub extern "C" fn html_dbg(ptr: *const Html) {
	let html = unsafe { &*ptr };
	dbg!(html);
}

#[unsafe(no_mangle)]
extern "C" fn html_show(ptr: *const Html) -> *const c_char {
	let html = unsafe { &*ptr };
	show(html)
}
