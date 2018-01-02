use super::*;

use std::collections::HashSet;
use std::hash::Hash;
use ::core::{PowerSetItem, PowerSetWrapper, PowerSetInner};

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
	
	fn join(&self, other: &Self)-> Self{
		Self{set: self.set.union(&other.set).cloned().collect()}
	}
	
	fn all(&self) -> Self::All{
		self.set.clone()
	}
}