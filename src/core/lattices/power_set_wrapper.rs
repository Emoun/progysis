use super::*;

use std::cmp::Ordering;
use std::ops::Add;
use std::iter::FromIterator;

use ::core::{CompleteLattice,Evaluable};

pub trait PowerSetInner: Add<Output=Self> + Clone
{
	type Item: PowerSetItem;
	type All: IntoIterator<Item=Self::Item>;
	
	fn empty() -> Self;
	
	fn singleton(s: Self::Item) -> Self;
	
	fn all(&self) -> Self::All;
}

#[derive(Clone, Debug)]
pub struct PowerSetWrapper<T>
	where
		T: PowerSetInner
{
	inner: T
}

impl<T> PowerSet for PowerSetWrapper<T>
	where
		T: PowerSetInner
{
	type Item= T::Item;
	type All = T::All;
	
	fn singleton(s: Self::Item) -> Self{
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
	fn bottom() -> Self{
		Self{inner: T::empty()}
	}
	
	fn is_bottom(&self) -> bool{
		self.inner.all().into_iter().count() == 0
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

impl<T> FromIterator<T::Item> for PowerSetWrapper<T>
	where
		T: PowerSetInner
{
	fn from_iter<I>(iter: I) -> Self
		where I: IntoIterator<Item=T::Item>
	{
		iter.into_iter().fold(
			PowerSetWrapper::bottom(),
			|result, next|
				result + PowerSetWrapper::singleton(next)
		)
	}
}

impl<F,T> From<F> for PowerSetWrapper<T>
	where
		F: IntoIterator<Item=T::Item>,
		T: PowerSetInner
{
	fn from(i: F) -> Self{
		PowerSetWrapper::from_iter(i)
	}
}

impl<T,V> Add<V> for PowerSetWrapper<T>
	where
		T: PowerSetInner,
		V: Evaluable<Value=Self>
{
	type Output = Self;
	
	fn add(self, rhs: V) -> Self::Output
	{
		let other = rhs.evaluate();
		Self{inner: self.inner + other.inner}
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
