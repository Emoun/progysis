
use std::collections::{HashMap,HashSet};
use std::hash::Hash;
use std::ops::{Index, IndexMut};
use std::cmp::Ordering;
use std::collections::hash_set::IntoIter;
use std::marker::PhantomData;

use ::core::{CompleteLattice, TFSpace, TFSpaceKey, TFSpaceElement, Element};

trait_alias!(HashTFSpaceKey: TFSpaceKey, Hash);
trait_alias!(HashTFSpaceElement: TFSpaceElement);

#[derive(Clone, Debug)]
pub struct HashTFSpace<'a,K,E>
	where
		K: HashTFSpaceKey,
		E: HashTFSpaceElement
{
	map: HashMap<K,Element<E>>,
	a: PhantomData<&'a i8>
}

impl<'a,K,E> TFSpace<'a,K,E> for HashTFSpace<'a,K,E>
	where
		K: HashTFSpaceKey,
		E: 'a + HashTFSpaceElement
{
	type Keys = IntoIter<K>;
	
	fn keys(&self) -> Self::Keys{
		self.map.keys().cloned().collect::<HashSet<K>>().into_iter()
	}
}

impl<'a,K,E> CompleteLattice for HashTFSpace<'a,K,E>
	where
		K: HashTFSpaceKey,
		E: HashTFSpaceElement
{
	fn bottom() -> Self
	{
		Self{map: HashMap::new(), a:PhantomData}
	}
	
	fn is_bottom(&self) -> bool
	{
		self.map.values().all(|e| e.is_bottom())
	}
	
	fn join(&mut self, other:&Self)
	{
		let self_keys = self.keys().collect::<Vec<_>>();
		//Join all the common keys' values
		let common_keys = self_keys.iter().filter(|key| other.map.contains_key(key));
		for &k in common_keys {
			let new_val =  self[k].clone() + other[k].clone();
			self.map.insert(k, new_val);
		}
		
		//Add all the values of the keys in other which are not in self
		let other_keys = other.keys().filter(|key| !self_keys.contains(key));
		for k in other_keys{
			self.map.insert(k, other[k].clone());
		}
	}
}

impl<'a,K,E> PartialOrd for HashTFSpace<'a,K,E>
	where
		K: HashTFSpaceKey,
		E: HashTFSpaceElement
{
	fn partial_cmp(&self, other:&Self) -> Option<Ordering>
	{
		if self.lt(other) {
			Some(Ordering::Less)
		} else if self.gt(other){
			Some(Ordering::Greater)
		} else if self == other {
			Some(Ordering::Equal)
		}else{
			None
		}
	}
	
	fn lt(&self, other: &Self) -> bool
	{
		for_each_pair(self, other,
					  |s_e, o_e| s_e < o_e,
					  |s| s.is_bottom(),
					  |_| true)
	}
	fn le(&self, other: &Self) -> bool
	{
		for_each_pair(self, other,
					  |s_e, o_e| s_e <= o_e,
					  |s| s.is_bottom(),
					  |_| true)
	}
	fn gt(&self, other: &Self) -> bool
	{
		for_each_pair(self, other,
					  |s_e, o_e| s_e > o_e,
					  |_| true,
					  |o| o.is_bottom())
	}
	fn ge(&self, other: &Self) -> bool
	{
		for_each_pair(self, other,
					  |s_e, o_e| s_e >= o_e,
					  |_| true,
					  |o| o.is_bottom())
	}
}

impl<'a,K,E> PartialEq for HashTFSpace<'a,K,E>
	where
		K: HashTFSpaceKey,
		E: HashTFSpaceElement
{
	fn eq(&self, other:&Self) -> bool
	{
		for k in self.keys(){
			if let Some(o) = other.map.get(&k){
				if self[k] != *o {
					return false;
				}
			}else{
				if !self[k].is_bottom() {
					return false;
				}
			}
		}
		//Check reflexive
		for k in other.keys(){
			if let Some(s) = self.map.get(&k){
				if other[k] != *s {
					return false;
				}
			}else{
				if !other[k].is_bottom() {
					return false;
				}
			}
		}
		true
	}
}

impl<'a,K,E> Index<K> for HashTFSpace<'a,K,E>
	where
		K: HashTFSpaceKey,
		E: HashTFSpaceElement
{
	type Output = Element<E>;
	
	fn index(&self, index: K) -> &Self::Output
	{
		&self.map[&index]
	}
}

impl<'a,K,E> IndexMut<K> for HashTFSpace<'a,K,E>
	where
		K: HashTFSpaceKey,
		E: HashTFSpaceElement
{
	fn index_mut(&mut self, index: K) -> &mut Self::Output
	{
		if !self.map.contains_key(&index){
			self.map.insert(index, Element::bottom());
		}
		
		self.map.get_mut(&index).unwrap()
	}
}

// Helper functions

///
/// Ensures that both arguments have the same keys, and that `f` holds for all
/// all value pairs (one from each argument) for all the keys.
/// If 'l' has key that isn't in 'r', 'd1' must hold for 'l's value
/// If 'r' has key that isn't in 'l', 'd2' must hold for 'r's value
///
fn for_each_pair<'a,K,E,F,D1,D2>(l: &HashTFSpace<'a,K,E>, r: &HashTFSpace<'a,K,E>, f: F, d1: D1, d2: D2) -> bool
	where
		K: 'a + HashTFSpaceKey,
		E: 'a + HashTFSpaceElement,
		F: Fn(&E,&E) -> bool,
		D1: Fn(&E) -> bool,
		D2: Fn(&E) -> bool
{
	// Check that all the elements in left accept f() for their right counterparts
	for s_key in l.keys() {
		if let Some(o) = r.map.get(&s_key){
			if !f(&l[s_key].inner, &o.inner) {
				return false;
			}
		}else{
			if !d1(&l[s_key].inner) {
				return false;
			}
		}
	}
	// Check that all the elements in right accept f() for their left counterparts
	for o_key in r.keys() {
		if let Some(s) = l.map.get(&o_key){
			if !f(&s.inner, &r[o_key].inner) {
				return false;
			}
		}else {
			if !d2(&r[o_key].inner) {
				return false;
			}
		}
	}
	// No inconsistencies were found
	true
}