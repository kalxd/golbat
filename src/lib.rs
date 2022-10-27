use std::ffi::{c_char, CStr};

use scraper::{html::Select, ElementRef, Html, Selector};

mod element;
mod rule;

pub use element::*;

#[no_mangle]
pub extern "C" fn parse_html(content: *const c_char) -> *const Html {
    let cstr = unsafe { CStr::from_ptr(content) };
    let html = Html::parse_document(cstr.to_str().unwrap());

    crate::raw_point!(html)
}

#[no_mangle]
pub extern "C" fn parse_fragment(char_ptr: *const c_char) -> *const Html {
    let cstr = unsafe { CStr::from_ptr(char_ptr) };
    let html = Html::parse_fragment(cstr.to_str().unwrap());

    crate::raw_point!(html)
}

#[no_mangle]
pub extern "C" fn build_select(char_ptr: *const c_char) -> *const Selector {
    let cstr = unsafe { CStr::from_ptr(char_ptr) };
    raw_point!(Selector::parse(cstr.to_str().unwrap()).unwrap())
}

#[no_mangle]
pub extern "C" fn select<'a, 'b>(
    html: *const Html,
    selector: *const Selector,
) -> *mut Select<'a, 'b> {
    let html = unsafe { html.as_ref().unwrap() };
    let selector = unsafe { selector.as_ref().unwrap() };
    raw_point!(html.select(&selector))
}

#[no_mangle]
pub extern "C" fn next_select<'a, 'b>(select: *mut Select<'a, 'b>) -> *const ElementRef<'a> {
    let select: &mut Select = unsafe { select.as_mut().unwrap() };

    match dbg!(select.next()) {
        Some(s) => raw_point!(s),
        None => std::ptr::null(),
    }
}
