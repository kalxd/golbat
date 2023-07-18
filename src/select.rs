use scraper::element_ref::Select as ElementSelect;
use scraper::html::Select as HtmlSelect;

use crate::into_ptr;
use crate::rule::drop_ptr;

#[no_mangle]
pub unsafe extern "C" fn free_html_select<'a, 'b>(ptr: *mut HtmlSelect<'a, 'b>) {
	drop_ptr(dbg!(ptr));
}

#[no_mangle]
pub unsafe extern "C" fn free_element_select<'a, 'b>(ptr: *mut ElementSelect<'a, 'b>) {
	drop_ptr(ptr);
}

#[no_mangle]
pub extern "C" fn html_select_next<'a, 'b>(
	ptr: *mut HtmlSelect<'a, 'b>,
) -> *const <HtmlSelect<'a, 'b> as Iterator>::Item {
	let iter = unsafe { &mut *ptr };
	match iter.next() {
		Some(v) => into_ptr!(v),
		None => std::ptr::null(),
	}
}

#[no_mangle]
pub extern "C" fn element_select_next<'a, 'b>(
	ptr: *mut ElementSelect<'a, 'b>,
) -> *const <ElementSelect<'a, 'b> as Iterator>::Item {
	let iter = unsafe { &mut *ptr };
	match iter.next() {
		Some(v) => into_ptr!(v),
		None => std::ptr::null(),
	}
}
