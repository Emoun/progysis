
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
	
	const DIRECTION: AnalysisDirection;
	
	fn transfer(&Element<Self::Lattice>, &Self::Action) -> Element<Self::Lattice>;
}
