use super::*;

///
/// A [Complete Lattice].
///
/// Types implementing this trait represent the [Complete Lattice], while instances
/// of the type are element of the lattice.
///
/// The trait specifies a set of methods related to all [Complete Lattice]s.
/// Additionally, the trait requires [`PartialOrd`], meaning `<=` can be used to compare
/// two elements of a [Complete Lattice].
///
/// [Complete Lattice]: http://mathworld.wolfram.com/CompleteLattice.html
/// [`PartialOrd`]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
///
pub trait CompleteLattice: Evaluable<Value=Self> + PartialOrd + Clone
{
	///
	/// The type of the elements of the [Complete Lattice](http://mathworld.wolfram.com/CompleteLattice.html)
	///
	type Element;

	///
	/// Returns the bottom (Greatest Lower Bound) element of the
	/// [Complete Lattice](http://mathworld.wolfram.com/CompleteLattice.html).
	///
	fn bottom() -> Self;
	
	///
	/// Whether this instance is the bottom element of the [Complete Lattice](http://mathworld.wolfram.com/CompleteLattice.html).
	///
	fn is_bottom(&self) -> bool;
	
	///
	/// Joins this instance with the given element, returning the most precise (least fixed point)
	/// element in the lattice larger than both:
	///
	/// Given `j = e1.join(e2)` then `(e1 <= j) && (e2 <= j)`
	///
	///
	fn join(&self, &Evaluable<Value=Self>) -> Self;
}

impl<E,T> Evaluable for T
	where
	T: CompleteLattice<Element=E>
{
	type Value = Self;
	
	fn evaluate(&self) -> Self::Value{
		return self.clone();
	}
}