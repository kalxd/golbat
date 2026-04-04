use std::ffi::{CString, c_char};

use reqwest::{
	Url,
	blocking::{Client, RequestBuilder},
};

use crate::{
	into_ptr, not_ok,
	result::CResult,
	rule::{drop_ptr, into_c_string, unsafe_str},
};

#[unsafe(no_mangle)]
extern "C" fn request_free(client: *mut RequestBuilder) {
	unsafe { drop_ptr(client) }
}

#[unsafe(no_mangle)]
extern "C" fn request_make_get(url: *const Url) -> *const RequestBuilder {
	let url = unsafe { &*url };
	let client = Client::new();
	let x = client.get(url.clone());

	into_ptr!(x)
}

#[unsafe(no_mangle)]
extern "C" fn request_set_header(
	key: *const c_char,
	value: *const c_char,
	req: *mut RequestBuilder,
) -> *const RequestBuilder {
	let key = unsafe { unsafe_str(key) };
	let value = unsafe { unsafe_str(value) };
	let req = unsafe { Box::from_raw(req) };

	into_ptr!(req.header(key, value))
}

fn receive_text(req: Box<RequestBuilder>) -> reqwest::Result<String> {
	req.send()?.text()
}

#[unsafe(no_mangle)]
extern "C" fn request_text(req: *mut RequestBuilder) -> *const CResult<*const c_char> {
	let req = unsafe { Box::from_raw(req) };

	let result = receive_text(req).map_err(|e| e.to_string());
	match result {
		Ok(c) => {
			let ptr: *const _ = CString::new(c).unwrap().into_raw();
			Box::into_raw(Box::new(Ok(ptr)))
		}
		Err(e) => not_ok!(e),
	}
}

fn saving_file(file_path: &str, req: Box<RequestBuilder>) -> Result<(), String> {
	use std::fs::File;

	let mut file = File::create(file_path).map_err(|e| e.to_string())?;
	let mut rsp = req.send().map_err(|e| e.to_string())?;
	rsp.copy_to(&mut file).map_err(|e| e.to_string())?;
	Ok(())
}

#[unsafe(no_mangle)]
extern "C" fn request_download(
	save_path: *const c_char,
	req: *mut RequestBuilder,
) -> *const c_char {
	let req = unsafe { Box::from_raw(req) };
	let file_path = unsafe { unsafe_str(save_path) };

	match saving_file(file_path, req) {
		Ok(_) => std::ptr::null(),
		Err(e) => CString::new(e).unwrap().into_raw(),
	}
}

#[unsafe(no_mangle)]
extern "C" fn request_show(client: *const RequestBuilder) -> *const c_char {
	let client = unsafe { &*client };
	let s = format!("{:?}", client);
	into_c_string(&s)
}
