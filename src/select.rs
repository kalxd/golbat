use scraper::element_ref::Select as ElementSelect;
use scraper::html::Select as HtmlSelect;

use crate::rule::{drop_ptr, take_iter_first};

#[no_mangle]
pub unsafe extern "C" fn free_html_select<'a, 'b>(ptr: *mut HtmlSelect<'a, 'b>) {
	drop_ptr(ptr);
}

#[no_mangle]
pub unsafe extern "C" fn free_element_select<'a, 'b>(ptr: *mut ElementSelect<'a, 'b>) {
	drop_ptr(ptr);
}

#[no_mangle]
pub unsafe extern "C" fn html_html_take_first<'a, 'b>(
	ptr: *mut HtmlSelect<'a, 'b>,
) -> *const <HtmlSelect<'a, 'b> as Iterator>::Item {
	take_iter_first(ptr)
}

#[no_mangle]
pub unsafe extern "C" fn html_element_take_first<'a, 'b>(
	ptr: *mut ElementSelect<'a, 'b>,
) -> *const <ElementSelect<'a, 'b> as Iterator>::Item {
	take_iter_first(ptr)
}
