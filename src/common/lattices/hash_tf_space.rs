use super::*;

use std::collections::{HashMap,HashSet};
use std::hash::Hash;
use std::ops::{Add, Index, AddAssign};
use std::iter::Cloned;
use std::collections::hash_set::IntoIter;
use std::marker::PhantomData;
use std::fmt::Debug;

use ::core::{CompleteLattice,Evaluable, TFSpaceInner, TFSpaceInnerKey, TFSpaceInnerElement, TFSpaceWrapper};

trait_alias!(HashTFSpaceKey: TFSpaceInnerKey, Hash);
trait_alias!(HashTFSpaceElement: TFSpaceInnerElement);

pub type HashTFSpace<'a,K,E> = TFSpaceWrapper<'a,K,E,HashTFSpaceInner<'a,K,E>>;

#[derive(Clone, Debug)]
pub struct HashTFSpaceInner<'a,K,E>
	where
		K: HashTFSpaceKey,
		E: HashTFSpaceElement
{
	map: HashMap<K,E>,
	a: PhantomData<&'a i8>
}

impl<'a,K,E> AddAssign for HashTFSpaceInner<'a,K,E>
	where
		K: 'a + HashTFSpaceKey,
		E: 'a + HashTFSpaceElement,
{
	fn add_assign(&mut self, other: HashTFSpaceInner<'a,K,E>)
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

impl<'a,K,E> Index<K> for HashTFSpaceInner<'a,K,E>
	where
		K: 'a + HashTFSpaceKey,
		E: 'a + HashTFSpaceElement,
{
	type Output = E;
	
	fn index(&self, index: K) -> &Self::Output
	{
		if let Some(r) = self.map.get(&index) {
			r
		}else{
			panic!("Invalid key.");
		}
	}
}

impl<'a,K,E> TFSpaceInner<'a,K,E> for HashTFSpaceInner<'a,K,E>
	where
		K: 'a + HashTFSpaceKey,
		E: 'a + HashTFSpaceElement,
{
	type Keys = IntoIter<K>;
	
	fn empty() -> Self{
		Self{map: HashMap::new(), a:PhantomData}
	}
	
	fn add_key_with(&mut self, k: K, e: E){
		self.map.insert(k, e);
	}
	
	fn keys(&self) -> Self::Keys{
		self.map.keys().cloned().collect::<HashSet<K>>().into_iter()
	}
}


