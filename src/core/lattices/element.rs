
use core::{
	CompleteLattice
};
use std::ops::{Add, AddAssign};
use std::cmp::Ordering;

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

impl<'a,T> AddAssign<&'a Element<T>> for Element<T>
	where
		T: CompleteLattice
{
	fn add_assign(&mut self, other: &'a Element<T>)
	{
		self.inner.join(&other.inner)
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

	

