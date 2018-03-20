use super::*;

use std::fmt::Debug;

use ::core::{CompleteLattice};

trait_alias!{PowerSetItem: Clone, Eq, Debug}

///
/// A [Power Set] [Complete Lattice] over a set of items.
///
///
/// [Complete Lattice]: http://mathworld.wolfram.com/CompleteLattice.html
/// [Power Set]: http://mathworld.wolfram.com/PowerSet.html
///
pub trait PowerSet: CompleteLattice
{
	///
	/// The type of item the [Power Set](http://mathworld.wolfram.com/PowerSet.html) is over.
	///
	type Item: PowerSetItem;

	///
	/// The type returned by [`all`].
	///
	/// [`all`]: #tymethod.all
	///
	type All: IntoIterator<Item=Self::Item>;
	
	///
	/// Creates an element of the [Power Set](http://mathworld.wolfram.com/PowerSet.html)
	/// which only comprises the given item.
	///
	fn singleton(s: Self::Item) -> Self;

	///
	/// Returns the set of items in the
	/// [Power Set](http://mathworld.wolfram.com/PowerSet.html) element.
	///
	fn all(&self) -> Self::All;
}

