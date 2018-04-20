
use ::core::{CompleteLattice};

impl CompleteLattice for u32
{
	fn bottom() -> Self
	{
		0
	}
	
	fn is_bottom(&self) -> bool
	{
		*self == 0
	}
	
	fn join(&mut self, other: &Self)
	{
		if *self < *other {
			*self = *other;
		}
	}
}