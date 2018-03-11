use super::*;

use std::ops::{Add, AddAssign, Index};
use std::cmp::Ordering;
use std::marker::PhantomData;

trait_alias!(TFSpaceInnerKey: Copy, Eq, Clone);
trait_alias!(TFSpaceInnerElement: CompleteLattice);


pub trait TFSpaceInner<'a,K,E>: AddAssign + Index<K, Output=E> + Clone
	where
		K: 'a + TFSpaceInnerKey,
		E: 'a + TFSpaceInnerElement,
{
	type Keys: Iterator<Item=K>;
	
	fn empty() -> Self;
	
	fn add_key_with(&mut self, k: K, e: E);
	
	fn keys(&self) -> Self::Keys;
}

#[derive(Clone, Debug)]
pub struct TFSpaceWrapper<'a,K,E,T>
	where
		T: TFSpaceInner<'a,K,E>,
		K: 'a + TFSpaceInnerKey,
		E: 'a + TFSpaceInnerElement,
{
	inner: T,
	k: PhantomData<&'a K>,
	e: PhantomData<E>,
}

impl<'a,K,E,T,R> Add<R> for TFSpaceWrapper<'a,K,E,T>
	where
		T: TFSpaceInner<'a,K,E>,
		K: 'a + TFSpaceInnerKey,
		E: 'a + TFSpaceInnerElement,
		R: Evaluable<Value=Self>
{
	type Output = Self;
	
	fn add(mut self, other: R) -> Self::Output
	{
		self.inner += other.evaluate().inner;
		self
	}
}

impl<'a,K,E,T> AddAssign for TFSpaceWrapper<'a,K,E,T>
	where
		T: TFSpaceInner<'a,K,E>,
		K: 'a + TFSpaceInnerKey,
		E: 'a + TFSpaceInnerElement,
{
	fn add_assign(&mut self, other: TFSpaceWrapper<'a,K,E,T>)
	{
		self.inner += other.inner;
	}
}

impl<'a,K,E,T>  PartialEq for TFSpaceWrapper<'a,K,E,T>
	where
		T: TFSpaceInner<'a,K,E>,
		K: 'a + TFSpaceInnerKey,
		E: 'a + TFSpaceInnerElement,
{
	fn eq(&self, other:&Self) -> bool
	{
		for k in self.keys(){
			if self[k] != other[k] {
				return false;
			}
		}
		true
	}
}

///
/// Ensures that both arguments have the same keys, and that `f` holds for all
/// all value pairs (one from each argument) for all the keys.
///
fn for_each_pair<'a,K,E,T,F>(l: &TFSpaceWrapper<'a,K,E,T>, r: &TFSpaceWrapper<'a,K,E,T>, f: F) -> bool
	where
		T: TFSpaceInner<'a,K,E>,
		K: 'a + TFSpaceInnerKey,
		E: 'a + TFSpaceInnerElement,
		F: Fn(&E,&E) -> bool
{
	// Check that all the elements in left accept f() for their right counterparts
	for s_key in l.inner.keys() {
		if !f(&l.inner[s_key], &r.inner[s_key]) {
			return false;
		}
	}
	// Check that all the elements in right accept f() for their left counterparts
	for o_key in r.inner.keys() {
		if !f(&l.inner[o_key], &r.inner[o_key]) {
			return false;
		}
	}
	// No inconsistencies were found
	true
}

impl<'a,K,E,T> PartialOrd for TFSpaceWrapper<'a,K,E,T>
	where
		T: TFSpaceInner<'a,K,E>,
		K: 'a + TFSpaceInnerKey,
		E: 'a + TFSpaceInnerElement,
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
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
			|s_e, o_e| s_e < o_e)
	}
	fn le(&self, other: &Self) -> bool
	{
		for_each_pair(self, other,
					  |s_e, o_e| s_e <= o_e)
	}
	fn gt(&self, other: &Self) -> bool
	{
		for_each_pair(self, other,
					  |s_e, o_e| s_e > o_e)
	}
	fn ge(&self, other: &Self) -> bool
	{
		for_each_pair(self, other,
					  |s_e, o_e| s_e >= o_e)
	}
}

impl<'a,K,E,T>  CompleteLattice for TFSpaceWrapper<'a,K,E,T>
	where
		T: TFSpaceInner<'a,K,E>,
		K: 'a + TFSpaceInnerKey,
		E: 'a + TFSpaceInnerElement,
{
	fn bottom() -> Self
	{
		Self{inner: T::empty(), k: PhantomData, e: PhantomData}
	}
	
	fn is_bottom(&self) -> bool
	{
		for key in self.inner.keys() {
			if !self.inner[key].is_bottom() {
				return false;
			}
		}
		true
	}
}

impl<'a,K,E,T> Index<K> for TFSpaceWrapper<'a,K,E,T>
	where
		T: TFSpaceInner<'a,K,E>,
		K: 'a + TFSpaceInnerKey,
		E: 'a + TFSpaceInnerElement,
{
	type Output = E;
	
	fn index(&self, index: K) -> &Self::Output
	{
		&self.inner[index]
	}
}

impl<'a,K,E,T> TFSpace<'a,K,E> for TFSpaceWrapper<'a,K,E,T>
	where
		T: TFSpaceInner<'a,K,E>,
		K: 'a + TFSpaceInnerKey,
		E: 'a + TFSpaceInnerElement,
{
	type Keys = T::Keys;
	
	fn add_key_with(&mut self, k: K, e: E){
		self.inner.add_key_with(k,e);
	}
	
	fn keys(&self) -> Self::Keys{
		self.inner.keys()
	}
}
