use std::fmt::Display;

pub type RangeVecResult<T> = Result<T, RangeVecErr>;

#[derive(Debug, PartialEq, Eq)]
pub enum RangeVecErr {
	TooShort,
	TooLong,
	CantAdd,
	CantRemove,
}
impl Display for RangeVecErr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use RangeVecErr::*;
		match &self {
			TooShort => write!(f, "Initialization vector is too short"),
			TooLong => write!(f, "Initialization vector is too long"),
			CantAdd => write!(f, "Cannot add element, the vector is already maximum size"),
			CantRemove => write!(f, "Cannot remove element, the vector is already minimum size"),
		}
	}
}
