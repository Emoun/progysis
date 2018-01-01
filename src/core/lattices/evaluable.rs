use super::*;

///
/// Evaluates to a [`CompleteLattice`] element.
///
/// This trait is useful for types that are not themselves elements of [`CompleteLattice`],
/// but can be evaluated to one.
///
/// [`CompleteLattice`]: trait.CompleteLattice.html
///
pub trait Evaluable
{
	///
	/// The [`CompleteLattice`] type of the element this evaluates to.
	///
	type Value: CompleteLattice;

	///
	/// Evaluates a [`CompleteLattice`] element based on the Self.
	///
	fn evaluate(&self) -> Self::Value;
}

