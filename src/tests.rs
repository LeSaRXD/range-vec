use crate::{RangeVec, error::RangeVecErr::*};

#[test]
fn create() {
	let mut vec = vec![1, 2, 3, 4, 5];

	assert_eq!(RangeVec::new((6, 10), &vec), Err(TooShort));
	assert_eq!(RangeVec::new((2, 4), &vec), Err(TooLong));

	vec.push(6);
	let range_vec: RangeVec<i32> = RangeVec::new((5, 10), &vec).unwrap();
	assert_eq!(range_vec, vec);
}

#[test]
fn push() {
	let mut range_vec = RangeVec::new((5, 10), &vec![0, 1, 4, 9, 16, 25]).unwrap();
	range_vec.push(36).unwrap();
	assert_eq!(range_vec, vec![0, 1, 4, 9, 16, 25, 36]);
}

#[test]
fn pop() {
	let mut range_vec = RangeVec::new((5, 10), &vec![0, 1, 4, 9, 16, 25, 37]).unwrap();
	assert_eq!(range_vec.try_pop(), Some(37));
	assert_eq!(range_vec, vec![0, 1, 4, 9, 16, 25]);
}

#[test]
fn extend() {
	let mut range_vec = RangeVec::new((5, 10), &vec![0, 1, 4, 9, 16, 25, 36]).unwrap();
	let mut adding = vec![49, 64, 81, 100];
	
	assert_eq!(range_vec.extend(&adding), Err(CantAdd));
	
	adding.pop();
	assert_eq!(range_vec.try_extend(&adding), Some(10));
	
	assert_eq!(range_vec, vec![0, 1, 4, 9, 16, 25, 36, 49, 64, 81]);
}

#[test]
fn clear() {
	let mut range_vec = RangeVec::new((3, 10), &vec![0, 1, 4, 9, 16, 25, 36]).unwrap();
	assert_eq!(range_vec.clear(), 4);
	assert_eq!(range_vec, vec![0, 1, 4]);
}

#[test]
fn iter() {
	let mut range_vec = RangeVec::new((5, 10), &vec![0, 1, 1, 2, 3, 5]).unwrap();
	for elem in range_vec.iter_mut() {
		*elem *= 2;
	}
	assert_eq!(range_vec, vec![0, 2, 2, 4, 6, 10]);
}

#[test]
fn equality() {
	let mut vec = vec![1, 2, 3, 4, 5];
	let range_vec1 = RangeVec::new((5, 12), &vec).unwrap();
	let range_vec2 = RangeVec::new((5, 12), &vec).unwrap();
	let range_vec3 = RangeVec::new((4, 12), &vec).unwrap();
	
	assert_eq!(range_vec1, vec);
	assert_eq!(range_vec2, vec);
	assert_eq!(range_vec3, vec);

	vec[4] = 6;
	let range_vec4 = RangeVec::new((4, 12), &vec).unwrap();
	assert_eq!(range_vec4, vec);

	assert_eq!(range_vec1, range_vec2);
	assert_ne!(range_vec1, range_vec3);
	assert_ne!(range_vec3, range_vec4);
}

#[test]
fn clone() {
	let range_vec1 = RangeVec::new((5, 10), &vec![0, 1, 1, 2, 3, 5, 8]).unwrap();
	let range_vec2 = range_vec1.clone();
	assert_eq!(range_vec1, range_vec2);
	assert_ne!(range_vec1.pointer, range_vec2.pointer);

	let vec1: Vec<i32> = range_vec1.into();
	let vec2: Vec<i32> = range_vec2.into();
	assert_eq!(vec1, vec2);
}

#[test]
fn to_string() {
	let range_vec = RangeVec::new((5, 10), &vec![0, 1, 1, 2, 3]).unwrap();
	assert_eq!(range_vec.to_string(),
r#"[
    0,
    1,
    1,
    2,
    3,
]"#
	);
}

#[test]
fn index_slice() {
	let range_vec = RangeVec::new((5, 10), &vec![0, 1, 1, 2, 3, 5, 8, 13]).unwrap();
	assert_eq!(range_vec[6], 8);

	let slice1 = range_vec.get_slice(3..6).unwrap();
	assert_eq!(slice1[0], 2);
	assert_eq!(slice1[1], 3);
	assert_eq!(slice1[2], 5);

	let slice2 = range_vec.get_slice(4..).unwrap();
	assert_eq!(slice2[3], 13);

	let slice3 = range_vec.as_slice();
	let slice4 = range_vec.get_slice(..).unwrap();
	let vec: Vec<i32> = range_vec.clone().into();
	assert_eq!(slice3, slice4);
	assert_eq!(slice3, vec.as_slice());
}

#[test]
fn memory_leak() {
	let vec: Vec<i64> = (0..256).into_iter().map(|n| n * n).collect();
	for _ in 0..100000 {
		let allocated = RangeVec::new((10, 256), &vec).unwrap();
		drop(allocated);
	}
}
