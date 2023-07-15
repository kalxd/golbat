#[macro_export]
macro_rules! into_ptr {
	($e:expr) => {
		Box::into_raw(Box::new($e))
	};
}
