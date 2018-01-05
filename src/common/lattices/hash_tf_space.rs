use super::*;

use std::collections::{HashMap,HashSet};
use std::hash::Hash;
use std::ops::{Add, Index};
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

impl<'a,K,E> Add for HashTFSpaceInner<'a,K,E>
	where
		K: 'a + HashTFSpaceKey,
		E: 'a + HashTFSpaceElement,
{
	type Output = Self;
	
	fn add(mut self, other: Self) -> Self::Output
	{
		let mut result = HashMap::new();
		for key in self.keys() {
			result.insert(key, self[key].clone() + other[key].clone());
		}
		for key in other.keys(){
			&self[key];
		}
		Self{map: result, a:PhantomData}
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


