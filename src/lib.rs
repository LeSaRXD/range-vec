pub mod range_vec;
pub mod error;

#[cfg(test)]
mod tests {
	use crate::{range_vec::*, error::RangeVecErr::*};

	#[test]
	fn create_test() {
		let mut vec = vec![1, 2, 3, 4, 5];

		assert!(RangeVec::new((6, 10), &vec).is_err_and(|e| e == TooShort));
		assert!(RangeVec::new((2, 4), &vec).is_err_and(|e| e == TooLong));

		vec.push(6);
		let range_vec: RangeVec<i32> = RangeVec::new((5, 10), &vec).unwrap();
		let back_into_vec: Vec<i32> = range_vec.into();
		assert_eq!(back_into_vec, vec);
	}

	#[test]
	fn memory_leak_test() {
		let vec: Vec<i64> = (0..256).into_iter().map(|n| n * n).collect();
		for _ in 0..100000 {
			let _vec = RangeVec::new((10, 256), &vec).unwrap();
			drop(_vec);
		}
	}
}
