
//!
//! Program analysis library.
//!
//!
//!
//!
//!
//!
//!

///
///
///```
/// 	trait_alias!(ex_alias_1: Copy, Eq, Clone);
/// 	trait_alias!(ex_alias_2: CompleteLattice);
///		trait_alias!(ex_alias_3<K>: Clone, Iterator<Item=K> where K: ex_alias_2);
/// 	trait_alias!(ex_alias_4<Q,W,E>: Copy, Clone, Eq, Iterator<Item=Q>
///			where
///				Q: Copy, Eq;
///				W: Clone;
///				E: Eq;
///		);
/// ```
///
///
///
///
#[macro_export]
macro_rules! trait_alias{
	{
		$alias_name:ident : $($trait_bound:path),* $(,)*
	}=>{
		pub trait $alias_name: $($trait_bound+)* {}
		impl<T> $alias_name for T where T: $($trait_bound+)* {}
	};
	{
		$alias_name:ident < $($generic_dec:ident),* > : $($trait_bound:path),* $(,)*
			where $($generic_bound:ident : $($generic_trait_bound:path),*);* $(;)*
	}=>{
		pub trait $alias_name<$($generic_dec,)*>: $($trait_bound+)* {}
		impl<T,$($generic_dec,)*> $alias_name<$($generic_dec,)*> for T
			where
				T: $($trait_bound+)* ,
				$($generic_bound: $($generic_trait_bound+)*,)*
		{}
	}

}


pub mod core;
pub mod common;