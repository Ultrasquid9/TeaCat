/// A macro to create a [VecDeque](std::collections::VecDeque).
/// # Examples
/// ```
/// use webcat_lib::vecdeque;
///
/// assert_eq!(
/// 	vecdeque![1, 2, 3,],
/// 	std::collections::VecDeque::from([1, 2, 3,])
/// )
/// ```
#[macro_export]
macro_rules! vecdeque {
	( $( $x:expr ),* $(,)? ) => {
		std::collections::VecDeque::from(vec![ $( $x, )* ])
	};
}
