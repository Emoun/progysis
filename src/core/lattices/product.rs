use core::{
	CompleteLattice, Element
};
use std::cmp::Ordering;

///
/// A complete lattice, that is the product of two other lattices.
///
#[derive(Clone, PartialEq)]
pub struct Product<A,B>
	where
		A: CompleteLattice,
		B: CompleteLattice,
{
	pub a: Element<A>,
	pub b: Element<B>,
}

impl<A,B> PartialOrd for Product<A,B>
	where
		A: CompleteLattice,
		B: CompleteLattice,
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		let a = self.a.partial_cmp(&other.a);
		let b = self.b.partial_cmp(&other.b);
		
		if a == b {
			a
		}else{
			None
		}
	}
}

impl<A,B> CompleteLattice for Product<A,B>
	where
		A: CompleteLattice,
		B: CompleteLattice,
{
	fn bottom() -> Self
	{
		Self{a: Element::bottom(), b: Element::bottom()}
	}
	
	fn is_bottom(&self) -> bool
	{
		self.a.is_bottom() && self.b.is_bottom()
	}
	
	fn join(&mut self, other: &Self)
	{
		self.a.join(&other.a);
		self.b.join(&other.b);
	}
}



