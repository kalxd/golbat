#[macro_export]
macro_rules! into_ptr {
	($e:expr) => {
		Box::into_raw(Box::new($e))
	};
}

pub(crate) unsafe fn drop_ptr<T>(ptr: *mut T) {
	let v = Box::from_raw(ptr);
	drop(v);
}
