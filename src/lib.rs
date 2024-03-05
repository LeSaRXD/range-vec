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
	fn push_test() {
		let mut range_vec = RangeVec::new((5, 10), &vec![0, 1, 4, 9, 16, 25]).unwrap();
		range_vec.push(36).unwrap();
		let back_to_vec: Vec<i32> = range_vec.into();
		assert_eq!(back_to_vec, vec![0, 1, 4, 9, 16, 25, 36]);
	}

	#[test]
	fn pop_test() {
		let mut range_vec = RangeVec::new((5, 10), &vec![0, 1, 4, 9, 16, 25, 37]).unwrap();
		assert_eq!(range_vec.try_pop(), Some(37));
		let back_to_vec: Vec<i32> = range_vec.into();
		assert_eq!(back_to_vec, vec![0, 1, 4, 9, 16, 25]);
	}

	#[test]
	fn clear_test() {
		let mut range_vec = RangeVec::new((3, 10), &vec![0, 1, 4, 9, 16, 25, 36]).unwrap();
		assert_eq!(range_vec.clear(), 4);
		let back_to_vec: Vec<i32> = range_vec.into();
		assert_eq!(back_to_vec, vec![0, 1, 4]);
	}

	#[test]
	fn iter_test() {
		let mut range_vec = RangeVec::new((5, 10), &vec![0, 1, 1, 2, 3, 5]).unwrap();
		for elem in range_vec.iter_mut() {
			*elem *= 2;
		}
		let back_to_vec: Vec<i32> = range_vec.into();
		assert_eq!(back_to_vec, vec![0, 2, 2, 4, 6, 10]);
 	}

	#[test]
	fn equality_test() {
		let mut vec = vec![1, 2, 3, 4, 5];
		let range_vec1 = RangeVec::new((5, 12), &vec).unwrap();
		let range_vec2 = RangeVec::new((5, 12), &vec).unwrap();
		let range_vec3 = RangeVec::new((4, 12), &vec).unwrap();
		vec[4] = 6;
		let range_vec4 = RangeVec::new((4, 12), &vec).unwrap();

		assert_eq!(range_vec1, range_vec2);
		assert_ne!(range_vec1, range_vec3);
		assert_ne!(range_vec3, range_vec4);
	}

	#[test]
	fn memory_leak_test() {
		let vec: Vec<i64> = (0..256).into_iter().map(|n| n * n).collect();
		for _ in 0..100000 {
			let allocated = RangeVec::new((10, 256), &vec).unwrap();
			drop(allocated);
		}
	}
}
