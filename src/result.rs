use std::ffi::{c_char, c_void};

pub type CResult<T> = std::result::Result<T, *mut c_char>;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn result_is_error(val: *const CResult<*const c_void>) -> u8 {
	let val = unsafe { &*val };
	if val.is_err() { 1 } else { 0 }
}

/// 获取CResult的错误信息。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn result_err_msg(val: *mut CResult<*const c_void>) -> *mut c_char {
	let val = unsafe { Box::from_raw(val) };

	val.unwrap_err()
}

/// 获取CResult真实的值。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn result_value(val: *mut CResult<*const c_void>) -> *const c_void {
	let val = unsafe { Box::from_raw(val) };
	val.unwrap()
}

#[macro_export]
macro_rules! ok {
	($e:expr) => {
		Box::into_raw(Box::new(Ok(Box::into_raw(Box::new($e)))))
	};
}

#[macro_export]
macro_rules! not_ok {
	($x:expr) => {{
		let s = std::ffi::CString::new($x).unwrap();
		Box::into_raw(Box::new(Err(s.into_raw())))
	}};
}
