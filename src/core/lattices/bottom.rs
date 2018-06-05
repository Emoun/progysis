
pub trait Bottom
{
	///
	/// Returns the bottom (Greatest Lower Bound) element of the
	/// [Complete Lattice](http://mathworld.wolfram.com/CompleteLattice.html).
	///
	fn bottom() -> Self;
}

pub fn bottom<B: Bottom>() -> B
{
	B::bottom()
}