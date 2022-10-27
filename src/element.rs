use std::ffi::{c_char, CString};

use scraper::{
    element_ref::{Select, Text},
    ElementRef, Selector,
};

use crate::raw_point;

#[no_mangle]
pub extern "C" fn element_name<'a>(el: *const ElementRef<'a>) -> *const c_char {
    let el = unsafe { el.as_ref().unwrap() };
    let cstring = CString::new(el.value().name()).unwrap();
    cstring.into_raw()
}

#[no_mangle]
pub extern "C" fn element_select<'a, 'b>(
    el: *const ElementRef<'a>,
    selector: *const Selector,
) -> *mut Select<'a, 'b> {
    let el = unsafe { el.as_ref().unwrap() };
    let selector = unsafe { selector.as_ref().unwrap() };

    let iter = el.select(&selector);
    raw_point!(iter)
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
pub extern "C" fn element_text<'a>(el: *const ElementRef<'a>) -> *const Text {
    let el = unsafe { el.as_ref().unwrap() };
    let text = el.text();
    raw_point!(text)
}
