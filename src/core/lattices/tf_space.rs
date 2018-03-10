use super::*;

use std::ops::{Index};

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
pub trait TFSpace<'a,K,E>: CompleteLattice + Index<K, Output=E>
	where
		K: Copy + Eq,
		E: 'a + CompleteLattice
{
	type Keys: Iterator<Item=K>;
	
	fn add_key_with(&mut self, k: K, e: E);
	
	fn keys(&self) -> Self::Keys;
	
	fn add_key(&mut self, k:K){
		self.add_key_with(k, E::bottom())
	}
	
	fn has_key(&self, k:K) -> bool
	{
		self.keys().any(|key| key == k)
	}
}


