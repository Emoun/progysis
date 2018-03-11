use super::*;

use std::ops::{Add, AddAssign};
use std::fmt::{Debug, Formatter, Error};
use std::collections::HashSet;
use std::hash::Hash;
use ::core::{PowerSetItem, PowerSetWrapper, PowerSetInner, Evaluable};

trait_alias!{HashPowerSetItem: PowerSetItem, Hash}

pub type HashPowerSet<V> = PowerSetWrapper<HashPowerSetInner<V>>;

#[derive(Clone)]
pub struct HashPowerSetInner<E>
	where
		E: HashPowerSetItem
{
	pub set: HashSet<E>
}

impl<E> Debug for HashPowerSetInner<E>
	where
		E: HashPowerSetItem + Debug
{
	fn fmt(&self, f:&mut Formatter) -> Result<(),Error>{
		match f.write_str("HashPowerSet") {
			Ok(_) => self.set.fmt(f),
			Err(r) => Err(r)
		}
		
	}
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

impl<E> AddAssign for HashPowerSetInner<E>
	where
		E: HashPowerSetItem
{
	fn add_assign(&mut self, other: HashPowerSetInner<E>)
	{
		for e in other.set.iter(){
			if !self.set.contains(e) {
				self.set.insert(e.clone());
			}
		}
	}
}