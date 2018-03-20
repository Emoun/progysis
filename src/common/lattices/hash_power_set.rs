use super::*;

use std::cmp::Ordering;
use std::ops::{Add, AddAssign};
use std::fmt::{Debug, Formatter, Error};
use std::collections::HashSet;
use std::hash::Hash;
use ::core::{CompleteLattice, Element, PowerSet, Evaluable, PowerSetItem};

trait_alias!(HashPowerSetItem: PowerSetItem, Hash);

#[derive(Clone)]
pub struct HashPowerSet<E>
	where
		E: HashPowerSetItem
{
	pub set: HashSet<E>
}

impl<E> PowerSet for HashPowerSet<E>
	where
		E: HashPowerSetItem
{
	type Item = E;
	type All = HashSet<E>;
	
	fn singleton(s: Self::Item) -> Self
	{
		let mut set = HashSet::new();
		set.insert(s);
		Self{set}
	}
	
	fn all(&self) -> Self::All{
		self.set.clone()
	}
}

impl<E> PowerSet for Element<HashPowerSet<E>>
	where
		E: HashPowerSetItem
{
	type Item = E;
	type All = HashSet<E>;
	
	fn singleton(s: Self::Item) -> Self
	{
		Element::new(HashPowerSet::singleton(s))
	}
	
	fn all(&self) -> Self::All{
		self.inner.all()
	}
}

impl<E> CompleteLattice for HashPowerSet<E>
	where
		E: HashPowerSetItem
{
	fn bottom() -> Self
	{
		Self{set: HashSet::new()}
	}
	
	fn is_bottom(&self) -> bool
	{
		unimplemented!()
	}
	
	fn join(&mut self, other: &Self)
	{
		unimplemented!()
	}
}

impl<E> Evaluable for HashPowerSet<E>
	where
		E: HashPowerSetItem
{
	type Value = HashPowerSet<E>;
	fn evaluate(&self) -> Element<Self::Value>
	{
		Element::new(self.clone())
	}
	
	fn consume(self) -> Element<Self::Value>
	{
		Element::new(self)
	}
}

impl<E> PartialOrd for HashPowerSet<E>
	where
		E: HashPowerSetItem
{
	fn partial_cmp(&self, other:&Self) -> Option<Ordering>
	{
		let self_subset = iter_subset(	&self.set,
										  &other.set);
		let other_subset = iter_subset(&other.set,
									   &self.set);
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

impl<E> PartialEq for HashPowerSet<E>
	where
		E: HashPowerSetItem
{
	fn eq(&self, other:&Self) -> bool
	{
		iter_subset::<E>(&self.set, &other.set)
			&&
			iter_subset(&other.set, &self.set)
	}
}



// Helper functions

pub fn iter_subset<E>(subset: &HashSet<E>, superset: &HashSet<E>) -> bool
	where
		E: HashPowerSetItem
{
	for e in subset.into_iter() {
		if !(superset.into_iter().any(
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





