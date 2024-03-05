use super::*;
use std::ptr;

pub struct RangeVecIntoIter<T> {
	range_vec: RangeVec<T>,
	index: usize,
}
impl<T> RangeVecIntoIter<T> {
	fn new(range_vec: RangeVec<T>) -> Self {
		Self {
			range_vec,
			index: 0,
		}
	}
}
impl<T> Iterator for RangeVecIntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		if self.index >= self.range_vec.len {
			None
		} else {
			Some(
				unsafe {
					ptr::read(self.range_vec.pointer.add(self.index))
				}
			)
		}
	}
}

pub struct RangeVecIter<'a, T> {
	range_vec: &'a RangeVec<T>,
	index: usize,
}
impl<'a, T> RangeVecIter<'a, T> {
	fn new(range_vec: &'a RangeVec<T>) -> Self {
		Self {
			range_vec,
			index: 0,
		}
	}
}
impl<'a, T> Iterator for RangeVecIter<'a, T> {
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		self.index += 1;
		self.range_vec.get(self.index - 1)
	}
}

pub struct RangeVecIterMut<'a, T> {
	range_vec: &'a mut RangeVec<T>,
	index: usize,
}
impl<'a, T> RangeVecIterMut<'a, T> {
	fn new(range_vec: &'a mut RangeVec<T>) -> Self {
		Self {
			range_vec,
			index: 0,
		}
	}
}
impl<'a, T> Iterator for RangeVecIterMut<'a, T> {
	type Item = &'a mut T;
	fn next(&mut self) -> Option<Self::Item> {
		self.index += 1;
		if self.index > self.range_vec.len {
			None
		} else {
			Some(
				unsafe {
					&mut *(self.range_vec.pointer.add(self.index - 1)) 
				}
			)
		}
	}
}



impl<T> RangeVec<T> {
	pub fn iter<'a>(&'a self) -> RangeVecIter<'a, T> {
		RangeVecIter::new(self)
	}
	pub fn iter_mut<'a>(&'a mut self) -> RangeVecIterMut<'a, T> {
		RangeVecIterMut::new(self)
	}
}
impl<T> IntoIterator for RangeVec<T> {
	type Item = T;
	type IntoIter = RangeVecIntoIter<Self::Item>;
	fn into_iter(self) -> Self::IntoIter {
		RangeVecIntoIter::new(self)
	}
}