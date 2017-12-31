use super::*;

use std::fmt::Debug;

use ::core::{CompleteLattice};

trait_alias!{PowerSetElement: Clone, Eq, Debug}

pub trait PowerSet: CompleteLattice
	where
		<Self as CompleteLattice>::Element : PowerSetElement
{
	type All: IntoIterator<Item=Self::Element>;
	
	fn singleton(s: Self::Element) -> Self;
	
	fn all(&self) -> Self::All;
}



