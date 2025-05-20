/// A macro to create a [VecDeque](std::collections::VecDeque).
/// # Examples
/// ```
/// assert_eq!(
/// 	vecde![1, 2, 3,],
/// 	std::collections::VecDeque::from([1, 2, 3,])
/// )
/// ```
#[macro_export]
macro_rules! vecde {
	( $( $x:expr ),* $(,)? ) => {
		std::collections::VecDeque::from(vec![ $( $x, )* ])
	};
}
