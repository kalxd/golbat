use scraper::{html::Select, Html, Selector};
use std::ffi::c_char;

use crate::into_ptr;
use crate::rule::{drop_ptr, unsafe_str};

#[no_mangle]
pub unsafe extern "C" fn free_html(ptr: *mut Html) {
	drop_ptr(ptr);
}

#[no_mangle]
pub extern "C" fn parse_html(content: *const c_char) -> *const Html {
	let content = unsafe { unsafe_str(content) };
	let html = Html::parse_document(content);
	into_ptr!(html)
}

#[no_mangle]
pub extern "C" fn parse_fragment(char_ptr: *const c_char) -> *const Html {
	let fragment = unsafe { unsafe_str(char_ptr) };
	let html = Html::parse_fragment(fragment);
	into_ptr!(html)
}

#[no_mangle]
pub extern "C" fn html_select<'a, 'b>(
	html: *const Html,
	selector: *const Selector,
) -> *mut Select<'a, 'b> {
	let html = unsafe { html.as_ref().unwrap() };
	let selector = unsafe { selector.as_ref().unwrap() };
	into_ptr!(html.select(&selector))
}
