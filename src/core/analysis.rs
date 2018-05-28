
use core::{
	CompleteLattice, Element
};

pub enum AnalysisDirection{
	Forward,
	Backward,
	Both
}

pub trait Analysis<L,A>
	where L: CompleteLattice
{
	fn transfer(&Element<L>, &A) -> Element<L>;
	
	fn direction() -> AnalysisDirection;
}
