
use core::{
	CompleteLattice, Element
};

pub enum AnalysisDirection{
	Forward,
	Backward,
	Both
}

pub trait Analysis
{
	type Lattice: CompleteLattice;
	type Action;
	
	fn transfer(&Element<Self::Lattice>, &Self::Action) -> Element<Self::Lattice>;
	
	fn direction() -> AnalysisDirection;
}
