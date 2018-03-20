use super::*;

use std::ops::{AddAssign, Index};

trait_alias!(TFSpaceKey: Copy, Eq);
trait_alias!(TFSpaceElement: CompleteLattice);

///
/// A Total Function Space that maps keys to [`CompleteLattice`] elements.
///
/// A Total Function Space is a complete lattice with the following properties:
///
/// * Given a [`TFSpace`], f, `f.is_bottom()` iff `f[s].is_bottom()` for all keys `s`.
/// * Given two [`TFSpace`]s, f1 and f2, `f1 <= f2` iff `f1[s] <= f2[s]` for all keys `s`.
/// * Given two [`TFSpace`]s, f1 and f2, and `f3 = f1 + f2`
/// then `f1[s] <= f3[s]` and `f2[s] <= f3[s]` for all keys `s`.
///
pub trait TFSpace<'a,K,E>: CompleteLattice + Index<K, Output=Element<E>>
	where
		K: TFSpaceKey,
		E: 'a + TFSpaceElement
{
	type Keys: Iterator<Item=K>;
	
	fn add_key_with(&mut self, k: K, e: Element<E>);
	
	fn keys(&self) -> Self::Keys;
	
	fn add_key(&mut self, k:K){
		self.add_key_with(k, Element::bottom())
	}
	
	fn has_key(&self, k:K) -> bool
	{
		self.keys().any(|key| key == k)
	}
}

impl<'a,K,E,T> TFSpace<'a,K,E> for Element<T>
	where
		K: TFSpaceKey,
		E: 'a + TFSpaceElement,
		T: TFSpace<'a,K,E>
{
	type Keys = T::Keys;
	
	fn add_key_with(&mut self, k: K, e: Element<E>)
	
	{
		self.inner.add_key_with(k,e)
	}
	
	fn keys(&self) -> Self::Keys
	{
		self.inner.keys()
	}
	
	fn add_key(&mut self, k:K){
		self.inner.add_key(k)
	}
	
	fn has_key(&self, k:K) -> bool
	{
		self.inner.has_key(k)
	}
	
}

impl<'a,K,E,T> Index<K> for Element<T>
	where
		K: TFSpaceKey,
		E: 'a + TFSpaceElement,
		T: TFSpace<'a,K,E> + Index<K, Output=Element<E>>
{
	type Output = T::Output;
	
	fn index(&self, index: K) -> &Self::Output
	{
		&self.inner[index]
	}
}
