
//!
//!
//!```
//!	#[macro_use]
//! extern crate progyis;
//!
//!
//!
//! trait_alias!(ExAlias1: Copy, Eq, Clone);
//! trait_alias!(ExAlias2: CompleteLattice);
//!	trait_alias!(ExAlias3<K>: Clone, Iterator<Item=K> where K: ExAlias2);
//! trait_alias!(ExAlias4<Q,W,E>: Copy, Clone, Eq, Iterator<Item=Q>
//!		where
//!			Q: Copy, Eq;
//!			W: Clone;
//!			E: Eq;
//!	);
//!
//! trait_alias!(ExAlias5<'a>: Copy, Eq, Clone);
//! trait_alias!(ExAlias6<'a,Q>: Copy, Eq, Clone where Q: Copy, Eq);
//!
//! trait_alias!(ExAlias7<'a>: ExAlias6<'a>, Hash);
//! ```
//!
//!
//!
//!

#[macro_export]
macro_rules! trait_alias{
	// No lifetimes, no generics
	{
		$alias_name:ident : $($trait_bound:path),* $(,)*
	}=>{
		pub trait $alias_name: $($trait_bound+)* {}
		impl<T> $alias_name for T where T: $($trait_bound+)* {}
	};
	// 1 Lifetime, no generics
	{
		$alias_name:ident<'a>: $($trait_bound:path),* $(,)*
	}=>{
		pub trait $alias_name<'a>: $($trait_bound+)* {}
		impl<'a, T> $alias_name<'a> for T where T: $($trait_bound+)* {}
	};
	// 1 Lifetime, 1 lifetime on bound, no generics
	{
		$alias_name:ident<'a>: $fst_trait_bound:ident<'a> $(, $trait_bound:path)* $(,)*
	}=>{
		pub trait $alias_name<'a>: $fst_trait_bound<'a> $(+ $trait_bound)* {}
		impl<'a, T> $alias_name<'a> for T where T: $fst_trait_bound<'a> $(+ $trait_bound)* {}
	};
	// No lifetime, generics
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
	};
	// 1 lifetime, generics
	{
		$alias_name:ident <'a, $($generic_dec:ident),* > : $($trait_bound:path),* $(,)*
			where $($generic_bound:ident : $($generic_trait_bound:path),*);* $(;)*
	}=>{
		pub trait $alias_name<'a,$($generic_dec,)*>: $($trait_bound+)* {}
		impl<'a,T,$($generic_dec,)*> $alias_name<'a,$($generic_dec,)*> for T
			where
				T: $($trait_bound+)* ,
				$($generic_bound: $($generic_trait_bound+)*,)*
		{}
	};
}

