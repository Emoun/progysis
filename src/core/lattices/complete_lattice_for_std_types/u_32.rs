
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
	
}

impl CompleteLattice for u64
{
	fn bottom() -> Self
	{
		0
	}
	
	fn is_bottom(&self) -> bool
	{
		*self == 0
	}
	
}