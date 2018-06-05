use std::ops::{
	Add, AddAssign
};
use core::Bottom;

///
/// A [Complete Lattice].
///
/// Types implementing this trait represent the [Complete Lattice], while instances
/// of the type are element of the lattice.
///
/// The trait specifies a set of methods related to all [Complete Lattice]s:
///
/// * [`Evaluable`]: Every element in the lattice can be evaluated to itself.
/// * [`PartialOrd`]: meaning `<=` can be used to compare two elements of a [Complete Lattice].
/// * [`Add`]: meaning `+` can be use to get the smallest element which is larger
/// than both the given (Least Fixed Point). I.e. if `e3 = e1 + e2` then `e3`
/// is the smallest element in the lattice where `e1 <= e3 && e2 <= e3`.
/// At least `Add<T> where T: Evaluable<Value=Self>` should be implemented.
///
/// [Complete Lattice]: http://mathworld.wolfram.com/CompleteLattice.html
/// [`Evaluable`]: trait.Evaluable.html
/// [`PartialOrd`]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
/// [`Add`]: https://doc.rust-lang.org/std/ops/trait.Add.html
///
pub trait CompleteLattice: Clone + Bottom + PartialOrd + AddAssign + Add<Output=Self>
	where
		for<'a> Self: AddAssign<&'a Self> + Add<&'a Self,Output=Self>,
{
	///
	/// Whether this instance is the bottom element of the
	/// [Complete Lattice](http://mathworld.wolfram.com/CompleteLattice.html).
	///
	fn is_bottom(&self) -> bool;
	
	///
	/// Whether this instance is comparable to the given.
	///
	/// I.e either `self <= other` or `self > other` holds.
	///
	fn comparable_to(&self, other: &Self) -> bool
	{
		self.le(&other) || self.gt(&other)
	}
}
