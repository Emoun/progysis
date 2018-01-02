use super::*;

use std::ops::Add;

///
/// A [Complete Lattice].
///
/// Types implementing this trait represent the [Complete Lattice], while instances
/// of the type are element of the lattice.
///
/// The trait specifies a set of methods related to all [Complete Lattice]s:
/// - [`Evaluable`]: Every element in the lattice can be evaluated to itself.
/// - [`PartialOrd`]: meaning `<=` can be used to compare two elements of a [Complete Lattice].
/// - [`Add`]: meaning `+` can be use to get the smallest element which is larger
/// than both the given (Least Fixed Point). I.e. if `e3 = e1 + e2` then `e3`
/// is the smallest element in the lattice where `e1 <= e3 && e2 <= e3`.
/// At least `Add<T> where T: Evaluable<Value=Self>` should be implemented.
///
/// [Complete Lattice]: http://mathworld.wolfram.com/CompleteLattice.html
/// [`Evaluable`]: trait.Evaluable.html
/// [`PartialOrd`]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
/// [`Add`]: https://doc.rust-lang.org/std/ops/trait.Add.html
///
pub trait CompleteLattice: Evaluable<Value=Self> + PartialOrd + Add<Output=Self> + Clone
{
	///
	/// Returns the bottom (Greatest Lower Bound) element of the
	/// [Complete Lattice](http://mathworld.wolfram.com/CompleteLattice.html).
	///
	fn bottom() -> Self;
	
	///
	/// Whether this instance is the bottom element of the [Complete Lattice](http://mathworld.wolfram.com/CompleteLattice.html).
	///
	fn is_bottom(&self) -> bool;
}

impl<T> Evaluable for T
	where
	T: CompleteLattice
{
	type Value = Self;
	
	fn evaluate(&self) -> Self::Value{
		return self.clone();
	}
}