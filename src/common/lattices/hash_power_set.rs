use super::*;

use std::ops::Add;
use std::collections::HashSet;
use std::hash::Hash;
use ::core::{PowerSetItem, PowerSetWrapper, PowerSetInner, Evaluable};

trait_alias!{HashPowerSetItem: PowerSetItem, Hash}

pub type HashPowerSet<V> = PowerSetWrapper<HashPowerSetInner<V>>;

#[derive(Debug, Clone)]
pub struct HashPowerSetInner<E>
	where
		E: HashPowerSetItem
{
	pub set: HashSet<E>
}

impl<E> PowerSetInner for HashPowerSetInner<E>
	where
		E: HashPowerSetItem
{
	type Item = E;
	type All = HashSet<E>;
	
	fn empty() -> Self{
		Self{set: HashSet::new()}
	}
	
	fn singleton(s: Self::Item) -> Self{
		let mut set = HashSet::new();
		set.insert(s);
		Self{set}
	}
	
	fn all(&self) -> Self::All{
		self.set.clone()
	}
}

impl<E> Add for HashPowerSetInner<E>
	where
		E: HashPowerSetItem
{
	type Output = Self;
	
	fn add(self, rhs: Self) -> Self::Output
	{
		Self{set: self.set.union(&rhs.set).cloned().collect()}
	}
}