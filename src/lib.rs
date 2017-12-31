
//!
//! Program analysis library.
//!
//!
//!
//!
//!
//!
//!


#[macro_export]
macro_rules! trait_alias{
	{
		$alias_name:ident : $first_trait_bound:path $(, $trait_bound:path)* $(,)*
	}=>{
		pub trait $alias_name: $first_trait_bound $(+ $trait_bound)* {}
		impl<T> $alias_name for T where T: $first_trait_bound $(+ $trait_bound)* {}
	}
}


pub mod core;
pub mod common;