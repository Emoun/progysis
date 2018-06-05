
use ::core::{CompleteLattice, Bottom};
use std::{
	ops::{
		Add, AddAssign
	},
	cmp::{
		max
	}
};

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct U64(pub u64);

impl CompleteLattice for U64
{
	
	fn is_bottom(&self) -> bool
	{
		self.0 == 0
	}
}

impl Bottom for U64
{
	fn bottom() -> Self
	{
		U64(0)
	}
}

impl Add for U64
{
	type Output = U64;
	fn add(self, rhs: Self) -> Self::Output
	{
		max(self, rhs)
	}
}

impl<'a> Add<&'a Self> for U64
{
	type Output = U64;
	fn add(self, rhs: &'a Self) -> Self::Output
	{
		max(self, *rhs)
	}
}

impl AddAssign for U64
{
	fn add_assign<'a>(&mut self, rhs: Self)
	{
		*self = max(*self, rhs);
	}
}

impl<'a> AddAssign<&'a Self> for U64
{
	fn add_assign(&mut self, rhs:&'a Self)
	{
		*self = max(*self, *rhs);
	}
}