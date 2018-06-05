
use ::core::{
	CompleteLattice, Bottom
};
use std::{
	ops::{
		Add, AddAssign
	},
	cmp::{
		max, Ordering
	}
};

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct U32(pub u32);

impl CompleteLattice for U32
{
	fn is_bottom(&self) -> bool
	{
		self.0 == 0
	}
}

impl Bottom for U32
{
	fn bottom() -> Self
	{
		U32(0)
	}
}

impl Add for U32
{
	type Output = U32;
	fn add(self, rhs: Self) -> Self::Output
	{
		max(self, rhs)
	}
}

impl<'a> Add<&'a Self> for U32
{
	type Output = U32;
	fn add(self, rhs: &'a Self) -> Self::Output
	{
		max(self, *rhs)
	}
}

impl AddAssign for U32
{
	fn add_assign<'a>(&mut self, rhs: Self)
	{
		*self = max(*self, rhs);
	}
}

impl<'a> AddAssign<&'a Self> for U32
{
	fn add_assign(&mut self, rhs:&'a Self)
	{
		*self += rhs;
	}
}