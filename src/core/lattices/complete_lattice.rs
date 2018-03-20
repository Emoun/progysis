use super::*;

use std::ops::{Add, AddAssign};
use std::cmp::Ordering;

///
/// A [Complete Lattice].
///
/// Types implementing this trait represent the [Complete Lattice], while instances
/// of the type are element of the lattice.
///
/// The trait specifies a set of methods related to all [Complete Lattice]s:
///
/// * [`Evaluable`]: Every element in the lattice can be evaluated to itself.
/// * [`PartialOrd`]: meaning `<=` can be used to compare two elements of a [Complete Lattice].
/// * [`Add`]: meaning `+` can be use to get the smallest element which is larger
/// than both the given (Least Fixed Point). I.e. if `e3 = e1 + e2` then `e3`
/// is the smallest element in the lattice where `e1 <= e3 && e2 <= e3`.
/// At least `Add<T> where T: Evaluable<Value=Self>` should be implemented.
///
/// [Complete Lattice]: http://mathworld.wolfram.com/CompleteLattice.html
/// [`Evaluable`]: trait.Evaluable.html
/// [`PartialOrd`]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
/// [`Add`]: https://doc.rust-lang.org/std/ops/trait.Add.html
///
pub trait CompleteLattice: PartialOrd + Clone
{
	///
	/// Returns the bottom (Greatest Lower Bound) element of the
	/// [Complete Lattice](http://mathworld.wolfram.com/CompleteLattice.html).
	///
	fn bottom() -> Self;
	
	///
	/// Whether this instance is the bottom element of the
	/// [Complete Lattice](http://mathworld.wolfram.com/CompleteLattice.html).
	///
	fn is_bottom(&self) -> bool;
	
	fn join(&mut self, other:&Self);
	
	///
	/// Used by &Element::Add
	///
	fn join_new(&self, other: &Self) -> Self
	{
		let mut result = Self::bottom();
		result.join(self);
		result.join(other);
		result
	}
	
	///
	/// Used by Element::AddAssign
	///
	fn add_assign(&mut self, other:Self)
	{
		self.join(&other);
	}
	
	///
	/// Used by Element::Add
	///
	fn add(mut self, other:Self) -> Self
	{
		self.join(&other);
		self
	}
	
	///
	/// Whether this instance is comparable to the given.
	///
	/// I.e either `self <= other` or `self > other` holds.
	///
	fn comparable_to(&self, other: &Self) -> bool
	{
		self.le(&other) || self.gt(&other)
	}
}

#[derive(Debug, Clone)]
pub struct Element<T>
	where
		T: CompleteLattice
{
	pub inner: T
}

impl<T> Element<T>
	where
		T: CompleteLattice
{
	pub fn new(inner: T) -> Element<T>
	{
		Element {inner}
	}
}

impl<T> PartialEq for Element<T>
	where
		T: CompleteLattice
{
	fn eq(&self, other:&Self) -> bool
	{
		self.inner == other.inner
	}
}

impl<T> PartialOrd for Element<T>
	where
		T: CompleteLattice
{
	fn partial_cmp(&self, other:&Self) -> Option<Ordering>
	{
		self.inner.partial_cmp(&other.inner)
	}
}

impl<T> Add<Self> for Element<T>
	where
		T: CompleteLattice,
{
	type Output = Self;
	
	fn add(self, rhs: Self) -> Self::Output
	{
		Self{inner: self.inner.add(rhs.inner)}
	}
}

impl<'a,T> Add<&'a Element<T>> for Element<T>
	where
		T: CompleteLattice,
{
	type Output = Self;
	
	fn add(mut self, rhs: &'a Element<T>) -> Self::Output
	{
		self.inner.join(&rhs.inner);
		self
	}
}

impl<'a,T> Add<Element<T>> for &'a Element<T>
	where
		T: CompleteLattice,
{
	type Output = Element<T>;
	
	fn add(self, mut rhs: Element<T>) -> Self::Output
	{
		rhs.inner.join(&self.inner);
		rhs
	}
}

impl<'a,'b,T> Add<&'b Element<T>> for &'a Element<T>
	where
		T: CompleteLattice
{
	type Output = Element<T>;
	
	fn add(self, rhs:&'b Element<T>) -> Self::Output
	{
		let mut result = T::bottom();
		
		result.join(&self.inner);
		result.join(&rhs.inner);
		
		Element::new(result)
	}
}

impl<T> AddAssign for Element<T>
	where
		T: CompleteLattice
{
	fn add_assign(&mut self, other: Element<T>)
	{
		self.inner.add_assign(other.inner)
	}
}

impl<T> CompleteLattice for Element<T>
	where
		T: CompleteLattice
{
	fn bottom() -> Self
	{
		Self::new(T::bottom())
	}
	
	fn is_bottom(&self) -> bool
	{
		self.inner.is_bottom()
	}
	
	fn join(&mut self, other: &Self)
	{
		self.inner.join(&other.inner)
	}
}

