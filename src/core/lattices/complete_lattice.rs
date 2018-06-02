
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
pub trait CompleteLattice: PartialOrd + Clone
{
	///
	/// Returns the bottom (Greatest Lower Bound) element of the
	/// [Complete Lattice](http://mathworld.wolfram.com/CompleteLattice.html).
	///
	fn bottom() -> Self;
	
	///
	/// Whether this instance is the bottom element of the
	/// [Complete Lattice](http://mathworld.wolfram.com/CompleteLattice.html).
	///
	fn is_bottom(&self) -> bool;
	
	fn join(&mut self, other:&Self);
	
	///
	/// Used by &Element::Add
	///
	fn join_new(&self, other: &Self) -> Self
	{
		let mut result = Self::bottom();
		result.join(self);
		result.join(other);
		result
	}
	
	///
	/// Used by Element::AddAssign
	///
	fn add_assign(&mut self, other:Self)
	{
		self.join(&other);
	}
	
	///
	/// Used by Element::Add
	///
	fn add(mut self, other:Self) -> Self
	{
		self.join(&other);
		self
	}
	
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

