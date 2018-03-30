use super::*;

use ::core::{Element, CompleteLattice};
use std::marker::PhantomData;

#[derive(Clone)]
pub struct MonotoneFunction<L>
	where L: CompleteLattice
{
	pub func: fn(&Element<L>) -> Element<L>
}

impl<L> Copy for MonotoneFunction<L>
	where L: CompleteLattice{}

impl<L> PartialEq for MonotoneFunction<L>
	where
		L: CompleteLattice,
{
	fn eq(&self, other: &Self) -> bool
	{
		self.func as *const usize == other.func as *const usize
	}
}

impl<L> Eq for MonotoneFunction<L>
	where
		L: CompleteLattice,
{}

impl<L> MonotoneFunction<L>
	where
		L: CompleteLattice,
{
	pub fn new(func: fn(&Element<L>)-> Element<L>) -> Self
	{
		MonotoneFunction{func}
	}
	pub fn func(&self, e: &Element<L>) -> Element<L>
	{
		(self.func)(e)
	}
}