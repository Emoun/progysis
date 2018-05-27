
use std::{
	convert::{
		From, Into
	},
	cmp::Ordering,
	collections::HashSet,
	hash::Hash,
};
use ::core::{CompleteLattice, Element, PowerSet, Evaluable, PowerSetItem};

trait_alias!(HashPowerSetItem: PowerSetItem, Hash);

#[derive(Debug, Clone)]
pub struct HashPowerSet<E>
	where
		E: HashPowerSetItem
{
	set: HashSet<E>
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
		self.set.is_empty()
	}
	
	fn join(&mut self, other: &Self)
	{
		for e in other.set.iter(){
			if !self.set.contains(e) {
				self.set.insert(e.clone());
			}
		}
	}
}

impl<E> Evaluable for HashPowerSet<E>
	where
		E: HashPowerSetItem
{
	type Value = Self;
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

impl<E> From<HashSet<E>> for HashPowerSet<E>
	where E: HashPowerSetItem
{
	fn from(set: HashSet<E>) -> Self
	{
		Self{set}
	}
}

impl<E> Into<HashSet<E>> for HashPowerSet<E>
	where E: HashPowerSetItem
{
	fn into(self) -> HashSet<E>
	{
		self.set
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





