pub mod iter;

use crate::error::{RangeVecResult, RangeVecErr::*};
use std::ops::RangeInclusive;
use std::alloc::{alloc, dealloc, Layout};
use std::mem::{size_of, align_of};
use std::ptr;

#[derive(Debug)]
pub struct RangeVec<T> {
	pointer: *mut T,
	min_size: usize,
	max_size: usize,
	len: usize,
}

impl<T> RangeVec<T> {
	// Create a new `RangeVec` given the minimum and maximum size, and the initial elements
	pub fn new((min_size, max_size): (usize, usize), elements: &Vec<T>) -> RangeVecResult<Self> {
		if elements.len() < min_size {
			return Err(TooShort)
		}
		if elements.len() > max_size {
			return Err(TooLong)
		}
			
		let pointer = unsafe {
			alloc(
				Layout::from_size_align_unchecked(max_size * size_of::<T>(), align_of::<T>())
			) as *mut T
		};
		unsafe {
			for (i, elem) in elements.iter().enumerate() {
				ptr::write(pointer.add(i), ptr::read(elem));
			}
		}
		Ok(Self { pointer, min_size, max_size, len: elements.len() })
	}
	
	// Get a reference to an element by `index` if it exists
	pub fn get(&self, index: usize) -> Option<&T> {
		if index < self.len {
			unsafe { Some(& *(self.pointer.add(index))) }
		} else {
			None
		}
	}
	// Get a mutable reference to an element by `index` if it exists
	pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
		if index < self.len {
			unsafe { Some(&mut *(self.pointer.add(index))) }
		} else {
			None
		}
	}
	
	// Get the size of the vector
	pub fn size(&self) -> (usize, usize) {
		(self.min_size, self.max_size)
	}
	// Get the size of the vector as a `RangeInclusive`
	pub fn size_as_range(&self) -> RangeInclusive<usize> {
		self.min_size..=self.max_size
	}
	// Get the minimum size of the vector
	pub fn min_size(&self) -> usize {
		self.min_size
	}
	// Get the maximum size of the vector
	pub fn max_size(&self) -> usize {
		self.max_size
	}
	// Get the current length of the vector
	pub fn len(&self) -> usize {
		self.len
	}

	// Adds an element to the end of the vector if there is space left
	pub fn push(&mut self, element: T) -> RangeVecResult<()> {
		if self.len >= self.max_size {
			return Err(CantAdd);
		}
		unsafe {
			ptr::write(self.pointer.add(self.len), element);
		}
		self.len += 1;
		Ok(())
	}
	// Tries adding an element to the end of the vector if there is space left.
	// Returns Some(()) if it succeeded, None otherwise
	pub fn try_push(&mut self, element: T) -> Option<()> {
		self.push(element).ok()
	}

	// Removes and returns an element from the end of the vector if there are enough left
	pub fn pop(&mut self) -> RangeVecResult<T> {
		if self.len <= self.min_size {
			return Err(CantRemove);
		}
		let elem = unsafe {
			ptr::read(self.pointer.add(self.len - 1))
		};
		self.len -= 1;
		Ok(elem)
	}
	// Tries removeing and returning an element from the end of the vector if there are enough left
	// Returns Some(element) if it succeeded, None otherwise
	pub fn try_pop(&mut self) -> Option<T> {
		self.pop().ok()
	}

	// Clears the extra elements from the array
	// Effectively resets the length to min_size
	// Returns the number of elements removed
	pub fn clear(&mut self) -> usize {
		let cleared = self.len - self.min_size;
		self.len = self.min_size;
		cleared
	}
}

impl<T> Drop for RangeVec<T> {
	fn drop(&mut self) {
		// if self.pointer is null, the memory has been transferred and should not be deallocated
		if self.pointer.is_null() {
			return;
		}
		unsafe {
			dealloc(
				self.pointer as *mut u8,
				Layout::from_size_align_unchecked(self.max_size * size_of::<T>(), align_of::<T>()),
			)
		};
	}
}

impl<T> std::ops::Index<usize> for RangeVec<T> {
	type Output = T;
	fn index(&self, index: usize) -> &Self::Output {
		self.get(index).unwrap()
	}
}
impl<T> std::ops::IndexMut<usize> for RangeVec<T> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		self.get_mut(index).unwrap()
	}
}

impl<T> Into<Vec<T>> for RangeVec<T> {
	fn into(mut self) -> Vec<T> {
		unsafe {
			let vec = Vec::from_raw_parts(self.pointer, self.len, self.max_size);
			// clear the pointer so memory doesn't get deallocated
			self.pointer = ptr::null_mut();
			vec
		}
	}
}

impl<T> PartialEq for RangeVec<T> where T: PartialEq {
	fn eq(&self, other: &Self) -> bool {
		self.max_size == other.max_size &&
		self.min_size == other.min_size &&
		self.len == other.len &&
		self.iter().zip(other.iter()).all(|(a, b)| a == b)
	}
	fn ne(&self, other: &Self) -> bool {
		self.max_size != other.max_size ||
		self.min_size != other.min_size ||
		self.len != other.len ||
		self.iter().zip(other.iter()).any(|(a, b)| a != b)
	}
}
impl<T> Eq for RangeVec<T> where T: Eq {}