use super::*;

use std::cmp::Ordering;
use std::iter::FromIterator;

use ::core::{CompleteLattice,Evaluable};

pub trait PowerSetInner: Clone
{
	type Element: PowerSetElement;
	type All: IntoIterator<Item=Self::Element>;
	
	fn empty() -> Self;
	
	fn singleton(s: Self::Element) -> Self;
	
	fn join(&self, other: &Self)-> Self;
	
	fn all(&self) -> Self::All;
}

#[derive(Clone, Debug)]
pub struct PowerSetWrapper<T>
	where
		T: PowerSetInner
{
	pub inner: T
}

impl<T> PowerSet for PowerSetWrapper<T>
	where
		T: PowerSetInner
{
	type All = T::All;
	
	fn singleton(s: Self::Element) -> Self{
		Self{inner: T::singleton(s)}
	}
	
	fn all(&self) -> Self::All{
		self.inner.all()
	}
}

impl<T> CompleteLattice for PowerSetWrapper<T>
	where
		T: PowerSetInner
{
	type Element = T::Element;
	
	fn bottom() -> Self{
		Self{inner: T::empty()}
	}
	
	fn is_bottom(&self) -> bool{
		self.inner.all().into_iter().count() == 0
	}
	
	fn join(&self, other: &Evaluable<Value=Self>) -> Self{
		Self{inner: self.inner.join(&other.evaluate().inner)}
	}
}


impl<T> PartialEq for PowerSetWrapper<T>
	where
		T: PowerSetInner
{
	fn eq(&self, other:&Self) -> bool
	{
		iter_subset::<T>(&self.inner, &other.inner)
		&&
		iter_subset(&other.inner, &self.inner)
	}
}

impl<T> PartialOrd for PowerSetWrapper<T>
	where
		T: PowerSetInner
{
	fn partial_cmp(&self, other:&Self) -> Option<Ordering>
	{
		let self_subset = iter_subset(	&self.inner,
										  &other.inner);
		let other_subset = iter_subset(&other.inner,
									   		&self.inner);
		if self_subset {
			if other_subset {
				Some(Ordering::Equal)
			} else {
				Some(Ordering::Less)
			}
		} else if other_subset {
			Some(Ordering::Greater)
		} else {
			None
		}
	}
}

impl<T> FromIterator<T::Element> for PowerSetWrapper<T>
	where
		T: PowerSetInner
{
	fn from_iter<I>(iter: I) -> Self
		where I: IntoIterator<Item=T::Element>
	{
		iter.into_iter().fold(
			PowerSetWrapper::bottom(),
			|result, next|
				result.join(&PowerSetWrapper::singleton(next))
		)
	}
}

impl<F,T> From<F> for PowerSetWrapper<T>
	where
		F: IntoIterator<Item=T::Element>,
		T: PowerSetInner
{
	fn from(i: F) -> Self{
		PowerSetWrapper::from_iter(i)
	}
}


pub fn iter_subset<T>(subset: &T, superset: &T) -> bool
	where
		T: PowerSetInner
{
	for e in subset.all().into_iter() {
		if !(superset.all().into_iter().any(
				|o|{
					o == e
				})
			)
		{
			return false;
		}
	}
	true
}
