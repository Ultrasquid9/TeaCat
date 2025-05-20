#[macro_export]
macro_rules! vecde {
	( $( $x:expr ),* $(,)? ) => {
		std::collections::VecDeque::from(vec![ $( $x, )* ])
	};
}

#[test]
fn vecde() {
	assert_eq!(
		vecde![1, 2, 3,],
		std::collections::VecDeque::from([1, 2, 3,])
	)
}
