use std::ffi::{CString, c_char};

use reqwest::{
	Url,
	blocking::{Client, RequestBuilder},
};

use crate::{
	into_ptr, into_result, not_ok,
	result::CResult,
	rule::{drop_ptr, unsafe_str},
};

#[unsafe(no_mangle)]
extern "C" fn client_dbg(client: *const RequestBuilder) {
	let client = unsafe { &*client };
	dbg!(client);
}

#[unsafe(no_mangle)]
extern "C" fn client_free(client: *mut RequestBuilder) {
	unsafe { drop_ptr(client) }
}

fn new_client_request(url: &str) -> Result<RequestBuilder, String> {
	let url = Url::parse(url).map_err(|e| e.to_string())?;
	let client = Client::new();
	Ok(client.get(url))
}

#[unsafe(no_mangle)]
extern "C" fn client_make_request(url: *const c_char) -> *const CResult<*const RequestBuilder> {
	let url = unsafe { unsafe_str(url) };

	into_result!(new_client_request(url))
}

#[unsafe(no_mangle)]
extern "C" fn client_set_header(
	key: *const c_char,
	value: *const c_char,
	req: *mut RequestBuilder,
) -> *const RequestBuilder {
	let (key, value) = unsafe { (unsafe_str(key), unsafe_str(value)) };
	let req = unsafe { Box::from_raw(req) };

	into_ptr!(req.header(key, value))
}

fn client_receive_text(req: Box<RequestBuilder>) -> reqwest::Result<String> {
	req.send()?.text()
}

#[unsafe(no_mangle)]
extern "C" fn client_text(req: *mut RequestBuilder) -> *const CResult<*const c_char> {
	let req = unsafe { Box::from_raw(req) };

	let result = client_receive_text(req).map_err(|e| e.to_string());
	match result {
		Ok(c) => {
			let ptr: *const _ = CString::new(c).unwrap().into_raw();
			Box::into_raw(Box::new(Ok(ptr)))
		}
		Err(e) => not_ok!(e),
	}
}
