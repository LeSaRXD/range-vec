use std::fmt::{self, Display};

use super::RangeVec;

impl<T: Display> Display for RangeVec<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[\n")?;
		for elem in self.iter() {
			write!(f, "    {},\n", elem)?;
		}
		write!(f, "]")
	}
}

impl<T: fmt::Debug> fmt::Debug for RangeVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RangeVec")
			.field("pointer", &self.pointer)
			.field("min_size", &self.min_size)
			.field("max_size", &self.max_size)
			.field("len", &self.len)
			.field("values", &self.iter().collect::<Vec<&T>>())
			.finish()
    }
}