
use std::{
	convert::{
		From, Into
	},
	cmp::Ordering,
	collections::HashSet,
	hash::Hash,
	ops::{
		Add,AddAssign
	}
};
use crate::core::{
	CompleteLattice, PowerSet, PowerSetItem, Bottom
};

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

impl<'a,E> Add<&'a Self> for HashPowerSet<E>
	where
		E: HashPowerSetItem
{
	type Output = Self;
	
	fn add(mut self, other: &'a Self) -> Self::Output
	{
		join(&mut self, other);
		self
	}
}

impl<E> Add<Self> for HashPowerSet<E>
	where
		E: HashPowerSetItem
{
	type Output = Self;
	
	fn add(mut self, other: Self) -> Self::Output
	{
		join(&mut self, &other);
		self
	}
}

impl<E> AddAssign for HashPowerSet<E>
	where
		E: HashPowerSetItem
{
	fn add_assign(&mut self, rhs: Self)
	{
		join(self, &rhs);
	}
}

impl<'a,E> AddAssign<&'a Self> for HashPowerSet<E>
	where
		E: HashPowerSetItem
{
	fn add_assign(&mut self, rhs: &'a Self)
	{
		join(self, rhs);
	}
}

impl<E> CompleteLattice for HashPowerSet<E>
	where
		E: HashPowerSetItem
{
	fn is_bottom(&self) -> bool
	{
		self.set.is_empty()
	}
}

impl<E> Bottom for HashPowerSet<E>
	where
		E: HashPowerSetItem
{
	fn bottom() -> Self
	{
		Self{set: HashSet::new()}
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

// Helper functions

fn join<E>(left: &mut  HashPowerSet<E>, right:&HashPowerSet<E>)
	where
		E: HashPowerSetItem
{
	for e in right.set.iter(){
		if !left.set.contains(e) {
			left.set.insert(e.clone());
		}
	}
}



