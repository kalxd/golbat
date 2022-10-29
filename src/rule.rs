#[macro_export]
macro_rules! raw_point {
	($e:expr) => {
		Box::into_raw(Box::new($e))
	};
}
