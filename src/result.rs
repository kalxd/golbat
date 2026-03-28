use std::ffi::{CString, c_char, c_void};

type CResult<T> = std::result::Result<T, String>;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn result_is_error(val: *const CResult<*const c_void>) -> bool {
	let val = unsafe { &*val };
	val.is_err()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn result_err_msg(val: *const CResult<*const c_void>) -> *mut c_char {
	let val = unsafe { &*val };

	let msg = val.as_ref().unwrap_err().clone();

	CString::new(msg).unwrap().into_raw()
}
